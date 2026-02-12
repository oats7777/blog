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

/// 스트로크 (펜으로 그린 선)
#[derive(Clone, Serialize, Deserialize)]
pub struct Stroke {
    pub id: u32,
    pub points: Vec<Point>,
    pub color: String,
    pub width: f64,
}

impl Stroke {
    /// 바운딩 박스 계산 (선 굵기 반영)
    pub fn bounding_box(&self) -> Option<BoundingBox> {
        if self.points.is_empty() {
            return None;
        }
        let half_w = self.width / 2.0;
        let mut bb = BoundingBox {
            min_x: f64::INFINITY,
            min_y: f64::INFINITY,
            max_x: f64::NEG_INFINITY,
            max_y: f64::NEG_INFINITY,
        };
        for p in &self.points {
            bb.min_x = bb.min_x.min(p.x - half_w);
            bb.min_y = bb.min_y.min(p.y - half_w);
            bb.max_x = bb.max_x.max(p.x + half_w);
            bb.max_y = bb.max_y.max(p.y + half_w);
        }
        Some(bb)
    }

    /// 모든 점을 (dx, dy)만큼 이동
    pub fn translate(&mut self, dx: f64, dy: f64) {
        for p in &mut self.points {
            p.x += dx;
            p.y += dy;
        }
    }
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
