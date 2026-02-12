/* tslint:disable */
/* eslint-disable */

/**
 * 캔버스 메인 구조체
 */
export class Canvas {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * 전체 지우기 (모든 스트로크 삭제)
     */
    clear(): void;
    /**
     * 그리기 중 - 점 추가 및 렌더링
     */
    draw(x: number, y: number): void;
    /**
     * 그리기 상태 확인
     */
    get_is_drawing(): boolean;
    /**
     * 지우개 모드 확인
     */
    get_is_eraser(): boolean;
    /**
     * 스트로크 개수 반환 (디버깅용)
     */
    get_stroke_count(): number;
    constructor(canvas_id: string, dpr: number);
    /**
     * 전체 렌더링 (Retained Mode 핵심)
     */
    render(): void;
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
     * 그리기 시작 - 새 스트로크 생성
     */
    start_drawing(x: number, y: number): void;
    /**
     * 그리기 종료 - 스트로크 확정
     */
    stop_drawing(): void;
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
    readonly canvas_start_drawing: (a: number, b: number, c: number) => void;
    readonly canvas_draw: (a: number, b: number, c: number) => void;
    readonly canvas_stop_drawing: (a: number) => void;
    readonly canvas_render: (a: number) => void;
    readonly canvas_clear: (a: number) => void;
    readonly canvas_get_stroke_count: (a: number) => number;
    readonly main: () => void;
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
