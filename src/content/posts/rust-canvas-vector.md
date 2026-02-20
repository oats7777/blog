---
title: '그림판을 벡터 에디터로: 줌/팬, 도형 도구, SVG 내보내기'
description: 'Stroke에서 Element로의 데이터 모델 리팩토링, 카메라 변환 기반 줌/팬, 도형 도구 추가, SVG 내보내기까지'
date: '2026-02-19'
category: 'WebAssembly'
tags: ['Rust', 'WebAssembly', 'Canvas API', 'Vector Graphics', 'SVG']
readTime: '20분 읽기'
draft: false
---

## 들어가며

[이전 글](/posts/rust-canvas-selection)에서 선택, 이동, 복사, Undo/Redo까지 구현했다. 그때 이런 생각이 들었다:

> 이거 선만 그리는 그림판이 아니라, 도형도 그리고 확대/축소도 되는 벡터 에디터로 만들 수 있지 않을까?

Retained Mode에서 모든 스트로크를 `Vec<Stroke>`에 보관하고 있으니, 이 구조를 조금 확장하면 될 것 같았다. "조금"이 실제로는 데이터 모델 전체 리팩토링 + 좌표계 변환 시스템 도입이었지만.

이 글에서는 4단계에 걸친 변환 과정을 다룬다:

1. **데이터 모델 리팩토링** — `Stroke` → `Element` + `Shape` enum
2. **줌/팬** — 카메라 변환과 좌표계 분리
3. **도형 도구** — 직선, 사각형, 원
4. **SVG 내보내기** — 벡터 데이터를 벡터 파일로

---

## 1. Stroke에서 Element로 — "왜 구조를 바꿔야 하나?"

기존 `Stroke` 구조체는 이렇게 생겼다:

```rust
pub struct Stroke {
    pub id: u32,
    pub points: Vec<Point>,
    pub color: String,
    pub width: f64,
}
```

프리핸드 그리기에는 완벽하다. 하지만 직선, 사각형, 원을 추가하려면? `points`에 2개만 넣어서 직선을 표현한다? `width`/`height` 필드를 `Option`으로 추가한다? 쓰지 않는 필드가 계속 늘어나는 구조다.

### Shape enum이 답이다

Rust의 enum은 이런 상황에 딱 맞는 도구다. 각 변형(variant)이 자신만의 데이터를 가지고, 사용하지 않는 필드가 존재하지 않는다:

```rust
pub struct Style { pub color: String, pub width: f64 }

pub enum Shape {
    Freehand { points: Vec<Point> },
    Line { x1: f64, y1: f64, x2: f64, y2: f64 },
    Rect { x: f64, y: f64, w: f64, h: f64 },
    Circle { cx: f64, cy: f64, r: f64 },
}

pub struct Element {
    pub id: u32,
    pub shape: Shape,
    pub style: Style,
}
```

`Element`가 `Shape`(무엇을)과 `Style`(어떻게)을 분리해서 들고 있다. 새로운 도형을 추가하려면 `Shape` enum에 변형 하나만 추가하면 된다.

### match로 분기 처리

`Element`의 모든 동작은 `match`로 Shape별 분기한다. 바운딩 박스 계산을 예로 들면:

```rust
impl Element {
    pub fn bounding_box(&self) -> Option<BoundingBox> {
        let half_w = self.style.width / 2.0;
        match &self.shape {
            Shape::Freehand { points } => {
                // 모든 점의 min/max ± half_w
            }
            Shape::Line { x1, y1, x2, y2 } => {
                // 양 끝점의 min/max ± half_w
            }
            Shape::Rect { x, y, w, h } => {
                // 직접 계산
            }
            Shape::Circle { cx, cy, r } => {
                // center ± (r + half_w)
            }
        }
    }
}
```

`translate()`, `hit_test()`도 같은 패턴이다. Rust 컴파일러가 모든 Shape 변형을 빠뜨리면 에러를 내주니까, 새 도형을 추가했는데 히트 테스트를 깜빡하는 일이 원천 차단된다.

### 히트 테스트의 Shape별 차이

재미있는 건 도형마다 "위에 있다"의 정의가 다르다는 것이다:

| Shape | 히트 테스트 방식 |
|-------|----------------|
| Freehand | 모든 연속 선분 쌍에 대해 점-선분 거리 계산 (기존과 동일) |
| Line | 하나의 선분에 대해 점-선분 거리 계산 |
| Rect | 4변 각각에 대해 점-선분 거리 계산 (채우기 없는 사각형이니까) |
| Circle | 클릭 좌표와 중심 사이 거리 - 반지름의 절대값 (원주 근처인지) |

