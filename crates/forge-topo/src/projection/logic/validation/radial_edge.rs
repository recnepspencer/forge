use std::collections::{BTreeMap, BTreeSet};

use forge_core::KernelError;

use crate::projection::data::{ProjectedEdgeId, ProjectedHalfEdgeId, ProjectedTopology};
use crate::projection::logic::ProjectedTopologyQueries;

use super::shared::vf;

pub fn validate_projected_radial_edge(topology: &ProjectedTopology) -> Result<(), KernelError> {
    validate_radial_rings(topology)?;
    validate_radial_edge_consistency(topology)?;
    validate_radial_cycle_uniqueness(topology)?;
    validate_radial_neighbor_consistency(topology)?;
    validate_no_broken_radial_splices(topology)?;
    Ok(())
}

fn validate_radial_rings(topology: &ProjectedTopology) -> Result<(), KernelError> {
    for (start_index, _) in topology.half_edges().iter().enumerate() {
        walk_radial_ring(topology, ProjectedHalfEdgeId::new(start_index as u32))
            .map_err(|err| vf("projected_radial_ring_closure", err))?;
    }
    Ok(())
}

fn validate_radial_edge_consistency(topology: &ProjectedTopology) -> Result<(), KernelError> {
    let mut checked = BTreeSet::new();
    for (start_index, start_half_edge) in topology.half_edges().iter().enumerate() {
        let start_id = ProjectedHalfEdgeId::new(start_index as u32);
        if !checked.insert(start_id.raw()) {
            continue;
        }
        let expected_edge = start_half_edge.edge;
        for current in walk_radial_ring(topology, start_id)
            .map_err(|err| vf("projected_radial_edge_consistency", err))?
        {
            checked.insert(current.raw());
            if topology.half_edge(current).edge != expected_edge {
                return Err(vf(
                    "projected_radial_edge_consistency",
                    format!(
                        "HE {} claims edge {} but ring seed {} claims edge {}",
                        current.raw(),
                        topology.half_edge(current).edge.raw(),
                        start_id.raw(),
                        expected_edge.raw()
                    ),
                ));
            }
        }
    }
    Ok(())
}

fn validate_radial_cycle_uniqueness(topology: &ProjectedTopology) -> Result<(), KernelError> {
    let mut globally_checked = BTreeSet::new();
    for (start_index, _) in topology.half_edges().iter().enumerate() {
        let start_id = ProjectedHalfEdgeId::new(start_index as u32);
        if globally_checked.contains(&start_id.raw()) {
            continue;
        }
        let ring = walk_radial_ring(topology, start_id)
            .map_err(|err| vf("projected_radial_cycle_uniqueness", err))?;
        let mut ring_seen = BTreeSet::new();
        for current in ring {
            globally_checked.insert(current.raw());
            if !ring_seen.insert(current.raw()) {
                return Err(vf(
                    "projected_radial_cycle_uniqueness",
                    format!(
                        "Radial ring seeded at HE {} contains duplicate HE {}",
                        start_id.raw(),
                        current.raw()
                    ),
                ));
            }
        }
    }
    Ok(())
}

fn validate_radial_neighbor_consistency(topology: &ProjectedTopology) -> Result<(), KernelError> {
    let mut checked = BTreeSet::new();
    for (he_index, half_edge) in topology.half_edges().iter().enumerate() {
        let he_id = ProjectedHalfEdgeId::new(he_index as u32);
        if checked.contains(&he_id.raw()) {
            continue;
        }
        checked.insert(he_id.raw());
        let neighbor_id = half_edge.radial_next;
        if neighbor_id == he_id {
            continue;
        }
        checked.insert(neighbor_id.raw());
        let neighbor = topology.half_edge(neighbor_id);
        if half_edge.origin == neighbor.origin {
            let valence = topology.edge_half_edges(half_edge.edge).len();
            if valence == 2 && half_edge.face != neighbor.face {
                tracing::warn!(
                    "projected_radial_neighbor_consistency: HE {} and HE {} are co-directional at vertex {}",
                    he_id.raw(),
                    neighbor_id.raw(),
                    half_edge.origin.raw()
                );
            }
        }
    }
    Ok(())
}

fn validate_no_broken_radial_splices(topology: &ProjectedTopology) -> Result<(), KernelError> {
    let mut edge_half_edge_counts = BTreeMap::new();
    for half_edge in topology.half_edges() {
        *edge_half_edge_counts.entry(half_edge.edge.raw()).or_insert(0usize) += 1;
    }

    for (edge_index, edge) in topology.edges().iter().enumerate() {
        let edge_id = ProjectedEdgeId::new(edge_index as u32);
        let ring = walk_radial_ring(topology, edge.half_edge)
            .map_err(|err| vf("projected_no_broken_radial_splices", err))?;
        let expected = edge_half_edge_counts
            .get(&edge_id.raw())
            .copied()
            .unwrap_or(0);
        if ring.len() != expected {
            return Err(vf(
                "projected_no_broken_radial_splices",
                format!(
                    "Edge {} has {} halfedges referencing it but radial ring from {} only reaches {}",
                    edge_id.raw(),
                    expected,
                    edge.half_edge.raw(),
                    ring.len()
                ),
            ));
        }
    }
    Ok(())
}

fn walk_radial_ring(
    topology: &ProjectedTopology,
    start: ProjectedHalfEdgeId,
) -> Result<Vec<ProjectedHalfEdgeId>, String> {
    let max_steps = topology.half_edge_count().max(1);
    let mut result = Vec::new();
    let mut seen = BTreeSet::new();
    let mut current = start;

    for _ in 0..max_steps {
        if !seen.insert(current.raw()) {
            return Err(format!(
                "radial ring seeded at {} revisited {} before closing",
                start.raw(),
                current.raw()
            ));
        }
        result.push(current);
        let next = topology.half_edge(current).radial_next;
        if next == start {
            return Ok(result);
        }
        current = next;
    }

    Err(format!(
        "radial ring seeded at {} does not close within {} halfedges",
        start.raw(),
        max_steps
    ))
}
