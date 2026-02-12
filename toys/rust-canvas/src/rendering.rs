use wasm_bindgen::prelude::*;

use crate::models::Stroke;
use crate::Canvas;

impl Canvas {
    /// 캔버스 클리어 (배경만)
    pub(crate) fn clear_canvas(&self) {
        self.ctx.set_fill_style_str("#ffffff");
        self.ctx.fill_rect(0.0, 0.0, self.canvas_width, self.canvas_height);
    }

    /// 단일 스트로크 그리기
    pub(crate) fn draw_stroke(&self, stroke: &Stroke) {
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

    /// 커서 미리보기 그리기 (펜: 선택 색상 실선, 지우개: 회색 점선)
    pub(crate) fn draw_cursor_preview(&self) {
        let radius = self.line_width / 2.0;

        self.ctx.save();

        if self.is_eraser {
            let _ = self.ctx.set_line_dash(&self.dash_cursor);
            self.ctx.set_stroke_style_str("#999999");
        } else {
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

        self.ctx.restore();
    }
}

#[wasm_bindgen]
impl Canvas {
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

        // 선택 하이라이트
        self.draw_selection_highlight();

        // 러버밴드 (드래그 영역) 선택
        self.draw_rubber_band();

        // 커서 미리보기
        if self.show_cursor {
            self.draw_cursor_preview();
        }
    }
}