사각형의 히트 테스트가 흥미롭다. 채우기 없는(stroke-only) 사각형이라, 내부를 클릭해도 선택되면 안 된다. 4개의 변을 각각 선분으로 보고 거리를 계산한다:

```rust
Shape::Rect { x, y, w, h } => {
    let corners = [
        (Point { x: *x, y: *y }, Point { x: x + w, y: *y }),         // 상변
        (Point { x: x + w, y: *y }, Point { x: x + w, y: y + h }),   // 우변
        (Point { x: x + w, y: y + h }, Point { x: *x, y: y + h }),   // 하변
        (Point { x: *x, y: y + h }, Point { x: *x, y: *y }),         // 좌변
    ];
    for (a, b) in &corners {
        if point_to_segment_distance(&p, a, b) <= threshold {
            return true;
        }
    }
    false
}
```

원은 더 간결하다. 중심에서 클릭 좌표까지의 거리가 반지름과 비슷하면 원주 위에 있는 것이다:

```rust
Shape::Circle { cx, cy, r } => {
    let dist_from_center = ((px - cx).powi(2) + (py - cy).powi(2)).sqrt();
    (dist_from_center - r).abs() <= threshold
}
```

### 리팩토링의 핵심: 외부 인터페이스 유지

이 리팩토링에서 가장 중요한 제약은 **기존 동작이 100% 동일해야 한다**는 것이다. WASM API 시그니처(`start_drawing`, `draw`, `stop_drawing` 등)는 그대로 유지하고, 내부적으로만 `Stroke` → `Element`로 바꿨다. 프론트엔드 코드는 한 줄도 수정하지 않고 Phase 1을 완료했다.

---

## 2. 줌/팬 — "두 개의 세계가 필요하다"

줌/팬을 구현하면서 가장 먼저 깨달은 것: **좌표계가 두 개 필요하다.**

### 스크린 스페이스 vs 월드 스페이스

```
스크린 스페이스 (0,0)                    (800,0)
┌──────────────────────────────────────────┐
│                                          │
│     ┌────────────────────┐               │
│     │ 월드 스페이스       │               │
│     │ (줌/팬에 따라 이동) │               │
│     │                    │               │
│     └────────────────────┘               │
│                                          │
└──────────────────────────────────────────┘
(0,500)                                (800,500)
```

- **스크린 스페이스**: CSS 논리 좌표 (0~800, 0~500). 항상 고정.
- **월드 스페이스**: 그림이 존재하는 무한 좌표계. 줌/팬에 따라 스크린 위 어디에 보일지 달라진다.

그림의 좌표는 항상 월드 스페이스에 저장한다. 렌더링할 때만 카메라 변환을 적용해서 스크린에 그린다.

### 카메라 변환 파이프라인

Canvas 2D API의 `translate()`과 `scale()`을 이용한 변환:

```rust
pub fn render(&self) {
    // 1. 물리 캔버스 전체 클리어 (변환 없이)
    self.clear_canvas();

    // 2. 카메라 변환 적용
    self.ctx.save();
    self.ctx.translate(self.pan_x, self.pan_y);
    self.ctx.scale(self.zoom, self.zoom);

    // 월드 스페이스 콘텐츠
    for element in &self.elements { self.draw_element(element); }
    self.draw_shape_preview();
    self.draw_selection_highlight();

    // 3. 카메라 변환 해제
    self.ctx.restore();

    // 4. 스크린 스페이스 UI (변환 바깥)
    self.draw_rubber_band();
    if self.show_cursor { self.draw_cursor_preview(); }
}
```

핵심은 **무엇이 카메라 변환 안에 있고, 무엇이 밖에 있는지**다:

| 카메라 변환 안 (월드 스페이스) | 카메라 변환 밖 (스크린 스페이스) |
|------|------|
| 그려진 요소들 | 러버밴드 선택 영역 |
| 도형 프리뷰 | 커서 미리보기 |
| 선택 하이라이트 | |

러버밴드와 커서를 스크린 스페이스에 두는 이유: 줌을 200%로 해도 선택 영역의 점선 두께가 굵어지면 안 되니까.

### 좌표 변환: 스크린 → 월드

사용자가 마우스를 클릭하면 스크린 좌표를 얻는다. 그런데 그 좌표로 선을 그리려면 월드 좌표가 필요하다. 변환 공식은 간단하다:

