use crate::impls::{PolygonInput, Vec3, ZoneInput};
use std::collections::{HashMap, VecDeque};

pub(crate) fn create_zone(
    positions: &[f32],
    indices: &[u32],
    tolerance: f64,
) -> Result<ZoneInput, String> {
    if !positions.len().is_multiple_of(3) {
        return Err("positions length must be multiple of 3".to_string());
    }
    if !indices.len().is_multiple_of(3) {
        return Err("indices length must be multiple of 3".to_string());
    }
    let vertex_count = positions.len() / 3;
    for idx in indices {
        if *idx as usize >= vertex_count {
            return Err("index out of bounds for positions".to_string());
        }
    }

    Ok(build_zone_impl(
        positions,
        indices,
        tolerance.max(f64::EPSILON),
    ))
}

fn build_zone_impl(positions: &[f32], indices: &[u32], tolerance: f64) -> ZoneInput {
    let mut hash_to_index: HashMap<(i64, i64, i64), usize> = HashMap::with_capacity(indices.len());
    let mut new_positions: Vec<Vec3> = Vec::with_capacity(indices.len());
    let mut remapped_indices: Vec<usize> = Vec::with_capacity(indices.len());
    let shift_multiplier = 1.0 / tolerance;

    for idx in indices {
        let i = *idx as usize;
        let x = positions[i * 3] as f64;
        let y = positions[i * 3 + 1] as f64;
        let z = positions[i * 3 + 2] as f64;
        let key = (
            (x * shift_multiplier).trunc() as i64,
            (y * shift_multiplier).trunc() as i64,
            (z * shift_multiplier).trunc() as i64,
        );
        if let Some(mapped) = hash_to_index.get(&key).copied() {
            remapped_indices.push(mapped);
        } else {
            let mapped = new_positions.len();
            new_positions.push(Vec3::new(x, y, z));
            hash_to_index.insert(key, mapped);
            remapped_indices.push(mapped);
        }
    }

    let triangle_count = remapped_indices.len() / 3;
    let mut triangles: Vec<PolygonInput> = Vec::with_capacity(triangle_count);

    for tri_idx in 0..triangle_count {
        let base = tri_idx * 3;
        let a = remapped_indices[base];
        let b = remapped_indices[base + 1];
        let c = remapped_indices[base + 2];
        let center = (new_positions[a] + new_positions[b] + new_positions[c]) * (1.0 / 3.0);
        let triangle = PolygonInput {
            id: tri_idx,
            group_id: -1,
            neighbours: Vec::with_capacity(3),
            portals: Vec::with_capacity(3),
            vertex_indices: [a, b, c],
            center,
        };
        triangles.push(triangle);
    }

    let mut edge_to_triangle: HashMap<(usize, usize), usize> =
        HashMap::with_capacity(triangle_count * 3);
    for tri_idx in 0..triangle_count {
        let [a, b, c] = triangles[tri_idx].vertex_indices;
        for portal in [[a, b], [b, c], [c, a]] {
            let edge = if portal[0] < portal[1] {
                (portal[0], portal[1])
            } else {
                (portal[1], portal[0])
            };
            if let Some(other_idx) = edge_to_triangle.insert(edge, tri_idx) {
                bind_neighbour(&mut triangles, tri_idx, other_idx, portal);
            }
        }
    }

    let mut current_group: i32 = 0;
    for tri_idx in 0..triangle_count {
        if triangles[tri_idx].group_id != -1 {
            continue;
        }
        triangles[tri_idx].group_id = current_group;
        spread_group_id(&mut triangles, tri_idx, current_group);
        current_group += 1;
    }

    let mut groups: Vec<Vec<PolygonInput>> = vec![Vec::new(); current_group as usize];
    let mut global_to_local = vec![usize::MAX; triangle_count];
    for mut triangle in triangles {
        let group_idx = triangle.group_id as usize;
        let global_id = triangle.id;
        let local_id = groups[group_idx].len();
        global_to_local[global_id] = local_id;
        triangle.id = local_id;
        groups[group_idx].push(triangle);
    }
    for group in &mut groups {
        for node in group {
            for neighbour_id in &mut node.neighbours {
                let local_id = global_to_local[*neighbour_id];
                debug_assert_ne!(local_id, usize::MAX);
                *neighbour_id = local_id;
            }
        }
    }

    ZoneInput {
        groups,
        vertices: new_positions,
    }
}

fn bind_neighbour(
    triangles: &mut [PolygonInput],
    source_idx: usize,
    other_idx: usize,
    portal: [usize; 2],
) {
    if source_idx == other_idx {
        return;
    }
    let (left, right) = if source_idx < other_idx {
        let (left, right) = triangles.split_at_mut(other_idx);
        (&mut left[source_idx], &mut right[0])
    } else {
        let (left, right) = triangles.split_at_mut(source_idx);
        (&mut right[0], &mut left[other_idx])
    };

    left.neighbours.push(right.id);
    left.portals.push(portal);
    right.neighbours.push(left.id);
    right.portals.push(portal);
}

fn spread_group_id(triangles: &mut [PolygonInput], seed_idx: usize, group_id: i32) {
    let mut queue = VecDeque::new();
    queue.push_back(seed_idx);
    while let Some(idx) = queue.pop_front() {
        triangles[idx].group_id = group_id;
        let neighbour_count = triangles[idx].neighbours.len();
        for neighbour_pos in 0..neighbour_count {
            let next_idx = triangles[idx].neighbours[neighbour_pos];
            if next_idx < triangles.len() && triangles[next_idx].group_id == -1 {
                triangles[next_idx].group_id = group_id;
                queue.push_back(next_idx);
            }
        }
    }
}
