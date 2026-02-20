mod models;
mod rendering;
mod selection;
mod svg_export;

use std::cell::{Cell, RefCell};
use std::collections::HashSet;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use js_sys::Array as JsArray;

use crate::models::{BoundingBox, Element, Point, Shape, Style};

// ===== 내부 타입 =====

/// 도구 모드
#[derive(Clone, Copy, PartialEq)]
pub(crate) enum ToolMode {
    Pen,
    Eraser,
    Select,
    Line,
    Rectangle,
    Circle,
}

/// Undo/Redo 가능한 액션
#[derive(Clone)]
enum Action {
    /// 요소 추가 (그리기/도형)
    AddElement { element: Element },
    /// 요소 삭제 (인덱스 포함, z-order 복원용)
    DeleteElements { elements: Vec<(usize, Element)> },
    /// 요소 이동
    MoveElements { ids: Vec<u32>, dx: f64, dy: f64 },
    /// 붙여넣기
    PasteElements { elements: Vec<Element> },
    /// 전체 지우기
    ClearAll { elements: Vec<Element> },
}

// ===== CanvasInner: 모든 상태 및 로직 (JS에 직접 노출되지 않음) =====

pub(crate) struct CanvasInner {
    pub(crate) ctx: CanvasRenderingContext2d,

    // Retained mode: 모든 요소 저장
    pub(crate) elements: Vec<Element>,
    pub(crate) current_element: Option<Element>,
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
    pub(crate) clipboard: Vec<Element>,

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

    // 줌/팬 상태
    pub(crate) zoom: f64,
    pub(crate) pan_x: f64,
    pub(crate) pan_y: f64,
    pub(crate) logical_width: f64,
    pub(crate) logical_height: f64,

    // 도형 그리기 상태
    pub(crate) is_drawing_shape: bool,
    pub(crate) shape_start_x: f64,
    pub(crate) shape_start_y: f64,
    pub(crate) shape_end_x: f64,
    pub(crate) shape_end_y: f64,

    // 팬 인터랙션
    pub(crate) is_panning: bool,
    pub(crate) pan_start_x: f64,
    pub(crate) pan_start_y: f64,
    pub(crate) pan_origin_x: f64,
    pub(crate) pan_origin_y: f64,

    // 렌더링 최적화: dirty flag
    pub(crate) needs_render: bool,
}

impl CanvasInner {
    pub(crate) fn new(canvas_id: &str, dpr: f64) -> Result<CanvasInner, JsValue> {
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

        let logical_width = canvas.width() as f64 / dpr;
        let logical_height = canvas.height() as f64 / dpr;

        // 레티나 디스플레이 지원
        ctx.scale(dpr, dpr).ok();
        ctx.set_line_cap("round");
        ctx.set_line_join("round");

        Ok(CanvasInner {
            ctx,
            elements: Vec::new(),
            current_element: None,
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
            is_drawing_shape: false,
            shape_start_x: 0.0,
            shape_start_y: 0.0,
            shape_end_x: 0.0,
            shape_end_y: 0.0,
            zoom: 1.0,
            pan_x: 0.0,
            pan_y: 0.0,
            logical_width,
            logical_height,
            is_panning: false,
            pan_start_x: 0.0,
            pan_start_y: 0.0,
            pan_origin_x: 0.0,
            pan_origin_y: 0.0,
            needs_render: false,
        })
    }

    // ===== 기본 도구 =====

    /// 색상 설정 (도구 모드는 변경하지 않음)
    pub(crate) fn set_color(&mut self, color: &str) {
        self.color = color.to_string();
        self.needs_render = true;
    }

    /// 선 굵기 설정
    pub(crate) fn set_line_width(&mut self, width: f64) {
        self.line_width = width;
        self.needs_render = true;
    }

    /// 지우개 모드 설정
    pub(crate) fn set_eraser(&mut self, is_eraser: bool) {
        self.is_eraser = is_eraser;
        if is_eraser {
            self.tool_mode = ToolMode::Eraser;
        }
        self.needs_render = true;
    }