```rust
pub fn screen_to_world_x(&self, sx: f64) -> f64 {
    (sx - self.pan_x) / self.zoom
}
```

팬 오프셋을 빼고 줌 배율로 나누면 된다. 프론트엔드에서는 이렇게 사용한다:

```typescript
/** 스크린 스페이스 좌표 */
function getPosition(e: MouseEvent, canvasEl: HTMLCanvasElement): Position {
    const rect = canvasEl.getBoundingClientRect();
    return {
        x: (e.clientX - rect.left) * (LOGICAL_WIDTH / rect.width),
        y: (e.clientY - rect.top) * (LOGICAL_HEIGHT / rect.height),
    };
}

/** 월드 스페이스 좌표 */
function getWorldPosition(e: MouseEvent, canvasEl: HTMLCanvasElement, canvas: Canvas): Position {
    const screen = getPosition(e, canvasEl);
    return {
        x: canvas.screen_to_world_x(screen.x),
        y: canvas.screen_to_world_y(screen.y),
    };
}
```

그리기, 선택, 이동 → 월드 좌표. 러버밴드, 커서, 팬 → 스크린 좌표. 이 규칙만 지키면 줌/팬 상태에서도 모든 기능이 정확하게 동작한다.

### 커서 기준 줌

줌에서 한 가지 신경 써야 할 것: **커서가 가리키는 지점이 줌 후에도 같은 곳을 가리켜야 한다.** Google Maps에서 마우스 위치 기준으로 확대되는 것처럼.

```rust
pub fn zoom_at(&mut self, screen_x: f64, screen_y: f64, delta: f64) {
    let factor = if delta < 0.0 { 1.1 } else { 1.0 / 1.1 };
    let new_zoom = (self.zoom * factor).clamp(0.1, 10.0);

    // 줌 전 커서가 가리키던 월드 좌표
    let wx = (screen_x - self.pan_x) / self.zoom;
    let wy = (screen_y - self.pan_y) / self.zoom;

    self.zoom = new_zoom;

    // 팬을 조정해서 같은 월드 좌표가 같은 스크린 좌표에 오도록
    self.pan_x = screen_x - wx * self.zoom;
    self.pan_y = screen_y - wy * self.zoom;
}
```

줌 전 `(screen_x, screen_y)` → 월드 좌표 `(wx, wy)`를 기억하고, 줌 후에 `(wx, wy)`가 다시 `(screen_x, screen_y)`에 오도록 팬을 역산한다.

### 팬: 델타 vs 오리진 방식

팬 구현에서 흔히 하는 실수: mousemove마다 `pan_x += dx`를 누적하는 것. 부동소수점 오차가 쌓여서 미세하게 드리프트한다.

대신 **오리진 방식**을 썼다. 팬 시작 시점의 마우스 위치와 팬 오프셋을 기억하고, 현재 마우스 위치와의 차이를 매번 새로 계산한다:

```rust
pub fn start_pan(&mut self, sx: f64, sy: f64) {
    self.is_panning = true;
    self.pan_start_x = sx;      // 드래그 시작 마우스 위치
    self.pan_origin_x = self.pan_x;  // 드래그 시작 시 팬 오프셋
}

pub fn update_pan(&mut self, sx: f64, sy: f64) {
    // 매 프레임 원점에서 새로 계산 (오차 누적 없음)
    self.pan_x = self.pan_origin_x + (sx - self.pan_start_x);
    self.pan_y = self.pan_origin_y + (sy - self.pan_start_y);
}
```

---

## 3. 도형 도구 — "Freehand와 같은 파이프라인, 다른 데이터"

### ToolMode 확장

기존 `ToolMode`에 3개 변형을 추가했다:

```rust
pub(crate) enum ToolMode {
    Pen,
    Eraser,
    Select,
    Line,       // 새로 추가
    Rectangle,  // 새로 추가
    Circle,     // 새로 추가
}
```

도형 도구의 인터랙션 패턴은 프리핸드와 다르다:

```
프리핸드: mousedown → mousemove(점 추가) × N → mouseup(확정)
도형:     mousedown(시작점) → mousemove(끝점 업데이트, 프리뷰) → mouseup(확정)
```

프리핸드는 매 mousemove마다 점을 `push`하지만, 도형은 시작점과 끝점 두 좌표만 기억하면 된다:

