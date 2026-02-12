use serde::{Deserialize, Serialize};

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
