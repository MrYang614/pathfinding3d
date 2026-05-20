use crate::astar::{astar_search, AstarScratch};
use crate::builder::create_zone as build_zone_data;
use crate::channel::funnel3d_into;
use crate::impls::{GroupData, PolygonInput, Portal3, Vec3, ZoneInput};
use crate::kdtree::{KdPoint, KdTree};
use crate::math::{is_point_in_triangle, is_vector_in_polygon, judge_dir, point_to_plane_distance};
use crate::utils;
use js_sys::{Float32Array, Float64Array, Uint32Array};
use std::cell::RefCell;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[derive(Clone, Copy)]
struct Aabb {
    min: Vec3,
    max: Vec3,
}

impl Aabb {
    fn from_triangle(vertices: &[Vec3], vertex_indices: [usize; 3]) -> Self {
        let a = vertices[vertex_indices[0]];
        let b = vertices[vertex_indices[1]];
        let c = vertices[vertex_indices[2]];
        let min = Vec3::new(
            a.x.min(b.x).min(c.x),
            a.y.min(b.y).min(c.y),
            a.z.min(b.z).min(c.z),
        );
        let max = Vec3::new(
            a.x.max(b.x).max(c.x),
            a.y.max(b.y).max(c.y),
            a.z.max(b.z).max(c.z),
        );
        Self { min, max }
    }

    fn contains_with_margin(&self, p: &Vec3, margin: f64) -> bool {
        p.x >= self.min.x - margin
            && p.x <= self.max.x + margin
            && p.y >= self.min.y - margin
            && p.y <= self.max.y + margin
            && p.z >= self.min.z - margin
            && p.z <= self.max.z + margin
    }

    fn distance_squared_to_point(&self, p: &Vec3) -> f64 {
        let dx = if p.x < self.min.x {
            self.min.x - p.x
        } else if p.x > self.max.x {
            p.x - self.max.x
        } else {
            0.0
        };
        let dy = if p.y < self.min.y {
            self.min.y - p.y
        } else if p.y > self.max.y {
            p.y - self.max.y
        } else {
            0.0
        };
        let dz = if p.z < self.min.z {
            self.min.z - p.z
        } else if p.z > self.max.z {
            p.z - self.max.z
        } else {
            0.0
        };
        dx * dx + dy * dy + dz * dz
    }
}

struct GroupSpatialData {
    bounds: Aabb,
    node_bounds: Vec<Aabb>,
    node_tree: KdTree<usize>,
}

#[derive(Clone, Copy)]
struct NodeRef {
    group_idx: usize,
    node_idx: usize,
}

struct ZoneData {
    zone: ZoneInput,
    group_data: Vec<GroupData>,
    group_spatial: Vec<GroupSpatialData>,
    node_tree: KdTree<NodeRef>,
    astar_scratch: RefCell<Vec<AstarScratch>>,
    path_scratch: RefCell<Vec<PathScratch>>,
}

struct PathScratch {
    channel_portals: Vec<Portal3>,
    points: Vec<Vec3>,
    flat_points: Vec<f32>,
}

impl PathScratch {
    fn with_capacity(len: usize) -> Self {
        Self {
            channel_portals: Vec::with_capacity(len.saturating_add(1)),
            points: Vec::with_capacity(len.saturating_add(1)),
            flat_points: Vec::with_capacity(len.saturating_add(1).saturating_mul(3)),
        }
    }
}

#[wasm_bindgen]
pub struct PathfindingWasm {
    zones: HashMap<u32, ZoneData>,
    zone_names: HashMap<String, u32>,
    next_zone_handle: u32,
}

