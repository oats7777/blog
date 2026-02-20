---
title: '렌더 루프를 Rust로: Rc<RefCell> + Closure로 requestAnimationFrame 소유하기'
description: 'JS에서 폴링하던 렌더 루프를 Rust가 직접 소유하도록 전환한 과정. CanvasInner/Canvas 래퍼 패턴, Closure 수명 관리, RefCell 빌림 안전성까지.'
date: '2026-02-20'
category: 'WebAssembly'
tags: ['Rust', 'WebAssembly', 'Canvas API', 'Architecture', 'requestAnimationFrame']
readTime: '15분 읽기'
draft: false
---

## 들어가며

[이전 글](/posts/rust-canvas-vector)에서 벡터 에디터로 확장을 마쳤다. 줌/팬, 도형 도구, SVG 내보내기까지 달았다. 그런데 코드를 다시 보니 한 가지 거슬리는 구조가 있었다:

```typescript
// JavaScript
function renderLoop() {
  canvas.render_if_needed();
  requestAnimationFrame(renderLoop);
}
requestAnimationFrame(renderLoop);
```

렌더링 타이밍을 JS가 결정하고 있다. Rust는 "렌더할 필요 있어요"라고 `needs_render` 플래그를 세울 뿐, **언제 렌더할지는 JS가 매 프레임 폴링해서 결정**한다.

지금은 충분하다. 하지만 이걸 피그마 같은 에디터로 키우려면 — 프레임 버짓팅, 더티 리전 렌더링, 줌 애니메이션, 레이어 컴포지팅 — 렌더 루프 안에서 프레임 단위 의사결정이 필요하다. 그 결정권이 JS에 있으면 매번 WASM 경계를 넘어야 한다.

> 렌더 루프를 Rust가 소유하면 어떨까?

이게 이 글의 출발점이다.

---

## 1. 문제: "누가 렌더 루프를 소유하는가?"

### 기존 구조: dirty flag + JS rAF

```
JS requestAnimationFrame 루프 (60fps)
  ↓
canvas.render_if_needed()  ← WASM 경계 crossing
  ↓
Rust: if needs_render { render(); }
```

이 구조의 특징:

- **JS가 스케줄링**, Rust가 판단. 책임이 분산되어 있다.
- `render_if_needed()`를 호출하는 건 JS. Rust는 수동적이다.
- 고급 렌더링 로직(프레임 스킵, 우선순위 렌더링)을 추가하려면 JS와 Rust 양쪽을 모두 수정해야 한다.

### 목표 구조: Rust 소유 rAF

```
Rust Closure (rAF 콜백)
  ↓
Rc<RefCell<CanvasInner>>.borrow_mut()
  ↓
if needs_render { render(); }
  ↓
request_animation_frame(self)  ← 다음 프레임 예약
```

Rust가 루프 전체를 소유한다. JS는 `canvas.start_render_loop()` 한 번 호출하면 끝. 이후 렌더링에 관한 모든 결정은 Rust 안에서 이뤄진다.

---

## 2. 핵심 도전: "Rust에서 rAF를 어떻게 돌리나?"

`requestAnimationFrame`은 브라우저 API다. 콜백 함수를 등록하면, 다음 프레임에 브라우저가 호출해준다. JavaScript에서는 간단하다:

```javascript
function loop() {
  doSomething();
  requestAnimationFrame(loop);  // 자기 자신을 다시 등록
}
```

Rust에서 이걸 하려면 세 가지 문제를 풀어야 한다:

1. **클로저가 자기 자신을 참조해야 한다** — 다음 프레임을 예약하려면 클로저 안에서 자기 자신을 `request_animation_frame`에 넘겨야 한다. Rust의 소유권 시스템에서 이건 직접적으로 불가능하다.

2. **클로저가 Canvas 상태에 접근해야 한다** — 렌더링하려면 `Canvas`의 `needs_render`, `ctx`, `elements` 등에 접근해야 한다. 그런데 `Canvas`는 `#[wasm_bindgen]`으로 JS에 노출되어 있어서, JS 이벤트 핸들러도 동시에 접근한다.

