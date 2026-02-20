---
title: 'Rust Canvas'
description: 'Rust와 WebAssembly로 만든 벡터 기반 브라우저 그림판'
date: '2026-02-05'
embedUrl: 'https://rust-canvas.vercel.app'
techStack: 'Rust + WASM'
tags: ['WebAssembly', 'Canvas API', 'wasm-bindgen']
draft: false
---

## 소개

Rust로 작성된 Canvas 그리기 로직을 WebAssembly로 컴파일하여 브라우저에서 실행하는 벡터 기반 그림판입니다.

> 아키텍처 설계부터 리팩토링까지의 전체 여정은 [블로그 포스트](/posts/rust-canvas-wasm)에서 자세히 다루고 있습니다.

## 기능

- 펜으로 자유롭게 그리기
- 도형 도구 (직선, 사각형, 원) + 실시간 프리뷰
- 지우개 도구 (커서 범위 미리보기 지원)
- 색상 선택 (컬러 피커 + 프리셋)
- 선 굵기 조절
- 줌 / 팬 (Ctrl+휠 줌, Space+드래그 팬, 줌 버튼)
- 요소 선택 (클릭 / Shift+클릭 다중 선택 / 드래그 영역 선택)
- 선택한 요소 이동 (드래그)
- 복사 / 붙여넣기 / 삭제 (버튼 + 키보드 단축키)
- 실행 취소 / 다시 실행 (Ctrl+Z / Ctrl+Shift+Z)
- SVG 내보내기 (벡터 파일 다운로드)
- 전체 지우기

## 아키텍처

### 벡터 기반 Element 모델

모든 그리기 객체를 `Element { id, shape: Shape, style: Style }` 구조로 저장합니다. `Shape` enum은 `Freehand`, `Line`, `Rect`, `Circle` 변형을 가지며, 각 변형마다 바운딩 박스, 이동, 히트 테스트가 구현되어 있습니다.

### 카메라 변환 (줌/팬)

`ctx.translate(pan_x, pan_y) + ctx.scale(zoom, zoom)` 카메라 변환 파이프라인으로 무한 캔버스를 구현합니다. 저장된 좌표는 월드 스페이스를 유지하고, 렌더링만 변환합니다. UI 오버레이(러버밴드, 커서)는 스크린 스페이스에서 렌더링됩니다.

### 모듈 구조

```
src/
├── lib.rs          # Canvas 구조체, 도구/드로잉/도형/선택/이동/복사/Undo/줌/팬 API
├── models.rs       # Point, Style, Shape(enum), Element, BoundingBox 데이터 모델
├── rendering.rs    # 카메라 변환 렌더링 파이프라인, 도형 프리뷰
├── selection.rs    # 히트 테스트, 선택 하이라이트, 러버밴드 렌더링
└── svg_export.rs   # SVG 문자열 생성 및 내보내기
```

## 기술 스택

- **Rust**: Canvas 그리기 로직 구현
- **wasm-bindgen**: Rust ↔ JavaScript 바인딩
- **web-sys**: Web API (Canvas, DOM) 접근
- **serde**: 요소 데이터 직렬화/역직렬화

## 배운 점

1. Rust에서 `wasm-bindgen`을 사용한 WASM 모듈 작성법
2. `web-sys` 크레이트로 브라우저 API 사용하기
3. Canvas 2D Context로 그리기 구현
4. 마우스/터치 이벤트 처리
5. Immediate Mode → Retained Mode 렌더링 전환
6. Rust 모듈 시스템을 활용한 WASM 프로젝트 구조화
7. 카메라 변환(줌/팬)과 스크린↔월드 좌표 변환
8. Shape enum을 활용한 벡터 도형 시스템 설계
9. SVG 수동 문자열 빌드로 WASM 바이너리 크기 최소화
