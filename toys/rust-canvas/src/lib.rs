mod models;
mod rendering;
mod selection;

use std::collections::HashSet;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use js_sys::Array as JsArray;

use crate::models::{BoundingBox, Point, Stroke};
use crate::selection::hit_test_stroke;

/// 도구 모드
#[derive(Clone, Copy, PartialEq)]
pub(crate) enum ToolMode {
    Pen,
    Eraser,
    Select,
}

/// Undo/Redo 가능한 액션
#[derive(Clone)]
enum Action {
    /// 스트로크 추가 (그리기/지우개)
    AddStroke { stroke: Stroke },
    /// 스트로크 삭제
    DeleteStrokes { strokes: Vec<Stroke> },
    /// 스트로크 이동
    MoveStrokes { ids: Vec<u32>, dx: f64, dy: f64 },
    /// 붙여넣기
    PasteStrokes { strokes: Vec<Stroke> },
    /// 전체 지우기
    ClearAll { strokes: Vec<Stroke> },
}

/// 캔버스 메인 구조체
#[wasm_bindgen]
pub struct Canvas {
    pub(crate) ctx: CanvasRenderingContext2d,
    pub(crate) canvas_width: f64,
    pub(crate) canvas_height: f64,

    // Retained mode: 모든 스트로크 저장
    pub(crate) strokes: Vec<Stroke>,
    pub(crate) current_stroke: Option<Stroke>,
    pub(crate) next_id: u32,

    // 현재 도구 상태
    pub(crate) color: String,
    pub(crate) line_width: f64,
    pub(crate) is_drawing: bool,
    pub(crate) is_eraser: bool,

    // 커서 미리보기 상태
    pub(crate) cursor_x: f64,
    pub(crate) cursor_y: f64,
    pub(crate) show_cursor: bool,

    // 선택 상태
    pub(crate) tool_mode: ToolMode,
    pub(crate) selected_ids: HashSet<u32>,
    pub(crate) clipboard: Vec<Stroke>,

    // 드래그 이동 상태
    pub(crate) is_moving: bool,
    pub(crate) move_start_x: f64,
    pub(crate) move_start_y: f64,
    pub(crate) move_total_dx: f64,
    pub(crate) move_total_dy: f64,

    // 러버밴드 (드래그 영역) 선택 상태
    pub(crate) is_rubber_band: bool,
    pub(crate) rubber_band_start_x: f64,
    pub(crate) rubber_band_start_y: f64,
    pub(crate) rubber_band_end_x: f64,
    pub(crate) rubber_band_end_y: f64,

    // Undo/Redo 스택
    undo_stack: Vec<Action>,
    redo_stack: Vec<Action>,