    /// 커서 위치 업데이트
    pub(crate) fn update_cursor(&mut self, x: f64, y: f64) {
        self.cursor_x = x;
        self.cursor_y = y;
        self.show_cursor = true;
        self.needs_render = true;
    }

    /// 커서 숨기기
    pub(crate) fn hide_cursor(&mut self) {
        self.show_cursor = false;
        self.needs_render = true;
    }

    /// 통합 도구 전환
    pub(crate) fn set_tool_mode(&mut self, mode: &str) {
        self.is_eraser = false;
        self.is_drawing = false;
        self.is_drawing_shape = false;
        match mode {
            "pen" => self.tool_mode = ToolMode::Pen,
            "eraser" => {
                self.tool_mode = ToolMode::Eraser;
                self.is_eraser = true;
            }
            "select" => self.tool_mode = ToolMode::Select,
            "line" => self.tool_mode = ToolMode::Line,
            "rect" => self.tool_mode = ToolMode::Rectangle,
            "circle" => self.tool_mode = ToolMode::Circle,
            _ => self.tool_mode = ToolMode::Pen,
        }
        self.needs_render = true;
    }

    /// 현재 도구가 도형 도구인지 확인
    pub(crate) fn is_shape_tool(&self) -> bool {
        matches!(
            self.tool_mode,
            ToolMode::Line | ToolMode::Rectangle | ToolMode::Circle
        )
    }

    // ===== 도형 도구 =====

    /// 도형 그리기 시작 (월드 좌표)
    pub(crate) fn start_shape(&mut self, x: f64, y: f64) {
        self.is_drawing_shape = true;
        self.shape_start_x = x;
        self.shape_start_y = y;
        self.shape_end_x = x;
        self.shape_end_y = y;
    }

    /// 도형 프리뷰 업데이트 (월드 좌표)
    pub(crate) fn update_shape(&mut self, x: f64, y: f64) {
        if !self.is_drawing_shape {
            return;
        }
        self.shape_end_x = x;
        self.shape_end_y = y;
        self.needs_render = true;
    }

    /// 도형 그리기 확정
    pub(crate) fn finish_shape(&mut self) {
        if !self.is_drawing_shape {
            return;
        }
        self.is_drawing_shape = false;

        let shape = match self.tool_mode {
            ToolMode::Line => Shape::Line {
                x1: self.shape_start_x,
                y1: self.shape_start_y,
                x2: self.shape_end_x,
                y2: self.shape_end_y,
            },
            ToolMode::Rectangle => {
                let x = self.shape_start_x.min(self.shape_end_x);
                let y = self.shape_start_y.min(self.shape_end_y);
                let w = (self.shape_end_x - self.shape_start_x).abs();
                let h = (self.shape_end_y - self.shape_start_y).abs();
                if w < 1.0 && h < 1.0 {
                    self.needs_render = true;
                    return;
                }
                Shape::Rect { x, y, w, h }
            }
            ToolMode::Circle => {
                let dx = self.shape_end_x - self.shape_start_x;
                let dy = self.shape_end_y - self.shape_start_y;
                let r = (dx * dx + dy * dy).sqrt();
                if r < 1.0 {
                    self.needs_render = true;
                    return;
                }
                Shape::Circle {
                    cx: self.shape_start_x,
                    cy: self.shape_start_y,
                    r,
                }
            }
            _ => {
                self.needs_render = true;
                return;
            }
        };

        let element = Element {
            id: self.next_id,
            shape,
            style: Style {
                color: self.color.clone(),
                width: self.line_width,
            },
        };
        self.next_id += 1;

        self.undo_stack.push(Action::AddElement {
            element: element.clone(),
        });
        self.redo_stack.clear();
        self.elements.push(element);
        self.needs_render = true;
    }

    // ===== 그리기 =====

    /// 그리기 시작 - 새 Freehand 요소 생성
    pub(crate) fn start_drawing(&mut self, x: f64, y: f64) {
        self.is_drawing = true;
        self.current_element = Some(Element {
            id: self.next_id,
            shape: Shape::Freehand {
                points: vec![Point { x, y }],
            },
            style: Style {
                color: if self.is_eraser {
                    "#ffffff".to_string()
                } else {
                    self.color.clone()
                },
                width: self.line_width,
            },
        });
        self.next_id += 1;
    }

