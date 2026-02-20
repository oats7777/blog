use crate::models::{Element, Shape};
use crate::{CanvasInner, ToolMode};

impl CanvasInner {
    /// 캔버스 클리어 (배경만, 변환 없이)
    pub(crate) fn clear_canvas(&self) {
        self.ctx.set_fill_style_str("#ffffff");
        self.ctx.fill_rect(0.0, 0.0, self.logical_width, self.logical_height);
    }

    /// 단일 요소 그리기 (Shape 별 디스패치)
    pub(crate) fn draw_element(&self, element: &Element) {
        let style = &element.style;
        self.ctx.set_stroke_style_str(&style.color);
        self.ctx.set_line_width(style.width);

        match &element.shape {
            Shape::Freehand { points } => {
                if points.len() < 2 {
                    return;
                }
                self.ctx.begin_path();
                self.ctx.move_to(points[0].x, points[0].y);
                for point in points.iter().skip(1) {
                    self.ctx.line_to(point.x, point.y);
                }
                self.ctx.stroke();
            }
            Shape::Line { x1, y1, x2, y2 } => {
                self.ctx.begin_path();
                self.ctx.move_to(*x1, *y1);
                self.ctx.line_to(*x2, *y2);
                self.ctx.stroke();
            }
            Shape::Rect { x, y, w, h } => {
                self.ctx.begin_path();
                self.ctx.rect(*x, *y, *w, *h);
                self.ctx.stroke();
            }
            Shape::Circle { cx, cy, r } => {
                self.ctx.begin_path();
                let _ = self.ctx.arc(*cx, *cy, *r, 0.0, std::f64::consts::PI * 2.0);
                self.ctx.stroke();
            }
        }
    }

    /// 도형 프리뷰 그리기 (반투명, 월드 스페이스)
    pub(crate) fn draw_shape_preview(&self) {
        if !self.is_drawing_shape {
            return;
        }

        self.ctx.save();
        self.ctx.set_global_alpha(0.6);
        self.ctx.set_stroke_style_str(&self.color);
        self.ctx.set_line_width(self.line_width);

        match self.tool_mode {
            ToolMode::Line => {
                self.ctx.begin_path();
                self.ctx.move_to(self.shape_start_x, self.shape_start_y);
                self.ctx.line_to(self.shape_end_x, self.shape_end_y);
                self.ctx.stroke();
            }
            ToolMode::Rectangle => {
                let x = self.shape_start_x.min(self.shape_end_x);
                let y = self.shape_start_y.min(self.shape_end_y);
                let w = (self.shape_end_x - self.shape_start_x).abs();
                let h = (self.shape_end_y - self.shape_start_y).abs();
                self.ctx.begin_path();
                self.ctx.rect(x, y, w, h);
                self.ctx.stroke();
            }
            ToolMode::Circle => {
                let dx = self.shape_end_x - self.shape_start_x;
                let dy = self.shape_end_y - self.shape_start_y;
                let r = (dx * dx + dy * dy).sqrt();
                self.ctx.begin_path();
                let _ = self.ctx.arc(
                    self.shape_start_x,
                    self.shape_start_y,
                    r,
                    0.0,
                    std::f64::consts::PI * 2.0,
                );
                self.ctx.stroke();
            }
            _ => {}
        }

        self.ctx.restore();
    }

    /// 커서 미리보기 그리기 (스크린 스페이스)
    pub(crate) fn draw_cursor_preview(&self) {
        // 줌 적용된 반경
        let radius = self.line_width / 2.0 * self.zoom;

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
            radius.max(1.0),
            0.0,
            std::f64::consts::PI * 2.0,
        );
        self.ctx.stroke();

        self.ctx.restore();
    }
}

impl CanvasInner {
    /// 전체 렌더링 (카메라 변환 파이프라인)
    pub(crate) fn render(&self) {
        // 1. 물리 캔버스 전체 클리어 (변환 없이)
        self.clear_canvas();

        // 2. 카메라 변환 적용
        self.ctx.save();
        let _ = self.ctx.translate(self.pan_x, self.pan_y);
        let _ = self.ctx.scale(self.zoom, self.zoom);

        // 저장된 모든 요소 그리기 (월드 스페이스)
        for element in &self.elements {
            self.draw_element(element);
        }

        // 현재 그리는 중인 요소 (월드 스페이스)
        if let Some(ref current) = self.current_element {
            self.draw_element(current);
        }

        // 도형 프리뷰 (월드 스페이스)
        self.draw_shape_preview();

        // 선택 하이라이트 (월드 스페이스)
        self.draw_selection_highlight();

        // 3. 카메라 변환 해제
        self.ctx.restore();

        // 4. 스크린 스페이스 UI
        // 러버밴드 (스크린 스페이스)
        self.draw_rubber_band();

        // 커서 미리보기 (스크린 스페이스)
        if self.show_cursor {
            self.draw_cursor_preview();
        }
    }
}