    // 캐시된 dash 패턴 (매 프레임 재할당 방지)
    pub(crate) dash_cursor: JsArray,
    pub(crate) dash_selection: JsArray,
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
            strokes: Vec::new(),
            current_stroke: None,
            next_id: 1,
            color: "#000000".to_string(),
            line_width: 5.0,
            is_drawing: false,
            is_eraser: false,
            cursor_x: 0.0,
            cursor_y: 0.0,
            show_cursor: false,
            tool_mode: ToolMode::Pen,
            selected_ids: HashSet::new(),
            clipboard: Vec::new(),
            is_moving: false,
            move_start_x: 0.0,
            move_start_y: 0.0,
            move_total_dx: 0.0,
            move_total_dy: 0.0,
            is_rubber_band: false,
            rubber_band_start_x: 0.0,
            rubber_band_start_y: 0.0,
            rubber_band_end_x: 0.0,
            rubber_band_end_y: 0.0,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            dash_cursor: {
                let arr = JsArray::new();
                arr.push(&JsValue::from_f64(4.0));
                arr.push(&JsValue::from_f64(4.0));
                arr
            },
            dash_selection: {
                let arr = JsArray::new();
                arr.push(&JsValue::from_f64(6.0));
                arr.push(&JsValue::from_f64(4.0));
                arr
            },
        })
    }

    // ===== 기본 도구 =====

    /// 색상 설정
    #[wasm_bindgen]
    pub fn set_color(&mut self, color: &str) {
        self.color = color.to_string();
        self.is_eraser = false;
        self.tool_mode = ToolMode::Pen;
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
        if is_eraser {
            self.tool_mode = ToolMode::Eraser;
        }
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

    /// 커서 위치 업데이트
    #[wasm_bindgen]
    pub fn update_cursor(&mut self, x: f64, y: f64) {
        self.cursor_x = x;
        self.cursor_y = y;
        self.show_cursor = true;
    }

    /// 커서 숨기기
    #[wasm_bindgen]
    pub fn hide_cursor(&mut self) {
        self.show_cursor = false;
    }

    // ===== 그리기 =====

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
                self.undo_stack.push(Action::AddStroke {
                    stroke: stroke.clone(),
                });
                self.redo_stack.clear();
                self.strokes.push(stroke);
            }
        }
    }

    /// 전체 지우기 (모든 스트로크 삭제)
    #[wasm_bindgen]
    pub fn clear(&mut self) {
        if !self.strokes.is_empty() {
            self.undo_stack.push(Action::ClearAll {
                strokes: self.strokes.clone(),
            });
            self.redo_stack.clear();
        }
        self.strokes.clear();
        self.current_stroke = None;
        self.selected_ids.clear();
        self.render();
    }

    /// 스트로크 개수 반환 (디버깅용)
    #[wasm_bindgen]
    pub fn get_stroke_count(&self) -> usize {
        self.strokes.len()
    }

    // ===== 선택 도구 =====

    /// 선택 도구 모드 설정
    #[wasm_bindgen]
    pub fn set_select_mode(&mut self, is_select: bool) {
        if is_select {
            self.tool_mode = ToolMode::Select;
            self.is_eraser = false;
            self.is_drawing = false;
        } else {
            self.tool_mode = ToolMode::Pen;
        }
    }

    /// 현재 선택 도구 모드인지 확인
    #[wasm_bindgen]
    pub fn get_is_select_mode(&self) -> bool {
        self.tool_mode == ToolMode::Select
    }

    /// 좌표에서 스트로크 선택 시도 (역순 탐색으로 최상위 우선)
    #[wasm_bindgen]
    pub fn try_select_at(&mut self, x: f64, y: f64, shift: bool) -> bool {
        let mut hit_id: Option<u32> = None;
        for stroke in self.strokes.iter().rev() {
            if hit_test_stroke(stroke, x, y) {
                hit_id = Some(stroke.id);
                break;
            }
        }

        match hit_id {
            Some(id) => {
                if shift {
                    if !self.selected_ids.remove(&id) {
                        self.selected_ids.insert(id);
                    }
                } else {
                    self.selected_ids.clear();
                    self.selected_ids.insert(id);
                }
                true
            }
            None => {
                if !shift {
                    self.selected_ids.clear();
                }
                false
            }
        }
    }

    /// 전체 선택
    #[wasm_bindgen]
    pub fn select_all(&mut self) {
        self.selected_ids.clear();
        for stroke in &self.strokes {
            self.selected_ids.insert(stroke.id);
        }
    }

    /// 선택 해제
    #[wasm_bindgen]
    pub fn deselect_all(&mut self) {
        self.selected_ids.clear();
    }

    /// 선택된 스트로크가 있는지 확인
    #[wasm_bindgen]
    pub fn has_selection(&self) -> bool {
        !self.selected_ids.is_empty()
    }

    /// 좌표가 선택된 스트로크 위에 있는지 확인
    #[wasm_bindgen]
    pub fn is_over_selected(&self, x: f64, y: f64) -> bool {
        for stroke in &self.strokes {
            if self.selected_ids.contains(&stroke.id) && hit_test_stroke(stroke, x, y) {
                return true;
            }
        }
        false
    }

    // ===== 이동 =====

    /// 이동 중인지 확인
    #[wasm_bindgen]
    pub fn get_is_moving(&self) -> bool {
        self.is_moving
    }

    /// 이동 시작
    #[wasm_bindgen]
    pub fn start_move(&mut self, x: f64, y: f64) {
        self.is_moving = true;
        self.move_start_x = x;
        self.move_start_y = y;
        self.move_total_dx = 0.0;
        self.move_total_dy = 0.0;
    }

    /// 이동 중 - 선택된 스트로크들을 델타만큼 이동
    #[wasm_bindgen]
    pub fn move_selected(&mut self, x: f64, y: f64) {
        if !self.is_moving {
            return;
        }
        let dx = x - self.move_start_x;
        let dy = y - self.move_start_y;

        for stroke in &mut self.strokes {
            if self.selected_ids.contains(&stroke.id) {
                stroke.translate(dx, dy);
            }
        }

        self.move_total_dx += dx;
        self.move_total_dy += dy;
        self.move_start_x = x;
        self.move_start_y = y;
        self.render();
    }

    /// 이동 종료
    #[wasm_bindgen]
    pub fn stop_move(&mut self) {
        if self.is_moving
            && (self.move_total_dx != 0.0 || self.move_total_dy != 0.0)
        {
            self.undo_stack.push(Action::MoveStrokes {
                ids: self.selected_ids.iter().cloned().collect(),
                dx: self.move_total_dx,
                dy: self.move_total_dy,
            });
            self.redo_stack.clear();
        }
        self.is_moving = false;
    }

    // ===== 클립보드 =====

    /// 선택된 스트로크 복사
    #[wasm_bindgen]
    pub fn copy_selected(&mut self) {
        self.clipboard.clear();
        for stroke in &self.strokes {
            if self.selected_ids.contains(&stroke.id) {
                self.clipboard.push(stroke.clone());
            }
        }
    }

    /// 클립보드에서 붙여넣기 (오프셋 적용, 새 ID 부여)
    #[wasm_bindgen]
    pub fn paste(&mut self) {
        if self.clipboard.is_empty() {
            return;
        }

        let offset = 20.0;
        self.selected_ids.clear();
        let mut pasted: Vec<Stroke> = Vec::new();

        for original in &self.clipboard.clone() {
            let mut cloned = original.clone();
            cloned.id = self.next_id;
            self.next_id += 1;
            cloned.translate(offset, offset);
            self.selected_ids.insert(cloned.id);
            pasted.push(cloned.clone());
            self.strokes.push(cloned);
        }

        self.undo_stack.push(Action::PasteStrokes {
            strokes: pasted,
        });
        self.redo_stack.clear();

        // 반복 붙여넣기 시 오프셋 누적을 위해 클립보드 갱신
        self.clipboard = self
            .strokes
            .iter()
            .filter(|s| self.selected_ids.contains(&s.id))
            .cloned()
            .collect();

        self.render();
    }

    /// 선택된 스트로크 삭제
    #[wasm_bindgen]
    pub fn delete_selected(&mut self) {
        let deleted: Vec<Stroke> = self
            .strokes
            .iter()
            .filter(|s| self.selected_ids.contains(&s.id))
            .cloned()
            .collect();

        if !deleted.is_empty() {
            self.undo_stack.push(Action::DeleteStrokes {
                strokes: deleted,
            });
            self.redo_stack.clear();
        }

        self.strokes
            .retain(|s| !self.selected_ids.contains(&s.id));
        self.selected_ids.clear();
        self.render();
    }

    // ===== Undo / Redo =====

    /// Undo 가능 여부
    #[wasm_bindgen]
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Redo 가능 여부
    #[wasm_bindgen]
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// 실행 취소
    #[wasm_bindgen]
    pub fn undo(&mut self) {
        let action = match self.undo_stack.pop() {
            Some(a) => a,
            None => return,
        };

        match &action {
            Action::AddStroke { stroke } => {
                self.strokes.retain(|s| s.id != stroke.id);
            }
            Action::DeleteStrokes { strokes } => {
                for s in strokes {
                    self.strokes.push(s.clone());
                }
            }
            Action::MoveStrokes { ids, dx, dy } => {
                for stroke in &mut self.strokes {
                    if ids.contains(&stroke.id) {
                        stroke.translate(-dx, -dy);
                    }
                }
            }
            Action::PasteStrokes { strokes } => {
                let ids: HashSet<u32> = strokes.iter().map(|s| s.id).collect();
                self.strokes.retain(|s| !ids.contains(&s.id));
            }
            Action::ClearAll { strokes } => {
                self.strokes = strokes.clone();
            }
        }

        self.redo_stack.push(action);
        self.selected_ids.clear();
        self.render();
    }

    /// 다시 실행
    #[wasm_bindgen]
    pub fn redo(&mut self) {
        let action = match self.redo_stack.pop() {
            Some(a) => a,
            None => return,
        };

        match &action {
            Action::AddStroke { stroke } => {
                self.strokes.push(stroke.clone());
            }
            Action::DeleteStrokes { strokes } => {
                let ids: HashSet<u32> = strokes.iter().map(|s| s.id).collect();
                self.strokes.retain(|s| !ids.contains(&s.id));
            }
            Action::MoveStrokes { ids, dx, dy } => {
                for stroke in &mut self.strokes {
                    if ids.contains(&stroke.id) {
                        stroke.translate(*dx, *dy);
                    }
                }
            }
            Action::PasteStrokes { strokes } => {
                for s in strokes {
                    self.strokes.push(s.clone());
                }
            }
            Action::ClearAll { .. } => {
                self.strokes.clear();
            }
        }

        self.undo_stack.push(action);
        self.selected_ids.clear();
        self.render();
    }

    // ===== 러버밴드 (드래그 영역) 선택 =====

    /// 러버밴드 선택 시작
    #[wasm_bindgen]
    pub fn start_rubber_band(&mut self, x: f64, y: f64) {
        self.is_rubber_band = true;
        self.rubber_band_start_x = x;
        self.rubber_band_start_y = y;
        self.rubber_band_end_x = x;
        self.rubber_band_end_y = y;
    }

    /// 러버밴드 드래그 중 — 영역 업데이트 및 렌더링
    #[wasm_bindgen]
    pub fn update_rubber_band(&mut self, x: f64, y: f64) {
        if !self.is_rubber_band {
            return;
        }
        self.rubber_band_end_x = x;
        self.rubber_band_end_y = y;
        self.render();
    }

    /// 러버밴드 선택 확정 — 영역 내 스트로크 선택
    #[wasm_bindgen]
    pub fn finish_rubber_band(&mut self, shift: bool) {
        if !self.is_rubber_band {
            return;
        }
        self.is_rubber_band = false;

        let rect = BoundingBox {
            min_x: self.rubber_band_start_x.min(self.rubber_band_end_x),
            min_y: self.rubber_band_start_y.min(self.rubber_band_end_y),
            max_x: self.rubber_band_start_x.max(self.rubber_band_end_x),
            max_y: self.rubber_band_start_y.max(self.rubber_band_end_y),
        };

        if !shift {
            self.selected_ids.clear();
        }

        for stroke in &self.strokes {
            if let Some(bb) = stroke.bounding_box() {
                if rect.intersects(&bb) {
                    self.selected_ids.insert(stroke.id);
                }
            }
        }

        self.render();
    }

    /// 러버밴드 드래그 중인지 확인
    #[wasm_bindgen]
    pub fn get_is_rubber_band(&self) -> bool {
        self.is_rubber_band
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    web_sys::console::log_1(&"Rust Canvas WASM (Retained Mode) loaded!".into());
}