#[wasm_bindgen]
impl PathfindingWasm {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        utils::set_panic_hook();
        Self {
            zones: HashMap::new(),
            zone_names: HashMap::new(),
            next_zone_handle: 1,
        }
    }

    pub fn create_zone_handle(
        &mut self,
        positions: Vec<f32>,
        indices: Vec<u32>,
        tolerance: f64,
    ) -> Result<u32, JsValue> {
        let zone =
            build_zone_data(&positions, &indices, tolerance).map_err(|e| JsValue::from_str(&e))?;
        let zone_data = build_zone_runtime_data(zone);
        let zone_handle = self.next_zone_handle;
        self.next_zone_handle = self.next_zone_handle.wrapping_add(1).max(1);
        self.zones.insert(zone_handle, zone_data);
        Ok(zone_handle)
    }

    pub fn create_zone(
        &mut self,
        zone_id: String,
        positions: Vec<f32>,
        indices: Vec<u32>,
        tolerance: f64,
    ) -> Result<(), JsValue> {
        let zone_handle = self.create_zone_handle(positions, indices, tolerance)?;
        if let Some(previous_handle) = self.zone_names.insert(zone_id, zone_handle) {
            self.zones.remove(&previous_handle);
        }
        Ok(())
    }

    pub fn group_count(&self, zone_id: String) -> Option<u32> {
        let zone_data = self.zone_from_name(&zone_id)?;
        Some(zone_data.zone.groups.len() as u32)
    }

    pub fn group_count_by_handle(&self, zone_handle: u32) -> Option<u32> {
        self.zones
            .get(&zone_handle)
            .map(|zone_data| zone_data.zone.groups.len() as u32)
    }

    pub fn group_node_count(&self, zone_id: String, group_id: usize) -> Option<u32> {
        self.zone_from_name(&zone_id)
            .and_then(|zone_data| zone_data.zone.groups.get(group_id))
            .map(|group| group.len() as u32)
    }

    pub fn group_node_ids(&self, zone_id: String, group_id: usize) -> Option<Uint32Array> {
        let zone_data = self.zone_from_name(&zone_id)?;
        let group = zone_data.zone.groups.get(group_id)?;
        let ids: Vec<u32> = group.iter().map(|node| node.id as u32).collect();
        Some(Uint32Array::from(ids.as_slice()))
    }

    pub fn group_node_centers(&self, zone_id: String, group_id: usize) -> Option<Float64Array> {
        let zone_data = self.zone_from_name(&zone_id)?;
        let group = zone_data.zone.groups.get(group_id)?;
        let mut centers = Vec::with_capacity(group.len() * 3);
        for node in group {
            centers.push(node.center.x);
            centers.push(node.center.y);
            centers.push(node.center.z);
        }
        Some(Float64Array::from(centers.as_slice()))
    }

    pub fn node_center(
        &self,
        zone_id: String,
        group_id: usize,
        node_id: usize,
    ) -> Option<Float64Array> {
        let zone_data = self.zone_from_name(&zone_id)?;
        let group_nodes = zone_data.zone.groups.get(group_id)?;
        let group_data = zone_data.group_data.get(group_id)?;
        let node = group_data.node_by_id(group_nodes, node_id)?;
        let center = [node.center.x, node.center.y, node.center.z];
        Some(Float64Array::from(center.as_slice()))
    }

    pub fn get_group(
        &self,
        zone_id: String,
        x: f64,
        y: f64,
        z: f64,
        check_polygon: bool,
    ) -> Option<u32> {
        let pos = Vec3::new(x, y, z);
        let zone_data = self.zone_from_name(&zone_id)?;
        compute_group(zone_data, &pos, check_polygon).map(|id| id as u32)
    }

    pub fn get_group_by_handle(
        &self,
        zone_handle: u32,
        x: f64,
        y: f64,
        z: f64,
        check_polygon: bool,
    ) -> Option<u32> {
        let pos = Vec3::new(x, y, z);
        let zone_data = self.zones.get(&zone_handle)?;
        compute_group(zone_data, &pos, check_polygon).map(|id| id as u32)
    }

    pub fn get_closest_node_id(
        &self,
        zone_id: String,
        x: f64,
        y: f64,
        z: f64,
        check_polygon: bool,
    ) -> Option<u32> {
        let pos = Vec3::new(x, y, z);
        let zone_data = self.zone_from_name(&zone_id)?;
        let group_id = compute_group(zone_data, &pos, check_polygon)?;
        let Some(group) = zone_data.zone.groups.get(group_id) else {
            return None;
        };
        let spatial = zone_data.group_spatial.get(group_id)?;
        get_closest_node_index(
            group,
            &zone_data.zone.vertices,
            spatial,
            &pos,
            check_polygon,
        )
        .and_then(|idx| group.get(idx))
        .map(|node| node.id as u32)
    }

    pub fn find_path(
        &self,
        zone_id: String,
        group_id: usize,
        start_x: f64,
        start_y: f64,
        start_z: f64,
        target_x: f64,
        target_y: f64,
        target_z: f64,
        output: &Float32Array,
    ) -> i32 {
        let start_pos = Vec3::new(start_x, start_y, start_z);
        let target_pos = Vec3::new(target_x, target_y, target_z);

        let Some(zone_data) = self.zone_from_name(&zone_id) else {
            return 0;
        };
        let Some(group_nodes) = zone_data.zone.groups.get(group_id) else {
            return 0;
        };
        let Some(group_data) = zone_data.group_data.get(group_id) else {
            return 0;
        };
        let Some(group_spatial) = zone_data.group_spatial.get(group_id) else {
            return 0;
        };
        let mut astar_scratch = zone_data.astar_scratch.borrow_mut();
        let Some(scratch) = astar_scratch.get_mut(group_id) else {
            return 0;
        };
        let mut path_scratch = zone_data.path_scratch.borrow_mut();
        let Some(path_scratch) = path_scratch.get_mut(group_id) else {
            return 0;
        };

        if compute_path_points(
            group_nodes,
            group_data,
            &zone_data.zone.vertices,
            group_spatial,
            &start_pos,
            &target_pos,
            scratch,
            path_scratch,
        )
        .is_none()
        {
            return 0;
        }

        write_path_to_output(&path_scratch.points, output, &mut path_scratch.flat_points)
    }
}