    /// 그리기 중 - 점 추가
    pub(crate) fn draw(&mut self, x: f64, y: f64) {
        if !self.is_drawing {
            return;
        }

        if let Some(ref mut elem) = self.current_element {
            if let Shape::Freehand { ref mut points } = elem.shape {
                points.push(Point { x, y });
            }
        }
        self.needs_render = true;
    }

    /// 그리기 종료 - 요소 확정
    pub(crate) fn stop_drawing(&mut self) {
        self.is_drawing = false;
        if let Some(elem) = self.current_element.take() {
            if let Shape::Freehand { ref points } = elem.shape {
                if points.len() > 1 {
                    self.undo_stack.push(Action::AddElement {
                        element: elem.clone(),
                    });
                    self.redo_stack.clear();
                    self.elements.push(elem);
                }
            }
        }
    }

    /// 전체 지우기 (모든 요소 삭제)
    pub(crate) fn clear(&mut self) {
        if !self.elements.is_empty() {
            self.undo_stack.push(Action::ClearAll {
                elements: self.elements.clone(),
            });
            self.redo_stack.clear();
        }
        self.elements.clear();
        self.current_element = None;
        self.selected_ids.clear();
        self.needs_render = true;
    }

    // ===== 선택 도구 =====

    /// 선택 도구 모드 설정
    pub(crate) fn set_select_mode(&mut self, is_select: bool) {
        if is_select {
            self.tool_mode = ToolMode::Select;
            self.is_eraser = false;
            self.is_drawing = false;
        } else {
            self.tool_mode = ToolMode::Pen;
        }
    }

    /// 현재 선택 도구 모드인지 확인
    pub(crate) fn get_is_select_mode(&self) -> bool {
        self.tool_mode == ToolMode::Select
    }

