#[cfg(test)]
use forge_relational::facade::identity::EntityId;

#[cfg(test)]
use super::source::MaterializedGraphReadRelationRow;
#[cfg(test)]
use crate::brep::topology_graph::TopologyView;

#[cfg(test)]
pub(super) fn relation_rows_from_topology(
    topology: &TopologyView,
) -> Vec<MaterializedGraphReadRelationRow> {
    let mut rows = Vec::new();
    for model in &topology.models {
        push_many(&mut rows, "model.body", model.entity_id, &model.body_ids);
    }
    for body in &topology.bodies {
        push_optional(&mut rows, "body.model", body.entity_id, body.model_id);
        push_many(&mut rows, "body.lump", body.entity_id, &body.lump_ids);
    }
    for lump in &topology.lumps {
        push_optional(&mut rows, "lump.body", lump.entity_id, lump.body_id);
        push_many(&mut rows, "lump.region", lump.entity_id, &lump.region_ids);
    }
    for region in &topology.regions {
        push_optional(&mut rows, "region.lump", region.entity_id, region.lump_id);
        push_many(
            &mut rows,
            "region.shell",
            region.entity_id,
            &region.shell_ids,
        );
    }
    for shell in &topology.shells {
        push_optional(&mut rows, "shell.region", shell.entity_id, shell.region_id);
        push_many(&mut rows, "shell.face", shell.entity_id, &shell.face_ids);
    }
    for face in &topology.faces {
        push_optional(&mut rows, "face.shell", face.entity_id, face.shell_id);
        push_optional(
            &mut rows,
            "face.outer_loop",
            face.entity_id,
            face.outer_loop_id,
        );
        push_many(
            &mut rows,
            "face.inner_loop",
            face.entity_id,
            &face.inner_loop_ids,
        );
        push_many(
            &mut rows,
            "face.boundary_half_edge",
            face.entity_id,
            &face.boundary_half_edge_ids,
        );
    }
    for loop_row in &topology.loops {
        push_many(
            &mut rows,
            "loop.face",
            loop_row.entity_id,
            &loop_row.face_ids,
        );
        push_many(
            &mut rows,
            "loop.half_edge",
            loop_row.entity_id,
            &loop_row.half_edge_ids,
        );
    }
    for wire in &topology.wires {
        push_many(
            &mut rows,
            "wire.half_edge",
            wire.entity_id,
            &wire.half_edge_ids,
        );
    }
    for half_edge in &topology.half_edges {
        push_optional(
            &mut rows,
            "half_edge.loop",
            half_edge.entity_id,
            half_edge.loop_id,
        );
        push_optional(
            &mut rows,
            "half_edge.wire",
            half_edge.entity_id,
            half_edge.wire_id,
        );
        push_optional(
            &mut rows,
            "half_edge.next",
            half_edge.entity_id,
            half_edge.next_half_edge_id,
        );
        push_optional(
            &mut rows,
            "half_edge.prev",
            half_edge.entity_id,
            half_edge.prev_half_edge_id,
        );
        push_optional(
            &mut rows,
            "half_edge.radial_next",
            half_edge.entity_id,
            half_edge.radial_next_half_edge_id,
        );
        push_optional(
            &mut rows,
            "half_edge.edge",
            half_edge.entity_id,
            half_edge.edge_id,
        );
        push_optional(
            &mut rows,
            "half_edge.origin_vertex",
            half_edge.entity_id,
            half_edge.origin_vertex_id,
        );
        push_optional(
            &mut rows,
            "half_edge.target_vertex",
            half_edge.entity_id,
            half_edge.target_vertex_id,
        );
        push_optional(
            &mut rows,
            "half_edge.face",
            half_edge.entity_id,
            half_edge.face_id,
        );
    }
    rows
}

#[cfg(test)]
fn push_optional(
    rows: &mut Vec<MaterializedGraphReadRelationRow>,
    kind: &'static str,
    source: EntityId,
    target: Option<EntityId>,
) {
    if let Some(target) = target {
        rows.push(MaterializedGraphReadRelationRow::new(kind, source, target));
    }
}

#[cfg(test)]
fn push_many(
    rows: &mut Vec<MaterializedGraphReadRelationRow>,
    kind: &'static str,
    source: EntityId,
    targets: &[EntityId],
) {
    rows.extend(
        targets
            .iter()
            .copied()
            .map(|target| MaterializedGraphReadRelationRow::new(kind, source, target)),
    );
}