3. **클로저의 수명이 영구적이어야 한다** — 렌더 루프는 페이지가 살아 있는 동안 계속 돌아야 한다. `Closure`가 Drop되면 콜백이 사라진다.

### 해법: `Rc<RefCell<Option<Closure>>>`

자기 참조 문제는 Rust WASM 커뮤니티에서 표준 패턴이 있다:

```rust
let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
let g = f.clone();

*g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
    // 렌더링 로직
    request_animation_frame(f.borrow().as_ref().unwrap());  // f는 자기 자신
}) as Box<dyn FnMut()>));

request_animation_frame(g.borrow().as_ref().unwrap());  // 첫 프레임 시작
```

처음 보면 머리가 아프다. 하나씩 풀어보자:

1. `f`는 `Rc<RefCell<Option<Closure>>>`다. 클로저를 감싸는 상자의 상자의 상자.
2. `g`는 `f`의 클론. `Rc`니까 같은 데이터를 가리킨다.
3. 클로저를 만들 때 `f`를 `move`로 캡처한다. 클로저 안에서 `f`를 통해 **자기 자신**에 접근할 수 있다.
4. 만들어진 클로저를 `g`를 통해 `Option` 안에 넣는다.
5. `g`로 첫 프레임을 등록한다.

`f`와 클로저 사이에 **Rc 순환 참조**가 생긴다. 보통 순환 참조는 메모리 누수라서 피해야 하지만, 여기서는 의도적이다. 렌더 루프는 페이지 수명 동안 영구적으로 돌아야 하니까, "누수"가 아니라 "영구 보존"이다.

---

## 3. 아키텍처 분리: CanvasInner + Canvas 래퍼

클로저가 Canvas 상태에 접근하는 문제. `#[wasm_bindgen]`으로 JS에 노출된 `Canvas` 구조체는 JS가 `&mut self`로 접근한다. rAF 클로저도 같은 상태에 접근해야 한다. 두 곳에서 동시에 `&mut` 빌림? Rust 컴파일러가 허락하지 않는다.

해법: **interior mutability**. `RefCell`로 런타임 빌림 체크로 전환한다.

### 구조

```
                    ┌─────────────────────────────┐
 JS 이벤트 핸들러 → │  Canvas (래퍼, #[wasm_bindgen]) │
                    │  inner: Rc<RefCell<CanvasInner>> │
                    └─────────────┬───────────────┘
                                  │ borrow_mut()
                                  ▼
                    ┌─────────────────────────────┐
                    │  CanvasInner (모든 상태+로직) │
                    │  elements, ctx, zoom, ...   │
                    └─────────────┬───────────────┘
                                  ▲ try_borrow_mut()
                    ┌─────────────┴───────────────┐
                    │  rAF Closure                 │
                    │  (Rc 순환 참조로 영구 유지)     │
                    └─────────────────────────────┘
```

기존 `Canvas`를 `CanvasInner`로 이름을 바꾸고, 새 `Canvas`는 `Rc<RefCell<CanvasInner>>`를 감싸는 얇은 래퍼다:

```rust
/// 모든 상태와 로직 (JS에 직접 노출되지 않음)
pub(crate) struct CanvasInner {
    ctx: CanvasRenderingContext2d,
    elements: Vec<Element>,
    needs_render: bool,
    // ... 42개 필드 그대로
}

/// JS에 노출되는 래퍼
#[wasm_bindgen]
pub struct Canvas {
    inner: Rc<RefCell<CanvasInner>>,
    loop_running: Cell<bool>,
}
```

### 래퍼의 역할: 위임만 한다

래퍼의 모든 메서드는 `&self`를 받고, 내부적으로 `borrow()` 또는 `borrow_mut()`로 `CanvasInner`에 위임한다:

```rust
#[wasm_bindgen]
impl Canvas {
    #[wasm_bindgen]
    pub fn set_color(&self, color: &str) {
        self.inner.borrow_mut().set_color(color);
    }

    #[wasm_bindgen]
    pub fn get_is_drawing(&self) -> bool {
        self.inner.borrow().is_drawing
    }

    #[wasm_bindgen]
    pub fn export_svg(&self) -> String {
        self.inner.borrow().export_svg()
    }
}
```

