use crate::impls::{GroupData, PolygonInput};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Clone)]
struct HeapNode {
    idx: usize,
    f: f64,
}

pub(crate) struct AstarScratch {
    open: BinaryHeap<HeapNode>,
    closed: Vec<bool>,
    visited: Vec<bool>,
    g_score: Vec<f64>,
    h_score: Vec<f64>,
    h_seen: Vec<bool>,
    parent: Vec<usize>,
    touched: Vec<usize>,
    path: Vec<usize>,
}

impl AstarScratch {
    pub(crate) fn with_len(len: usize) -> Self {
        Self {
            open: BinaryHeap::with_capacity(len),
            closed: vec![false; len],
            visited: vec![false; len],
            g_score: vec![f64::INFINITY; len],
            h_score: vec![0.0; len],
            h_seen: vec![false; len],
            parent: vec![usize::MAX; len],
            touched: Vec::with_capacity(len),
            path: Vec::with_capacity(len),
        }
    }

    pub(crate) fn reset(&mut self, len: usize) {
        if self.closed.len() != len {
            *self = Self::with_len(len);
            return;
        }
        self.open.clear();
        self.path.clear();
        for idx in self.touched.drain(..) {
            self.closed[idx] = false;
            self.visited[idx] = false;
            self.g_score[idx] = f64::INFINITY;
            self.h_seen[idx] = false;
            self.parent[idx] = usize::MAX;
        }
    }

    #[inline]
    pub(crate) fn touch_if_unseen(&mut self, idx: usize) {
        if !self.visited[idx] {
            self.touched.push(idx);
        }
    }
}

impl PartialEq for HeapNode {
    fn eq(&self, other: &Self) -> bool {
        self.idx == other.idx && self.f == other.f
    }
}

impl Eq for HeapNode {}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .f
            .partial_cmp(&self.f)
            .unwrap_or(Ordering::Equal)
            .then_with(|| self.idx.cmp(&other.idx))
    }
}

pub(crate) fn astar_search<'scratch>(
    group_nodes: &[PolygonInput],
    group_data: &GroupData,
    start_idx: usize,
    end_idx: usize,
    scratch: &'scratch mut AstarScratch,
) -> &'scratch [usize] {
    scratch.path.clear();
    let Some(end_center) = group_nodes.get(end_idx).map(|n| n.center) else {
        return scratch.path.as_slice();
    };
    if start_idx >= group_data.len() {
        return scratch.path.as_slice();
    }

    let len = group_data.len();
    scratch.reset(len);

    scratch.open.push(HeapNode {
        idx: start_idx,
        f: 0.0,
    });
    scratch.touch_if_unseen(start_idx);
    scratch.g_score[start_idx] = 0.0;
    scratch.visited[start_idx] = true;

    while let Some(current) = scratch.open.pop() {
        let current_idx = current.idx;
        if current_idx == end_idx {
            let mut cursor_idx = current_idx;
            while cursor_idx != start_idx {
                scratch.path.push(cursor_idx);
                cursor_idx = scratch.parent[cursor_idx];
            }
            scratch.path.reverse();
            return scratch.path.as_slice();
        }

        if scratch.closed[current_idx] {
            continue;
        }
        scratch.closed[current_idx] = true;

        let current_node = &group_nodes[current_idx];
        let current_g = scratch.g_score[current_idx];

        for neighbour in group_data.neighbours(current_idx) {
            let neighbour_idx = neighbour.index;
            if scratch.closed[neighbour_idx] {
                continue;
            }

            let tentative_g = current_g
                + current_node
                    .center
                    .distance_squared(group_nodes[neighbour_idx].center);
            let known_g = scratch.g_score[neighbour_idx];
            let been_visited = scratch.visited[neighbour_idx];
            if !been_visited || tentative_g < known_g {
                scratch.touch_if_unseen(neighbour_idx);
                scratch.visited[neighbour_idx] = true;
                scratch.parent[neighbour_idx] = current_idx;
                let h = if !scratch.h_seen[neighbour_idx] {
                    let h = group_nodes[neighbour_idx].center.distance(end_center);
                    scratch.h_score[neighbour_idx] = h;
                    scratch.h_seen[neighbour_idx] = true;
                    h
                } else {
                    scratch.h_score[neighbour_idx]
                };
                scratch.g_score[neighbour_idx] = tentative_g;
                scratch.open.push(HeapNode {
                    idx: neighbour_idx,
                    f: tentative_g + h,
                });
            }
        }
    }

    scratch.path.as_slice()
}
