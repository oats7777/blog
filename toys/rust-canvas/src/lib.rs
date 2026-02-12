mod models;
mod rendering;

use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::models::{Point, Stroke};

/// 캔버스 메인 구조체
#[wasm_bindgen]
pub struct Canvas {
    pub(crate) ctx: CanvasRenderingContext2d,
    pub(crate) canvas_width: f64,
    pub(crate) canvas_height: f64,

    // Retained mode: 모든 스트로크 저장
    pub(crate) strokes: Vec<Stroke>,
    pub(crate) current_stroke: Option<Stroke>,
    pub(crate) next_id: u32,

    // 현재 도구 상태
    pub(crate) color: String,
    pub(crate) line_width: f64,
    pub(crate) is_drawing: bool,
    pub(crate) is_eraser: bool,

    // 지우개 커서 미리보기 상태
    pub(crate) cursor_x: f64,
    pub(crate) cursor_y: f64,
    pub(crate) show_cursor: bool,
}

#[wasm_bindgen]
impl Canvas {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str, dpr: f64) -> Result<Canvas, JsValue> {
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

        let canvas_width = canvas.width() as f64;
        let canvas_height = canvas.height() as f64;

        // 레티나 디스플레이 지원
        ctx.scale(dpr, dpr).ok();
        ctx.set_line_cap("round");
        ctx.set_line_join("round");

        Ok(Canvas {
            ctx,
            canvas_width,
            canvas_height,
            strokes: Vec::new(),
            current_stroke: None,
            next_id: 1,
            color: "#000000".to_string(),
            line_width: 5.0,
            is_drawing: false,
            is_eraser: false,
            cursor_x: 0.0,
            cursor_y: 0.0,
            show_cursor: false,
        })
    }

    /// 색상 설정
    #[wasm_bindgen]
    pub fn set_color(&mut self, color: &str) {
        self.color = color.to_string();
        self.is_eraser = false;
    }

    /// 선 굵기 설정
    #[wasm_bindgen]
    pub fn set_line_width(&mut self, width: f64) {
        self.line_width = width;
    }

    /// 지우개 모드 설정
    #[wasm_bindgen]
    pub fn set_eraser(&mut self, is_eraser: bool) {
        self.is_eraser = is_eraser;
    }

    /// 그리기 상태 확인
    #[wasm_bindgen]
    pub fn get_is_drawing(&self) -> bool {
        self.is_drawing
    }

    /// 지우개 모드 확인
    #[wasm_bindgen]
    pub fn get_is_eraser(&self) -> bool {
        self.is_eraser
    }

    /// 커서 위치 업데이트 (지우개 미리보기용)
    #[wasm_bindgen]
    pub fn update_cursor(&mut self, x: f64, y: f64) {
        self.cursor_x = x;
        self.cursor_y = y;
        self.show_cursor = true;
    }

    /// 커서 숨기기
    #[wasm_bindgen]
    pub fn hide_cursor(&mut self) {
        self.show_cursor = false;
    }

    /// 그리기 시작 - 새 스트로크 생성
    #[wasm_bindgen]
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

    /// 그리기 중 - 점 추가 및 렌더링
    #[wasm_bindgen]
    pub fn draw(&mut self, x: f64, y: f64) {
        if !self.is_drawing {
            return;
        }

        if let Some(ref mut stroke) = self.current_stroke {
            stroke.points.push(Point { x, y });
        }
        self.render();
    }

    /// 그리기 종료 - 스트로크 확정
    #[wasm_bindgen]
    pub fn stop_drawing(&mut self) {
        self.is_drawing = false;
        if let Some(stroke) = self.current_stroke.take() {
            if stroke.points.len() > 1 {
                self.strokes.push(stroke);
            }
        }
    }

    /// 전체 지우기 (모든 스트로크 삭제)
    #[wasm_bindgen]
    pub fn clear(&mut self) {
        self.strokes.clear();
        self.current_stroke = None;
        self.render();
    }

    /// 스트로크 개수 반환 (디버깅용)
    #[wasm_bindgen]
    pub fn get_stroke_count(&self) -> usize {
        self.strokes.len()
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    web_sys::console::log_1(&"Rust Canvas WASM (Retained Mode) loaded!".into());
}
