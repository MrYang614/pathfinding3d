use crate::impls::{GroupData, PolygonInput};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Clone)]
struct HeapNode {
    idx: usize,
    f: f64,
}

#[derive(Clone, Copy, Default)]
struct FlagWord {
    visited: usize,
    closed: usize,
}

const WORD_BITS: usize = usize::BITS as usize;

#[inline]
fn bit_position(idx: usize) -> (usize, usize) {
    (idx / WORD_BITS, 1 << (idx % WORD_BITS))
}

pub(crate) struct AstarScratch {
    open: BinaryHeap<HeapNode>,
    flags: Vec<FlagWord>,
    dirty_words: Vec<usize>,
    g_score: Vec<f64>,
    h_score: Vec<f64>,
    parent: Vec<usize>,
    path: Vec<usize>,
}

impl AstarScratch {
    pub(crate) fn with_len(len: usize) -> Self {
        let word_count = len.div_ceil(WORD_BITS);
        Self {
            open: BinaryHeap::with_capacity(len),
            flags: vec![FlagWord::default(); word_count],
            dirty_words: Vec::with_capacity(word_count),
            g_score: vec![0.0; len],
            h_score: vec![0.0; len],
            parent: vec![0; len],
            path: Vec::with_capacity(len),
        }
    }

    pub(crate) fn reset(&mut self, len: usize) {
        if self.g_score.len() != len {
            *self = Self::with_len(len);
            return;
        }
        self.open.clear();
        self.path.clear();
        // A first visit overwrites the scores and parent before they are used.
        for word in self.dirty_words.drain(..) {
            self.flags[word] = FlagWord::default();
        }
    }

    #[inline]
    fn visit(&mut self, idx: usize) -> bool {
        let (word, bit) = bit_position(idx);
        let visited = &mut self.flags[word].visited;
        if *visited & bit != 0 {
            return false;
        }
        if *visited == 0 {
            self.dirty_words.push(word);
        }
        *visited |= bit;
        true
    }

    #[inline]
    fn is_closed(&self, idx: usize) -> bool {
        let (word, bit) = bit_position(idx);
        self.flags[word].closed & bit != 0
    }

    #[inline]
    fn close(&mut self, idx: usize) {
        let (word, bit) = bit_position(idx);
        self.flags[word].closed |= bit;
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
    scratch.visit(start_idx);
    scratch.g_score[start_idx] = 0.0;

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

        if scratch.is_closed(current_idx) {
            continue;
        }
        scratch.close(current_idx);

        let current_node = &group_nodes[current_idx];
        let current_g = scratch.g_score[current_idx];

        for neighbour in group_data.neighbours(current_idx) {
            let neighbour_idx = neighbour.index;
            if scratch.is_closed(neighbour_idx) {
                continue;
            }

            let tentative_g = current_g
                + current_node
                    .center
                    .distance(group_nodes[neighbour_idx].center);
            let first_visit = scratch.visit(neighbour_idx);
            if first_visit || tentative_g < scratch.g_score[neighbour_idx] {
                scratch.parent[neighbour_idx] = current_idx;
                if first_visit {
                    scratch.h_score[neighbour_idx] =
                        group_nodes[neighbour_idx].center.distance(end_center);
                }
                scratch.g_score[neighbour_idx] = tentative_g;
                scratch.open.push(HeapNode {
                    idx: neighbour_idx,
                    f: tentative_g + scratch.h_score[neighbour_idx],
                });
            }
        }
    }

    scratch.path.as_slice()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::impls::Vec3;

    fn chain(len: usize) -> (Vec<PolygonInput>, GroupData) {
        let nodes: Vec<_> = (0..len)
            .map(|id| {
                let mut neighbours = Vec::new();
                if id > 0 {
                    neighbours.push(id - 1);
                }
                if id + 1 < len {
                    neighbours.push(id + 1);
                }
                PolygonInput {
                    id,
                    group_id: 0,
                    portals: vec![[0, 1]; neighbours.len()],
                    neighbours,
                    vertex_indices: [0, 1, 2],
                    center: Vec3::new(id as f64, 0.0, 0.0),
                }
            })
            .collect();
        let group = GroupData::from_nodes(&nodes);
        (nodes, group)
    }

    #[test]
    fn reuses_scratch_for_reverse_path_across_word_boundaries() {
        let last = 2 * usize::BITS as usize;
        let (nodes, group) = chain(last + 1);
        let mut scratch = AstarScratch::with_len(nodes.len());

        assert_eq!(
            astar_search(&nodes, &group, 0, last, &mut scratch),
            (1..=last).collect::<Vec<_>>()
        );
        assert_eq!(
            astar_search(&nodes, &group, last, 0, &mut scratch),
            (0..last).rev().collect::<Vec<_>>()
        );
    }

    #[test]
    fn resizes_scratch_when_node_count_grows_within_the_same_word() {
        let boundary = usize::BITS as usize;
        let mut scratch = AstarScratch::with_len(boundary + 1);
        let (nodes, group) = chain(boundary + 2);

        assert_eq!(
            astar_search(&nodes, &group, boundary, boundary + 1, &mut scratch),
            [boundary + 1]
        );
    }
}
