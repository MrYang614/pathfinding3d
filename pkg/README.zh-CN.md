# pathfinding3d

[English](README.md)

最快的 JavaScript 三维寻路库。`pathfinding3d` 在 Rust 中实现核心 NavMesh 算法并编译为 WebAssembly，在浏览器与 Node.js 中提供接近原生的三维寻路性能。

它不是仅限 Three.js 的插件，而是通用的 WASM 三维寻路引擎。只要你的 JavaScript 三维引擎能提供网格顶点与索引数据，就可以用本库构建导航区域、查询分组并搜索路径。

## 为何选择 pathfinding3d

| 优势 | 实际意义 |
|------|----------|
| **Rust + WebAssembly** | 热点路径（网格构建、空间查询、A\*、漏斗平滑）以编译代码运行，而非 JavaScript 解释执行 — 在典型场景中约为 `three-pathfinding-3d` 的 **10–20 倍** 性能。 |
| **引擎无关** | 不依赖 Three.js、Babylon.js 或任何渲染器。任意 WebGL/WebGPU 引擎均可传入扁平的 `positions` 与 `indices` 数组。 |
| **完整 NavMesh 管线** | 顶点焊接、三角形邻接、连通 **分组**、导航图 A\*、**三维漏斗拉直（String Pulling）** 生成平滑路径 — 全部集成在一个库中。 |
| **低 GC 压力** | 内部分组搜索缓冲区可复用；路径结果写入调用方持有的 `Float32Array`，避免每次查询分配 JS 对象。 |
| **快速空间查询** | KD 树与 AABB 剪枝加速最近三角形与分组查找。 |
| **灵活的 Zone 标识** | 可按字符串 ID（`create_zone`）或数字句柄（`create_zone_handle`）注册区域，适配不同架构。 |
| **随处可用** | 通过 `wasm-pack` 打包，适用于 Vite/Webpack、Electron、Node.js 等 ESM 环境。 |

![基准测试：pathfinding3d vs three-pathfinding-3d](../benchmark.png)

*Demo 导航网格（`level.nav.glb`）：`findPath` **10.4x**，三项合计约 **7.3x**（相对 `three-pathfinding-3d`）。可复现：[`demo/benchmark.html`](../demo/benchmark.html)。*

## 适用场景

- 大型三维场景中的角色导航
- Web 游戏、数字孪生、仿真、编辑器与可视化项目
- 需要可复用寻路、又不想绑定 Three.js 的多引擎项目
- 寻路查询需要比纯 JavaScript 方案更快的项目

## 安装

从 npm 安装（推荐）：

```bash
npm install pathfinding3d
# yarn add pathfinding3d
# pnpm add pathfinding3d
```

本包为 **纯 ESM** 模块（`"type": "module"`），自带 TypeScript 类型定义，无需额外安装 `@types` 包。