```rust
pub fn start_shape(&mut self, x: f64, y: f64) {
    self.is_drawing_shape = true;
    self.shape_start_x = x;
    self.shape_start_y = y;
}

pub fn update_shape(&mut self, x: f64, y: f64) {
    self.shape_end_x = x;
    self.shape_end_y = y;
    self.render();  // 프리뷰 갱신
}
```

### 도형 프리뷰

도형을 그리는 동안 반투명 프리뷰를 보여준다. `draw_shape_preview()`는 카메라 변환 안에서 호출되므로 월드 스페이스 좌표를 그대로 쓴다:

```rust
pub(crate) fn draw_shape_preview(&self) {
    if !self.is_drawing_shape { return; }

    self.ctx.save();
    self.ctx.set_global_alpha(0.6);  // 반투명
    self.ctx.set_stroke_style_str(&self.color);
    self.ctx.set_line_width(self.line_width);

    match self.tool_mode {
        ToolMode::Line => {
            self.ctx.begin_path();
            self.ctx.move_to(self.shape_start_x, self.shape_start_y);
            self.ctx.line_to(self.shape_end_x, self.shape_end_y);
            self.ctx.stroke();
        }
        ToolMode::Rectangle => { /* min/max 정규화 후 rect() */ }
        ToolMode::Circle => { /* 시작점=중심, 거리=반지름 */ }
        _ => {}
    }

    self.ctx.restore();
}
```

### finish_shape: 시작점과 끝점에서 Element 생성

mouseup 시 `finish_shape()`가 호출되면, 시작점과 끝점으로부터 최종 Shape를 만든다:

```rust
pub fn finish_shape(&mut self) {
    let shape = match self.tool_mode {
        ToolMode::Line => Shape::Line {
            x1: self.shape_start_x, y1: self.shape_start_y,
            x2: self.shape_end_x, y2: self.shape_end_y,
        },
        ToolMode::Rectangle => {
            let x = self.shape_start_x.min(self.shape_end_x);
            let y = self.shape_start_y.min(self.shape_end_y);
            let w = (self.shape_end_x - self.shape_start_x).abs();
            let h = (self.shape_end_y - self.shape_start_y).abs();
            Shape::Rect { x, y, w, h }
        }
        ToolMode::Circle => {
            let dx = self.shape_end_x - self.shape_start_x;
            let dy = self.shape_end_y - self.shape_start_y;
            let r = (dx * dx + dy * dy).sqrt();
            Shape::Circle { cx: self.shape_start_x, cy: self.shape_start_y, r }
        }
        _ => return,
    };

    let element = Element { id: self.next_id, shape, style: /* ... */ };
    self.undo_stack.push(Action::AddElement { element: element.clone() });
    self.elements.push(element);
}
```

사각형에서 `min`/`abs`를 쓰는 이유: 사용자가 오른쪽 아래에서 왼쪽 위로 드래그해도 정상적인 사각형이 만들어져야 하니까. 시작점이 반드시 좌상단이 아닐 수 있다.

### Undo/Redo는 공짜

Phase 1에서 `Action` enum을 `Stroke` 대신 `Element` 기반으로 바꿔놨기 때문에, 도형 도구의 Undo/Redo는 추가 작업 없이 동작한다. `AddElement` 액션 하나로 프리핸드든 직선이든 원이든 모두 커버된다.

이것이 "데이터 모델을 제대로 설계하면 기능 추가가 쉬워진다"의 구체적인 사례다.

---

## 4. SVG 내보내기 — "벡터 데이터니까 벡터 파일로"

Canvas에 그린 그림은 래스터(픽셀) 이미지로 내보낼 수도 있다. 하지만 우리는 벡터 데이터를 가지고 있다. `Element`에 정확한 좌표와 스타일 정보가 들어 있으니, SVG로 변환하면 확대해도 깨지지 않는 벡터 파일을 얻을 수 있다.

### Shape → SVG 요소 매핑

각 Shape 변형은 자연스럽게 SVG 요소에 대응된다:

