# pathfinding3d

[简体中文](README.zh-CN.md)

The fastest JavaScript 3D pathfinding library. `pathfinding3d` implements its core algorithms in Rust and compiles them to WebAssembly, bringing near-native 3D NavMesh pathfinding performance to browsers and Node.js.

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

## Install

Install from npm (recommended):

```bash
npm install pathfinding3d
# yarn add pathfinding3d
# pnpm add pathfinding3d
```

The package is **ESM-only** (`"type": "module"`). TypeScript definitions are included — no `@types` package needed.

### Using with npm

Import and use directly in your app:

```js
import { PathfindingWasm } from "pathfinding3d";

const pathfinding = new PathfindingWasm();
```

WASM initializes automatically on first import — no separate `init()` call is required.

**Vite** — install WASM plugins and add them to your config:

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

**Webpack 5+** — enable the `asyncWebAssembly` experiment in your config.

**Local development** — after building with `wasm-pack build --release`, link the generated package:

```bash
npm install ./pkg
# or: cd pkg && npm link && cd ../your-app && npm link pathfinding3d
```

### Build from source

Install [Rust](https://rustup.rs/) and [wasm-pack](https://rustwasm.github.io/wasm-pack/), then:

```bash
cargo install wasm-pack
wasm-pack build --release
```

The generated npm package is written to `pkg/`. See [pkg/README.md](pkg/README.md) for the full API and integration guide.

## Quick Start

```js
import { PathfindingWasm } from "pathfinding3d";

const pathfinding = new PathfindingWasm();

// positions: Float32Array [x, y, z, x, y, z, ...]
// indices:   Uint32Array  [a, b, c, a, b, c, ...]
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
