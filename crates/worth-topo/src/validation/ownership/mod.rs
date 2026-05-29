use forge_relational::facade::identity::EntityId;

use crate::brep::topology_graph::TopologyView;
use crate::derived_topology::materialized_graph::MaterializedTopologyView;
use crate::validation::error::TopologyValidationError;
use crate::validation::shared::{err, face_outer_loop_map, loop_face_map, unique_ids};

pub fn validate(view: &MaterializedTopologyView) -> Result<(), TopologyValidationError> {
    let view = view.topology();
    validate_unique_entity_ids(view)?;
    validate_hierarchy(view)?;
    validate_face_loop_existence(view)?;
    validate_loop_face_membership(view)?;
    validate_half_edge_membership_refs(view)?;
    Ok(())
}

fn validate_unique_entity_ids(view: &TopologyView) -> Result<(), TopologyValidationError> {
    unique_ids(&view.models, |record| record.entity_id)?;
    unique_ids(&view.bodies, |record| record.entity_id)?;
    unique_ids(&view.lumps, |record| record.entity_id)?;
    unique_ids(&view.regions, |record| record.entity_id)?;
    unique_ids(&view.shells, |record| record.entity_id)?;
    unique_ids(&view.faces, |record| record.entity_id)?;
    unique_ids(&view.loops, |record| record.entity_id)?;
    unique_ids(&view.wires, |record| record.entity_id)?;
    unique_ids(&view.half_edges, |record| record.entity_id)?;
    unique_ids(&view.edges, |record| record.entity_id)?;
    unique_ids(&view.vertices, |record| record.entity_id)?;
    Ok(())
}

fn validate_hierarchy(view: &TopologyView) -> Result<(), TopologyValidationError> {
    for body in &view.bodies {
        if body.model_id.is_none() {
            return Err(err(
                "ownership.hierarchy",
                format!("body {:?} has no model parent", body.entity_id),
            ));
        }
    }
    for lump in &view.lumps {
        if lump.body_id.is_none() {
            return Err(err(
                "ownership.hierarchy",
                format!("lump {:?} has no body parent", lump.entity_id),
            ));
        }
    }
    for region in &view.regions {
        if region.lump_id.is_none() {
            return Err(err(
                "ownership.hierarchy",
                format!("region {:?} has no lump parent", region.entity_id),
            ));
        }
    }
    for shell in &view.shells {
        if shell.region_id.is_none() {
            return Err(err(
                "ownership.hierarchy",
                format!("shell {:?} has no region parent", shell.entity_id),
            ));
        }
    }
    for face in &view.faces {
        if face.shell_id.is_none() {
            return Err(err(
                "ownership.hierarchy",
                format!("face {:?} has no shell parent", face.entity_id),
            ));
        }
    }
    Ok(())
}

fn validate_face_loop_existence(view: &TopologyView) -> Result<(), TopologyValidationError> {
    for face in &view.faces {
        if face.outer_loop_id.is_none() {
            return Err(err(
                "ownership.face_loop_existence",
                format!("face {:?} has no outer loop", face.entity_id),
            ));
        }
    }
    Ok(())
}

fn validate_loop_face_membership(view: &TopologyView) -> Result<(), TopologyValidationError> {
    let outer_loop_map = face_outer_loop_map(view);
    let loop_face_map = loop_face_map(view);
    for face in &view.faces {
        let Some(outer_loop_id) = outer_loop_map.get(&face.entity_id).copied() else {
            continue;
        };
        match loop_face_map.get(&outer_loop_id).copied() {
            Some(parent_face_id) if parent_face_id == face.entity_id => {}
            Some(parent_face_id) => {
                return Err(err(
                    "ownership.loop_face_membership",
                    format!(
                        "outer loop {:?} for face {:?} is owned by face {:?}",
                        outer_loop_id, face.entity_id, parent_face_id
                    ),
                ));
            }
            None => {
                return Err(err(
                    "ownership.loop_face_membership",
                    format!(
                        "outer loop {:?} for face {:?} is not owned by any face",
                        outer_loop_id, face.entity_id
                    ),
                ));
            }
        }
    }
    Ok(())
}

fn validate_half_edge_membership_refs(view: &TopologyView) -> Result<(), TopologyValidationError> {
    let loop_ids = unique_ids(&view.loops, |record| record.entity_id)?;
    let wire_ids = unique_ids(&view.wires, |record| record.entity_id)?;
    let face_ids = unique_ids(&view.faces, |record| record.entity_id)?;
    let vertex_ids = unique_ids(&view.vertices, |record| record.entity_id)?;

    for half_edge in &view.half_edges {
        validate_optional_ref(
            "ownership.half_edge_loop_ref",
            half_edge.entity_id,
            half_edge.loop_id,
            &loop_ids,
        )?;
        validate_optional_ref(
            "ownership.half_edge_wire_ref",
            half_edge.entity_id,
            half_edge.wire_id,
            &wire_ids,
        )?;
        validate_optional_ref(
            "ownership.half_edge_face_ref",
            half_edge.entity_id,
            half_edge.face_id,
            &face_ids,
        )?;
        validate_optional_ref(
            "ownership.half_edge_origin_vertex_ref",
            half_edge.entity_id,
            half_edge.origin_vertex_id,
            &vertex_ids,
        )?;
        validate_optional_ref(
            "ownership.half_edge_target_vertex_ref",
            half_edge.entity_id,
            half_edge.target_vertex_id,
            &vertex_ids,
        )?;
    }

    Ok(())
}

fn validate_optional_ref(
    validator: &'static str,
    owner_id: EntityId,
    target_id: Option<EntityId>,
    valid_ids: &std::collections::BTreeSet<EntityId>,
) -> Result<(), TopologyValidationError> {
    if let Some(target_id) = target_id {
        if !valid_ids.contains(&target_id) {
            return Err(err(
                validator,
                format!(
                    "entity {:?} references missing entity {:?}",
                    owner_id, target_id
                ),
            ));
        }
    }
    Ok(())
}




