use std::collections::{BTreeMap, BTreeSet};

use forge_relational::facade::identity::EntityId;

use crate::data::topology_view::WorthTopologyView;
use crate::validators::error::WorthTopologyValidationError;
use crate::validators::shared::err;

pub fn validate(view: &WorthTopologyView) -> Result<(), WorthTopologyValidationError> {
    validate_shell_faces(view)?;
    validate_face_shell_membership(view)?;
    validate_face_boundaries(view)?;
    validate_shell_closure_mode(view)?;
    Ok(())
}

fn validate_shell_faces(view: &WorthTopologyView) -> Result<(), WorthTopologyValidationError> {
    for shell in &view.shells {
        if shell.face_ids.is_empty() {
            return Err(err(
                "shell_closure.shell_faces",
                format!("shell {:?} contains no faces", shell.entity_id),
            ));
        }
    }
    Ok(())
}

fn validate_face_shell_membership(
    view: &WorthTopologyView,
) -> Result<(), WorthTopologyValidationError> {
    let shell_face_sets: BTreeMap<EntityId, std::collections::BTreeSet<EntityId>> = view
        .shells
        .iter()
        .map(|shell| (shell.entity_id, shell.face_ids.iter().copied().collect()))
        .collect();

    for face in &view.faces {
        let Some(shell_id) = face.shell_id else {
            return Err(err(
                "shell_closure.face_shell_membership",
                format!("face {:?} has no shell", face.entity_id),
            ));
        };
        let Some(face_set) = shell_face_sets.get(&shell_id) else {
            return Err(err(
                "shell_closure.face_shell_membership",
                format!("face {:?} references missing shell {:?}", face.entity_id, shell_id),
            ));
        };
        if !face_set.contains(&face.entity_id) {
            return Err(err(
                "shell_closure.face_shell_membership",
                format!("shell {:?} does not list face {:?}", shell_id, face.entity_id),
            ));
        }
    }
    Ok(())
}

fn validate_face_boundaries(view: &WorthTopologyView) -> Result<(), WorthTopologyValidationError> {
    for face in &view.faces {
        if face.boundary_half_edge_ids.is_empty() {
            return Err(err(
                "shell_closure.face_boundaries",
                format!("face {:?} has no boundary half-edges", face.entity_id),
            ));
        }
    }
    Ok(())
}

fn validate_shell_closure_mode(view: &WorthTopologyView) -> Result<(), WorthTopologyValidationError> {
    let half_edge_map: BTreeMap<EntityId, _> = view
        .half_edges
        .iter()
        .map(|record| (record.entity_id, record))
        .collect();
    let face_map: BTreeMap<EntityId, _> = view.faces.iter().map(|record| (record.entity_id, record)).collect();

    for shell in &view.shells {
        let shell_face_ids: BTreeSet<EntityId> = shell.face_ids.iter().copied().collect();
        let shell_half_edges = shell_boundary_half_edges(view, &shell_face_ids)?;

        let mut boundary_half_edges = Vec::new();

        for half_edge_id in &shell_half_edges {
            let half_edge = half_edge_map.get(half_edge_id).copied().ok_or_else(|| {
                err(
                    "shell_closure.closure_mode",
                    format!("shell {:?} references missing half-edge {:?}", shell.entity_id, half_edge_id),
                )
            })?;
            let radial_id = half_edge.radial_next_half_edge_id.ok_or_else(|| {
                err(
                    "shell_closure.closure_mode",
                    format!("half-edge {:?} has no radial next", half_edge.entity_id),
                )
            })?;

            if radial_id == half_edge.entity_id {
                boundary_half_edges.push(half_edge.entity_id);
                continue;
            }

            let radial = half_edge_map.get(&radial_id).copied().ok_or_else(|| {
                err(
                    "shell_closure.closure_mode",
                    format!("half-edge {:?} references missing radial neighbor {:?}", half_edge.entity_id, radial_id),
                )
            })?;
            if radial.edge_id != half_edge.edge_id {
                return Err(err(
                    "shell_closure.closure_mode",
                    format!(
                        "half-edge {:?} and radial neighbor {:?} disagree on edge ownership",
                        half_edge.entity_id, radial.entity_id
                    ),
                ));
            }
            let radial_face_id = radial.face_id.ok_or_else(|| {
                err(
                    "shell_closure.closure_mode",
                    format!("radial neighbor {:?} has no face", radial.entity_id),
                )
            })?;
            if !face_map.contains_key(&radial_face_id) || !shell_face_ids.contains(&radial_face_id) {
                return Err(err(
                    "shell_closure.closure_mode",
                    format!(
                        "half-edge {:?} in shell {:?} points radially to face {:?} outside the shell",
                        half_edge.entity_id, shell.entity_id, radial_face_id
                    ),
                ));
            }
        }

        if boundary_half_edges.is_empty() {
            validate_closed_shell_manifold_edges(shell.entity_id, &shell_half_edges, &half_edge_map)?;
        }
    }

    Ok(())
}