    /// 좌표에서 요소 선택 시도 (역순 탐색으로 최상위 우선)
    pub(crate) fn try_select_at(&mut self, x: f64, y: f64, shift: bool) -> bool {
        let mut hit_id: Option<u32> = None;
        for elem in self.elements.iter().rev() {
            if elem.hit_test(x, y) {
                hit_id = Some(elem.id);
                break;
            }
        }

        self.needs_render = true;

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
    pub(crate) fn select_all(&mut self) {
        self.selected_ids.clear();
        for elem in &self.elements {
            self.selected_ids.insert(elem.id);
        }
        self.needs_render = true;
    }

    /// 선택 해제
    pub(crate) fn deselect_all(&mut self) {
        self.selected_ids.clear();
        self.needs_render = true;
    }

    /// 선택된 요소가 있는지 확인
    pub(crate) fn has_selection(&self) -> bool {
        !self.selected_ids.is_empty()
    }

    /// 좌표가 선택된 요소 위에 있는지 확인
    pub(crate) fn is_over_selected(&self, x: f64, y: f64) -> bool {
        for elem in &self.elements {
            if self.selected_ids.contains(&elem.id) && elem.hit_test(x, y) {
                return true;
            }
        }
        false
    }

    // ===== 이동 =====

    /// 이동 시작
    pub(crate) fn start_move(&mut self, x: f64, y: f64) {
        self.is_moving = true;
        self.move_start_x = x;
        self.move_start_y = y;
        self.move_total_dx = 0.0;
        self.move_total_dy = 0.0;
    }

    /// 이동 중 - 선택된 요소들을 델타만큼 이동
    pub(crate) fn move_selected(&mut self, x: f64, y: f64) {
        if !self.is_moving {
            return;
        }
        let dx = x - self.move_start_x;
        let dy = y - self.move_start_y;

        for elem in &mut self.elements {
            if self.selected_ids.contains(&elem.id) {
                elem.translate(dx, dy);
            }
        }

        self.move_total_dx += dx;
        self.move_total_dy += dy;
        self.move_start_x = x;
        self.move_start_y = y;
        self.needs_render = true;
    }

    /// 이동 종료
    pub(crate) fn stop_move(&mut self) {
        if self.is_moving
            && (self.move_total_dx != 0.0 || self.move_total_dy != 0.0)
        {
            self.undo_stack.push(Action::MoveElements {
                ids: self.selected_ids.iter().cloned().collect(),
                dx: self.move_total_dx,
                dy: self.move_total_dy,
            });
            self.redo_stack.clear();
        }
        self.is_moving = false;
    }

    // ===== 클립보드 =====

    /// 선택된 요소 복사
    pub(crate) fn copy_selected(&mut self) {
        self.clipboard.clear();
        for elem in &self.elements {
            if self.selected_ids.contains(&elem.id) {
                self.clipboard.push(elem.clone());
            }
        }
    }

    /// 클립보드에서 붙여넣기 (오프셋 적용, 새 ID 부여)
    pub(crate) fn paste(&mut self) {
        if self.clipboard.is_empty() {
            return;
        }

        let offset = 20.0;
        let start_id = self.next_id;

        let pasted: Vec<Element> = self
            .clipboard
            .iter()
            .enumerate()
            .map(|(i, orig)| {
                let mut e = orig.clone();
                e.id = start_id + i as u32;
                e.translate(offset, offset);
                e
            })
            .collect();

        self.next_id = start_id + pasted.len() as u32;
        self.selected_ids.clear();

        for e in &pasted {
            self.selected_ids.insert(e.id);
            self.elements.push(e.clone());
        }

        self.undo_stack.push(Action::PasteElements {
            elements: pasted,
        });
        self.redo_stack.clear();

        // 반복 붙여넣기 시 오프셋 누적을 위해 클립보드 갱신
        self.clipboard = self
            .elements
            .iter()
            .filter(|e| self.selected_ids.contains(&e.id))
            .cloned()
            .collect();

        self.needs_render = true;
    }

    /// 선택된 요소 삭제 (인덱스 저장으로 z-order 보존)
    pub(crate) fn delete_selected(&mut self) {
        let deleted: Vec<(usize, Element)> = self
            .elements
            .iter()
            .enumerate()
            .filter(|(_, e)| self.selected_ids.contains(&e.id))
            .map(|(i, e)| (i, e.clone()))
            .collect();

        if !deleted.is_empty() {
            self.undo_stack.push(Action::DeleteElements {
                elements: deleted,
            });
            self.redo_stack.clear();
        }

        self.elements
            .retain(|e| !self.selected_ids.contains(&e.id));
        self.selected_ids.clear();
        self.needs_render = true;
    }

    // ===== Undo / Redo =====

    /// Undo 가능 여부
    pub(crate) fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Redo 가능 여부
    pub(crate) fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// 실행 취소
    pub(crate) fn undo(&mut self) {
        let action = match self.undo_stack.pop() {
            Some(a) => a,
            None => return,
        };

        match &action {
            Action::AddElement { element } => {
                self.elements.retain(|e| e.id != element.id);
            }
            Action::DeleteElements { elements } => {
                for (idx, e) in elements {
                    let insert_at = (*idx).min(self.elements.len());
                    self.elements.insert(insert_at, e.clone());
                }
            }
            Action::MoveElements { ids, dx, dy } => {
                for elem in &mut self.elements {
                    if ids.contains(&elem.id) {
                        elem.translate(-dx, -dy);
                    }
                }
            }
            Action::PasteElements { elements } => {
                let ids: HashSet<u32> = elements.iter().map(|e| e.id).collect();
                self.elements.retain(|e| !ids.contains(&e.id));
            }
            Action::ClearAll { elements } => {
                self.elements = elements.clone();
            }
        }

        self.redo_stack.push(action);
        self.selected_ids.clear();
        self.needs_render = true;
    }

    /// 다시 실행
    pub(crate) fn redo(&mut self) {
        let action = match self.redo_stack.pop() {
            Some(a) => a,
            None => return,
        };

        match &action {
            Action::AddElement { element } => {
                self.elements.push(element.clone());
            }
            Action::DeleteElements { elements } => {
                let ids: HashSet<u32> = elements.iter().map(|(_, e)| e.id).collect();
                self.elements.retain(|e| !ids.contains(&e.id));
            }
            Action::MoveElements { ids, dx, dy } => {
                for elem in &mut self.elements {
                    if ids.contains(&elem.id) {
                        elem.translate(*dx, *dy);
                    }
                }
            }
            Action::PasteElements { elements } => {
                for e in elements {
                    self.elements.push(e.clone());
                }
            }
            Action::ClearAll { .. } => {
                self.elements.clear();
            }
        }

        self.undo_stack.push(action);
        self.selected_ids.clear();
        self.needs_render = true;
    }

    // ===== 러버밴드 (드래그 영역) 선택 =====

    /// 러버밴드 선택 시작
    pub(crate) fn start_rubber_band(&mut self, x: f64, y: f64) {
        self.is_rubber_band = true;
        self.rubber_band_start_x = x;
        self.rubber_band_start_y = y;
        self.rubber_band_end_x = x;
        self.rubber_band_end_y = y;
    }

    /// 러버밴드 드래그 중 — 영역 업데이트
    pub(crate) fn update_rubber_band(&mut self, x: f64, y: f64) {
        if !self.is_rubber_band {
            return;
        }
        self.rubber_band_end_x = x;
        self.rubber_band_end_y = y;
        self.needs_render = true;
    }

    /// 러버밴드 선택 확정 — 스크린→월드 변환 후 교차 검사
    pub(crate) fn finish_rubber_band(&mut self, shift: bool) {
        if !self.is_rubber_band {
            return;
        }
        self.is_rubber_band = false;

        let w_sx = self.screen_to_world_x(self.rubber_band_start_x);
        let w_sy = self.screen_to_world_y(self.rubber_band_start_y);
        let w_ex = self.screen_to_world_x(self.rubber_band_end_x);
        let w_ey = self.screen_to_world_y(self.rubber_band_end_y);

        let rect = BoundingBox {
            min_x: w_sx.min(w_ex),
            min_y: w_sy.min(w_ey),
            max_x: w_sx.max(w_ex),
            max_y: w_sy.max(w_ey),
        };

        if !shift {
            self.selected_ids.clear();
        }

        for elem in &self.elements {
            if let Some(bb) = elem.bounding_box() {
                if rect.intersects(&bb) {
                    self.selected_ids.insert(elem.id);
                }
            }
        }

        self.needs_render = true;
    }
}

// ===== 줌 / 팬 =====

impl CanvasInner {
    /// 스크린 좌표 → 월드 좌표 (X)
    pub(crate) fn screen_to_world_x(&self, sx: f64) -> f64 {
        (sx - self.pan_x) / self.zoom
    }