impl PathfindingWasm {
    fn zone_handle_by_name(&self, zone_id: &str) -> Option<u32> {
        self.zone_names.get(zone_id).copied()
    }

    fn zone_from_name(&self, zone_id: &str) -> Option<&ZoneData> {
        let zone_handle = self.zone_handle_by_name(zone_id)?;
        self.zones.get(&zone_handle)
    }
}

fn build_zone_runtime_data(zone: ZoneInput) -> ZoneData {
    let group_data: Vec<GroupData> = zone
        .groups
        .iter()
        .map(|group_nodes| GroupData::from_nodes(group_nodes))
        .collect();
    let (group_spatial, node_tree) = build_spatial_index(&zone);
    let astar_scratch = group_data
        .iter()
        .map(|group_data| AstarScratch::with_len(group_data.len()))
        .collect();
    let path_scratch = group_data
        .iter()
        .map(|group_data| PathScratch::with_capacity(group_data.len()))
        .collect();
    ZoneData {
        zone,
        group_data,
        group_spatial,
        node_tree,
        astar_scratch: RefCell::new(astar_scratch),
        path_scratch: RefCell::new(path_scratch),
    }
}

fn build_spatial_index(zone: &ZoneInput) -> (Vec<GroupSpatialData>, KdTree<NodeRef>) {
    let mut groups = Vec::with_capacity(zone.groups.len());
    let total_nodes = zone.groups.iter().map(Vec::len).sum();
    let mut all_points = Vec::with_capacity(total_nodes);
    for (group_idx, group) in zone.groups.iter().enumerate() {
        let mut node_bounds = Vec::with_capacity(group.len());
        let mut group_points = Vec::with_capacity(group.len());
        let mut group_min = Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut group_max = Vec3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
        for (node_idx, node) in group.iter().enumerate() {
            let bounds = Aabb::from_triangle(&zone.vertices, node.vertex_indices);
            group_min = group_min.min(bounds.min);
            group_max = group_max.max(bounds.max);
            node_bounds.push(bounds);
            group_points.push(KdPoint {
                point: node.center,
                item: node_idx,
            });
            all_points.push(KdPoint {
                point: node.center,
                item: NodeRef {
                    group_idx,
                    node_idx,
                },
            });
        }
        groups.push(GroupSpatialData {
            bounds: Aabb {
                min: group_min,
                max: group_max,
            },
            node_bounds,
            node_tree: KdTree::from_points(group_points),
        });
    }
    (groups, KdTree::from_points(all_points))
}

fn write_path_to_output(path: &[Vec3], output: &Float32Array, flat_points: &mut Vec<f32>) -> i32 {
    let path_len = path.len().saturating_sub(1);
    if path_len > i32::MAX as usize {
        return i32::MIN;
    }
    let required_output_len = path_len.saturating_mul(3);
    if output.length() < required_output_len as u32 {
        return path_len as i32;
    }

    flat_points.clear();
    flat_points.reserve(required_output_len);
    for p in path.iter().skip(1) {
        flat_points.push(p.x as f32);
        flat_points.push(p.y as f32);
        flat_points.push(p.z as f32);
    }

    if required_output_len > 0 {
        output
            .subarray(0, required_output_len as u32)
            .copy_from(flat_points.as_slice());
    }

    path_len as i32
}

fn compute_path_points(
    group_nodes: &[PolygonInput],
    group_data: &GroupData,
    vertices: &[Vec3],
    group_spatial: &GroupSpatialData,
    start: &Vec3,
    target: &Vec3,
    scratch: &mut AstarScratch,
    path_scratch: &mut PathScratch,
) -> Option<()> {
    let closest_idx = get_closest_node_index(group_nodes, vertices, group_spatial, start, true)?;
    let farthest_idx = get_closest_node_index(group_nodes, vertices, group_spatial, target, true)?;
    let path_indices = astar_search(group_nodes, group_data, closest_idx, farthest_idx, scratch);

    let channel_portals = &mut path_scratch.channel_portals;
    channel_portals.clear();
    channel_portals.reserve(path_indices.len().saturating_add(1));
    if !path_indices.is_empty() {
        if let Some(portal1) = group_data.portal_between_indices(closest_idx, path_indices[0]) {
            let left = vertices[portal1[0]];
            let right = vertices[portal1[1]];
            let v1 = (*start - left).normalize_or_zero();
            let v2 = (right - *start).normalize_or_zero();
            if v1.dot(v2) != 1.0 {
                channel_portals.push(Portal3 { left, right });
            }
        }
    }

    for pair in path_indices.windows(2) {
        if let [current_idx, next_idx] = pair {
            if let Some(portal) = group_data.portal_between_indices(*current_idx, *next_idx) {
                channel_portals.push(Portal3 {
                    left: vertices[portal[0]],
                    right: vertices[portal[1]],
                });
            }
        }
    }
    channel_portals.push(Portal3 {
        left: *target,
        right: *target,
    });

    let mut apex = *start;
    for portal in channel_portals.iter_mut() {
        if judge_dir(&apex, &portal.left, &portal.right) < 0.0 {
        } else {
            std::mem::swap(&mut portal.left, &mut portal.right);
        }
        apex = (portal.left + portal.right) * 0.5;
    }

    funnel3d_into(*start, *target, channel_portals, &mut path_scratch.points);
    Some(())
}