fn shell_boundary_half_edges(
    view: &WorthTopologyView,
    shell_face_ids: &BTreeSet<EntityId>,
) -> Result<BTreeSet<EntityId>, WorthTopologyValidationError> {
    let mut half_edge_ids = BTreeSet::new();
    for face in &view.faces {
        if shell_face_ids.contains(&face.entity_id) {
            for half_edge_id in &face.boundary_half_edge_ids {
                half_edge_ids.insert(*half_edge_id);
            }
        }
    }
    if half_edge_ids.is_empty() {
        return Err(err(
            "shell_closure.closure_mode",
            "shell has no boundary half-edges",
        ));
    }
    Ok(half_edge_ids)
}

fn validate_closed_shell_manifold_edges(
    shell_id: EntityId,
    shell_half_edges: &BTreeSet<EntityId>,
    half_edge_map: &BTreeMap<EntityId, &crate::data::topology_view::WorthTopologyHalfEdge>,
) -> Result<(), WorthTopologyValidationError> {
    let mut validated_edges = BTreeSet::new();

    for half_edge_id in shell_half_edges {
        let half_edge = half_edge_map.get(half_edge_id).copied().ok_or_else(|| {
            err(
                "shell_closure.closed_shell_manifold",
                format!("shell {:?} references missing half-edge {:?}", shell_id, half_edge_id),
            )
        })?;
        let edge_id = half_edge.edge_id.ok_or_else(|| {
            err(
                "shell_closure.closed_shell_manifold",
                format!("half-edge {:?} has no edge", half_edge.entity_id),
            )
        })?;
        if !validated_edges.insert(edge_id) {
            continue;
        }

        let ring = walk_radial_ring(half_edge.entity_id, half_edge_map)?;
        if ring.len() != 2 {
            return Err(err(
                "shell_closure.closed_shell_manifold",
                format!(
                    "shell {:?} edge {:?} has radial valence {} (expected 2 for a closed solid shell)",
                    shell_id,
                    edge_id,
                    ring.len()
                ),
            ));
        }
        if !ring.iter().all(|radial_id| shell_half_edges.contains(radial_id)) {
            return Err(err(
                "shell_closure.closed_shell_manifold",
                format!("shell {:?} edge {:?} has radial uses outside the shell", shell_id, edge_id),
            ));
        }

        let mut face_ids = BTreeSet::new();
        for radial_id in ring {
            let radial = half_edge_map.get(&radial_id).copied().ok_or_else(|| {
                err(
                    "shell_closure.closed_shell_manifold",
                    format!("missing radial half-edge {:?}", radial_id),
                )
            })?;
            let face_id = radial.face_id.ok_or_else(|| {
                err(
                    "shell_closure.closed_shell_manifold",
                    format!("half-edge {:?} has no face", radial.entity_id),
                )
            })?;
            face_ids.insert(face_id);
        }
        if face_ids.len() != 2 {
            return Err(err(
                "shell_closure.closed_shell_manifold",
                format!(
                    "shell {:?} edge {:?} does not separate two distinct faces",
                    shell_id, edge_id
                ),
            ));
        }
    }

    Ok(())
}

fn walk_radial_ring(
    start_id: EntityId,
    half_edge_map: &BTreeMap<EntityId, &crate::data::topology_view::WorthTopologyHalfEdge>,
) -> Result<Vec<EntityId>, WorthTopologyValidationError> {
    let mut ring = Vec::new();
    let mut seen = BTreeSet::new();
    let mut current_id = start_id;

    loop {
        if !seen.insert(current_id) {
            if current_id == start_id {
                break;
            }
            return Err(err(
                "shell_closure.closed_shell_manifold",
                format!("radial ring seeded at {:?} revisits {:?} before closing", start_id, current_id),
            ));
        }
        ring.push(current_id);

        let current = half_edge_map.get(&current_id).copied().ok_or_else(|| {
            err(
                "shell_closure.closed_shell_manifold",
                format!("missing half-edge {:?}", current_id),
            )
        })?;
        let next_id = current.radial_next_half_edge_id.ok_or_else(|| {
            err(
                "shell_closure.closed_shell_manifold",
                format!("half-edge {:?} has no radial next", current.entity_id),
            )
        })?;
        current_id = next_id;
    }

    Ok(ring)
}