    /// 스크린 좌표 → 월드 좌표 (Y)
    pub(crate) fn screen_to_world_y(&self, sy: f64) -> f64 {
        (sy - self.pan_y) / self.zoom
    }

    /// 커서 위치 기준 줌 (휠 이벤트용)
    pub(crate) fn zoom_at(&mut self, screen_x: f64, screen_y: f64, delta: f64) {
        let factor = if delta < 0.0 { 1.1 } else { 1.0 / 1.1 };
        let new_zoom = (self.zoom * factor).clamp(0.1, 10.0);

        let wx = (screen_x - self.pan_x) / self.zoom;
        let wy = (screen_y - self.pan_y) / self.zoom;

        self.zoom = new_zoom;
        self.pan_x = screen_x - wx * self.zoom;
        self.pan_y = screen_y - wy * self.zoom;

        self.needs_render = true;
    }

    /// 줌 레벨 직접 설정 (캔버스 중심 기준)
    pub(crate) fn set_zoom(&mut self, new_zoom: f64) {
        let new_zoom = new_zoom.clamp(0.1, 10.0);
        let cx = self.logical_width / 2.0;
        let cy = self.logical_height / 2.0;

        let wx = (cx - self.pan_x) / self.zoom;
        let wy = (cy - self.pan_y) / self.zoom;

        self.zoom = new_zoom;
        self.pan_x = cx - wx * self.zoom;
        self.pan_y = cy - wy * self.zoom;

        self.needs_render = true;
    }

