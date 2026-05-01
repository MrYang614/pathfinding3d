use crate::impls::Vec3;
use std::cmp::Ordering;

#[derive(Clone, Copy)]
pub(crate) struct KdPoint<T: Copy> {
    pub(crate) point: Vec3,
    pub(crate) item: T,
}

pub(crate) struct KdTree<T: Copy> {
    nodes: Vec<KdNode<T>>,
    root: Option<usize>,
}

struct KdNode<T: Copy> {
    point: Vec3,
    item: T,
    axis: usize,
    left: Option<usize>,
    right: Option<usize>,
}

impl<T: Copy> KdTree<T> {
    pub(crate) fn from_points(mut points: Vec<KdPoint<T>>) -> Self {
        let mut nodes = Vec::with_capacity(points.len());
        let root = build_nodes(&mut points, &mut nodes, 0);
        Self { nodes, root }
    }

    pub(crate) fn nearest_matching<F>(
        &self,
        query: &Vec3,
        max_distance_squared: f64,
        mut predicate: F,
    ) -> Option<(T, f64)>
    where
        F: FnMut(T, f64) -> bool,
    {
        let mut best = None;
        let mut best_distance = max_distance_squared;
        if let Some(root) = self.root {
            self.search_nearest(root, query, &mut best_distance, &mut best, &mut predicate);
        }
        best.map(|item| (item, best_distance))
    }

    fn search_nearest<F>(
        &self,
        node_idx: usize,
        query: &Vec3,
        best_distance: &mut f64,
        best: &mut Option<T>,
        predicate: &mut F,
    ) where
        F: FnMut(T, f64) -> bool,
    {
        let node = &self.nodes[node_idx];
        let distance = node.point.distance_squared(*query);
        if distance < *best_distance && predicate(node.item, distance) {
            *best_distance = distance;
            *best = Some(node.item);
        }

        let delta = axis_value(query, node.axis) - axis_value(&node.point, node.axis);
        let (near, far) = if delta <= 0.0 {
            (node.left, node.right)
        } else {
            (node.right, node.left)
        };

        if let Some(near_idx) = near {
            self.search_nearest(near_idx, query, best_distance, best, predicate);
        }
        if delta * delta < *best_distance {
            if let Some(far_idx) = far {
                self.search_nearest(far_idx, query, best_distance, best, predicate);
            }
        }
    }
}

fn build_nodes<T: Copy>(
    points: &mut [KdPoint<T>],
    nodes: &mut Vec<KdNode<T>>,
    depth: usize,
) -> Option<usize> {
    if points.is_empty() {
        return None;
    }

    let axis = depth % 3;
    points.sort_unstable_by(|a, b| {
        axis_value(&a.point, axis)
            .partial_cmp(&axis_value(&b.point, axis))
            .unwrap_or(Ordering::Equal)
    });

    let mid = points.len() / 2;
    let (left_points, rest) = points.split_at_mut(mid);
    let (mid_point, right_points) = rest.split_at_mut(1);
    let point = mid_point[0];
    let node_idx = nodes.len();
    nodes.push(KdNode {
        point: point.point,
        item: point.item,
        axis,
        left: None,
        right: None,
    });

    let left = build_nodes(left_points, nodes, depth + 1);
    let right = build_nodes(right_points, nodes, depth + 1);
    nodes[node_idx].left = left;
    nodes[node_idx].right = right;
    Some(node_idx)
}

#[inline]
fn axis_value(point: &Vec3, axis: usize) -> f64 {
    match axis {
        0 => point.x,
        1 => point.y,
        _ => point.z,
    }
}
