# pathfinding3d

[简体中文](README.zh-CN.md)

One of the fastest JavaScript 3D pathfinding libraries. `pathfinding3d` implements its core algorithms in Rust and compiles them to WebAssembly, bringing near-native 3D NavMesh pathfinding performance to browsers and Node.js.

It is not a Three.js-only plugin. It is a general-purpose WASM 3D pathfinding engine. As long as your JavaScript 3D engine can provide mesh vertex and index data, it can use this library to build navigation zones, query groups, and search paths.

## Highlights

- Extreme performance: the core pathfinding pipeline is implemented with Rust + WebAssembly, delivering roughly 10x the performance of `three-pathfinding-3d`.
- Engine agnostic: not limited to Three.js. It works with Babylon.js, PlayCanvas, Cesium, custom WebGL/WebGPU engines, and any JavaScript 3D scene.
- Built for 3D NavMesh workflows: create zones from triangle mesh data, then generate smooth paths with groups, nodes, A*, and funnel channels.
- Low JavaScript overhead: path results are written into a preallocated `Float32Array`, reducing object allocation and GC pressure.
- Frontend and server ready: packaged with `wasm-pack` for Web, Electron, Node.js, and other JavaScript environments.

## Use Cases

- Character navigation in large 3D scenes
- Web games, digital twins, simulations, editors, and visualization projects
- Multi-engine projects that need reusable pathfinding without being tied to Three.js
- Projects that need faster path queries than `three-pathfinding-3d`

## Build

Install Rust and `wasm-pack` first:

```bash
cargo install wasm-pack
```

Build the WebAssembly npm package:

```bash
wasm-pack build --release
```

The generated package will be written to `pkg/` and can be imported directly from JavaScript or TypeScript projects.

## Quick Start

```js
import init, { PathfindingWasm } from "./pkg/three_pathfinding_3d_wasm.js";

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

## API Overview

- `create_zone(zoneId, positions, indices, tolerance)`: creates a pathfinding zone from triangle mesh data.
- `create_zone_handle(positions, indices, tolerance)`: creates a zone and returns a numeric handle.
- `get_group(zoneId, x, y, z, checkPolygon)`: finds the group containing or nearest to a position.
- `get_closest_node_id(zoneId, groupId, x, y, z, checkPolygon)`: finds the closest navigation node.
- `find_path(zoneId, groupId, startX, startY, startZ, targetX, targetY, targetZ, output)`: computes a path and writes it into a `Float32Array`.
- `group_count(zoneId)`, `group_node_count(zoneId, groupId)`, `group_node_ids(zoneId, groupId)`, `group_node_centers(zoneId, groupId)`: reads zone and group metadata.

## Why It Is Not Tied to Three.js

Three.js is only one rendering engine. A pathfinding algorithm needs navigation mesh data, not a specific renderer object model. `pathfinding3d` accepts generic `positions` and `indices` arrays, so any 3D engine can convert its mesh data and pass it in.

This means you can use it with Three.js, Babylon.js, PlayCanvas, Cesium, or a custom engine while keeping the same high-performance pathfinding logic.

## License

Licensed under either of:

- Apache License, Version 2.0, see [LICENSE_APACHE](LICENSE_APACHE)
- MIT license, see [LICENSE_MIT](LICENSE_MIT)