    /// 팬 시작 (스크린 좌표)
    pub(crate) fn start_pan(&mut self, sx: f64, sy: f64) {
        self.is_panning = true;
        self.pan_start_x = sx;
        self.pan_start_y = sy;
        self.pan_origin_x = self.pan_x;
        self.pan_origin_y = self.pan_y;
    }

    /// 팬 업데이트
    pub(crate) fn update_pan(&mut self, sx: f64, sy: f64) {
        if !self.is_panning {
            return;
        }
        self.pan_x = self.pan_origin_x + (sx - self.pan_start_x);
        self.pan_y = self.pan_origin_y + (sy - self.pan_start_y);
        self.needs_render = true;
    }

    /// 팬 종료
    pub(crate) fn stop_pan(&mut self) {
        self.is_panning = false;
    }

    /// 전체 컨텐츠에 맞춤 (fit to view)
    pub(crate) fn fit_to_view(&mut self) {
        if self.elements.is_empty() {
            self.reset_view();
            return;
        }

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

        if min_x >= max_x || min_y >= max_y {
            self.reset_view();
            return;
        }

        let content_w = max_x - min_x;
        let content_h = max_y - min_y;
        let padding = 40.0;

        let scale_x = (self.logical_width - padding * 2.0) / content_w;
        let scale_y = (self.logical_height - padding * 2.0) / content_h;
        let new_zoom = scale_x.min(scale_y).clamp(0.1, 10.0);

        let center_x = (min_x + max_x) / 2.0;
        let center_y = (min_y + max_y) / 2.0;

        self.zoom = new_zoom;
        self.pan_x = self.logical_width / 2.0 - center_x * self.zoom;
        self.pan_y = self.logical_height / 2.0 - center_y * self.zoom;

        self.needs_render = true;
    }

    /// 뷰 초기화 (1:1)
    pub(crate) fn reset_view(&mut self) {
        self.zoom = 1.0;
        self.pan_x = 0.0;
        self.pan_y = 0.0;
        self.needs_render = true;
    }
}

// ===== rAF 헬퍼 =====

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

/// Rust 소유 rAF 렌더 루프 시작
fn start_render_loop(inner: Rc<RefCell<CanvasInner>>) {
    let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        match inner.try_borrow_mut() {
            Ok(mut state) => {
                if state.needs_render {
                    state.needs_render = false;
                    state.render();
                }
            }
            Err(_) => {
                web_sys::console::warn_1(
                    &"render loop: CanvasInner already borrowed".into(),
                );
            }
        }
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
    // Rc 순환 참조 (f → Closure → f)로 영구 유지. 페이지 수명 = 루프 수명.
}

// ===== Canvas: JS에 노출되는 래퍼 (Rc<RefCell<CanvasInner>>) =====

#[wasm_bindgen]
pub struct Canvas {
    inner: Rc<RefCell<CanvasInner>>,
    loop_running: Cell<bool>,
}

#[wasm_bindgen]
impl Canvas {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str, dpr: f64) -> Result<Canvas, JsValue> {
        let inner = CanvasInner::new(canvas_id, dpr)?;
        Ok(Canvas {
            inner: Rc::new(RefCell::new(inner)),
            loop_running: Cell::new(false),
        })
    }

    /// Rust 소유 rAF 렌더 루프 시작 (중복 호출 방지)
    #[wasm_bindgen]
    pub fn start_render_loop(&self) {
        if self.loop_running.get() {
            return;
        }
        self.loop_running.set(true);
        start_render_loop(self.inner.clone());
    }

    // ===== 기본 도구 위임 =====

    #[wasm_bindgen]
    pub fn set_color(&self, color: &str) {
        self.inner.borrow_mut().set_color(color);
    }

