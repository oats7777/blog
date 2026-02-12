---
title: 'Rust Canvas'
description: 'Rust와 WebAssembly로 만든 브라우저 그림판'
date: '2026-02-05'
embedUrl: 'https://rust-canvas.vercel.app'
techStack: 'Rust + WASM'
tags: ['WebAssembly', 'Canvas API', 'wasm-bindgen']
draft: false
---

## 소개

Rust로 작성된 Canvas 그리기 로직을 WebAssembly로 컴파일하여 브라우저에서 실행하는 그림판입니다.

> 아키텍처 설계부터 리팩토링까지의 전체 여정은 [블로그 포스트](/posts/rust-canvas-wasm)에서 자세히 다루고 있습니다.

## 기능

- 펜으로 자유롭게 그리기
- 지우개 도구 (커서 범위 미리보기 지원)
- 색상 선택 (컬러 피커 + 프리셋)
- 선 굵기 조절
- 스트로크 선택 (클릭 / Shift+클릭 다중 선택 / 드래그 영역 선택)
- 선택한 스트로크 이동 (드래그)
- 복사 / 붙여넣기 / 삭제 (버튼 + 키보드 단축키)
- 실행 취소 / 다시 실행 (Ctrl+Z / Ctrl+Shift+Z)
- 전체 지우기

## 아키텍처

### Retained Mode 렌더링

모든 스트로크를 `Vec<Stroke>` 데이터 구조에 저장하고, 매 프레임마다 전체를 다시 그리는 **Retained Mode** 방식을 사용합니다. 이전의 Immediate Mode와 달리 드로잉 히스토리가 완전히 보존되어 undo/redo, 저장/불러오기 등의 확장이 가능합니다.

### 모듈 구조

```
src/
├── lib.rs          # Canvas 구조체, 생성자, 도구/드로잉/선택/이동/복사/Undo API
├── models.rs       # Point, Stroke, BoundingBox 데이터 모델
├── rendering.rs    # 렌더링 엔진 (스트로크 그리기, 커서 미리보기)
└── selection.rs    # 히트 테스트, 선택 하이라이트, 러버밴드 렌더링
```

## 기술 스택

- **Rust**: Canvas 그리기 로직 구현
- **wasm-bindgen**: Rust ↔ JavaScript 바인딩
- **web-sys**: Web API (Canvas, DOM) 접근
- **serde**: 스트로크 데이터 직렬화/역직렬화

## 배운 점

1. Rust에서 `wasm-bindgen`을 사용한 WASM 모듈 작성법
2. `web-sys` 크레이트로 브라우저 API 사용하기
3. Canvas 2D Context로 그리기 구현
4. 마우스/터치 이벤트 처리
5. Immediate Mode → Retained Mode 렌더링 전환
6. Rust 모듈 시스템을 활용한 WASM 프로젝트 구조화
