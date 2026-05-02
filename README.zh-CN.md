# pathfinding3d

[English](README.md)

最快的 JavaScript 三维寻路库。`pathfinding3d` 在 Rust 中实现核心算法并编译为 WebAssembly，在浏览器与 Node.js 中提供接近原生的 3D NavMesh 寻路性能。

它不是仅限 Three.js 的插件，而是通用的 WASM 三维寻路引擎。只要你的 JavaScript 三维引擎能提供网格顶点与索引数据，就可以用本库构建导航区域、查询分组并搜索路径。

## 特点

- **极高性能**：核心寻路管线由 Rust + WebAssembly 实现，性能约为 `three-pathfinding-3d` 的 10-20 倍量级。
- **引擎无关**：不限于 Three.js，可与 Babylon.js、PlayCanvas、Cesium、自研 WebGL/WebGPU 引擎及任意 JavaScript 三维场景配合使用。
- **面向 3D NavMesh 流程**：由三角网格数据创建区域，再通过分组、节点、A* 与漏斗通道生成平滑路径。
- **JavaScript 开销低**：路径结果写入预分配的 `Float32Array`，减少对象分配与 GC 压力。
- **前后端通用**：通过 `wasm-pack` 打包，适用于 Web、Electron、Node.js 等 JavaScript 环境。

## 适用场景

- 大型三维场景中的角色导航
- Web 游戏、数字孪生、仿真、编辑器与可视化项目
- 需要可复用寻路、又不想绑定 Three.js 的多引擎项目
- 寻路查询需要比 `three-pathfinding-3d` 更快的项目

## 构建

先安装 Rust 与 `wasm-pack`：

```bash
cargo install wasm-pack
```

构建 WebAssembly npm 包：

```bash
wasm-pack build --release
```

生成内容会输出到 `pkg/`，可在 JavaScript 或 TypeScript 项目中直接引用。

## 快速开始

```js
import init, { PathfindingWasm } from "./pkg/pathfinding3d.js";

await init();

const pathfinding = new PathfindingWasm();

// positions: [x, y, z, x, y, z, ...]
// indices: [a, b, c, a, b, c, ...]
pathfinding.create_zone(
  "level-1",
  positions,
  indices,
  0.0001
);

const groupId = pathfinding.get_group(
  "level-1",
  start.x,
  start.y,
  start.z,
  true
);

const output = new Float32Array(1024 * 3);
const pointCount = pathfinding.find_path(
  "level-1",
  groupId,
  start.x,
  start.y,
  start.z,
  target.x,
  target.y,
  target.z,
  output
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

- `create_zone(zoneId, positions, indices, tolerance)`：由三角网格数据创建寻路区域。
- `create_zone_handle(positions, indices, tolerance)`：创建区域并返回数字句柄。
- `get_group(zoneId, x, y, z, checkPolygon)`：查找包含或最接近某位置的分组。
- `get_closest_node_id(zoneId, groupId, x, y, z, checkPolygon)`：查找最近的导航节点。
- `find_path(zoneId, groupId, startX, startY, startZ, targetX, targetY, targetZ, output)`：计算路径并写入 `Float32Array`。
- `group_count(zoneId)`、`group_node_count(zoneId, groupId)`、`group_node_ids(zoneId, groupId)`、`group_node_centers(zoneId, groupId)`：读取区域与分组元数据。

## 为何不绑定 Three.js

Three.js 只是众多渲染引擎之一。寻路算法需要的是导航网格数据，而不是特定渲染器的对象模型。`pathfinding3d` 接受通用的 `positions` 与 `indices` 数组，任意三维引擎都可以转换自身网格数据后传入。

因此你可以在 Three.js、Babylon.js、PlayCanvas、Cesium 或自研引擎中使用同一套高性能寻路逻辑。

## 许可

在以下两种许可中任选其一：

- Apache License, Version 2.0，见 [LICENSE_APACHE](LICENSE_APACHE)
- MIT license，见 [LICENSE_MIT](LICENSE_MIT)