    #[wasm_bindgen]
    pub fn set_line_width(&self, width: f64) {
        self.inner.borrow_mut().set_line_width(width);
    }

    #[wasm_bindgen]
    pub fn set_eraser(&self, is_eraser: bool) {
        self.inner.borrow_mut().set_eraser(is_eraser);
    }

    #[wasm_bindgen]
    pub fn get_is_drawing(&self) -> bool {
        self.inner.borrow().is_drawing
    }

    #[wasm_bindgen]
    pub fn get_is_eraser(&self) -> bool {
        self.inner.borrow().is_eraser
    }

    #[wasm_bindgen]
    pub fn update_cursor(&self, x: f64, y: f64) {
        self.inner.borrow_mut().update_cursor(x, y);
    }

    #[wasm_bindgen]
    pub fn hide_cursor(&self) {
        self.inner.borrow_mut().hide_cursor();
    }

    #[wasm_bindgen]
    pub fn set_tool_mode(&self, mode: &str) {
        self.inner.borrow_mut().set_tool_mode(mode);
    }

    #[wasm_bindgen]
    pub fn is_shape_tool(&self) -> bool {
        self.inner.borrow().is_shape_tool()
    }

    #[wasm_bindgen]
    pub fn get_is_drawing_shape(&self) -> bool {
        self.inner.borrow().is_drawing_shape
    }

    // ===== 도형 도구 위임 =====

    #[wasm_bindgen]
    pub fn start_shape(&self, x: f64, y: f64) {
        self.inner.borrow_mut().start_shape(x, y);
    }

    #[wasm_bindgen]
    pub fn update_shape(&self, x: f64, y: f64) {
        self.inner.borrow_mut().update_shape(x, y);
    }

    #[wasm_bindgen]
    pub fn finish_shape(&self) {
        self.inner.borrow_mut().finish_shape();
    }

    // ===== 그리기 위임 =====

    #[wasm_bindgen]
    pub fn start_drawing(&self, x: f64, y: f64) {
        self.inner.borrow_mut().start_drawing(x, y);
    }

    #[wasm_bindgen]
    pub fn draw(&self, x: f64, y: f64) {
        self.inner.borrow_mut().draw(x, y);
    }

    #[wasm_bindgen]
    pub fn stop_drawing(&self) {
        self.inner.borrow_mut().stop_drawing();
    }

    #[wasm_bindgen]
    pub fn clear(&self) {
        self.inner.borrow_mut().clear();
    }

    #[wasm_bindgen]
    pub fn get_stroke_count(&self) -> usize {
        self.inner.borrow().elements.len()
    }

    // ===== 선택 도구 위임 =====

    #[wasm_bindgen]
    pub fn set_select_mode(&self, is_select: bool) {
        self.inner.borrow_mut().set_select_mode(is_select);
    }

    #[wasm_bindgen]
    pub fn get_is_select_mode(&self) -> bool {
        self.inner.borrow().get_is_select_mode()
    }

    #[wasm_bindgen]
    pub fn try_select_at(&self, x: f64, y: f64, shift: bool) -> bool {
        self.inner.borrow_mut().try_select_at(x, y, shift)
    }

    #[wasm_bindgen]
    pub fn select_all(&self) {
        self.inner.borrow_mut().select_all();
    }

    #[wasm_bindgen]
    pub fn deselect_all(&self) {
        self.inner.borrow_mut().deselect_all();
    }

    #[wasm_bindgen]
    pub fn has_selection(&self) -> bool {
        self.inner.borrow().has_selection()
    }

    #[wasm_bindgen]
    pub fn is_over_selected(&self, x: f64, y: f64) -> bool {
        self.inner.borrow().is_over_selected(x, y)
    }

    // ===== 이동 위임 =====

    #[wasm_bindgen]
    pub fn get_is_moving(&self) -> bool {
        self.inner.borrow().is_moving
    }

    #[wasm_bindgen]
    pub fn start_move(&self, x: f64, y: f64) {
        self.inner.borrow_mut().start_move(x, y);
    }

