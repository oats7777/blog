use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen]
pub struct Canvas {
    ctx: CanvasRenderingContext2d,
    is_drawing: bool,
    color: String,
    line_width: f64,
    is_eraser: bool,
    dpr: f64,
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

        // 레티나 디스플레이 지원: context를 dpr만큼 scale
        ctx.scale(dpr, dpr).ok();
        ctx.set_line_cap("round");
        ctx.set_line_join("round");

        Ok(Canvas {
            ctx,
            is_drawing: false,
            color: "#000000".to_string(),
            line_width: 5.0,
            is_eraser: false,
            dpr,
        })
    }

    #[wasm_bindgen]
    pub fn set_color(&mut self, color: &str) {
        self.color = color.to_string();
        self.is_eraser = false;
    }

    #[wasm_bindgen]
    pub fn set_line_width(&mut self, width: f64) {
        self.line_width = width;
    }

    #[wasm_bindgen]
    pub fn set_eraser(&mut self, is_eraser: bool) {
        self.is_eraser = is_eraser;
    }

    #[wasm_bindgen]
    pub fn start_drawing(&mut self, x: f64, y: f64) {
        self.is_drawing = true;
        self.ctx.begin_path();
        self.ctx.move_to(x, y);
    }

    #[wasm_bindgen]
    pub fn draw(&mut self, x: f64, y: f64) {
        if !self.is_drawing {
            return;
        }

        self.ctx.set_line_width(self.line_width);

        if self.is_eraser {
            self.ctx.set_stroke_style_str("#ffffff");
        } else {
            self.ctx.set_stroke_style_str(&self.color);
        }

        self.ctx.line_to(x, y);
        self.ctx.stroke();
        self.ctx.begin_path();
        self.ctx.move_to(x, y);
    }

    #[wasm_bindgen]
    pub fn stop_drawing(&mut self) {
        self.is_drawing = false;
    }

    #[wasm_bindgen]
    pub fn clear(&self) {
        let canvas = self.ctx.canvas().unwrap();
        let width = canvas.width() as f64;
        let height = canvas.height() as f64;
        self.ctx.set_fill_style_str("#ffffff");
        self.ctx.fill_rect(0.0, 0.0, width, height);
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    web_sys::console::log_1(&"Rust Canvas WASM loaded!".into());
}
