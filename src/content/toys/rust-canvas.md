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

## 기능

- 펜으로 자유롭게 그리기
- 지우개 도구
- 색상 선택 (컬러 피커 + 프리셋)
- 선 굵기 조절
- 전체 지우기

## 기술 스택

- **Rust**: Canvas 그리기 로직 구현
- **wasm-bindgen**: Rust ↔ JavaScript 바인딩
- **web-sys**: Web API (Canvas, DOM) 접근
- **Vite**: 프론트엔드 빌드 도구

## 배운 점

1. Rust에서 `wasm-bindgen`을 사용한 WASM 모듈 작성법
2. `web-sys` 크레이트로 브라우저 API 사용하기
3. Canvas 2D Context로 그리기 구현
4. 마우스/터치 이벤트 처리
