use serde::{Deserialize, Serialize};

/// 2D 점 구조체
#[derive(Clone, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// 바운딩 박스
#[derive(Clone, Debug)]
pub struct BoundingBox {
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
}

/// 스타일 (색상, 선 굵기)
#[derive(Clone, Serialize, Deserialize)]
pub struct Style {
    pub color: String,
    pub width: f64,
}

/// 도형 종류
#[derive(Clone, Serialize, Deserialize)]
pub enum Shape {
    Freehand { points: Vec<Point> },
    Line { x1: f64, y1: f64, x2: f64, y2: f64 },
    Rect { x: f64, y: f64, w: f64, h: f64 },
    Circle { cx: f64, cy: f64, r: f64 },
}

/// 벡터 요소 (모든 그리기 객체의 공통 구조)
#[derive(Clone, Serialize, Deserialize)]
pub struct Element {
    pub id: u32,
    pub shape: Shape,
    pub style: Style,
}

impl Element {
    /// 바운딩 박스 계산 (선 굵기 반영)
    pub fn bounding_box(&self) -> Option<BoundingBox> {
        let half_w = self.style.width / 2.0;
        match &self.shape {
            Shape::Freehand { points } => {
                if points.is_empty() {
                    return None;
                }
                let mut bb = BoundingBox {
                    min_x: f64::INFINITY,
                    min_y: f64::INFINITY,
                    max_x: f64::NEG_INFINITY,
                    max_y: f64::NEG_INFINITY,
                };
                for p in points {
                    bb.min_x = bb.min_x.min(p.x - half_w);
                    bb.min_y = bb.min_y.min(p.y - half_w);
                    bb.max_x = bb.max_x.max(p.x + half_w);
                    bb.max_y = bb.max_y.max(p.y + half_w);
                }
                Some(bb)
            }
            Shape::Line { x1, y1, x2, y2 } => {
                Some(BoundingBox {
                    min_x: x1.min(*x2) - half_w,
                    min_y: y1.min(*y2) - half_w,
                    max_x: x1.max(*x2) + half_w,
                    max_y: y1.max(*y2) + half_w,
                })
            }
            Shape::Rect { x, y, w, h } => {
                Some(BoundingBox {
                    min_x: *x - half_w,
                    min_y: *y - half_w,
                    max_x: x + w + half_w,
                    max_y: y + h + half_w,
                })
            }
            Shape::Circle { cx, cy, r } => {
                Some(BoundingBox {
                    min_x: cx - r - half_w,
                    min_y: cy - r - half_w,
                    max_x: cx + r + half_w,
                    max_y: cy + r + half_w,
                })
            }
        }
    }

    /// 모든 좌표를 (dx, dy)만큼 이동
    pub fn translate(&mut self, dx: f64, dy: f64) {
        match &mut self.shape {
            Shape::Freehand { points } => {
                for p in points {
                    p.x += dx;
                    p.y += dy;
                }
            }
            Shape::Line { x1, y1, x2, y2 } => {
                *x1 += dx;
                *y1 += dy;
                *x2 += dx;
                *y2 += dy;
            }
            Shape::Rect { x, y, .. } => {
                *x += dx;
                *y += dy;
            }
            Shape::Circle { cx, cy, .. } => {
                *cx += dx;
                *cy += dy;
            }
        }
    }

    /// 좌표가 이 요소 위에 있는지 히트 테스트
    pub fn hit_test(&self, px: f64, py: f64) -> bool {
        let threshold = (self.style.width / 2.0 + 4.0).max(8.0);
        let p = Point { x: px, y: py };

        match &self.shape {
            Shape::Freehand { points } => {
                for i in 0..points.len().saturating_sub(1) {
                    if point_to_segment_distance(&p, &points[i], &points[i + 1]) <= threshold {
                        return true;
                    }
                }
                false
            }
            Shape::Line { x1, y1, x2, y2 } => {
                let a = Point { x: *x1, y: *y1 };
                let b = Point { x: *x2, y: *y2 };
                point_to_segment_distance(&p, &a, &b) <= threshold
            }
            Shape::Rect { x, y, w, h } => {
                // 4변 각각에 대해 거리 검사
                let corners = [
                    (Point { x: *x, y: *y }, Point { x: x + w, y: *y }),         // top
                    (Point { x: x + w, y: *y }, Point { x: x + w, y: y + h }),   // right
                    (Point { x: x + w, y: y + h }, Point { x: *x, y: y + h }),   // bottom
                    (Point { x: *x, y: y + h }, Point { x: *x, y: *y }),         // left
                ];
                for (a, b) in &corners {
                    if point_to_segment_distance(&p, a, b) <= threshold {
                        return true;
                    }
                }
                false
            }
            Shape::Circle { cx, cy, r } => {
                let dist_from_center = ((px - cx).powi(2) + (py - cy).powi(2)).sqrt();
                (dist_from_center - r).abs() <= threshold
            }
        }
    }
}

/// 점 P에서 선분 AB까지의 최소 거리
pub(crate) fn point_to_segment_distance(p: &Point, a: &Point, b: &Point) -> f64 {
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

impl BoundingBox {
    /// 두 바운딩 박스가 겹치는지 판정
    pub fn intersects(&self, other: &BoundingBox) -> bool {
        self.min_x <= other.max_x
            && self.max_x >= other.min_x
            && self.min_y <= other.max_y
            && self.max_y >= other.min_y
    }
}