```rust
impl Element {
    fn to_svg(&self) -> String {
        let s = &self.style;
        match &self.shape {
            Shape::Freehand { points } => {
                // <path d="M x y L x y L x y..." />
                let mut d = format!("M {} {}", points[0].x, points[0].y);
                for p in points.iter().skip(1) {
                    d.push_str(&format!(" L {} {}", p.x, p.y));
                }
                format!(r#"<path d="{d}" stroke="{}" stroke-width="{}" fill="none"
                    stroke-linecap="round" stroke-linejoin="round"/>"#, s.color, s.width)
            }
            Shape::Line { x1, y1, x2, y2 } => {
                format!(r#"<line x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}"
                    stroke="{}" stroke-width="{}"/>"#, s.color, s.width)
            }
            Shape::Rect { x, y, w, h } => {
                format!(r#"<rect x="{x}" y="{y}" width="{w}" height="{h}"
                    stroke="{}" stroke-width="{}" fill="none"/>"#, s.color, s.width)
            }
            Shape::Circle { cx, cy, r } => {
                format!(r#"<circle cx="{cx}" cy="{cy}" r="{r}"
                    stroke="{}" stroke-width="{}" fill="none"/>"#, s.color, s.width)
            }
        }
    }
}
```

Freehand → `<path>`, Line → `<line>`, Rect → `<rect>`, Circle → `<circle>`. 1:1 대응이다.

### viewBox 자동 계산

SVG의 `viewBox`는 전체 컨텐츠를 감싸는 영역으로 설정한다:

```rust
pub fn export_svg(&self) -> String {
    // 모든 요소의 바운딩 박스 합집합
    let mut min_x = f64::INFINITY;
    // ... max_x, min_y, max_y 계산

    let padding = 10.0;
    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="{} {} {} {}">"#,
        min_x - padding, min_y - padding,
        (max_x - min_x) + padding * 2.0,
        (max_y - min_y) + padding * 2.0
    )
}
```

이렇게 하면 그림이 캔버스 한구석에 있어도, SVG를 열면 컨텐츠에 딱 맞게 보인다.

### 왜 SVG 크레이트를 안 쓰나?

Rust 생태계에 `svg` 크레이트가 있지만, 일부러 수동 문자열 빌드를 택했다. 이유: **WASM 바이너리 크기**. 외부 크레이트를 추가하면 바이너리가 커지고, 우리가 생성하는 SVG는 6가지 태그(`svg`, `rect`, `circle`, `line`, `path`, `line`)뿐이다. `format!` 매크로면 충분하다.

### 프론트엔드: Blob 다운로드

```typescript
exportSvgBtn.addEventListener('click', () => {
    const svgStr = canvas.export_svg();
    const blob = new Blob([svgStr], { type: 'image/svg+xml' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'canvas.svg';
    a.click();
    URL.revokeObjectURL(url);
});
```

WASM에서 SVG 문자열을 받아와서, Blob으로 감싸고, `<a>` 태그의 download 속성으로 다운로드를 트리거한다. 서버 왕복 없이 클라이언트에서 완결.

---

## 회고: 구조가 만든 확장성

4단계를 거치면서 느낀 것들:

**enum은 정말 강력하다.** `Shape` enum 하나로 히트 테스트, 바운딩 박스, 이동, SVG 변환까지 깔끔하게 분기된다. 새 도형을 추가할 때 컴파일러가 "여기도 처리해야 해"라고 알려주는 건 동적 언어에서는 상상하기 어려운 안전망이다.

**좌표계 분리가 핵심이다.** 줌/팬 구현에서 가장 중요한 건 "이 좌표가 어느 공간인지"를 항상 의식하는 것이다. 스크린 좌표와 월드 좌표를 섞는 순간 버그가 터진다. `getPosition()`과 `getWorldPosition()`을 명확히 나눈 것이 모든 기능이 줌 상태에서도 정확히 동작하는 기반이 됐다.

**데이터 모델이 기능을 결정한다.** Phase 1의 리팩토링이 가장 지루했지만 가장 중요했다. `Stroke` → `Element`로 바꿔놓으니, 도형 도구는 `finish_shape()`에서 Shape variant만 만들면 됐고, SVG 내보내기는 Shape별 문자열 매핑이 전부였고, Undo/Redo는 수정할 것이 없었다.

### 최종 모듈 구조

```
src/
├── lib.rs          # Canvas 구조체, 50+ WASM API 메서드
├── models.rs       # Point, Style, Shape(enum), Element, BoundingBox
├── rendering.rs    # 카메라 변환 렌더링 파이프라인, 도형 프리뷰
├── selection.rs    # 선택 하이라이트, 러버밴드
└── svg_export.rs   # SVG 문자열 생성 및 내보내기
```

처음에 "그림판을 벡터 에디터로 만들 수 있을까?"라고 물었는데, 답은 "데이터 모델만 제대로 바꾸면 나머지는 따라온다"였다.

> 완성된 그림판은 [Toys 페이지](/toys/rust-canvas)에서 직접 사용해볼 수 있다.
