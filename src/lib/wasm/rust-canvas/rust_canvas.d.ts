/* tslint:disable */
/* eslint-disable */

/**
 * 캔버스 메인 구조체
 */
export class Canvas {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Redo 가능 여부
     */
    can_redo(): boolean;
    /**
     * Undo 가능 여부
     */
    can_undo(): boolean;
    /**
     * 전체 지우기 (모든 스트로크 삭제)
     */
    clear(): void;
    /**
     * 선택된 스트로크 복사
     */
    copy_selected(): void;
    /**
     * 선택된 스트로크 삭제
     */
    delete_selected(): void;
    /**
     * 선택 해제
     */
    deselect_all(): void;
    /**
     * 그리기 중 - 점 추가 및 렌더링
     */
    draw(x: number, y: number): void;
    /**
     * 러버밴드 선택 확정 — 영역 내 스트로크 선택
     */
    finish_rubber_band(shift: boolean): void;
    /**
     * 그리기 상태 확인
     */
    get_is_drawing(): boolean;
    /**
     * 지우개 모드 확인
     */
    get_is_eraser(): boolean;
    /**
     * 이동 중인지 확인
     */
    get_is_moving(): boolean;
    /**
     * 러버밴드 드래그 중인지 확인
     */
    get_is_rubber_band(): boolean;
    /**
     * 현재 선택 도구 모드인지 확인
     */
    get_is_select_mode(): boolean;
    /**
     * 스트로크 개수 반환 (디버깅용)
     */
    get_stroke_count(): number;
    /**
     * 선택된 스트로크가 있는지 확인
     */
    has_selection(): boolean;
    /**
     * 커서 숨기기
     */
    hide_cursor(): void;
    /**
     * 좌표가 선택된 스트로크 위에 있는지 확인
     */
    is_over_selected(x: number, y: number): boolean;
    /**
     * 이동 중 - 선택된 스트로크들을 델타만큼 이동
     */
    move_selected(x: number, y: number): void;
    constructor(canvas_id: string, dpr: number);
    /**
     * 클립보드에서 붙여넣기 (오프셋 적용, 새 ID 부여)
     */
    paste(): void;
    /**
     * 다시 실행
     */
    redo(): void;
    /**
     * 전체 렌더링 (Retained Mode 핵심)
     */
    render(): void;
    /**
     * 전체 선택
     */
    select_all(): void;
    /**
     * 색상 설정
     */
    set_color(color: string): void;
    /**
     * 지우개 모드 설정
     */
    set_eraser(is_eraser: boolean): void;
    /**
     * 선 굵기 설정
     */
    set_line_width(width: number): void;
    /**
     * 선택 도구 모드 설정
     */
    set_select_mode(is_select: boolean): void;
    /**
     * 그리기 시작 - 새 스트로크 생성
     */
    start_drawing(x: number, y: number): void;
    /**
     * 이동 시작
     */
    start_move(x: number, y: number): void;
    /**
     * 러버밴드 선택 시작
     */
    start_rubber_band(x: number, y: number): void;
    /**
     * 그리기 종료 - 스트로크 확정
     */
    stop_drawing(): void;
    /**
     * 이동 종료
     */
    stop_move(): void;
    /**
     * 좌표에서 스트로크 선택 시도 (역순 탐색으로 최상위 우선)
     */
    try_select_at(x: number, y: number, shift: boolean): boolean;
    /**
     * 실행 취소
     */
    undo(): void;
    /**
     * 커서 위치 업데이트
     */
    update_cursor(x: number, y: number): void;
    /**
     * 러버밴드 드래그 중 — 영역 업데이트 및 렌더링
     */
    update_rubber_band(x: number, y: number): void;
}

export function main(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_canvas_free: (a: number, b: number) => void;
    readonly canvas_new: (a: number, b: number, c: number) => [number, number, number];
    readonly canvas_set_color: (a: number, b: number, c: number) => void;
    readonly canvas_set_line_width: (a: number, b: number) => void;
    readonly canvas_set_eraser: (a: number, b: number) => void;
    readonly canvas_get_is_drawing: (a: number) => number;
    readonly canvas_get_is_eraser: (a: number) => number;
    readonly canvas_update_cursor: (a: number, b: number, c: number) => void;
    readonly canvas_hide_cursor: (a: number) => void;
    readonly canvas_start_drawing: (a: number, b: number, c: number) => void;
    readonly canvas_draw: (a: number, b: number, c: number) => void;
    readonly canvas_stop_drawing: (a: number) => void;
    readonly canvas_clear: (a: number) => void;
    readonly canvas_get_stroke_count: (a: number) => number;
    readonly canvas_set_select_mode: (a: number, b: number) => void;
    readonly canvas_get_is_select_mode: (a: number) => number;
    readonly canvas_try_select_at: (a: number, b: number, c: number, d: number) => number;
    readonly canvas_select_all: (a: number) => void;
    readonly canvas_deselect_all: (a: number) => void;
    readonly canvas_has_selection: (a: number) => number;
    readonly canvas_is_over_selected: (a: number, b: number, c: number) => number;
    readonly canvas_get_is_moving: (a: number) => number;
    readonly canvas_start_move: (a: number, b: number, c: number) => void;
    readonly canvas_move_selected: (a: number, b: number, c: number) => void;
    readonly canvas_stop_move: (a: number) => void;
    readonly canvas_copy_selected: (a: number) => void;
    readonly canvas_paste: (a: number) => void;
    readonly canvas_delete_selected: (a: number) => void;
    readonly canvas_can_undo: (a: number) => number;
    readonly canvas_can_redo: (a: number) => number;
    readonly canvas_undo: (a: number) => void;
    readonly canvas_redo: (a: number) => void;
    readonly canvas_start_rubber_band: (a: number, b: number, c: number) => void;
    readonly canvas_update_rubber_band: (a: number, b: number, c: number) => void;
    readonly canvas_finish_rubber_band: (a: number, b: number) => void;
    readonly canvas_get_is_rubber_band: (a: number) => number;
    readonly main: () => void;
    readonly canvas_render: (a: number) => void;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
