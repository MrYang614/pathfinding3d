pub type Vec3 = glam::DVec3;

#[derive(Debug, Clone)]
pub struct PolygonInput {
    pub id: usize,
    pub group_id: i32,
    pub neighbours: Vec<usize>,
    pub portals: Vec<[usize; 2]>,
    pub vertex_indices: [usize; 3],
    pub center: Vec3,
}

#[derive(Debug, Clone)]
pub struct ZoneInput {
    pub groups: Vec<Vec<PolygonInput>>,
    pub vertices: Vec<Vec3>,
}

#[derive(Debug, Clone)]
pub struct Portal3 {
    pub left: Vec3,
    pub right: Vec3,
}

#[derive(Debug, Clone)]
pub struct GroupData {
    id_to_index: Vec<Option<usize>>,
    neighbours_by_index: Vec<Vec<NeighborLink>>,
    len: usize,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct NeighborLink {
    pub(crate) index: usize,
    pub(crate) portal: [usize; 2],
}

impl GroupData {
    pub fn from_nodes(nodes: &[PolygonInput]) -> Self {
        let max_id = nodes.iter().map(|node| node.id).max().unwrap_or(0);
        let mut id_to_index = vec![None; max_id + 1];
        for (idx, node) in nodes.iter().enumerate() {
            id_to_index[node.id] = Some(idx);
        }

        let mut neighbours_by_index = Vec::with_capacity(nodes.len());
        for node in nodes {
            let mut neighbour_links = Vec::with_capacity(node.neighbours.len());
            for (portal_idx, neighbour_id) in node.neighbours.iter().enumerate() {
                if let (Some(index), Some(portal)) = (
                    id_to_index.get(*neighbour_id).and_then(|idx| *idx),
                    node.portals.get(portal_idx).copied(),
                ) {
                    neighbour_links.push(NeighborLink { index, portal });
                }
            }
            neighbours_by_index.push(neighbour_links);
        }
        Self {
            id_to_index,
            neighbours_by_index,
            len: nodes.len(),
        }
    }

    pub fn node_by_id<'a>(&self, nodes: &'a [PolygonInput], id: usize) -> Option<&'a PolygonInput> {
        self.id_to_index
            .get(id)
            .and_then(|idx| idx.map(|idx| &nodes[idx]))
    }

    pub(crate) fn portal_between_indices(
        &self,
        from_idx: usize,
        to_idx: usize,
    ) -> Option<[usize; 2]> {
        self.neighbours_by_index.get(from_idx).and_then(|list| {
            list.iter()
                .find_map(|neighbour| (neighbour.index == to_idx).then_some(neighbour.portal))
        })
    }

    pub(crate) fn neighbours(&self, idx: usize) -> &[NeighborLink] {
        self.neighbours_by_index
            .get(idx)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    pub fn len(&self) -> usize {
        self.len
    }
}
