# pathfinding3d

[简体中文](README.zh-CN.md)

The fastest JavaScript 3D pathfinding library. `pathfinding3d` implements its core NavMesh algorithms in Rust and compiles them to WebAssembly, bringing near-native 3D pathfinding performance to browsers and Node.js.

It is not a Three.js-only plugin. It is a general-purpose WASM 3D pathfinding engine. As long as your JavaScript 3D engine can provide mesh vertex and index data, you can use this library to build navigation zones, query groups, and search paths.

## Why Choose pathfinding3d

| Advantage | What it means in practice |
|-----------|---------------------------|
| **Rust + WebAssembly** | The hot path (mesh build, spatial queries, A\*, funnel smoothing) runs as compiled code instead of interpreted JavaScript — roughly **10–20× faster** than `three-pathfinding-3d` in typical scenes. |
| **Engine agnostic** | No dependency on Three.js, Babylon.js, or any renderer. Pass flat `positions` and `indices` arrays from any WebGL/WebGPU engine. |
| **Full NavMesh pipeline** | Vertex welding, triangle adjacency, connected **groups**, A\* on the nav graph, and **3D funnel string-pulling** for smooth paths — all in one library. |
| **Low GC pressure** | Per-group search buffers are reused internally; path results are written into a caller-owned `Float32Array` instead of allocating JS objects every query. |
| **Fast spatial queries** | KD-trees and AABB pruning accelerate nearest-triangle and group lookup. |
| **Flexible zone identity** | Register zones by string ID (`create_zone`) or numeric handle (`create_zone_handle`) depending on your app architecture. |
| **Runs everywhere JS runs** | Packaged with `wasm-pack` for Vite/Webpack, Electron, Node.js, and other ESM environments. |

## Use Cases

- Character navigation in large 3D scenes
- Web games, digital twins, simulations, editors, and visualization projects
- Multi-engine projects that need reusable pathfinding without being tied to Three.js
- Projects that need faster path queries than pure-JS alternatives

## Install

```bash
npm install pathfinding3d
```

