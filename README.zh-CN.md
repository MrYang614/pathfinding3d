# pathfinding3d

[![npm version](https://img.shields.io/npm/v/pathfinding3d)](https://www.npmjs.com/package/pathfinding3d)

[English](README.md)

最快的 JavaScript 三维寻路库。核心算法用 Rust 实现并编译为 WebAssembly，在浏览器与 Node.js 中提供接近原生的 3D NavMesh 性能。

不是仅限 Three.js 的插件 —— 只要能提供网格顶点与索引，任意 JavaScript 三维引擎都可以构建区域、查询分组并寻路。

## 特点

- 极高性能：Rust + WebAssembly 寻路，`findPath` 约为 `three-pathfinding-3d` 的 **10 倍**。
- 引擎无关：可用于 Three.js、Babylon.js、PlayCanvas、Cesium、自研 WebGL/WebGPU 及任意 JS 三维场景。
- 完整 NavMesh 流程：三角网格 → 区域 → 分组 / 节点 → A* → 漏斗平滑路径。
- JavaScript 开销低：结果写入预分配的 `Float32Array`，减少分配与 GC 压力。
- 通过 `wasm-pack` 打包，适用于 Web、Electron、Node.js 等 ESM 环境。

![基准测试：pathfinding3d vs three-pathfinding-3d](benchmark.png)

*Demo 导航网格（`level.nav.glb`）：`findPath` **10.4x**，三项合计约 **7.3x**（相对 `three-pathfinding-3d`）。可复现：[`demo/benchmark.html`](demo/benchmark.html)。*

## 安装

```bash
npm install pathfinding3d
# yarn add pathfinding3d
# pnpm add pathfinding3d
```

纯 ESM（`"type": "module"`），自带 TypeScript 类型定义。

```js
import { PathfindingWasm } from "pathfinding3d";

const pathfinding = new PathfindingWasm();
```

首次 import 时自动初始化 WASM，无需单独调用 `init()`。

**Vite** — 安装并启用 WASM 插件：

```bash
npm install -D vite-plugin-wasm vite-plugin-top-level-await
```

```ts
// vite.config.ts
import { defineConfig } from "vite";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

export default defineConfig({
  build: { target: "esnext" },
  plugins: [wasm(), topLevelAwait()],
});
```

**Webpack 5+** — 开启 `asyncWebAssembly` 实验选项。

**本地 / 源码构建** — 需 [Rust](https://rustup.rs/) 与 [wasm-pack](https://rustwasm.github.io/wasm-pack/)：

```bash
cargo install wasm-pack
wasm-pack build --release
npm install ./pkg
```

完整 API：[pkg/README.zh-CN.md](pkg/README.zh-CN.md)。

## 快速开始

```js
import { PathfindingWasm } from "pathfinding3d";

const pathfinding = new PathfindingWasm();

// positions: Float32Array [x, y, z, ...]
// indices:   Uint32Array  [a, b, c, ...]
pathfinding.create_zone("level-1", positions, indices, 0.001);

const groupId = pathfinding.get_group("level-1", start.x, start.y, start.z, true);
if (groupId === undefined) return;

const output = new Float32Array(1024 * 3);
const pointCount = pathfinding.find_path(
  "level-1",
  groupId,
  start.x, start.y, start.z,
  target.x, target.y, target.z,
  output,
);

const path = [];
for (let i = 0; i < pointCount; i += 1) {
  path.push({
    x: output[i * 3],
    y: output[i * 3 + 1],
    z: output[i * 3 + 2],
  });
}
```

## API 概览

- `create_zone(zoneId, positions, indices, tolerance)` — 由三角网格创建区域。
- `create_zone_handle(positions, indices, tolerance)` — 同上，返回数字句柄。
- `get_group(zoneId, x, y, z, checkPolygon)` — 查找包含或最接近某位置的分组。
- `get_closest_node_id(zoneId, x, y, z, checkPolygon)` — 该分组内最近的导航节点。
- `find_path(zoneId, groupId, sx, sy, sz, tx, ty, tz, output)` — 将路径写入 `Float32Array`。
- `group_count` / `group_node_count` / `group_node_ids` / `group_node_centers` — 区域元数据。

破坏性变更见 [CHANGELOG.zh-CN.md](CHANGELOG.zh-CN.md)。

## 节点 ID

节点 ID 为**组内局部索引**（`0` … `group_node_count(zoneId, groupId) - 1`），不是整个 Zone 的全局三角形序号。

| API | 含义 |
|-----|------|
| `get_closest_node_id(zoneId, …)` | 查询点所在分组内最近三角形的下标 |
| `group_node_ids(zoneId, groupId)` | 该分组内所有三角形下标 |
| `node_center(zoneId, groupId, nodeId)` | 分组 `groupId` 中三角形 `nodeId` 的质心 |

保存节点 ID 时请始终配合 `get_group` 的 `groupId`（或与 `find_path` 传入的一致）。不同分组的 ID 不可比较。

## 许可

[MIT](LICENSE)。更新日志：[CHANGELOG.zh-CN.md](CHANGELOG.zh-CN.md)。
