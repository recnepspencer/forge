use std::collections::BTreeSet;

use forge_core::KernelError;

use crate::projection::data::{ProjectedHalfEdgeId, ProjectedTopology};
use crate::projection::logic::ProjectedTopologyQueries;

use super::shared::vf;

pub fn validate_projected_loop_wiring(topology: &ProjectedTopology) -> Result<(), KernelError> {
    validate_prev_consistency(topology)?;
    validate_loops(topology)?;
    validate_loop_minimum_cardinality(topology)?;
    validate_no_duplicate_coedges_in_loop(topology)?;
    validate_face_loop_membership_complete(topology)?;
    validate_edge_endpoints_match_loop_vertices(topology)?;
    validate_vertex_continuity(topology)?;
    Ok(())
}

fn validate_prev_consistency(topology: &ProjectedTopology) -> Result<(), KernelError> {
    for (index, half_edge) in topology.half_edges().iter().enumerate() {
        let he_id = ProjectedHalfEdgeId::new(index as u32);
        let prev_data = topology.half_edge(half_edge.prev);
        if prev_data.next != he_id {
            return Err(vf(
                "projected_prev_consistency",
                format!(
                    "HE[{}].prev = {} but HE[{}].next = {} (expected {})",
                    he_id.raw(),
                    half_edge.prev.raw(),
                    half_edge.prev.raw(),
                    prev_data.next.raw(),
                    he_id.raw()
                ),
            ));
        }
    }
    Ok(())
}

fn validate_loops(topology: &ProjectedTopology) -> Result<(), KernelError> {
    for (face_index, _) in topology.faces().iter().enumerate() {
        let face_id = crate::projection::data::ProjectedFaceId::new(face_index as u32);
        for half_edge in topology
            .face_half_edges(face_id)
            .map_err(|err| vf("projected_loop_closure", err.to_string()))?
        {
            let half_edge_data = topology.half_edge(half_edge);
            if half_edge_data.face != face_id {
                return Err(vf(
                    "projected_loop_closure",
                    format!(
                        "HE {} is reachable from face {} loops but claims face {}",
                        half_edge.raw(),
                        face_id.raw(),
                        half_edge_data.face.raw()
                    ),
                ));
            }
        }
    }
    Ok(())
}

fn validate_loop_minimum_cardinality(topology: &ProjectedTopology) -> Result<(), KernelError> {
    for (loop_index, _) in topology.loops().iter().enumerate() {
        let loop_id = crate::projection::data::ProjectedLoopId::new(loop_index as u32);
        let half_edges = topology
            .loop_half_edges(loop_id)
            .map_err(|err| vf("projected_loop_minimum_cardinality", err.to_string()))?;
        if half_edges.len() < 2 {
            return Err(vf(
                "projected_loop_minimum_cardinality",
                format!(
                    "Loop {} contains {} halfedge(s); expected at least 2",
                    loop_id.raw(),
                    half_edges.len()
                ),
            ));
        }
    }
    Ok(())
}

fn validate_no_duplicate_coedges_in_loop(topology: &ProjectedTopology) -> Result<(), KernelError> {
    for (loop_index, _) in topology.loops().iter().enumerate() {
        let loop_id = crate::projection::data::ProjectedLoopId::new(loop_index as u32);
        let mut seen = BTreeSet::new();
        for half_edge in topology
            .loop_half_edges(loop_id)
            .map_err(|err| vf("projected_duplicate_coedges", err.to_string()))?
        {
            if !seen.insert(half_edge.raw()) {
                return Err(vf(
                    "projected_duplicate_coedges",
                    format!(
                        "Loop {} contains duplicate halfedge {}",
                        loop_id.raw(),
                        half_edge.raw()
                    ),
                ));
            }
        }
    }
    Ok(())
}

fn validate_face_loop_membership_complete(topology: &ProjectedTopology) -> Result<(), KernelError> {
    for (face_index, _) in topology.faces().iter().enumerate() {
        let face_id = crate::projection::data::ProjectedFaceId::new(face_index as u32);
        let reachable = topology
            .face_half_edges(face_id)
            .map_err(|err| vf("projected_face_loop_membership_complete", err.to_string()))?
            .into_iter()
            .map(|half_edge| half_edge.raw())
            .collect::<BTreeSet<_>>();
        for (he_index, half_edge) in topology.half_edges().iter().enumerate() {
            if half_edge.face == face_id && !reachable.contains(&(he_index as u32)) {
                return Err(vf(
                    "projected_face_loop_membership_complete",
                    format!(
                        "HE {} claims face {} but is unreachable from its loops",
                        he_index,
                        face_id.raw()
                    ),
                ));
            }
        }
    }
    Ok(())
}

fn validate_edge_endpoints_match_loop_vertices(topology: &ProjectedTopology) -> Result<(), KernelError> {
    for (index, half_edge) in topology.half_edges().iter().enumerate() {
        let he_id = ProjectedHalfEdgeId::new(index as u32);
        let dest = topology.half_edge(half_edge.next).origin;
        let twin_id = half_edge.radial_next;
        if twin_id == he_id {
            continue;
        }

        let twin = topology.half_edge(twin_id);
        let twin_dest = topology.half_edge(twin.next).origin;
        let is_opposite = twin.origin == dest && twin_dest == half_edge.origin;
        let is_same = twin.origin == half_edge.origin && twin_dest == dest;

        if !is_opposite && !is_same {
            return Err(vf(
                "projected_edge_endpoints_match",
                format!(
                    "HE {} ({}->{}) and radial neighbor {} ({}->{}) do not span the same vertices",
                    he_id.raw(),
                    half_edge.origin.raw(),
                    dest.raw(),
                    twin_id.raw(),
                    twin.origin.raw(),
                    twin_dest.raw()
                ),
            ));
        }
    }
    Ok(())
}

fn validate_vertex_continuity(topology: &ProjectedTopology) -> Result<(), KernelError> {
    for (edge_index, edge) in topology.edges().iter().enumerate() {
        let edge_id = crate::projection::data::ProjectedEdgeId::new(edge_index as u32);
        let mut endpoints = BTreeSet::new();
        for half_edge_id in topology.radial_half_edges(edge.half_edge) {
            let half_edge = topology.half_edge(half_edge_id);
            let next = topology.half_edge(half_edge.next);
            endpoints.insert(half_edge.origin.raw());
            endpoints.insert(next.origin.raw());
        }
        if endpoints.len() > 2 {
            return Err(vf(
                "projected_vertex_continuity",
                format!(
                    "Edge {} has {} distinct endpoint vertices; expected 1 or 2",
                    edge_id.raw(),
                    endpoints.len()
                ),
            ));
        }
    }
    Ok(())
}