Or build from source (requires [Rust](https://rustup.rs/) and [wasm-pack](https://rustwasm.github.io/wasm-pack/)):

```bash
cargo install wasm-pack
wasm-pack build --release
```

The generated npm package is written to `pkg/`.

## Quick Start

```js
import { PathfindingWasm } from "pathfinding3d";

const pathfinding = new PathfindingWasm();

// positions: Float32Array [x, y, z, x, y, z, ...]
// indices:   Uint32Array  [a, b, c, a, b, c, ...]  (triangle list)
pathfinding.create_zone("level-1", positions, indices, 0.001);

const groupId = pathfinding.get_group("level-1", start.x, start.y, start.z, true);
if (groupId === undefined) {
  // start point is not on the navmesh
  return;
}

const output = new Float32Array(1024 * 3); // reuse this buffer every frame
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

> **Note:** WASM is initialized automatically on first import — no separate `init()` call is required with the current build.

## Typical Workflow

```text
NavMesh mesh data
       │
       ▼
 create_zone()          ── build adjacency, groups, spatial index
       │
       ▼
 get_group()             ── which connected region contains the agent?
       │
       ▼
 find_path()             ── A* + funnel → Float32Array waypoints
       │
       ▼
 Move agent along path   ── skip reached points, re-query as needed
```

1. **Prepare navmesh data** — export walkable geometry as world-space vertex positions and triangle indices. Disconnected walkable islands become separate **groups**; pathfinding only works within one group at a time.
2. **Create a zone** — call `create_zone` once per level/scene. Pick a `tolerance` that merges nearly-coincident vertices without collapsing real geometry (see [Parameters](#parameters) below).
3. **Resolve the agent's group** — call `get_group` at the agent position. Store the returned `groupId` and refresh it when the agent teleports to a new island.
4. **Query paths** — call `find_path` with the same `groupId`. Reuse a preallocated `Float32Array` output buffer to avoid allocations in your game loop.
5. **Handle unreachable targets** — `find_path` returns `0` when no path exists (different group, blocked area, etc.).

## Three.js Integration

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

The same pattern applies to Babylon.js, PlayCanvas, Cesium, or any engine that exposes geometry buffers.

## Parameters

### `tolerance` (zone build)

Distance threshold for **welding** nearly-identical vertices when building the nav graph. Smaller values preserve detail; larger values merge close vertices and simplify the mesh.

- Start with `0.001` for meter-scale scenes (as in the demo).
- Use `0.0001` or smaller for centimeter-precision models.

### `checkPolygon` (group / node queries)

When `true`, `get_group` and `get_closest_node_id` require the query point to lie **on a walkable triangle** (within a small plane-distance tolerance). When `false`, the nearest triangle by centroid distance is returned — useful for rough pre-selection but less precise.

Always pass `true` for gameplay queries unless you have a specific reason not to.

### `output` buffer (`find_path`)

- Must be a `Float32Array` with length ≥ `pointCount * 3`.
- The **start position is not written** to the output; only intermediate and target waypoints are returned.
- If the buffer is too small, the return value still reports the required point count so you can resize and retry.
- Keep one buffer alive and reuse it across frames to minimize GC.

## API Reference

All methods are on `PathfindingWasm`.

### Zone management

| Method | Description |
|--------|-------------|
| `create_zone(zoneId, positions, indices, tolerance)` | Build a named zone. Replaces any previous zone with the same ID. |
| `create_zone_handle(positions, indices, tolerance)` | Build a zone and return a numeric handle (no string lookup). |

### Spatial queries

| Method | Returns | Description |
|--------|---------|-------------|
| `get_group(zoneId, x, y, z, checkPolygon)` | `number \| undefined` | Group index containing (or nearest to) the world position. |
| `get_group_by_handle(zoneHandle, x, y, z, checkPolygon)` | `number \| undefined` | Same as `get_group`, keyed by handle. |
| `get_closest_node_id(zoneId, x, y, z, checkPolygon)` | `number \| undefined` | Closest navigation triangle ID within the resolved group. |

### Pathfinding

| Method | Returns | Description |
|--------|---------|-------------|
| `find_path(zoneId, groupId, sx, sy, sz, tx, ty, tz, output)` | `number` | Waypoint count written to `output`. Returns `0` if no path. |

### Metadata & debugging

| Method | Returns | Description |
|--------|---------|-------------|
| `group_count(zoneId)` | `number \| undefined` | Number of connected groups in the zone. |
| `group_count_by_handle(zoneHandle)` | `number \| undefined` | Same, keyed by handle. |
| `group_node_count(zoneId, groupId)` | `number \| undefined` | Triangle count in a group. |
| `group_node_ids(zoneId, groupId)` | `Uint32Array \| undefined` | Triangle IDs in a group. |
| `group_node_centers(zoneId, groupId)` | `Float64Array \| undefined` | Flat `[x,y,z, ...]` centroids for all triangles in a group. |
| `node_center(zoneId, groupId, nodeId)` | `Float64Array \| undefined` | Centroid `[x, y, z]` of one triangle. |

### Lifecycle

| Method | Description |
|--------|-------------|
| `free()` / `[Symbol.dispose]()` | Release WASM resources when the pathfinder is no longer needed. |

## TypeScript

Type definitions ship with the package:

```ts
import { PathfindingWasm } from "pathfinding3d";
```

See `pathfinding3d.d.ts` for full signatures.

## Integration Notes

- **Coordinate system** — the library does not transform axes. Pass world-space data in the same coordinate system as your renderer (Y-up, Z-up, etc.).
- **Navmesh quality** — non-manifold geometry, duplicate faces, or overly aggressive welding can break adjacency. Disconnected regions are separate groups; bridge them in your level data or handle cross-group travel in game logic.
- **Sloped surfaces** — point-in-triangle tests assume roughly horizontal walkable surfaces (Y-axis projection). Steep or arbitrary-orientation meshes may need preprocessing.
- **Bundlers** — for Vite, enable WASM support (`vite-plugin-wasm` + `vite-plugin-top-level-await`). Webpack 5+ supports `asyncWebAssembly` experiments.

## License

Licensed under either of:

- Apache License, Version 2.0, see [LICENSE_APACHE](LICENSE_APACHE)
- MIT license, see [LICENSE_MIT](LICENSE_MIT)
