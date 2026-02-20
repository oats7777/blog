use crate::CanvasInner;

// ===== 선택 하이라이트 렌더링 =====

impl CanvasInner {
    /// 선택된 요소의 바운딩 박스 하이라이트 그리기
    pub(crate) fn draw_selection_highlight(&self) {
        if self.selected_ids.is_empty() {
            return;
        }

        self.ctx.save();

        let _ = self.ctx.set_line_dash(&self.dash_selection);
        self.ctx.set_stroke_style_str("#3b82f6");
        self.ctx.set_line_width(1.5);

        let padding = 6.0;

        for elem in &self.elements {
            if !self.selected_ids.contains(&elem.id) {
                continue;
            }
            if let Some(bb) = elem.bounding_box() {
                self.ctx.begin_path();
                self.ctx.rect(
                    bb.min_x - padding,
                    bb.min_y - padding,
                    (bb.max_x - bb.min_x) + padding * 2.0,
                    (bb.max_y - bb.min_y) + padding * 2.0,
                );
                self.ctx.stroke();
            }
        }

        self.ctx.restore();
    }

    /// 러버밴드 (드래그 영역) 사각형 그리기
    pub(crate) fn draw_rubber_band(&self) {
        if !self.is_rubber_band {
            return;
        }

        let x = self.rubber_band_start_x.min(self.rubber_band_end_x);
        let y = self.rubber_band_start_y.min(self.rubber_band_end_y);
        let w = (self.rubber_band_end_x - self.rubber_band_start_x).abs();
        let h = (self.rubber_band_end_y - self.rubber_band_start_y).abs();

        self.ctx.save();

        // 반투명 파란 배경
        self.ctx.set_fill_style_str("rgba(59, 130, 246, 0.1)");
        self.ctx.fill_rect(x, y, w, h);

        // 파란 점선 테두리
        let _ = self.ctx.set_line_dash(&self.dash_selection);
        self.ctx.set_stroke_style_str("#3b82f6");
        self.ctx.set_line_width(1.0);
        self.ctx.begin_path();
        self.ctx.rect(x, y, w, h);
        self.ctx.stroke();

        self.ctx.restore();
    }
}
