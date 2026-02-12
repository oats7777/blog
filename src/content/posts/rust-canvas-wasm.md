---
title: 'Rust + WASM으로 브라우저 그림판 만들기: 아키텍처 설계부터 리팩토링까지'
description: 'Canvas API의 Immediate Mode 한계를 Retained Mode로 극복하고, Figma의 렌더링 아키텍처와 비교하며 배운 것들'
date: '2026-02-12'
category: 'WebAssembly'
tags: ['Rust', 'WebAssembly', 'Canvas API', 'Architecture']
readTime: '25분 읽기'
draft: false
---

## 들어가며

솔직히 말하면, Rust를 써보고 싶었다. 그게 시작이었다.

Canvas 2D API를 다루는 데 Rust가 반드시 필요한 건 아니다. JavaScript만으로도 충분히 만들 수 있다. 하지만 "Rust로 뭔가 만들어보고 싶다"는 단순한 호기심이 있었고, WebAssembly를 통해 브라우저에서 Rust를 돌릴 수 있다는 걸 알게 되면서 그림판이라는 주제가 딱 맞아떨어졌다.

그런데 막상 만들기 시작하니, 예상하지 못한 질문과 마주쳤다.

> "명령을 즉시 실행할 것인가, 데이터를 저장해두고 나중에 그릴 것인가?"

이건 단순한 구현 선택이 아니라, **Immediate Mode**와 **Retained Mode**라는 그래픽 아키텍처의 근본적인 갈림길이었다. 그리고 이 갈림길에서 어떤 길을 택하느냐에 따라 애플리케이션의 확장성이 완전히 달라진다는 걸, 직접 만들어보면서 알게 되었다.

이 글에서는 브라우저 그림판을 처음부터 만들면서 겪은 아키텍처 전환 과정, 그 과정에서 알게 된 Figma의 렌더링 설계, 그리고 WASM 바인딩 패턴과 Rust 모듈 리팩토링까지의 전체 여정을 다룬다.

---

## 1. 두 가지 렌더링 모드

그림판 구현에 앞서, 그래픽 프로그래밍의 두 가지 근본적인 접근법을 짚고 넘어가자. 이 개념은 Canvas 2D뿐 아니라 게임 엔진, GUI 프레임워크, 디자인 툴 전반에 걸쳐 등장한다.

### Immediate Mode — "지금 당장 그려라"

Immediate Mode는 말 그대로 **즉시 실행**이다. "여기서부터 저기까지 선을 그어라"라고 명령하면, 시스템이 곧바로 화면에 픽셀을 찍는다. 명령이 끝나면 시스템은 그 명령을 잊어버린다.

```
사용자 입력 → 즉시 화면에 그리기 → (끝, 아무것도 기억하지 않음)
```

Canvas 2D API가 대표적인 Immediate Mode API다:

```javascript
ctx.beginPath();
ctx.moveTo(100, 100);
ctx.lineTo(200, 200);
ctx.stroke(); // 이 순간 화면에 그려지고, 명령은 사라진다
```

이 코드가 실행된 후, Canvas는 "100,100에서 200,200으로 선을 그었다"는 사실을 기억하지 않는다. 화면에 픽셀만 남아있을 뿐이다.

**장점**: 구현이 단순하고, 상태 관리가 필요 없다.
**단점**: 이미 그린 것을 수정하거나, 전체를 다시 구성할 수 없다.