fn compute_group(zone_data: &ZoneData, position: &Vec3, check_polygon: bool) -> Option<usize> {
    const MAX_GROUP_DISTANCE_SQUARED: f64 = 50.0 * 50.0;

    if check_polygon {
        if let Some((node_ref, _)) = zone_data.node_tree.nearest_matching(
            position,
            MAX_GROUP_DISTANCE_SQUARED,
            |node_ref, _| {
                let Some(group) = zone_data.zone.groups.get(node_ref.group_idx) else {
                    return false;
                };
                let Some(node) = group.get(node_ref.node_idx) else {
                    return false;
                };
                let Some(bounds) = zone_data
                    .group_spatial
                    .get(node_ref.group_idx)
                    .and_then(|spatial| spatial.node_bounds.get(node_ref.node_idx))
                else {
                    return false;
                };
                if !bounds.contains_with_margin(position, 0.5) {
                    return false;
                }
                let [ia, ib, ic] = node.vertex_indices;
                let a = zone_data.zone.vertices[ia];
                let b = zone_data.zone.vertices[ib];
                let c = zone_data.zone.vertices[ic];
                point_to_plane_distance(position, &a, &b, &c).abs() < 0.01
                    && is_point_in_triangle(a, b, c, *position)
            },
        ) {
            return Some(node_ref.group_idx);
        }
    }

    zone_data
        .node_tree
        .nearest_matching(
            position,
            MAX_GROUP_DISTANCE_SQUARED,
            |node_ref, distance| {
                zone_data
                    .group_spatial
                    .get(node_ref.group_idx)
                    .is_some_and(|spatial| {
                        spatial.bounds.distance_squared_to_point(position) <= distance
                    })
            },
        )
        .map(|(node_ref, _)| node_ref.group_idx)
}

fn get_closest_node_index(
    nodes: &[PolygonInput],
    vertices: &[Vec3],
    spatial: &GroupSpatialData,
    position: &Vec3,
    check_polygon: bool,
) -> Option<usize> {
    spatial
        .node_tree
        .nearest_matching(position, f64::INFINITY, |idx, distance| {
            let Some(node) = nodes.get(idx) else {
                return false;
            };
            let Some(bounds) = spatial.node_bounds.get(idx) else {
                return false;
            };
            if bounds.distance_squared_to_point(position) > distance {
                return false;
            }
            !check_polygon
                || (bounds.contains_with_margin(position, 0.5)
                    && is_vector_in_polygon(position, node, vertices))
        })
        .map(|(idx, _)| idx)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn square_zone() -> ZoneData {
        let positions = [
            0.0f32, 0.0, 0.0, //
            1.0, 0.0, 0.0, //
            0.0, 0.0, 1.0, //
            1.0, 0.0, 1.0,
        ];
        let indices = [0u32, 1, 2, 1, 3, 2];
        let zone = build_zone_data(&positions, &indices, 0.0001).expect("zone builds");
        build_zone_runtime_data(zone)
    }

    #[test]
    fn computes_group_and_path_for_two_triangle_square() {
        let zone_data = square_zone();
        let start = Vec3::new(0.1, 0.0, 0.1);
        let target = Vec3::new(0.9, 0.0, 0.9);

        assert_eq!(compute_group(&zone_data, &start, true), Some(0));

        let mut scratch = AstarScratch::with_len(zone_data.group_data[0].len());
        let mut path_scratch = PathScratch::with_capacity(zone_data.group_data[0].len());
        compute_path_points(
            &zone_data.zone.groups[0],
            &zone_data.group_data[0],
            &zone_data.zone.vertices,
            &zone_data.group_spatial[0],
            &start,
            &target,
            &mut scratch,
            &mut path_scratch,
        )
        .expect("path exists");

        assert!(!path_scratch.points.is_empty());
        assert!(path_scratch.points.last().unwrap().distance_squared(target) < 0.0001);
    }
}
