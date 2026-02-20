/* @ts-self-types="./rust_canvas.d.ts" */

export class Canvas {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        CanvasFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_canvas_free(ptr, 0);
    }
    /**
     * @returns {boolean}
     */
    can_redo() {
        const ret = wasm.canvas_can_redo(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {boolean}
     */
    can_undo() {
        const ret = wasm.canvas_can_undo(this.__wbg_ptr);
        return ret !== 0;
    }
    clear() {
        wasm.canvas_clear(this.__wbg_ptr);
    }
    copy_selected() {
        wasm.canvas_copy_selected(this.__wbg_ptr);
    }
    delete_selected() {
        wasm.canvas_delete_selected(this.__wbg_ptr);
    }
    deselect_all() {
        wasm.canvas_deselect_all(this.__wbg_ptr);
    }
    /**
     * @param {number} x
     * @param {number} y
     */
    draw(x, y) {
        wasm.canvas_draw(this.__wbg_ptr, x, y);
    }
    /**
     * @returns {string}
     */
    export_svg() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.canvas_export_svg(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {boolean} shift
     */
    finish_rubber_band(shift) {
        wasm.canvas_finish_rubber_band(this.__wbg_ptr, shift);
    }
    finish_shape() {
        wasm.canvas_finish_shape(this.__wbg_ptr);
    }
    fit_to_view() {
        wasm.canvas_fit_to_view(this.__wbg_ptr);
    }
    /**
     * @returns {boolean}
     */
    get_is_drawing() {
        const ret = wasm.canvas_get_is_drawing(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {boolean}
     */
    get_is_drawing_shape() {
        const ret = wasm.canvas_get_is_drawing_shape(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {boolean}
     */
    get_is_eraser() {
        const ret = wasm.canvas_get_is_eraser(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {boolean}
     */
    get_is_moving() {
        const ret = wasm.canvas_get_is_moving(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {boolean}
     */
    get_is_panning() {
        const ret = wasm.canvas_get_is_panning(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {boolean}
     */
    get_is_rubber_band() {
        const ret = wasm.canvas_get_is_rubber_band(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {boolean}
     */
    get_is_select_mode() {
        const ret = wasm.canvas_get_is_select_mode(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {number}
     */
    get_stroke_count() {
        const ret = wasm.canvas_get_stroke_count(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    get_zoom() {
        const ret = wasm.canvas_get_zoom(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {boolean}
     */
    has_selection() {
        const ret = wasm.canvas_has_selection(this.__wbg_ptr);
        return ret !== 0;
    }
    hide_cursor() {
        wasm.canvas_hide_cursor(this.__wbg_ptr);
    }
    /**
     * @param {number} x
     * @param {number} y
     * @returns {boolean}
     */
    is_over_selected(x, y) {
        const ret = wasm.canvas_is_over_selected(this.__wbg_ptr, x, y);
        return ret !== 0;
    }
    /**
     * @returns {boolean}
     */
    is_shape_tool() {
        const ret = wasm.canvas_is_shape_tool(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @param {number} x
     * @param {number} y
     */
    move_selected(x, y) {
        wasm.canvas_move_selected(this.__wbg_ptr, x, y);
    }
    /**
     * @param {string} canvas_id
     * @param {number} dpr
     */
    constructor(canvas_id, dpr) {
        const ptr0 = passStringToWasm0(canvas_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.canvas_new(ptr0, len0, dpr);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0] >>> 0;
        CanvasFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    paste() {
        wasm.canvas_paste(this.__wbg_ptr);
    }
    redo() {
        wasm.canvas_redo(this.__wbg_ptr);
    }
    render() {
        wasm.canvas_render(this.__wbg_ptr);
    }
    reset_view() {
        wasm.canvas_reset_view(this.__wbg_ptr);
    }
    /**
     * @param {number} sx
     * @returns {number}
     */
    screen_to_world_x(sx) {
        const ret = wasm.canvas_screen_to_world_x(this.__wbg_ptr, sx);
        return ret;
    }
    /**
     * @param {number} sy
     * @returns {number}
     */
    screen_to_world_y(sy) {
        const ret = wasm.canvas_screen_to_world_y(this.__wbg_ptr, sy);
        return ret;
    }
    select_all() {
        wasm.canvas_select_all(this.__wbg_ptr);
    }
    /**
     * @param {string} color
     */
    set_color(color) {
        const ptr0 = passStringToWasm0(color, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.canvas_set_color(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @param {boolean} is_eraser
     */
    set_eraser(is_eraser) {
        wasm.canvas_set_eraser(this.__wbg_ptr, is_eraser);
    }
    /**
     * @param {number} width
     */
    set_line_width(width) {
        wasm.canvas_set_line_width(this.__wbg_ptr, width);
    }
    /**
     * @param {boolean} is_select
     */
    set_select_mode(is_select) {
        wasm.canvas_set_select_mode(this.__wbg_ptr, is_select);
    }
    /**
     * @param {string} mode
     */
    set_tool_mode(mode) {
        const ptr0 = passStringToWasm0(mode, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.canvas_set_tool_mode(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @param {number} new_zoom
     */
    set_zoom(new_zoom) {
        wasm.canvas_set_zoom(this.__wbg_ptr, new_zoom);
    }
    /**
     * @param {number} x
     * @param {number} y
     */
    start_drawing(x, y) {
        wasm.canvas_start_drawing(this.__wbg_ptr, x, y);
    }
    /**
     * @param {number} x
     * @param {number} y
     */
    start_move(x, y) {
        wasm.canvas_start_move(this.__wbg_ptr, x, y);
    }
    /**
     * @param {number} sx
     * @param {number} sy
     */
    start_pan(sx, sy) {
        wasm.canvas_start_pan(this.__wbg_ptr, sx, sy);
    }
    /**
     * Rust 소유 rAF 렌더 루프 시작 (중복 호출 방지)
     */
    start_render_loop() {
        wasm.canvas_start_render_loop(this.__wbg_ptr);
    }
    /**
     * @param {number} x
     * @param {number} y
     */
    start_rubber_band(x, y) {
        wasm.canvas_start_rubber_band(this.__wbg_ptr, x, y);
    }
    /**
     * @param {number} x
     * @param {number} y
     */
    start_shape(x, y) {
        wasm.canvas_start_shape(this.__wbg_ptr, x, y);
    }
    stop_drawing() {
        wasm.canvas_stop_drawing(this.__wbg_ptr);
    }
    stop_move() {
        wasm.canvas_stop_move(this.__wbg_ptr);
    }
    stop_pan() {
        wasm.canvas_stop_pan(this.__wbg_ptr);
    }
    /**
     * @param {number} x
     * @param {number} y
     * @param {boolean} shift
     * @returns {boolean}
     */
    try_select_at(x, y, shift) {
        const ret = wasm.canvas_try_select_at(this.__wbg_ptr, x, y, shift);
        return ret !== 0;
    }
    undo() {
        wasm.canvas_undo(this.__wbg_ptr);
    }
    /**
     * @param {number} x
     * @param {number} y
     */
    update_cursor(x, y) {
        wasm.canvas_update_cursor(this.__wbg_ptr, x, y);
    }
    /**
     * @param {number} sx
     * @param {number} sy
     */
    update_pan(sx, sy) {
        wasm.canvas_update_pan(this.__wbg_ptr, sx, sy);
    }
    /**
     * @param {number} x
     * @param {number} y
     */
    update_rubber_band(x, y) {
        wasm.canvas_update_rubber_band(this.__wbg_ptr, x, y);
    }
    /**
     * @param {number} x
     * @param {number} y
     */
    update_shape(x, y) {
        wasm.canvas_update_shape(this.__wbg_ptr, x, y);
    }
    /**
     * @param {number} screen_x
     * @param {number} screen_y
     * @param {number} delta
     */
    zoom_at(screen_x, screen_y, delta) {
        wasm.canvas_zoom_at(this.__wbg_ptr, screen_x, screen_y, delta);
    }
}
if (Symbol.dispose) Canvas.prototype[Symbol.dispose] = Canvas.prototype.free;

export function main() {
    wasm.main();
}

function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg___wbindgen_debug_string_0bc8482c6e3508ae: function(arg0, arg1) {
            const ret = debugString(arg1);
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg___wbindgen_is_undefined_9e4d92534c42d778: function(arg0) {
            const ret = arg0 === undefined;
            return ret;
        },
        __wbg___wbindgen_throw_be289d5034ed271b: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbg__wbg_cb_unref_d9b87ff7982e3b21: function(arg0) {
            arg0._wbg_cb_unref();
        },
        __wbg_arc_60bf829e1bd2add5: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
            arg0.arc(arg1, arg2, arg3, arg4, arg5);
        }, arguments); },
        __wbg_beginPath_9873f939d695759c: function(arg0) {
            arg0.beginPath();
        },
        __wbg_call_389efe28435a9388: function() { return handleError(function (arg0, arg1) {
            const ret = arg0.call(arg1);
            return ret;
        }, arguments); },
        __wbg_document_ee35a3d3ae34ef6c: function(arg0) {
            const ret = arg0.document;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_fillRect_d44afec47e3a3fab: function(arg0, arg1, arg2, arg3, arg4) {
            arg0.fillRect(arg1, arg2, arg3, arg4);
        },
        __wbg_getContext_2a5764d48600bc43: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = arg0.getContext(getStringFromWasm0(arg1, arg2));
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        }, arguments); },
        __wbg_getElementById_e34377b79d7285f6: function(arg0, arg1, arg2) {
            const ret = arg0.getElementById(getStringFromWasm0(arg1, arg2));
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_height_38750dc6de41ee75: function(arg0) {
            const ret = arg0.height;
            return ret;
        },
        __wbg_instanceof_CanvasRenderingContext2d_4bb052fd1c3d134d: function(arg0) {
            let result;
            try {
                result = arg0 instanceof CanvasRenderingContext2D;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_HtmlCanvasElement_3f2f6e1edb1c9792: function(arg0) {
            let result;
            try {
                result = arg0 instanceof HTMLCanvasElement;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Window_ed49b2db8df90359: function(arg0) {
            let result;
            try {
                result = arg0 instanceof Window;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_lineTo_c584cff6c760c4a5: function(arg0, arg1, arg2) {
            arg0.lineTo(arg1, arg2);
        },
        __wbg_log_6b5ca2e6124b2808: function(arg0) {
            console.log(arg0);
        },
        __wbg_moveTo_e9190fc700d55b40: function(arg0, arg1, arg2) {
            arg0.moveTo(arg1, arg2);
        },
        __wbg_new_3eb36ae241fe6f44: function() {
            const ret = new Array();
            return ret;
        },
        __wbg_new_no_args_1c7c842f08d00ebb: function(arg0, arg1) {
            const ret = new Function(getStringFromWasm0(arg0, arg1));
            return ret;
        },
        __wbg_push_8ffdcb2063340ba5: function(arg0, arg1) {
            const ret = arg0.push(arg1);
            return ret;
        },
        __wbg_rect_967665357db991e9: function(arg0, arg1, arg2, arg3, arg4) {
            arg0.rect(arg1, arg2, arg3, arg4);
        },
        __wbg_requestAnimationFrame_43682f8e1c5e5348: function() { return handleError(function (arg0, arg1) {
            const ret = arg0.requestAnimationFrame(arg1);
            return ret;
        }, arguments); },
        __wbg_restore_0d233789d098ba64: function(arg0) {
            arg0.restore();
        },
        __wbg_save_e0cc2e58b36d33c9: function(arg0) {
            arg0.save();
        },
        __wbg_scale_543277ecf8cf836b: function() { return handleError(function (arg0, arg1, arg2) {
            arg0.scale(arg1, arg2);
        }, arguments); },
        __wbg_setLineDash_ecf27050368658c9: function() { return handleError(function (arg0, arg1) {
            arg0.setLineDash(arg1);
        }, arguments); },
        __wbg_set_fillStyle_783d3f7489475421: function(arg0, arg1, arg2) {
            arg0.fillStyle = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_globalAlpha_c32898c5532572f4: function(arg0, arg1) {
            arg0.globalAlpha = arg1;
        },
        __wbg_set_lineCap_59a017de1ad2b0be: function(arg0, arg1, arg2) {
            arg0.lineCap = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_lineJoin_9b9f1aaa283be35a: function(arg0, arg1, arg2) {
            arg0.lineJoin = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_lineWidth_89fa506592f5b994: function(arg0, arg1) {
            arg0.lineWidth = arg1;
        },
        __wbg_set_strokeStyle_087121ed5350b038: function(arg0, arg1, arg2) {
            arg0.strokeStyle = getStringFromWasm0(arg1, arg2);
        },
        __wbg_static_accessor_GLOBAL_12837167ad935116: function() {
            const ret = typeof global === 'undefined' ? null : global;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_static_accessor_GLOBAL_THIS_e628e89ab3b1c95f: function() {
            const ret = typeof globalThis === 'undefined' ? null : globalThis;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_static_accessor_SELF_a621d3dfbb60d0ce: function() {
            const ret = typeof self === 'undefined' ? null : self;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_static_accessor_WINDOW_f8727f0cf888e0bd: function() {
            const ret = typeof window === 'undefined' ? null : window;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_stroke_240ea7f2407d73c0: function(arg0) {
            arg0.stroke();
        },
        __wbg_translate_3aa10730376a8c06: function() { return handleError(function (arg0, arg1, arg2) {
            arg0.translate(arg1, arg2);
        }, arguments); },
        __wbg_warn_f7ae1b2e66ccb930: function(arg0) {
            console.warn(arg0);
        },
        __wbg_width_5f66bde2e810fbde: function(arg0) {
            const ret = arg0.width;
            return ret;
        },
        __wbindgen_cast_0000000000000001: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { dtor_idx: 7, function: Function { arguments: [], shim_idx: 8, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm.wasm_bindgen__closure__destroy__he6ffe883f7d79201, wasm_bindgen__convert__closures_____invoke__h4563321ca26cc569);
            return ret;
        },
        __wbindgen_cast_0000000000000002: function(arg0) {
            // Cast intrinsic for `F64 -> Externref`.
            const ret = arg0;
            return ret;
        },
        __wbindgen_cast_0000000000000003: function(arg0, arg1) {
            // Cast intrinsic for `Ref(String) -> Externref`.
            const ret = getStringFromWasm0(arg0, arg1);
            return ret;
        },
        __wbindgen_init_externref_table: function() {
            const table = wasm.__wbindgen_externrefs;
            const offset = table.grow(4);
            table.set(0, undefined);
            table.set(offset + 0, undefined);
            table.set(offset + 1, null);
            table.set(offset + 2, true);
            table.set(offset + 3, false);
        },
    };
    return {
        __proto__: null,
        "./rust_canvas_bg.js": import0,
    };
}

function wasm_bindgen__convert__closures_____invoke__h4563321ca26cc569(arg0, arg1) {
    wasm.wasm_bindgen__convert__closures_____invoke__h4563321ca26cc569(arg0, arg1);
}

const CanvasFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_canvas_free(ptr >>> 0, 1));

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
}

const CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(state => state.dtor(state.a, state.b));

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches && builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function makeMutClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {

        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            state.a = a;
            real._wbg_cb_unref();
        }
    };
    real._wbg_cb_unref = () => {
        if (--state.cnt === 0) {
            state.dtor(state.a, state.b);
            state.a = 0;
            CLOSURE_DTORS.unregister(state);
        }
    };
    CLOSURE_DTORS.register(real, state, state);
    return real;
}

function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_externrefs.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };
}

let WASM_VECTOR_LEN = 0;

let wasmModule, wasm;
function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    wasmModule = module;
    cachedDataViewMemory0 = null;
    cachedUint8ArrayMemory0 = null;
    wasm.__wbindgen_start();
    return wasm;
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = module.ok && expectedResponseType(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else { throw e; }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);
    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };
        } else {
            return instance;
        }
    }

    function expectedResponseType(type) {
        switch (type) {
            case 'basic': case 'cors': case 'default': return true;
        }
        return false;
    }
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (module !== undefined) {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();
    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (module_or_path !== undefined) {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (module_or_path === undefined) {
        module_or_path = new URL('rust_canvas_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync, __wbg_init as default };
