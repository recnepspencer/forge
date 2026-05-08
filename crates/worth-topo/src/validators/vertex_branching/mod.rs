use std::collections::{BTreeMap, BTreeSet, VecDeque};

use forge_relational::facade::identity::EntityId;
use schema::facade::WireInterpretationClass;

use crate::data::topology_view::TopologyHalfEdge;
use crate::interpretation::InterpretedTopologyView;
use crate::validators::error::TopologyValidationError;
use crate::validators::shared::err;

pub fn validate(view: &InterpretedTopologyView) -> Result<(), TopologyValidationError> {
    let topology = view.materialized().topology();
    validate_wire_interpretation_summary(view)?;
    validate_wire_half_edge_membership(topology)?;
    validate_vertex_presence_for_branching(topology)?;
    validate_wire_connectivity(topology)?;
    validate_branch_vertices_use_distinct_edges(topology, view)?;
    Ok(())
}

fn validate_wire_interpretation_summary(
    view: &InterpretedTopologyView,
) -> Result<(), TopologyValidationError> {
    for wire in &view.interpretations().wires {
        match wire.class {
            WireInterpretationClass::ConnectedBranch if wire.branch_vertex_ids.is_empty() => {
                return Err(err(
                    "vertex_branching.interpretation_summary",
                    format!(
                        "wire {:?} is classified as a branch without branch vertices",
                        wire.wire_id
                    ),
                ));
            }
            WireInterpretationClass::ClosedCycle if !wire.terminal_vertex_ids.is_empty() => {
                return Err(err(
                    "vertex_branching.interpretation_summary",
                    format!(
                        "wire {:?} is classified as closed with terminal vertices present",
                        wire.wire_id
                    ),
                ));
            }
            _ => {}
        }
    }
    Ok(())
}

fn validate_wire_half_edge_membership(
    view: &crate::data::topology_view::TopologyView,
) -> Result<(), TopologyValidationError> {
    for wire in &view.wires {
        if wire.half_edge_ids.is_empty() {
            return Err(err(
                "vertex_branching.wire_membership",
                format!("wire {:?} has no half-edges", wire.entity_id),
            ));
        }
        for half_edge_id in &wire.half_edge_ids {
            let Some(half_edge) = view
                .half_edges
                .iter()
                .find(|record| record.entity_id == *half_edge_id)
            else {
                return Err(err(
                    "vertex_branching.wire_membership",
                    format!(
                        "wire {:?} references missing half-edge {:?}",
                        wire.entity_id, half_edge_id
                    ),
                ));
            };
            if half_edge.wire_id != Some(wire.entity_id) {
                return Err(err(
                    "vertex_branching.wire_membership",
                    format!(
                        "half-edge {:?} is listed in wire {:?} but records wire {:?}",
                        half_edge.entity_id, wire.entity_id, half_edge.wire_id
                    ),
                ));
            }
        }
    }
    Ok(())
}

fn validate_vertex_presence_for_branching(
    view: &crate::data::topology_view::TopologyView,
) -> Result<(), TopologyValidationError> {
    let vertex_ids: BTreeSet<EntityId> = view
        .vertices
        .iter()
        .map(|record| record.entity_id)
        .collect();
    for half_edge in &view.half_edges {
        let Some(vertex_id) = half_edge.origin_vertex_id else {
            return Err(err(
                "vertex_branching.vertex_presence",
                format!("half-edge {:?} has no start vertex", half_edge.entity_id),
            ));
        };
        if !vertex_ids.contains(&vertex_id) {
            return Err(err(
                "vertex_branching.vertex_presence",
                format!(
                    "half-edge {:?} references missing vertex {:?}",
                    half_edge.entity_id, vertex_id
                ),
            ));
        }
        let Some(target_vertex_id) = half_edge.target_vertex_id else {
            return Err(err(
                "vertex_branching.vertex_presence",
                format!("half-edge {:?} has no end vertex", half_edge.entity_id),
            ));
        };
        if !vertex_ids.contains(&target_vertex_id) {
            return Err(err(
                "vertex_branching.vertex_presence",
                format!(
                    "half-edge {:?} references missing target vertex {:?}",
                    half_edge.entity_id, target_vertex_id
                ),
            ));
        }
    }
    Ok(())
}

fn validate_wire_connectivity(
    view: &crate::data::topology_view::TopologyView,
) -> Result<(), TopologyValidationError> {
    let half_edge_map: BTreeMap<EntityId, &TopologyHalfEdge> = view
        .half_edges
        .iter()
        .map(|record| (record.entity_id, record))
        .collect();

    for wire in &view.wires {
        if wire.half_edge_ids.len() <= 1 {
            continue;
        }

        let mut queue = VecDeque::new();
        let mut visited = BTreeSet::new();
        let seed_id = wire.half_edge_ids[0];
        queue.push_back(seed_id);
        visited.insert(seed_id);

        while let Some(current_id) = queue.pop_front() {
            let current = half_edge_map.get(&current_id).copied().ok_or_else(|| {
                err(
                    "vertex_branching.wire_connectivity",
                    format!(
                        "wire {:?} references missing half-edge {:?}",
                        wire.entity_id, current_id
                    ),
                )
            })?;
            let current_vertices = incident_vertices(current, &half_edge_map)?;

            for neighbor_id in &wire.half_edge_ids {
                if visited.contains(neighbor_id) {
                    continue;
                }
                let neighbor = half_edge_map.get(neighbor_id).copied().ok_or_else(|| {
                    err(
                        "vertex_branching.wire_connectivity",
                        format!(
                            "wire {:?} references missing half-edge {:?}",
                            wire.entity_id, neighbor_id
                        ),
                    )
                })?;
                let neighbor_vertices = incident_vertices(neighbor, &half_edge_map)?;
                if current_vertices
                    .iter()
                    .any(|vertex_id| neighbor_vertices.contains(vertex_id))
                {
                    visited.insert(*neighbor_id);
                    queue.push_back(*neighbor_id);
                }
            }
        }

        if visited.len() != wire.half_edge_ids.len() {
            return Err(err(
                "vertex_branching.wire_connectivity",
                format!(
                    "wire {:?} is disconnected: reached {} of {} half-edges",
                    wire.entity_id,
                    visited.len(),
                    wire.half_edge_ids.len()
                ),
            ));
        }
    }

    Ok(())
}