53개 메서드를 전부 이 패턴으로 위임한다. 지루하지만 기계적인 작업이다. 중요한 건 **모든 래퍼 메서드가 `&self`**라는 것이다. `&mut self`가 아니다. 뮤터블 접근은 `RefCell`이 런타임에 관리한다.

왜 `&self`로 통일하는가? `#[wasm_bindgen]`이 `&mut self` 메서드를 생성하면, JS 측에서 해당 객체에 대한 exclusive reference를 잡는다. 그런데 rAF 클로저도 같은 객체의 내부 상태에 접근해야 하니, 외부적으로는 `&self`(공유 참조)만 노출하고 내부적으로 `RefCell`로 관리하는 게 맞다.

---

## 4. 렌더 루프 구현

모든 재료가 준비됐다. 조합하면:

```rust
fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn start_render_loop(inner: Rc<RefCell<CanvasInner>>) {
    let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        match inner.try_borrow_mut() {
            Ok(mut state) => {
                if state.needs_render {
                    state.needs_render = false;
                    state.render();
                }
            }
            Err(_) => {
                web_sys::console::warn_1(
                    &"render loop: CanvasInner already borrowed".into(),
                );
            }
        }
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
}
```

### try_borrow_mut — 방어적 빌림

rAF 콜백에서 `borrow_mut()` 대신 `try_borrow_mut()`를 쓰는 이유.

JS는 싱글스레드다. rAF 콜백은 이벤트 핸들러 실행이 끝난 후에만 호출된다. 이론적으로 `borrow_mut()`가 실패할 일이 없다. 하지만 "이론적으로"라는 단어를 믿고 패닉을 허용하면, 나중에 코드가 복잡해졌을 때 디버깅 지옥에 빠진다.

`try_borrow_mut()`는 빌림이 실패하면 `Err`을 반환한다. 패닉 대신 경고 로그를 남기고 다음 프레임으로 넘어간다. 실제로 이 경고가 뜨면 아키텍처에 문제가 있다는 신호다.

### loop_running 가드

`start_render_loop()`를 두 번 호출하면 두 개의 rAF 루프가 동시에 돈다. 쓸데없이 `render()`가 두 번 호출될 수 있다. `Cell<bool>`로 간단히 방지한다:

```rust
#[wasm_bindgen]
pub struct Canvas {
    inner: Rc<RefCell<CanvasInner>>,
    loop_running: Cell<bool>,
}

#[wasm_bindgen]
impl Canvas {
    pub fn start_render_loop(&self) {
        if self.loop_running.get() { return; }
        self.loop_running.set(true);
        start_render_loop(self.inner.clone());
    }
}
```

`Cell<bool>`을 쓰는 이유: `Canvas`의 메서드가 `&self`이므로 `bool` 필드를 직접 변경할 수 없다. `Cell`은 `Copy` 타입에 대해 interior mutability를 제공한다. `RefCell`보다 가볍고, `bool` 하나에는 이게 적절하다.

---

## 5. 보조 파일 수정

`rendering.rs`, `selection.rs`, `svg_export.rs`는 모두 `impl Canvas` 블록을 가지고 있었다. `Canvas` → `CanvasInner`로 이름만 바꾸면 된다:

```rust
// rendering.rs — Before
use crate::{Canvas, ToolMode};
impl Canvas {
    pub(crate) fn render(&self) { /* ... */ }
}

// rendering.rs — After
use crate::{CanvasInner, ToolMode};
impl CanvasInner {
    pub(crate) fn render(&self) { /* ... */ }
}
```

로직은 한 글자도 바뀌지 않는다. `CanvasInner`의 내부 구조가 동일하니까.

---

## 6. 프론트엔드 변경

JS 쪽 변경은 놀라울 정도로 적다:

