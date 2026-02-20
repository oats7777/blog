/* tslint:disable */
/* eslint-disable */

export class Canvas {
    free(): void;
    [Symbol.dispose](): void;
    can_redo(): boolean;
    can_undo(): boolean;
    clear(): void;
    copy_selected(): void;
    delete_selected(): void;
    deselect_all(): void;
    draw(x: number, y: number): void;
    export_svg(): string;
    finish_rubber_band(shift: boolean): void;
    finish_shape(): void;
    fit_to_view(): void;
    get_is_drawing(): boolean;
    get_is_drawing_shape(): boolean;
    get_is_eraser(): boolean;
    get_is_moving(): boolean;
    get_is_panning(): boolean;
    get_is_rubber_band(): boolean;
    get_is_select_mode(): boolean;
    get_stroke_count(): number;
    get_zoom(): number;
    has_selection(): boolean;
    hide_cursor(): void;
    is_over_selected(x: number, y: number): boolean;
    is_shape_tool(): boolean;
    move_selected(x: number, y: number): void;
    constructor(canvas_id: string, dpr: number);
    paste(): void;
    redo(): void;
    render(): void;
    reset_view(): void;
    screen_to_world_x(sx: number): number;
    screen_to_world_y(sy: number): number;
    select_all(): void;
    set_color(color: string): void;
    set_eraser(is_eraser: boolean): void;
    set_line_width(width: number): void;
    set_select_mode(is_select: boolean): void;
    set_tool_mode(mode: string): void;
    set_zoom(new_zoom: number): void;
    start_drawing(x: number, y: number): void;
    start_move(x: number, y: number): void;
    start_pan(sx: number, sy: number): void;
    /**
     * Rust 소유 rAF 렌더 루프 시작 (중복 호출 방지)
     */
    start_render_loop(): void;
    start_rubber_band(x: number, y: number): void;
    start_shape(x: number, y: number): void;
    stop_drawing(): void;
    stop_move(): void;
    stop_pan(): void;
    try_select_at(x: number, y: number, shift: boolean): boolean;
    undo(): void;
    update_cursor(x: number, y: number): void;
    update_pan(sx: number, sy: number): void;
    update_rubber_band(x: number, y: number): void;
    update_shape(x: number, y: number): void;
    zoom_at(screen_x: number, screen_y: number, delta: number): void;
}

export function main(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_canvas_free: (a: number, b: number) => void;
    readonly canvas_new: (a: number, b: number, c: number) => [number, number, number];
    readonly canvas_start_render_loop: (a: number) => void;
    readonly canvas_set_color: (a: number, b: number, c: number) => void;
    readonly canvas_set_line_width: (a: number, b: number) => void;
    readonly canvas_set_eraser: (a: number, b: number) => void;
    readonly canvas_get_is_drawing: (a: number) => number;
    readonly canvas_get_is_eraser: (a: number) => number;
    readonly canvas_update_cursor: (a: number, b: number, c: number) => void;
    readonly canvas_hide_cursor: (a: number) => void;
    readonly canvas_set_tool_mode: (a: number, b: number, c: number) => void;
    readonly canvas_is_shape_tool: (a: number) => number;
    readonly canvas_get_is_drawing_shape: (a: number) => number;
    readonly canvas_start_shape: (a: number, b: number, c: number) => void;
    readonly canvas_update_shape: (a: number, b: number, c: number) => void;
    readonly canvas_finish_shape: (a: number) => void;
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
    readonly canvas_screen_to_world_x: (a: number, b: number) => number;
    readonly canvas_screen_to_world_y: (a: number, b: number) => number;
    readonly canvas_zoom_at: (a: number, b: number, c: number, d: number) => void;
    readonly canvas_set_zoom: (a: number, b: number) => void;
    readonly canvas_get_zoom: (a: number) => number;
    readonly canvas_start_pan: (a: number, b: number, c: number) => void;
    readonly canvas_update_pan: (a: number, b: number, c: number) => void;
    readonly canvas_stop_pan: (a: number) => void;
    readonly canvas_get_is_panning: (a: number) => number;
    readonly canvas_fit_to_view: (a: number) => void;
    readonly canvas_reset_view: (a: number) => void;
    readonly canvas_render: (a: number) => void;
    readonly canvas_export_svg: (a: number) => [number, number];
    readonly main: () => void;
    readonly wasm_bindgen__closure__destroy__he6ffe883f7d79201: (a: number, b: number) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h4563321ca26cc569: (a: number, b: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
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