fn validate_branch_vertices_use_distinct_edges(
    view: &crate::data::topology_view::TopologyView,
    interpreted: &InterpretedTopologyView,
) -> Result<(), TopologyValidationError> {
    let half_edge_map: BTreeMap<EntityId, &TopologyHalfEdge> = view
        .half_edges
        .iter()
        .map(|record| (record.entity_id, record))
        .collect();

    for wire in &view.wires {
        let interpreted_wire = interpreted
            .interpretations()
            .wires
            .iter()
            .find(|record| record.wire_id == wire.entity_id)
            .ok_or_else(|| {
                err(
                    "vertex_branching.interpretation_summary",
                    format!("wire {:?} has no interpreted wire summary", wire.entity_id),
                )
            })?;
        let mut vertex_incident_half_edges: BTreeMap<EntityId, BTreeSet<EntityId>> =
            BTreeMap::new();
        let mut vertex_incident_edges: BTreeMap<EntityId, BTreeSet<EntityId>> = BTreeMap::new();

        for half_edge_id in &wire.half_edge_ids {
            let half_edge = half_edge_map.get(half_edge_id).copied().ok_or_else(|| {
                err(
                    "vertex_branching.branch_valence",
                    format!(
                        "wire {:?} references missing half-edge {:?}",
                        wire.entity_id, half_edge_id
                    ),
                )
            })?;
            let edge_id = half_edge.edge_id.ok_or_else(|| {
                err(
                    "vertex_branching.branch_valence",
                    format!("half-edge {:?} has no edge", half_edge.entity_id),
                )
            })?;

            for vertex_id in incident_vertices(half_edge, &half_edge_map)? {
                vertex_incident_half_edges
                    .entry(vertex_id)
                    .or_default()
                    .insert(half_edge.entity_id);
                vertex_incident_edges
                    .entry(vertex_id)
                    .or_default()
                    .insert(edge_id);
            }
        }

        for (vertex_id, incident_half_edges) in vertex_incident_half_edges {
            if incident_half_edges.len() < 3 {
                continue;
            }
            let distinct_edges = vertex_incident_edges
                .get(&vertex_id)
                .map(BTreeSet::len)
                .unwrap_or_default();
            if distinct_edges < 3 {
                return Err(err(
                    "vertex_branching.branch_valence",
                    format!(
                        "branch vertex {:?} in wire {:?} has {} incident half-edges but only {} distinct edges",
                        vertex_id,
                        wire.entity_id,
                        incident_half_edges.len(),
                        distinct_edges
                    ),
                ));
            }
        }
        if matches!(
            interpreted_wire.class,
            WireInterpretationClass::ConnectedBranch
        ) && interpreted_wire.branch_vertex_ids.is_empty()
        {
            return Err(err(
                "vertex_branching.interpretation_summary",
                format!(
                    "wire {:?} is classified as connected branch without interpreted branch vertices",
                    wire.entity_id
                ),
            ));
        }
    }

    Ok(())
}

fn incident_vertices(
    half_edge: &TopologyHalfEdge,
    half_edge_map: &BTreeMap<EntityId, &TopologyHalfEdge>,
) -> Result<BTreeSet<EntityId>, TopologyValidationError> {
    let mut vertices = BTreeSet::new();
    let origin = half_edge.origin_vertex_id.ok_or_else(|| {
        err(
            "vertex_branching.incident_vertices",
            format!("half-edge {:?} has no start vertex", half_edge.entity_id),
        )
    })?;
    vertices.insert(origin);

    if let Some(target) = half_edge.target_vertex_id {
        vertices.insert(target);
        return Ok(vertices);
    }

    let next_id = half_edge.next_half_edge_id.ok_or_else(|| {
        err(
            "vertex_branching.incident_vertices",
            format!("half-edge {:?} has no next link", half_edge.entity_id),
        )
    })?;
    let next = half_edge_map.get(&next_id).copied().ok_or_else(|| {
        err(
            "vertex_branching.incident_vertices",
            format!(
                "half-edge {:?} references missing next {:?}",
                half_edge.entity_id, next_id
            ),
        )
    })?;
    let target = next.origin_vertex_id.ok_or_else(|| {
        err(
            "vertex_branching.incident_vertices",
            format!("next half-edge {:?} has no start vertex", next.entity_id),
        )
    })?;
    vertices.insert(target);

    Ok(vertices)
}