게임 엔진의 디버그 UI인 [Dear ImGui](https://github.com/ocornut/imgui)가 대표적인 Immediate Mode GUI다. 매 프레임 "이 버튼을 그려라, 이 텍스트를 그려라"라고 명령하고, 프레임이 끝나면 모든 UI 상태를 잊는다. 도구 UI처럼 매 프레임 새로 구성해도 문제없는 경우에 적합하다.

### Retained Mode — "데이터를 보존하고, 필요할 때 그려라"

Retained Mode는 **데이터를 먼저 저장**하고, 그 데이터를 기반으로 렌더링한다. 시스템이 "무엇을 그려야 하는지" 항상 알고 있다.

```
사용자 입력 → 데이터 저장 → 데이터를 기반으로 렌더링
```

HTML DOM이 대표적인 Retained Mode 시스템이다:

```html
<div id="box" style="color: red">Hello</div>
```

브라우저는 이 DOM 노드를 메모리에 보유하고 있다. `box.style.color = 'blue'`로 바꾸면, 브라우저가 알아서 화면을 다시 그린다. 개발자는 "무엇을 보여줄지"만 선언하고, "어떻게 그릴지"는 시스템에 맡긴다.

**장점**: 데이터를 수정하면 화면이 자동으로 반영된다. Undo, 저장, 레이어 같은 기능 확장이 자연스럽다.
**단점**: 데이터 구조 설계와 상태 관리가 필요하다.

> **핵심 차이**: Immediate Mode는 "어떻게 그릴지"를 명령하고, Retained Mode는 "무엇을 보여줄지"를 선언한다.

---

## 2. 첫 번째 구현: Immediate Mode

### Canvas 2D와 Immediate Mode

Canvas 2D API는 태생적으로 Immediate Mode다. 나의 첫 번째 그림판도 이 방식을 따랐다.

실제 Rust 코드에서 Immediate Mode는 이런 형태였다:

```rust
pub fn start_drawing(&mut self, x: f64, y: f64) {
    self.is_drawing = true;
    self.ctx.begin_path();
    self.ctx.move_to(x, y);
}

pub fn draw(&mut self, x: f64, y: f64) {
    if !self.is_drawing { return; }

    self.ctx.set_stroke_style_str(&self.color);
    self.ctx.set_line_width(self.line_width);
    self.ctx.line_to(x, y);
    self.ctx.stroke();

    // 다음 선분을 위해 새 경로 시작
    self.ctx.begin_path();
    self.ctx.move_to(x, y);
}
```

`mousemove` 이벤트가 발생할 때마다 Canvas에 직접 선을 긋는다. 빠르게 동작하고, 마우스를 움직이면 즉시 선이 나타난다. 문제 없어 보인다.

### 벽에 부딪히다

하지만 기능을 추가하려고 하면 벽에 부딪힌다.

**1. 히스토리가 없다**

화면에 찍힌 픽셀은 기억하지 않는다. 사용자가 무엇을 그렸는지, 어떤 순서로 그렸는지 알 수 없다. Undo를 구현하려면? 방법이 없다.

**2. 지우개가 "지우기"가 아니다**

Immediate Mode에서 지우개를 구현하는 유일한 방법은 **흰색으로 덧칠하기**다.

```rust
if self.is_eraser {
    self.ctx.set_stroke_style_str("#ffffff"); // 배경색으로 위장
} else {
    self.ctx.set_stroke_style_str(&self.color);
}
```

이건 진짜 지우개가 아니다. 배경이 흰색이 아니거나, 나중에 배경색을 바꾸고 싶다면? 흰색 흔적이 고스란히 남는다.

**3. 화면 재구성이 불가능하다**

캔버스를 리사이즈하거나, 저장 후 불러오거나, 레이어를 추가하거나—화면을 재구성해야 하는 모든 시나리오에서 Immediate Mode는 무력하다. 이미 지나간 명령은 어디에도 저장되어 있지 않기 때문이다.

> 이 한계는 Canvas API의 문제가 아니라, **아키텍처의 문제**다. 그리고 이건 나만 겪은 문제가 아니었다.

---

## 3. Figma는 어떻게 했을까

### Canvas 2D를 거부한 이유

그림판을 만들면서 자연스럽게 궁금해졌다—Figma 같은 프로덕션 디자인 툴은 이 문제를 어떻게 풀었을까?

Figma의 공동 창업자 Evan Wallace는 [기술 블로그](https://madebyevan.com/figma/building-a-professional-design-tool-on-the-web/)에서 이렇게 밝혔다:

> *"The 2D canvas API is an **immediate mode API** instead of a retained mode API so all geometry has to be re-uploaded to the graphics card every frame."*

Canvas 2D는 Immediate Mode API이기 때문에, 매 프레임 모든 도형 데이터를 GPU에 다시 올려야 한다. 수십 개의 스트로크를 가진 나의 그림판에서는 문제가 되지 않지만, 수천 개의 레이어를 다루는 Figma에서는 이게 치명적인 병목이었다.

그래서 Figma는 Canvas 2D를 버리고 **WebGL**을 선택했다. WebGL은 GPU 버퍼에 도형 데이터를 올려두고 유지할 수 있다. 변경된 부분만 갱신하면 되니, 매 프레임 모든 걸 다시 올리는 낭비가 없다.

### Figma의 아키텍처

Figma의 내부를 들여다보면, 데이터 모델과 렌더링이 명확하게 분리되어 있다:

| 레이어 | 나의 그림판 | Figma |
|--------|-----------|-------|
| **데이터 모델** | `Vec<Stroke>` (Rust) | C++ SceneGraph (WASM) |
| **그래픽 API** | Canvas 2D | WebGL → WebGPU |
| **렌더링 전략** | 매 프레임 전체 재렌더 | 타일 기반, 변경분만 GPU 업데이트 |
| **UI 레이어** | Astro/JS | React/TypeScript |

Figma는 C++로 작성된 SceneGraph를 Emscripten으로 WASM에 컴파일하고, 커스텀 GPU 렌더러로 화면에 그린다. Evan Wallace의 표현을 빌리면 Figma는 **"브라우저 안의 브라우저"**다. 자체 DOM, 자체 컴포지터, 자체 텍스트 레이아웃 엔진을 가지고 있다.

### 공통점과 차이점

여기서 흥미로운 발견이 있었다. 나의 그림판과 Figma는 **데이터 모델 레이어에서는 같은 철학**을 공유한다—둘 다 그리기 데이터를 메모리에 보존하는 Retained 데이터 모델을 사용한다. 하지만 **렌더링 레이어에서는 완전히 다른 전략**을 취한다.

나의 그림판은 Canvas 2D로 매 프레임 전체를 다시 그린다. 스트로크 수백 개 규모에서는 이걸로 충분하다. 하지만 Figma처럼 수천 개의 레이어를 50명이 동시에 편집하는 규모에서는, Canvas 2D의 "매 프레임 전체를 GPU에 다시 올리기"가 병목이 되어 WebGL/WebGPU가 필요해진다.

> **"Retained Mode"라는 단어가 두 가지 다른 레이어를 가리킬 수 있다는 걸 깨달았다.** 데이터를 보존한다는 의미의 Retained와, GPU 리소스를 유지한다는 의미의 Retained는 같은 단어지만 다른 층위의 이야기다.

---

## 4. 아키텍처 전환: Retained 데이터 모델

### 핵심 인사이트 — "그리기"와 "렌더링"의 분리

Figma든 나의 그림판이든, 핵심은 같다. **사용자 입력(그리기)**과 **화면 출력(렌더링)**을 분리하는 것이다.

```
[Immediate Mode]  사용자 입력 → 즉시 화면에 그리기
[Retained Mode]   사용자 입력 → 데이터 저장 → 데이터 기반 렌더링
```

나의 그림판에 이 원칙을 적용해보자.

### 데이터 모델 설계

먼저 그림의 구성 요소를 데이터로 정의한다:

```rust
// models.rs

/// 2D 좌표
#[derive(Clone, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// 하나의 스트로크 (펜을 누르고 떼기까지 그린 하나의 선)
#[derive(Clone, Serialize, Deserialize)]
pub struct Stroke {
    pub id: u32,
    pub points: Vec<Point>,
    pub color: String,
    pub width: f64,
}
```

`Point`는 캔버스 위의 한 점이고, `Stroke`는 펜을 누른 순간부터 뗄 때까지의 점들의 집합이다. 각 스트로크는 자신만의 색상과 굵기를 기억한다.

여기서 `Serialize`, `Deserialize`를 derive한 건 의미가 있다. 이 데이터 모델은 나중에 JSON으로 직렬화해서 저장/불러오기를 구현할 수 있는 기반이 된다. Figma가 C++ SceneGraph를 CRDT로 동기화하는 것처럼, 데이터 모델이 있으면 그 위에 무엇이든 쌓을 수 있다.

### Canvas 상태 구조

Canvas 구조체는 모든 상태를 보유한다:

```rust
// lib.rs

#[wasm_bindgen]
pub struct Canvas {
    ctx: CanvasRenderingContext2d,
    canvas_width: f64,
    canvas_height: f64,

    // Retained 데이터 모델: 모든 스트로크 저장
    strokes: Vec<Stroke>,
    current_stroke: Option<Stroke>,
    next_id: u32,

    // 현재 도구 상태
    color: String,
    line_width: f64,
    is_drawing: bool,
    is_eraser: bool,
}
```

핵심은 두 필드다:

- **`strokes: Vec<Stroke>`** — 완료된 모든 스트로크의 히스토리
- **`current_stroke: Option<Stroke>`** — 현재 그리고 있는 스트로크 (아직 확정되지 않은)

`Option<Stroke>`를 쓴 건 Rust답다. "지금 그리고 있을 수도 있고, 아닐 수도 있다"라는 상태를 타입 시스템으로 표현한다. null 체크 따위는 필요 없다.

### 드로잉 라이프사이클

사용자의 그리기 동작은 세 단계로 분리된다:

```rust
/// 1단계: 펜을 누른다 → 새 스트로크 생성
pub fn start_drawing(&mut self, x: f64, y: f64) {
    self.is_drawing = true;
    self.current_stroke = Some(Stroke {
        id: self.next_id,
        points: vec![Point { x, y }],
        color: if self.is_eraser {
            "#ffffff".to_string()
        } else {
            self.color.clone()
        },
        width: self.line_width,
    });
    self.next_id += 1;
}

/// 2단계: 펜을 움직인다 → 점 추가 + 렌더링
pub fn draw(&mut self, x: f64, y: f64) {
    if !self.is_drawing { return; }

    if let Some(ref mut stroke) = self.current_stroke {
        stroke.points.push(Point { x, y });
    }
    self.render(); // 전체 화면을 다시 그린다
}

/// 3단계: 펜을 뗀다 → 스트로크 확정
pub fn stop_drawing(&mut self) {
    self.is_drawing = false;
    if let Some(stroke) = self.current_stroke.take() {
        if stroke.points.len() > 1 {
            self.strokes.push(stroke); // 히스토리에 추가
        }
    }
}
```

`current_stroke.take()`는 `Option`에서 값을 꺼내면서 자리에 `None`을 남기는 메서드다. 소유권 이동이 명확하게 표현된다—현재 스트로크가 "진행 중" 상태에서 "확정된 히스토리"로 넘어가는 순간이다.

### 렌더 파이프라인

이제 핵심인 `render()` 함수:

```rust
pub fn render(&self) {
    // 1. 화면 초기화
    self.clear_canvas();

    // 2. 저장된 모든 스트로크를 처음부터 다시 그린다
    for stroke in &self.strokes {
        self.draw_stroke(stroke);
    }

    // 3. 현재 그리고 있는 스트로크 (실시간 미리보기)
    if let Some(ref current) = self.current_stroke {
        self.draw_stroke(current);
    }
}
```

매번 전체를 다시 그린다. 처음에는 "비효율적이지 않을까?" 했지만, 실제로는:

- Canvas 2D의 `clearRect` + 선 수백 개 그리기는 수 밀리초 안에 끝난다
- `mousemove`는 초당 60~120회 발생하는데, 이 정도면 16ms 안에 충분히 처리된다
- 800×500 해상도에서 스트로크 수천 개까지는 체감 지연이 없다

물론 이건 내 그림판의 규모에서 통하는 얘기다. Figma가 Canvas 2D를 버린 건, 이 "매 프레임 전체 재업로드"가 프로덕션 규모에서는 병목이 되기 때문이었다. 하지만 학습 프로젝트와 소규모 도구에서는, Canvas 2D + 전체 재렌더가 가장 단순하면서도 충분히 실용적인 선택이다.

### Immediate Mode vs Retained 데이터 모델

| 관점 | Immediate Mode | Retained 데이터 모델 |
|------|---------------|---------------------|
| **데이터 보존** | 없음 | 전체 히스토리 저장 |
| **Undo/Redo** | 불가능 | `strokes.pop()` 한 줄 |
| **저장/불러오기** | 불가능 | 직렬화 가능 |
| **화면 재구성** | 불가능 | `render()` 호출 |
| **렌더링 비용** | 변경분만 | 매 프레임 전체 (Canvas 2D 기준) |
| **코드 복잡도** | 낮음 | 중간 |
| **확장성** | 낮음 | 높음 (레이어, 변환 등) |

데이터를 보존하는 것만으로 이렇게 많은 것을 얻는다. Figma, Photoshop, Excalidraw, tldraw—내가 찾아본 모든 프로덕션 그래픽 에디터는 어떤 형태로든 Retained 데이터 모델을 채택하고 있었다. 렌더링 방식은 Canvas 2D, WebGL, SVG, HTML/CSS 등 천차만별이지만, **"그리기 데이터를 메모리에 보존한다"는 원칙은 동일**했다.

---

## 5. WASM 바인딩 패턴

### Rust 구조체를 JavaScript 클래스로 노출하기

`wasm-bindgen`은 Rust 구조체에 `#[wasm_bindgen]`을 붙이면 JavaScript에서 `new Canvas()`로 인스턴스를 생성할 수 있게 해준다:

```rust
#[wasm_bindgen]
pub struct Canvas { /* ... */ }

#[wasm_bindgen]
impl Canvas {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str, dpr: f64) -> Result<Canvas, JsValue> {
        // DOM에서 Canvas 요소를 찾고, 2D Context를 얻는다
        let document = web_sys::window()
            .ok_or("No window")?
            .document()
            .ok_or("No document")?;

        let canvas = document
            .get_element_by_id(canvas_id)
            .ok_or("Canvas not found")?
            .dyn_into::<HtmlCanvasElement>()?;

        let ctx = canvas
            .get_context("2d")?
            .ok_or("No 2d context")?
            .dyn_into::<CanvasRenderingContext2d>()?;

        // ...
    }
}
```

JavaScript에서는 이렇게 쓴다:

```typescript
import init, { Canvas } from './rust_canvas.js';

await init(); // WASM 모듈 로드
const canvas = new Canvas('my-canvas', window.devicePixelRatio);
```

마치 일반 JavaScript 클래스를 쓰는 것처럼 자연스럽다. 하지만 내부에서 일어나는 일은 전혀 다르다—메모리 관리는 Rust가, 실행은 WASM이 담당한다. Figma가 C++ + Emscripten으로 한 것을, 나는 Rust + wasm-bindgen으로 한 셈이다.

### web-sys로 Canvas API 호출하기

Rust에서 브라우저 API를 쓰려면 `web-sys` 크레이트가 필요하다. 이 크레이트는 Web API의 Rust 바인딩을 제공한다.

```toml
# Cargo.toml
[dependencies.web-sys]
version = "0.3"
features = [
    "console",
    "Document",
    "Element",
    "HtmlCanvasElement",
    "CanvasRenderingContext2d",
    "MouseEvent",
    "Window",
]
```

주목할 점은 **feature flag**로 필요한 API만 선택적으로 포함한다는 것이다. WASM 바이너리의 크기를 최소화하기 위한 설계로, `opt-level = "s"`(크기 최적화)와 `lto = true`(링크 타임 최적화)를 함께 설정하면 수십 KB 수준의 바이너리가 나온다.

`web-sys`의 Canvas API 호출은 JavaScript와 거의 1:1로 대응한다:

```rust
// JavaScript: ctx.beginPath()
self.ctx.begin_path();

// JavaScript: ctx.moveTo(x, y)
self.ctx.move_to(first.x, first.y);

// JavaScript: ctx.lineTo(x, y)
self.ctx.line_to(point.x, point.y);

// JavaScript: ctx.stroke()
self.ctx.stroke();
```

차이점이 있다면, Rust에서는 메서드 이름이 snake_case이고, 일부 메서드는 `Result`를 반환한다는 것이다. 하지만 중요한 건 **타입 안전성**이다. JavaScript에서는 `ctx.setStrokeStyle(123)`처럼 잘못된 타입을 넘겨도 런타임에서야 알 수 있지만, Rust에서는 컴파일 시점에 잡아준다.

### JS ↔ WASM 경계: 좌표계 변환과 DPR

Canvas 그림판에서 가장 까다로운 부분 중 하나는 **좌표계 변환**이다. 세 가지 좌표계가 공존한다:

```
[브라우저 화면]         [CSS 논리 좌표]         [Canvas 물리 픽셀]
clientX, clientY  →   논리 800×500px    →    물리 1600×1000px (DPR 2)
```

Astro 컴포넌트에서 좌표를 변환하는 코드:

```typescript
const LOGICAL_WIDTH = 800;
const LOGICAL_HEIGHT = 500;

function getPosition(e: MouseEvent | TouchEvent, canvasEl: HTMLCanvasElement): Position {
    const rect = canvasEl.getBoundingClientRect();
    const scaleX = LOGICAL_WIDTH / rect.width;
    const scaleY = LOGICAL_HEIGHT / rect.height;

    const mouseEvent = e as MouseEvent;
    return {
        x: (mouseEvent.clientX - rect.left) * scaleX,
        y: (mouseEvent.clientY - rect.top) * scaleY,
    };
}
```

Rust 생성자에서는 DPR만큼 Context를 스케일한다:

```rust
ctx.scale(dpr, dpr).ok();
```

이렇게 하면 Rust 쪽에서는 항상 논리 좌표(800×500)로 작업하면서, 레티나 디스플레이에서도 선명한 렌더링이 가능하다.

---

## 6. 커서 미리보기 시스템

### 문제: "내가 어디에 그리고 있는지 모르겠다"

그림판을 실제로 사용해 보면 금방 느낀다—펜이든 지우개든, **현재 도구의 영향 범위**를 모르면 정밀한 작업이 어렵다. 특히 굵기를 크게 설정했을 때, 기본 마우스 커서로는 어디까지 칠해질지 감을 잡기 힘들다.

### 설계: 렌더 파이프라인에 커서 레이어 추가

Retained 데이터 모델의 장점이 여기서 빛난다. 기존 `render()` 함수에 한 레이어만 추가하면 된다:

```rust
pub fn render(&self) {
    self.clear_canvas();

    for stroke in &self.strokes {
        self.draw_stroke(stroke);
    }

    if let Some(ref current) = self.current_stroke {
        self.draw_stroke(current);
    }

    // 커서 미리보기 — 새로 추가된 레이어
    if self.show_cursor {
        self.draw_cursor_preview();
    }
}
```

매 프레임 전체를 다시 그리기 때문에, 커서가 이전 위치에 "잔상"을 남기는 걱정이 없다. Immediate Mode였다면 이전 커서 위치의 픽셀을 복원해야 하는 골치 아픈 문제가 생겼을 것이다.

참고로, Excalidraw는 이 문제를 **듀얼 캔버스**로 풀었다. 스트로크를 그리는 StaticCanvas와 커서/선택을 그리는 InteractiveCanvas를 분리해서, 커서가 움직일 때 스트로크 전체를 다시 그릴 필요가 없게 했다. 규모가 커지면 이런 최적화가 필요해진다.

### 구현: 모드별 시각적 차별화

커서 미리보기는 도구에 따라 다른 시각적 피드백을 제공한다:

```rust
fn draw_cursor_preview(&self) {
    let radius = self.line_width / 2.0;

    self.ctx.save(); // 현재 Canvas 상태 저장

    if self.is_eraser {
        // 지우개: 회색 점선 원
        let dash_pattern = js_sys::Array::new();
        dash_pattern.push(&JsValue::from_f64(4.0));
        dash_pattern.push(&JsValue::from_f64(4.0));
        let _ = self.ctx.set_line_dash(&dash_pattern);
        self.ctx.set_stroke_style_str("#999999");
    } else {
        // 펜: 선택한 색상의 실선 원
        self.ctx.set_stroke_style_str(&self.color);
    }

    self.ctx.set_line_width(1.0);
    self.ctx.begin_path();
    let _ = self.ctx.arc(
        self.cursor_x,
        self.cursor_y,
        radius,
        0.0,
        std::f64::consts::PI * 2.0,
    );
    self.ctx.stroke();

    self.ctx.restore(); // Canvas 상태 복원
}
```

여기서 `ctx.save()`와 `ctx.restore()`가 중요하다. 커서를 그리면서 변경한 Canvas 상태(선 스타일, 대시 패턴, 굵기)가 이후의 스트로크 렌더링에 영향을 주면 안 되기 때문이다. 이 패턴은 Canvas 2D에서 **임시 상태 변경**이 필요할 때 반드시 써야 하는 관용구다.

### JS 쪽 이벤트 연결

커서 미리보기가 자연스러우려면 마우스가 움직일 때마다 위치를 갱신하고 렌더링해야 한다:

```typescript
canvasEl.addEventListener('mousemove', (e) => {
    const pos = getPosition(e, canvasEl);
    canvas.update_cursor(pos.x, pos.y); // 커서 위치 갱신

    if (canvas.get_is_drawing()) {
        canvas.draw(pos.x, pos.y); // 그리는 중이면 draw() (내부에서 render() 호출)
    } else {
        canvas.render(); // 아니면 커서 프리뷰만 렌더
    }
});

canvasEl.addEventListener('mouseleave', () => {
    canvas.stop_drawing();
    canvas.hide_cursor(); // 마우스가 캔버스를 벗어나면 커서 숨김
    canvas.render();
});
```

그리고 기본 마우스 커서는 숨긴다:

```typescript
canvasEl.style.cursor = 'none';
```

이제 도구의 크기와 색상이 실시간으로 마우스를 따라다닌다.

---

## 7. Rust 모듈 리팩토링

### 단일 파일의 한계

처음에는 모든 코드가 `lib.rs` 하나에 있었다. 기능이 추가되면서 268줄까지 늘어났고, 파일을 열 때마다 "렌더링 코드는 어디였지?" 하고 스크롤해야 했다.

### `#[wasm_bindgen]` impl 블록의 분산

Rust에서는 같은 구조체에 대한 `impl` 블록을 **여러 파일에 나누어** 정의할 수 있다. `wasm-bindgen`도 이를 지원한다:

```
src/
├── lib.rs          # Canvas 구조체 정의, 생성자, 도구/드로잉 API
├── models.rs       # Point, Stroke 데이터 모델
└── rendering.rs    # 렌더링 엔진 (render, draw_stroke, draw_cursor_preview)
```

핵심은 Canvas 구조체의 필드를 `pub(crate)`로 선언하는 것이다:

```rust
// lib.rs
#[wasm_bindgen]
pub struct Canvas {
    pub(crate) ctx: CanvasRenderingContext2d,
    pub(crate) strokes: Vec<Stroke>,
    pub(crate) current_stroke: Option<Stroke>,
    // ...
}
```

`pub(crate)`는 "같은 크레이트 내에서만 접근 가능"하다는 의미다. 외부(JavaScript)에서는 이 필드에 접근할 수 없지만, `rendering.rs`에서는 자유롭게 사용할 수 있다.

`rendering.rs`에서는 두 개의 `impl` 블록을 정의한다:

```rust
// rendering.rs
use crate::Canvas;

// wasm_bindgen이 필요 없는 내부 메서드
impl Canvas {
    pub(crate) fn clear_canvas(&self) { /* ... */ }
    pub(crate) fn draw_stroke(&self, stroke: &Stroke) { /* ... */ }
    pub(crate) fn draw_cursor_preview(&self) { /* ... */ }
}

// JavaScript에 노출되는 메서드
#[wasm_bindgen]
impl Canvas {
    #[wasm_bindgen]
    pub fn render(&self) { /* ... */ }
}
```

내부 전용 메서드(`clear_canvas`, `draw_stroke`, `draw_cursor_preview`)는 `#[wasm_bindgen]` 없이 `pub(crate)`로 선언하고, JavaScript에서 호출해야 하는 `render()`만 `#[wasm_bindgen]`을 붙인다.

이렇게 하면 WASM 바이너리에 불필요한 바인딩 코드가 추가되지 않으면서도, Rust 내부에서는 모듈 간 자유로운 호출이 가능하다.

---

## 8. 회고

### Retained 데이터 모델이 열어주는 가능성

지금은 스트로크를 `Vec<Stroke>`에 저장하기만 하지만, 이 데이터 구조를 기반으로:

- **Undo/Redo**: `strokes`에서 마지막 요소를 빼고(`pop`) 별도의 redo 스택에 넣으면 된다
- **저장/불러오기**: `Stroke`가 이미 `Serialize`를 derive하고 있으니 `serde_json::to_string()`이면 끝이다
- **레이어**: `Vec<Vec<Stroke>>`로 확장하면 레이어별 독립 관리가 가능하다
- **협업 편집**: 각 스트로크에 사용자 ID를 추가하면, Figma가 CRDT로 한 것의 간소화된 버전을 만들 수 있다

처음 아키텍처 결정이 이후의 모든 확장을 결정짓는다는 걸 실감했다.

### Rust가 상태 관리에 주는 안전성

`Option<Stroke>`으로 "그리는 중" 상태를 표현한 것, `take()`로 소유권을 명시적으로 이동한 것—이런 패턴들이 JavaScript였다면 런타임 버그로 이어졌을 상황을 컴파일 타임에 잡아준다.

특히 `current_stroke.take()`는 인상적이었다:

```rust
if let Some(stroke) = self.current_stroke.take() {
    self.strokes.push(stroke); // 소유권이 strokes로 이동
}
// 이 시점에서 current_stroke는 확실히 None
```

"현재 그리고 있던 스트로크를 히스토리로 옮긴다"는 의도가 코드에 그대로 드러난다. 실수로 같은 스트로크를 두 번 push하는 일은 **구조적으로 불가능**하다.

### 규모에 따라 달라지는 정답

이 프로젝트를 하면서 가장 크게 배운 건, **"정답"은 규모에 따라 다르다**는 것이다.

솔직히 말하면, 이 그림판의 성능은 JavaScript로도 충분했을 것이다. Canvas 2D의 병목은 대부분 브라우저 렌더링 엔진 쪽이지, JS 실행 속도가 아니기 때문이다.

| 규모 | 적합한 선택 |
|------|-----------|
| 학습/프로토타입 | Canvas 2D + JS, 전체 재렌더 |
| 소규모 도구 (스트로크 수천 개) | Canvas 2D + WASM, 전체 재렌더 (이 프로젝트) |
| 중규모 에디터 (요소 수만 개) | Canvas 2D + 더티 렉트/듀얼 캔버스 (Excalidraw) |
| 프로덕션 디자인 툴 | WebGL/WebGPU + 타일 렌더러 (Figma) |

하지만 모든 규모에서 공통인 건 하나다—**데이터를 보존하라**. 렌더링 전략은 규모에 맞게 바꿀 수 있지만, 데이터 없이는 아무것도 확장할 수 없다.

---

## 마무리

"그림판"이라는 단순해 보이는 프로젝트에서 아키텍처 설계(Immediate → Retained), Figma의 렌더링 철학, WASM 바인딩 패턴, 모듈 리팩토링까지—웹 그래픽 프로그래밍의 핵심 주제를 전부 경험할 수 있었다.

처음에는 "매 프레임마다 전체를 다시 그리는 건 비효율적이다"라고 생각했다. 그리고 Figma를 연구하면서 "아, 규모가 커지면 정말 비효율적이구나"라는 것도 알게 되었다. 하지만 동시에, 모든 규모의 그래픽 에디터가 데이터를 보존한다는 공통 원칙도 발견했다.

결국 중요한 건 "얼마나 자주 그리느냐"가 아니라 **"무엇을 기억하고 있느냐"**다.

---

### 참고 자료

- [Building a professional design tool on the web — Evan Wallace](https://madebyevan.com/figma/building-a-professional-design-tool-on-the-web/)
- [Figma is powered by WebAssembly — Figma Blog](https://www.figma.com/blog/webassembly-cut-figmas-load-time-by-3x/)
- [How Figma's multiplayer technology works — Figma Blog](https://www.figma.com/blog/how-figmas-multiplayer-technology-works/)
- [Retained Mode Versus Immediate Mode — Microsoft Learn](https://learn.microsoft.com/en-us/windows/win32/learnwin32/retained-mode-versus-immediate-mode)
- [Dear ImGui — About the IMGUI paradigm](https://github.com/ocornut/imgui/wiki/About-the-IMGUI-paradigm)