    #[wasm_bindgen]
    pub fn move_selected(&self, x: f64, y: f64) {
        self.inner.borrow_mut().move_selected(x, y);
    }

    #[wasm_bindgen]
    pub fn stop_move(&self) {
        self.inner.borrow_mut().stop_move();
    }

    // ===== 클립보드 위임 =====

    #[wasm_bindgen]
    pub fn copy_selected(&self) {
        self.inner.borrow_mut().copy_selected();
    }

    #[wasm_bindgen]
    pub fn paste(&self) {
        self.inner.borrow_mut().paste();
    }

    #[wasm_bindgen]
    pub fn delete_selected(&self) {
        self.inner.borrow_mut().delete_selected();
    }

    // ===== Undo / Redo 위임 =====

    #[wasm_bindgen]
    pub fn can_undo(&self) -> bool {
        self.inner.borrow().can_undo()
    }

    #[wasm_bindgen]
    pub fn can_redo(&self) -> bool {
        self.inner.borrow().can_redo()
    }

    #[wasm_bindgen]
    pub fn undo(&self) {
        self.inner.borrow_mut().undo();
    }

    #[wasm_bindgen]
    pub fn redo(&self) {
        self.inner.borrow_mut().redo();
    }

    // ===== 러버밴드 위임 =====

    #[wasm_bindgen]
    pub fn start_rubber_band(&self, x: f64, y: f64) {
        self.inner.borrow_mut().start_rubber_band(x, y);
    }

    #[wasm_bindgen]
    pub fn update_rubber_band(&self, x: f64, y: f64) {
        self.inner.borrow_mut().update_rubber_band(x, y);
    }

    #[wasm_bindgen]
    pub fn finish_rubber_band(&self, shift: bool) {
        self.inner.borrow_mut().finish_rubber_band(shift);
    }

    #[wasm_bindgen]
    pub fn get_is_rubber_band(&self) -> bool {
        self.inner.borrow().is_rubber_band
    }

    // ===== 줌 / 팬 위임 =====

    #[wasm_bindgen]
    pub fn screen_to_world_x(&self, sx: f64) -> f64 {
        self.inner.borrow().screen_to_world_x(sx)
    }

    #[wasm_bindgen]
    pub fn screen_to_world_y(&self, sy: f64) -> f64 {
        self.inner.borrow().screen_to_world_y(sy)
    }

    #[wasm_bindgen]
    pub fn zoom_at(&self, screen_x: f64, screen_y: f64, delta: f64) {
        self.inner.borrow_mut().zoom_at(screen_x, screen_y, delta);
    }

    #[wasm_bindgen]
    pub fn set_zoom(&self, new_zoom: f64) {
        self.inner.borrow_mut().set_zoom(new_zoom);
    }

    #[wasm_bindgen]
    pub fn get_zoom(&self) -> f64 {
        self.inner.borrow().zoom
    }

    #[wasm_bindgen]
    pub fn start_pan(&self, sx: f64, sy: f64) {
        self.inner.borrow_mut().start_pan(sx, sy);
    }

    #[wasm_bindgen]
    pub fn update_pan(&self, sx: f64, sy: f64) {
        self.inner.borrow_mut().update_pan(sx, sy);
    }

    #[wasm_bindgen]
    pub fn stop_pan(&self) {
        self.inner.borrow_mut().stop_pan();
    }

    #[wasm_bindgen]
    pub fn get_is_panning(&self) -> bool {
        self.inner.borrow().is_panning
    }

    #[wasm_bindgen]
    pub fn fit_to_view(&self) {
        self.inner.borrow_mut().fit_to_view();
    }

    #[wasm_bindgen]
    pub fn reset_view(&self) {
        self.inner.borrow_mut().reset_view();
    }

    // ===== 렌더링 / 내보내기 =====

    #[wasm_bindgen]
    pub fn render(&self) {
        self.inner.borrow().render();
    }

    #[wasm_bindgen]
    pub fn export_svg(&self) -> String {
        self.inner.borrow().export_svg()
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    web_sys::console::log_1(&"Rust Canvas WASM (Retained Mode) loaded!".into());
}