```typescript
// Before
const canvas = new Canvas('rust-canvas', dpr);
canvas.clear();
// ... 700줄의 이벤트 핸들러 ...
function renderLoop() {
  canvas.render_if_needed();
  requestAnimationFrame(renderLoop);
}
requestAnimationFrame(renderLoop);

// After
const canvas = new Canvas('rust-canvas', dpr);
canvas.start_render_loop();  // 이 한 줄 추가
canvas.clear();
// ... 700줄의 이벤트 핸들러 (동일) ...
// renderLoop 함수 삭제
```

`render_if_needed()`는 더 이상 존재하지 않는다. `start_render_loop()` 한 줄이 대체한다. 나머지 700줄의 이벤트 핸들러 코드는 한 글자도 바뀌지 않는다 — 래퍼가 동일한 API를 노출하니까.

---

## 회고: 복잡성의 정당화

솔직히 말하면, 이 리팩토링의 실질적 이점은 **지금 당장은** 없다. JS rAF 4줄이 하던 일을 Rust에서 30줄로 하고 있다. WASM 바이너리도 4KB 커졌다 (87KB → 91KB).

그럼 왜 했는가?

**피그마 같은 에디터를 만들겠다는 방향성** 때문이다. 렌더 루프가 Rust 안에 있으면:

- **프레임 버짓팅** — 16ms 안에 못 끝나면 렌더링을 분할하는 로직을 Rust에서 작성할 수 있다
- **더티 리전** — 변경된 영역만 다시 그리는 최적화를 렌더 루프 안에서 결정할 수 있다
- **애니메이션** — 줌/팬 이징, 스냅 애니메이션의 타이밍을 프레임 단위로 제어할 수 있다
- **WebGL 전환** — 나중에 Canvas 2D → WebGL로 렌더러를 바꿀 때, 렌더 파이프라인이 이미 Rust에 있으면 마이그레이션이 수월하다

그리고 학습 가치. `Rc<RefCell<T>>` + `Closure` 조합은 Rust WASM에서 게임 루프, 비동기 콜백, 애니메이션 등에 반복적으로 쓰이는 패턴이다. 한번 익혀두면 다음 프로젝트에서 바로 쓸 수 있다.

### 컴파일 타임 안전성 vs 런타임 안전성

이 리팩토링에서 하나를 잃었다. 기존에는 `#[wasm_bindgen]`의 `&mut self`가 **컴파일 타임**에 exclusive access를 보장했다. 이제는 `RefCell`의 `borrow_mut()`가 **런타임**에 체크한다. 컴파일러가 잡아주던 버그를 런타임 패닉이 잡게 된 것이다.

이건 트레이드오프다. 컴파일 타임 안전성을 일부 포기하는 대신, 렌더 루프 소유권을 얻었다. JS 싱글스레드 환경에서 실제로 `BorrowMutError`가 발생할 가능성은 0에 가깝지만, `try_borrow_mut()` 같은 방어 코드를 습관적으로 넣는 것은 나쁘지 않다.

### 최종 구조

```
src/
├── lib.rs          # CanvasInner(상태+로직) + Canvas(래퍼) + rAF 인프라
├── models.rs       # Point, Style, Shape, Element, BoundingBox
├── rendering.rs    # 카메라 변환 렌더링 파이프라인
├── selection.rs    # 선택 하이라이트, 러버밴드
└── svg_export.rs   # SVG 문자열 생성
```

이전 글에서 "데이터 모델을 제대로 설계하면 기능 추가가 쉬워진다"고 했다. 이번에 배운 건 한 단계 위의 이야기다:

> **소유권 구조를 제대로 설계하면, 나중에 아키텍처를 바꿀 수 있다.**

데이터 모델이 "무엇을 저장하느냐"의 문제라면, 소유권 구조는 "누가 언제 접근하느냐"의 문제다. `Rc<RefCell>`은 그 "누가"를 런타임으로 미루는 도구이고, `Closure`는 "언제"를 브라우저 이벤트 루프에 맡기는 도구다.

> 완성된 그림판은 [Toys 페이지](/toys/rust-canvas)에서 직접 사용해볼 수 있다.
