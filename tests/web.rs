//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

use js_sys::Float32Array;
use pathfinding3d::PathfindingWasm;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

wasm_bindgen_test_configure!(run_in_browser);

const ZONE: &str = "test-mesh";
const SENTINEL: f32 = -99.0;

fn mesh_with_island() -> PathfindingWasm {
    // The first two triangles share an edge. The third is a separate island.
    let vertices = [
        [0.0, 2.0, 0.0],
        [1.0, 2.0, 0.0],
        [0.0, 2.0, 1.0],
        [1.0, 2.0, 1.0],
        [4.0, 2.0, 0.0],
        [5.0, 2.0, 0.0],
        [4.0, 2.0, 1.0],
    ];
    let mut pathfinding = PathfindingWasm::new();
    pathfinding
        .create_zone(
            ZONE.to_owned(),
            vertices.concat(),
            vec![0, 1, 2, 1, 3, 2, 4, 5, 6],
            0.0001,
        )
        .expect("the test mesh should build");
    pathfinding
}

#[wasm_bindgen_test]
fn reachable_route_writes_target_and_preserves_unused_capacity() {
    let pathfinding = mesh_with_island();
    let group = pathfinding
        .get_group(ZONE.to_owned(), 0.25, 2.0, 0.25, true)
        .expect("start lies on the square");
    let find_route = |output: &Float32Array| {
        pathfinding.find_path(
            ZONE.to_owned(),
            group as usize,
            0.25,
            2.0,
            0.25,
            0.875,
            2.0,
            0.75,
            output,
        )
    };

    let short = Float32Array::from([SENTINEL; 2].as_slice());
    assert_eq!(find_route(&short), 1, "return the required point count");
    assert_eq!(short.to_vec(), [SENTINEL; 2], "avoid a partial point write");

    let output = Float32Array::from([SENTINEL; 6].as_slice());
    assert_eq!(find_route(&output), 1);
    assert_eq!(
        output.to_vec(),
        [0.875, 2.0, 0.75, SENTINEL, SENTINEL, SENTINEL]
    );
}

#[wasm_bindgen_test]
fn disconnected_route_returns_zero_and_preserves_output() {
    let pathfinding = mesh_with_island();
    let start_group = pathfinding
        .get_group(ZONE.to_owned(), 0.25, 2.0, 0.25, true)
        .expect("start lies on the square");
    let target_group = pathfinding
        .get_group(ZONE.to_owned(), 4.25, 2.0, 0.25, true)
        .expect("target lies on the island");
    assert_ne!(start_group, target_group);

    let output = Float32Array::from([SENTINEL; 6].as_slice());
    let count = pathfinding.find_path(
        ZONE.to_owned(),
        start_group as usize,
        0.25,
        2.0,
        0.25,
        4.25,
        2.0,
        0.25,
        &output,
    );
    assert_eq!(count, 0);
    assert_eq!(output.to_vec(), [SENTINEL; 6]);
}
