use std::fmt::Write;

use crate::models::{Element, Shape};
use crate::CanvasInner;

impl Element {
    /// SVG 요소 문자열 생성
    fn to_svg(&self) -> String {
        let s = &self.style;
        match &self.shape {
            Shape::Freehand { points } => {
                if points.len() < 2 {
                    return String::new();
                }
                let mut d = format!("M {} {}", points[0].x, points[0].y);
                for p in points.iter().skip(1) {
                    let _ = write!(d, " L {} {}", p.x, p.y);
                }
                format!(
                    r#"<path d="{}" stroke="{}" stroke-width="{}" fill="none" stroke-linecap="round" stroke-linejoin="round"/>"#,
                    d, s.color, s.width
                )
            }
            Shape::Line { x1, y1, x2, y2 } => {
                format!(
                    r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}" stroke-width="{}" stroke-linecap="round"/>"#,
                    x1, y1, x2, y2, s.color, s.width
                )
            }
            Shape::Rect { x, y, w, h } => {
                format!(
                    r#"<rect x="{}" y="{}" width="{}" height="{}" stroke="{}" stroke-width="{}" fill="none" stroke-linejoin="round"/>"#,
                    x, y, w, h, s.color, s.width
                )
            }
            Shape::Circle { cx, cy, r } => {
                format!(
                    r#"<circle cx="{}" cy="{}" r="{}" stroke="{}" stroke-width="{}" fill="none"/>"#,
                    cx, cy, r, s.color, s.width
                )
            }
        }
    }
}

impl CanvasInner {
    /// SVG 문자열 내보내기
    pub(crate) fn export_svg(&self) -> String {
        if self.elements.is_empty() {
            return String::from(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 800 500"><rect width="800" height="500" fill="white"/></svg>"#);
        }

        // 전체 컨텐츠 바운딩 박스 계산
        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut max_y = f64::NEG_INFINITY;

        for elem in &self.elements {
            if let Some(bb) = elem.bounding_box() {
                min_x = min_x.min(bb.min_x);
                min_y = min_y.min(bb.min_y);
                max_x = max_x.max(bb.max_x);
                max_y = max_y.max(bb.max_y);
            }
        }

        let padding = 10.0;
        let vx = min_x - padding;
        let vy = min_y - padding;
        let vw = (max_x - min_x) + padding * 2.0;
        let vh = (max_y - min_y) + padding * 2.0;

        let mut svg = format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="{} {} {} {}">"#,
            vx, vy, vw, vh
        );
        svg.push('\n');

        // 흰 배경
        let _ = write!(
            svg,
            r#"  <rect x="{}" y="{}" width="{}" height="{}" fill="white"/>"#,
            vx, vy, vw, vh
        );
        svg.push('\n');

        // 요소들
        for elem in &self.elements {
            let s = elem.to_svg();
            if !s.is_empty() {
                svg.push_str("  ");
                svg.push_str(&s);
                svg.push('\n');
            }
        }

        svg.push_str("</svg>");
        svg
    }
}