或从源码构建（需安装 [Rust](https://rustup.rs/) 与 [wasm-pack](https://rustwasm.github.io/wasm-pack/)）：

```bash
cargo install wasm-pack
wasm-pack build --release
```

生成的 npm 包会输出到 `pkg/` 目录。

## npm 用法

### 在项目中引入

安装完成后，直接在代码中 import 即可使用：

```js
import { PathfindingWasm } from "pathfinding3d";

const pathfinding = new PathfindingWasm();
```

首次 import 时 WASM 会自动初始化，**无需** 单独调用 `init()`。

TypeScript 项目同样直接 import，类型由包内 `pathfinding3d.d.ts` 提供：

```ts
import { PathfindingWasm } from "pathfinding3d";
```

### Vite 项目配置

本库依赖 WebAssembly，Vite 项目需安装并启用 WASM 插件：

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

仓库中的 [demo](../demo) 即采用此配置，可作为完整参考。

### Webpack 5+

在 webpack 配置中开启 `asyncWebAssembly` 实验选项，以支持 `.wasm` 模块加载。

### 本地开发 / 未发布版本

若在本仓库中自行构建，可将 `pkg/` 作为本地依赖安装到项目中：

```bash
# 在仓库根目录构建
wasm-pack build --release

# 在你的项目中安装本地 pkg
npm install /path/to/pathfinding3d/pkg
# 或相对路径：npm install ../pathfinding3d/pkg
```

也可使用 `npm link` 进行联调：

```bash
cd pkg && npm link
cd ../your-app && npm link pathfinding3d
```

### package.json 依赖示例

```json
{
  "dependencies": {
    "pathfinding3d": "^1.0.1"
  }
}
```

## 快速开始

```js
import { PathfindingWasm } from "pathfinding3d";

const pathfinding = new PathfindingWasm();

// positions: Float32Array [x, y, z, x, y, z, ...]
// indices:   Uint32Array  [a, b, c, a, b, c, ...]  （三角形索引）
pathfinding.create_zone("level-1", positions, indices, 0.001);

const groupId = pathfinding.get_group("level-1", start.x, start.y, start.z, true);
if (groupId === undefined) {
  // 起点不在 NavMesh 上
  return;
}

const output = new Float32Array(1024 * 3); // 每帧复用此缓冲区
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

> **说明：** 当前构建在首次 import 时自动初始化 WASM，无需单独调用 `init()`。

## 典型工作流

```text
NavMesh 网格数据
       │
       ▼
 create_zone()          ── 构建邻接、分组、空间索引
       │
       ▼
 get_group()             ── 角色位于哪个连通区域？
       │
       ▼
 find_path()             ── A* + 漏斗 → Float32Array 路径点
       │
       ▼
 沿路径移动角色          ── 跳过已到达点，按需重新查询
```

1. **准备 NavMesh 数据** — 将可走区域导出为世界空间顶点坐标与三角形索引。互不相连的可走岛屿会成为不同的 **分组（group）**；寻路仅在同一分组内进行。
2. **创建 Zone** — 每个关卡/场景调用一次 `create_zone`。选择合适的 `tolerance`，合并近似重合顶点但不破坏真实几何（见下方 [参数说明](#参数说明)）。
3. **确定角色分组** — 在角色位置调用 `get_group`，保存返回的 `groupId`；角色传送到新岛屿时需重新查询。
4. **查询路径** — 使用相同 `groupId` 调用 `find_path`。在游戏循环中复用预分配的 `Float32Array` 输出缓冲区，避免频繁分配。
5. **处理不可达目标** — 当路径不存在（不同分组、区域被阻挡等）时，`find_path` 返回 `0`。

## Three.js 集成

```js
import * as THREE from "three";
import { GLTFLoader } from "three/addons/loaders/GLTFLoader.js";
import { PathfindingWasm } from "pathfinding3d";

const ZONE = "level";
const pathfinder = new PathfindingWasm();
const pathOutput = new Float32Array(1024 * 3);

const loader = new GLTFLoader();
loader.load("/level.nav.glb", (gltf) => {
  const navMesh = gltf.scene.getObjectByName("Navmesh_Mesh");
  const positions = navMesh.geometry.attributes.position.array; // Float32Array
  const indices = navMesh.geometry.index.array;                 // Uint32Array

  pathfinder.create_zone(ZONE, positions, indices, 0.001);

  const groupId = pathfinder.get_group(
    ZONE,
    playerPosition.x,
    playerPosition.y,
    playerPosition.z,
    true,
  );

  const pointCount = pathfinder.find_path(
    ZONE,
    groupId,
    playerPosition.x, playerPosition.y, playerPosition.z,
    targetPosition.x, targetPosition.y, targetPosition.z,
    pathOutput,
  );

  const path = [];
  for (let i = 0; i < pointCount; i++) {
    path.push(new THREE.Vector3(
      pathOutput[i * 3],
      pathOutput[i * 3 + 1],
      pathOutput[i * 3 + 2],
    ));
  }
});
```

Babylon.js、PlayCanvas、Cesium 或任何能暴露几何缓冲区的引擎，均可采用相同模式。

## 参数说明

### `tolerance`（构建 Zone）

构建导航图时 **焊接** 近似重合顶点的距离阈值。值越小保留细节越多；值越大合并更多邻近顶点、简化网格。

- 米级场景（与 demo 一致）建议从 `0.001` 开始。
- 厘米级精度模型可使用 `0.0001` 或更小。

### `checkPolygon`（分组 / 节点查询）

为 `true` 时，`get_group` 与 `get_closest_node_id` 要求查询点 **落在可走三角形上**（含小范围平面距离容差）。为 `false` 时按质心距离返回最近三角形 — 适合粗略预选，精度较低。

除非有特殊需求，游戏逻辑中应始终传入 `true`。

### `output` 缓冲区（`find_path`）

- 必须是长度 ≥ `pointCount * 3` 的 `Float32Array`。
- **起点不会写入** 输出；仅返回中间路径点与目标点。
- 若缓冲区不足，返回值仍会报告所需点数，便于扩容后重试。
- 保留一个缓冲区并在各帧间复用，以减少 GC。

## API 参考

所有方法均在 `PathfindingWasm` 实例上。

### Zone 管理

| 方法 | 说明 |
|------|------|
| `create_zone(zoneId, positions, indices, tolerance)` | 构建具名 Zone。同名 Zone 会被替换。 |
| `create_zone_handle(positions, indices, tolerance)` | 构建 Zone 并返回数字句柄（无字符串查找开销）。 |

### 空间查询

| 方法 | 返回值 | 说明 |
|------|--------|------|
| `get_group(zoneId, x, y, z, checkPolygon)` | `number \| undefined` | 包含（或最接近）世界坐标的分组索引。 |
| `get_group_by_handle(zoneHandle, x, y, z, checkPolygon)` | `number \| undefined` | 同 `get_group`，按句柄查询。 |
| `get_closest_node_id(zoneId, x, y, z, checkPolygon)` | `number \| undefined` | 在解析出的分组内，最近的导航三角形 ID。 |

### 寻路

| 方法 | 返回值 | 说明 |
|------|--------|------|
| `find_path(zoneId, groupId, sx, sy, sz, tx, ty, tz, output)` | `number` | 写入 `output` 的路径点数。无路径时返回 `0`。 |

### 元数据与调试

| 方法 | 返回值 | 说明 |
|------|--------|------|
| `group_count(zoneId)` | `number \| undefined` | Zone 内连通分组数量。 |
| `group_count_by_handle(zoneHandle)` | `number \| undefined` | 同上，按句柄查询。 |
| `group_node_count(zoneId, groupId)` | `number \| undefined` | 分组内三角形数量。 |
| `group_node_ids(zoneId, groupId)` | `Uint32Array \| undefined` | 分组内三角形 ID 列表。 |
| `group_node_centers(zoneId, groupId)` | `Float64Array \| undefined` | 分组内所有三角形质心，扁平 `[x,y,z, ...]`。 |
| `node_center(zoneId, groupId, nodeId)` | `Float64Array \| undefined` | 单个三角形质心 `[x, y, z]`。 |

### 生命周期

| 方法 | 说明 |
|------|------|
| `free()` / `[Symbol.dispose]()` | 不再使用时释放 WASM 资源。 |

## TypeScript

包内自带类型定义：

```ts
import { PathfindingWasm } from "pathfinding3d";
```

完整签名见 `pathfinding3d.d.ts`。

## 集成注意事项

- **坐标系** — 库不做轴向转换。请传入与渲染器一致的世界空间数据（Y-up、Z-up 等）。
- **NavMesh 质量** — 非流形几何、重复面或过度焊接会破坏邻接关系。不相连区域属于不同分组；请在关卡数据中桥接，或在游戏逻辑中处理跨组移动。
- **坡面** — 点在三角形内判定假设可走面大致水平（Y 轴投影）。陡坡或任意朝向网格可能需要预处理。
- **打包工具** — Vite 需启用 WASM 支持（`vite-plugin-wasm` + `vite-plugin-top-level-await`）。Webpack 5+ 可开启 `asyncWebAssembly` 实验选项。

## 许可

在以下两种许可中任选其一：

- Apache License, Version 2.0，见 [LICENSE_APACHE](LICENSE_APACHE)
- MIT license，见 [LICENSE_MIT](LICENSE_MIT)
