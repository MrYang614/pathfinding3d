/* @ts-self-types="./pathfinding3d.d.ts" */
import * as wasm from "./pathfinding3d_bg.wasm";
import { __wbg_set_wasm } from "./pathfinding3d_bg.js";

__wbg_set_wasm(wasm);
wasm.__wbindgen_start();
export {
    PathfindingWasm
} from "./pathfinding3d_bg.js";
