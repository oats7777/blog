use crate::models::Point;
use crate::models::Stroke;
use crate::Canvas;

// ===== 히트 테스트 =====

/// 점 P에서 선분 AB까지의 최소 거리
fn point_to_segment_distance(p: &Point, a: &Point, b: &Point) -> f64 {
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    let len_sq = dx * dx + dy * dy;

    if len_sq == 0.0 {
        let ex = p.x - a.x;
        let ey = p.y - a.y;
        return (ex * ex + ey * ey).sqrt();
    }

    let t = ((p.x - a.x) * dx + (p.y - a.y) * dy) / len_sq;
    let t = t.clamp(0.0, 1.0);

    let proj_x = a.x + t * dx;
    let proj_y = a.y + t * dy;

    let ex = p.x - proj_x;
    let ey = p.y - proj_y;
    (ex * ex + ey * ey).sqrt()
}

/// 특정 좌표가 스트로크 위에 있는지 검사
pub(crate) fn hit_test_stroke(stroke: &Stroke, x: f64, y: f64) -> bool {
    let threshold = (stroke.width / 2.0 + 4.0).max(8.0);
    let p = Point { x, y };

    for i in 0..stroke.points.len().saturating_sub(1) {
        let dist = point_to_segment_distance(&p, &stroke.points[i], &stroke.points[i + 1]);
        if dist <= threshold {
            return true;
        }
    }
    false
}

// ===== 선택 하이라이트 렌더링 =====

impl Canvas {
    /// 선택된 스트로크의 바운딩 박스 하이라이트 그리기
    pub(crate) fn draw_selection_highlight(&self) {
        if self.selected_ids.is_empty() {
            return;
        }

        self.ctx.save();

        let _ = self.ctx.set_line_dash(&self.dash_selection);
        self.ctx.set_stroke_style_str("#3b82f6");
        self.ctx.set_line_width(1.5);

        let padding = 6.0;

        for stroke in &self.strokes {
            if !self.selected_ids.contains(&stroke.id) {
                continue;
            }
            if let Some(bb) = stroke.bounding_box() {
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
