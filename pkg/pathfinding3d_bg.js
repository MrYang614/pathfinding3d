export class PathfindingWasm {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PathfindingWasmFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_pathfindingwasm_free(ptr, 0);
    }
    /**
     * @param {string} zone_id
     * @param {Float32Array} positions
     * @param {Uint32Array} indices
     * @param {number} tolerance
     */
    create_zone(zone_id, positions, indices, tolerance) {
        const ptr0 = passStringToWasm0(zone_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArrayF32ToWasm0(positions, wasm.__wbindgen_malloc);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passArray32ToWasm0(indices, wasm.__wbindgen_malloc);
        const len2 = WASM_VECTOR_LEN;
        const ret = wasm.pathfindingwasm_create_zone(this.__wbg_ptr, ptr0, len0, ptr1, len1, ptr2, len2, tolerance);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {Float32Array} positions
     * @param {Uint32Array} indices
     * @param {number} tolerance
     * @returns {number}
     */
    create_zone_handle(positions, indices, tolerance) {
        const ptr0 = passArrayF32ToWasm0(positions, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArray32ToWasm0(indices, wasm.__wbindgen_malloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.pathfindingwasm_create_zone_handle(this.__wbg_ptr, ptr0, len0, ptr1, len1, tolerance);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return ret[0] >>> 0;
    }
    /**
     * @param {string} zone_id
     * @param {number} group_id
     * @param {number} start_x
     * @param {number} start_y
     * @param {number} start_z
     * @param {number} target_x
     * @param {number} target_y
     * @param {number} target_z
     * @param {Float32Array} output
     * @returns {number}
     */
    find_path(zone_id, group_id, start_x, start_y, start_z, target_x, target_y, target_z, output) {
        const ptr0 = passStringToWasm0(zone_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.pathfindingwasm_find_path(this.__wbg_ptr, ptr0, len0, group_id, start_x, start_y, start_z, target_x, target_y, target_z, output);
        return ret;
    }
    /**
     * @param {string} zone_id
     * @param {number} group_id
     * @param {number} x
     * @param {number} y
     * @param {number} z
     * @param {boolean} check_polygon
     * @returns {number | undefined}
     */
    get_closest_node_id(zone_id, group_id, x, y, z, check_polygon) {
        const ptr0 = passStringToWasm0(zone_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.pathfindingwasm_get_closest_node_id(this.__wbg_ptr, ptr0, len0, group_id, x, y, z, check_polygon);
        return ret === Number.MAX_SAFE_INTEGER ? undefined : ret;
    }
    /**
     * @param {string} zone_id
     * @param {number} x
     * @param {number} y
     * @param {number} z
     * @param {boolean} check_polygon
     * @returns {number | undefined}
     */
    get_group(zone_id, x, y, z, check_polygon) {
        const ptr0 = passStringToWasm0(zone_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.pathfindingwasm_get_group(this.__wbg_ptr, ptr0, len0, x, y, z, check_polygon);
        return ret === Number.MAX_SAFE_INTEGER ? undefined : ret;
    }
    /**
     * @param {number} zone_handle
     * @param {number} x
     * @param {number} y
     * @param {number} z
     * @param {boolean} check_polygon
     * @returns {number | undefined}
     */
    get_group_by_handle(zone_handle, x, y, z, check_polygon) {
        const ret = wasm.pathfindingwasm_get_group_by_handle(this.__wbg_ptr, zone_handle, x, y, z, check_polygon);
        return ret === Number.MAX_SAFE_INTEGER ? undefined : ret;
    }
    /**
     * @param {string} zone_id
     * @returns {number | undefined}
     */
    group_count(zone_id) {
        const ptr0 = passStringToWasm0(zone_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.pathfindingwasm_group_count(this.__wbg_ptr, ptr0, len0);
        return ret === Number.MAX_SAFE_INTEGER ? undefined : ret;
    }
    /**
     * @param {number} zone_handle
     * @returns {number | undefined}
     */
    group_count_by_handle(zone_handle) {
        const ret = wasm.pathfindingwasm_group_count_by_handle(this.__wbg_ptr, zone_handle);
        return ret === Number.MAX_SAFE_INTEGER ? undefined : ret;
    }
    /**
     * @param {string} zone_id
     * @param {number} group_id
     * @returns {Float64Array | undefined}
     */
    group_node_centers(zone_id, group_id) {
        const ptr0 = passStringToWasm0(zone_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.pathfindingwasm_group_node_centers(this.__wbg_ptr, ptr0, len0, group_id);
        return ret;
    }
    /**
     * @param {string} zone_id
     * @param {number} group_id
     * @returns {number | undefined}
     */
    group_node_count(zone_id, group_id) {
        const ptr0 = passStringToWasm0(zone_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.pathfindingwasm_group_node_count(this.__wbg_ptr, ptr0, len0, group_id);
        return ret === Number.MAX_SAFE_INTEGER ? undefined : ret;
    }
    /**
     * @param {string} zone_id
     * @param {number} group_id
     * @returns {Uint32Array | undefined}
     */
    group_node_ids(zone_id, group_id) {
        const ptr0 = passStringToWasm0(zone_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.pathfindingwasm_group_node_ids(this.__wbg_ptr, ptr0, len0, group_id);
        return ret;
    }
    constructor() {
        const ret = wasm.pathfindingwasm_new();
        this.__wbg_ptr = ret;
        PathfindingWasmFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {string} zone_id
     * @param {number} group_id
     * @param {number} node_id
     * @returns {Float64Array | undefined}
     */
    node_center(zone_id, group_id, node_id) {
        const ptr0 = passStringToWasm0(zone_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.pathfindingwasm_node_center(this.__wbg_ptr, ptr0, len0, group_id, node_id);
        return ret;
    }
}
if (Symbol.dispose) PathfindingWasm.prototype[Symbol.dispose] = PathfindingWasm.prototype.free;
export function __wbg___wbindgen_throw_9c75d47bf9e7731e(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
}
export function __wbg_error_a6fa202b58aa1cd3(arg0, arg1) {
    let deferred0_0;
    let deferred0_1;
    try {
        deferred0_0 = arg0;
        deferred0_1 = arg1;
        console.error(getStringFromWasm0(arg0, arg1));
    } finally {
        wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
    }
}
export function __wbg_length_5693120f2a64a00d(arg0) {
    const ret = arg0.length;
    return ret;
}
export function __wbg_new_227d7c05414eb861() {
    const ret = new Error();
    return ret;
}
export function __wbg_new_from_slice_3ca7c4e9a43341b6(arg0, arg1) {
    const ret = new Float64Array(getArrayF64FromWasm0(arg0, arg1));
    return ret;
}
export function __wbg_new_from_slice_823acd363b3844cf(arg0, arg1) {
    const ret = new Uint32Array(getArrayU32FromWasm0(arg0, arg1));
    return ret;
}
export function __wbg_set_15b3678c712ded6b(arg0, arg1, arg2) {
    arg0.set(getArrayF32FromWasm0(arg1, arg2));
}
export function __wbg_stack_3b0d974bbf31e44f(arg0, arg1) {
    const ret = arg1.stack;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}
export function __wbg_subarray_2a79e7a5db50bc18(arg0, arg1, arg2) {
    const ret = arg0.subarray(arg1 >>> 0, arg2 >>> 0);
    return ret;
}
export function __wbindgen_cast_0000000000000001(arg0, arg1) {
    // Cast intrinsic for `Ref(String) -> Externref`.
    const ret = getStringFromWasm0(arg0, arg1);
    return ret;
}
export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_externrefs;
    const offset = table.grow(4);
    table.set(0, undefined);
    table.set(offset + 0, undefined);
    table.set(offset + 1, null);
    table.set(offset + 2, true);
    table.set(offset + 3, false);
}
const PathfindingWasmFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_pathfindingwasm_free(ptr, 1));

function getArrayF32FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getFloat32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);
}

function getArrayF64FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getFloat64ArrayMemory0().subarray(ptr / 8, ptr / 8 + len);
}

function getArrayU32FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

let cachedFloat32ArrayMemory0 = null;
function getFloat32ArrayMemory0() {
    if (cachedFloat32ArrayMemory0 === null || cachedFloat32ArrayMemory0.byteLength === 0) {
        cachedFloat32ArrayMemory0 = new Float32Array(wasm.memory.buffer);
    }
    return cachedFloat32ArrayMemory0;
}

let cachedFloat64ArrayMemory0 = null;
function getFloat64ArrayMemory0() {
    if (cachedFloat64ArrayMemory0 === null || cachedFloat64ArrayMemory0.byteLength === 0) {
        cachedFloat64ArrayMemory0 = new Float64Array(wasm.memory.buffer);
    }
    return cachedFloat64ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    return decodeText(ptr >>> 0, len);
}

let cachedUint32ArrayMemory0 = null;
function getUint32ArrayMemory0() {
    if (cachedUint32ArrayMemory0 === null || cachedUint32ArrayMemory0.byteLength === 0) {
        cachedUint32ArrayMemory0 = new Uint32Array(wasm.memory.buffer);
    }
    return cachedUint32ArrayMemory0;
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function passArray32ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 4, 4) >>> 0;
    getUint32ArrayMemory0().set(arg, ptr / 4);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function passArrayF32ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 4, 4) >>> 0;
    getFloat32ArrayMemory0().set(arg, ptr / 4);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
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


let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}
