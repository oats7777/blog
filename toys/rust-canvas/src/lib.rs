use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

/// 2D 점 구조체
#[derive(Clone, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// 스트로크 (펜으로 그린 선)
#[derive(Clone, Serialize, Deserialize)]
pub struct Stroke {
    pub id: u32,
    pub points: Vec<Point>,
    pub color: String,
    pub width: f64,
}

/// 캔버스 메인 구조체
#[wasm_bindgen]
pub struct Canvas {
    ctx: CanvasRenderingContext2d,
    canvas_width: f64,
    canvas_height: f64,
    dpr: f64,

    // Retained mode: 모든 스트로크 저장
    strokes: Vec<Stroke>,
    current_stroke: Option<Stroke>,
    next_id: u32,

    // 현재 도구 상태
    color: String,
    line_width: f64,
    is_drawing: bool,
    is_eraser: bool,
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
            dpr,
            strokes: Vec::new(),
            current_stroke: None,
            next_id: 1,
            color: "#000000".to_string(),
            line_width: 5.0,
            is_drawing: false,
            is_eraser: false,
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

    /// 캔버스 클리어 (배경만)
    fn clear_canvas(&self) {
        self.ctx.set_fill_style_str("#ffffff");
        self.ctx.fill_rect(0.0, 0.0, self.canvas_width, self.canvas_height);
    }

    /// 단일 스트로크 그리기
    fn draw_stroke(&self, stroke: &Stroke) {
        if stroke.points.len() < 2 {
            return;
        }

        self.ctx.begin_path();
        self.ctx.set_stroke_style_str(&stroke.color);
        self.ctx.set_line_width(stroke.width);

        let first = &stroke.points[0];
        self.ctx.move_to(first.x, first.y);

        for point in stroke.points.iter().skip(1) {
            self.ctx.line_to(point.x, point.y);
        }
        self.ctx.stroke();
    }

    /// 전체 렌더링 (Retained Mode 핵심)
    #[wasm_bindgen]
    pub fn render(&self) {
        self.clear_canvas();

        // 저장된 모든 스트로크 그리기
        for stroke in &self.strokes {
            self.draw_stroke(stroke);
        }

        // 현재 그리는 중인 스트로크
        if let Some(ref current) = self.current_stroke {
            self.draw_stroke(current);
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
