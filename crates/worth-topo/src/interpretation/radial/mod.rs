use std::collections::{BTreeMap, BTreeSet};

use forge_relational::facade::identity::EntityId;

use crate::data::topology_view::TopologyHalfEdge;
use crate::interpretation::types::RadialInterpretationSummary;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RadialInterpretation {
    pub boundary_half_edge_count: usize,
    pub non_manifold_edge_ids: Vec<EntityId>,
}

pub fn summarize_shell_radial(
    shell_id: EntityId,
    shell_half_edges: &BTreeSet<EntityId>,
    half_edge_map: &BTreeMap<EntityId, &TopologyHalfEdge>,
) -> RadialInterpretationSummary {
    let radial = interpret_radial_surface(shell_half_edges, half_edge_map);
    RadialInterpretationSummary {
        shell_id,
        boundary_half_edge_count: radial.boundary_half_edge_count,
        non_manifold_edge_ids: radial.non_manifold_edge_ids,
    }
}

pub fn interpret_radial_surface(
    shell_half_edges: &BTreeSet<EntityId>,
    half_edge_map: &BTreeMap<EntityId, &TopologyHalfEdge>,
) -> RadialInterpretation {
    let mut boundary_half_edge_count = 0;
    let mut non_manifold_edge_ids = BTreeSet::new();

    for half_edge_id in shell_half_edges {
        let Some(half_edge) = half_edge_map.get(half_edge_id).copied() else {
            continue;
        };
        if half_edge.radial_next_half_edge_id == Some(half_edge.entity_id) {
            boundary_half_edge_count += 1;
            continue;
        }
        if let Some(edge_id) = half_edge.edge_id {
            let ring_len = walk_radial_ring_len(half_edge.entity_id, half_edge_map);
            if ring_len > 2 {
                non_manifold_edge_ids.insert(edge_id);
            }
        }
    }

    RadialInterpretation {
        boundary_half_edge_count,
        non_manifold_edge_ids: non_manifold_edge_ids.into_iter().collect(),
    }
}

pub fn walk_radial_ring_len(
    start_id: EntityId,
    half_edge_map: &BTreeMap<EntityId, &TopologyHalfEdge>,
) -> usize {
    let mut count = 0;
    let mut seen = BTreeSet::new();
    let mut current_id = start_id;

    loop {
        if !seen.insert(current_id) {
            break;
        }
        count += 1;
        let Some(current) = half_edge_map.get(&current_id).copied() else {
            break;
        };
        let Some(next_id) = current.radial_next_half_edge_id else {
            break;
        };
        current_id = next_id;
    }

    count
}
