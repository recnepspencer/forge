use std::collections::BTreeMap;

use forge_relational::facade::identity::EntityId;
use worth_schema::facade::{
    WorthEntityKind, WorthGeometryRelationKind, WorthRelationKind, WorthTopologyEntityKind,
    WorthTopologyRelationKind,
};

use crate::data::topology_view::WorthTopologyView;
use crate::materialization::errors::WorthTopologyMaterializationError;
use crate::materialization::input_rows::MaterializationRelationRow;
use crate::materialization::traits::HasEntityId;

pub fn apply_relation(
    view: &mut WorthTopologyView,
    entity_kind_map: &BTreeMap<EntityId, WorthEntityKind>,
    relation: &MaterializationRelationRow,
) -> Result<(), WorthTopologyMaterializationError> {
    let kind = relation.kind;
    let Some(source_kind) = entity_kind_map.get(&relation.source).copied() else {
        return Err(WorthTopologyMaterializationError::new(format!(
            "worth relation `{}` references missing source entity {:?}",
            kind.kind_name(),
            relation.source
        )));
    };
    let Some(target_kind) = entity_kind_map.get(&relation.target).copied() else {
        return Err(WorthTopologyMaterializationError::new(format!(
            "worth relation `{}` references missing target entity {:?}",
            kind.kind_name(),
            relation.target
        )));
    };

    match kind {
        WorthRelationKind::Topology(WorthTopologyRelationKind::ModelOwnsBody) => {
            ensure_relation_types(
                kind,
                source_kind,
                WorthEntityKind::Topology(WorthTopologyEntityKind::Model),
                target_kind,
                WorthEntityKind::Topology(WorthTopologyEntityKind::Body),
            )?;
            push_child_to_parent(
                &mut view.models,
                relation.source,
                relation.target,
                |model| &mut model.body_ids,
            )?;
            set_optional_parent(&mut view.bodies, relation.target, relation.source, |body| {
                &mut body.model_id
            })?;
        }
        WorthRelationKind::Topology(WorthTopologyRelationKind::BodyOwnsLump) => {
            ensure_relation_types(
                kind,
                source_kind,
                WorthEntityKind::Topology(WorthTopologyEntityKind::Body),
                target_kind,
                WorthEntityKind::Topology(WorthTopologyEntityKind::Lump),
            )?;
            push_child_to_parent(&mut view.bodies, relation.source, relation.target, |body| {
                &mut body.lump_ids
            })?;
            set_optional_parent(&mut view.lumps, relation.target, relation.source, |lump| {
                &mut lump.body_id
            })?;
        }
        WorthRelationKind::Topology(WorthTopologyRelationKind::LumpOwnsRegion) => {
            ensure_relation_types(
                kind,
                source_kind,
                WorthEntityKind::Topology(WorthTopologyEntityKind::Lump),
                target_kind,
                WorthEntityKind::Topology(WorthTopologyEntityKind::Region),
            )?;
            push_child_to_parent(&mut view.lumps, relation.source, relation.target, |lump| {
                &mut lump.region_ids
            })?;
            set_optional_parent(
                &mut view.regions,
                relation.target,
                relation.source,
                |region| &mut region.lump_id,
            )?;
        }
        WorthRelationKind::Topology(WorthTopologyRelationKind::RegionOwnsShell) => {
            ensure_relation_types(
                kind,
                source_kind,
                WorthEntityKind::Topology(WorthTopologyEntityKind::Region),
                target_kind,
                WorthEntityKind::Topology(WorthTopologyEntityKind::Shell),
            )?;
            push_child_to_parent(
                &mut view.regions,
                relation.source,
                relation.target,
                |region| &mut region.shell_ids,
            )?;
            set_optional_parent(
                &mut view.shells,
                relation.target,
                relation.source,
                |shell| &mut shell.region_id,
            )?;
        }
        WorthRelationKind::Topology(WorthTopologyRelationKind::ShellOwnsFace) => {
            ensure_relation_types(
                kind,
                source_kind,
                WorthEntityKind::Topology(WorthTopologyEntityKind::Shell),
                target_kind,
                WorthEntityKind::Topology(WorthTopologyEntityKind::Face),
            )?;
            push_child_to_parent(
                &mut view.shells,
                relation.source,
                relation.target,
                |shell| &mut shell.face_ids,
            )?;
            set_optional_parent(&mut view.faces, relation.target, relation.source, |face| {
                &mut face.shell_id
            })?;
        }
        WorthRelationKind::Topology(WorthTopologyRelationKind::FaceOuterLoop) => {
            ensure_relation_types(
                kind,
                source_kind,
                WorthEntityKind::Topology(WorthTopologyEntityKind::Face),
                target_kind,
                WorthEntityKind::Topology(WorthTopologyEntityKind::Loop),
            )?;
            set_optional_parent(&mut view.faces, relation.source, relation.target, |face| {
                &mut face.outer_loop_id
            })?;
            push_child_to_parent(
                &mut view.loops,
                relation.target,
                relation.source,
                |loop_record| &mut loop_record.face_ids,
            )?;
        }
        WorthRelationKind::Topology(WorthTopologyRelationKind::FaceInnerLoop) => {
            ensure_relation_types(
                kind,
                source_kind,
                WorthEntityKind::Topology(WorthTopologyEntityKind::Face),
                target_kind,
                WorthEntityKind::Topology(WorthTopologyEntityKind::Loop),
            )?;
            push_child_to_parent(&mut view.faces, relation.source, relation.target, |face| {
                &mut face.inner_loop_ids
            })?;
            push_child_to_parent(
                &mut view.loops,
                relation.target,
                relation.source,
                |loop_record| &mut loop_record.face_ids,
            )?;
        }
        WorthRelationKind::Topology(WorthTopologyRelationKind::LoopOwnsHalfEdge) => {
            ensure_relation_types(
                kind,
                source_kind,
                WorthEntityKind::Topology(WorthTopologyEntityKind::Loop),
                target_kind,
                WorthEntityKind::Topology(WorthTopologyEntityKind::HalfEdge),
            )?;
            push_child_to_parent(
                &mut view.loops,
                relation.source,
                relation.target,
                |loop_record| &mut loop_record.half_edge_ids,
            )?;
            set_optional_parent(
                &mut view.half_edges,
                relation.target,
                relation.source,
                |half_edge| &mut half_edge.loop_id,
            )?;
        }
        WorthRelationKind::Topology(WorthTopologyRelationKind::WireOwnsHalfEdge) => {
            ensure_relation_types(
                kind,
                source_kind,
                WorthEntityKind::Topology(WorthTopologyEntityKind::Wire),
                target_kind,
                WorthEntityKind::Topology(WorthTopologyEntityKind::HalfEdge),
            )?;
            push_child_to_parent(&mut view.wires, relation.source, relation.target, |wire| {
                &mut wire.half_edge_ids
            })?;
            set_optional_parent(
                &mut view.half_edges,
                relation.target,
                relation.source,
                |half_edge| &mut half_edge.wire_id,
            )?;
        }
        WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeNext) => {
            ensure_relation_types(
                kind,
                source_kind,
                WorthEntityKind::Topology(WorthTopologyEntityKind::HalfEdge),
                target_kind,
                WorthEntityKind::Topology(WorthTopologyEntityKind::HalfEdge),
            )?;
            set_optional_parent(
                &mut view.half_edges,
                relation.source,
                relation.target,
                |half_edge| &mut half_edge.next_half_edge_id,
            )?;
        }
        WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgePrev) => {
            ensure_relation_types(
                kind,
                source_kind,
                WorthEntityKind::Topology(WorthTopologyEntityKind::HalfEdge),
                target_kind,
                WorthEntityKind::Topology(WorthTopologyEntityKind::HalfEdge),
            )?;
            set_optional_parent(
                &mut view.half_edges,
                relation.source,
                relation.target,
                |half_edge| &mut half_edge.prev_half_edge_id,
            )?;
        }
        WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeRadialNext) => {
            ensure_relation_types(
                kind,
                source_kind,
                WorthEntityKind::Topology(WorthTopologyEntityKind::HalfEdge),
                target_kind,
                WorthEntityKind::Topology(WorthTopologyEntityKind::HalfEdge),
            )?;
            set_optional_parent(
                &mut view.half_edges,
                relation.source,
                relation.target,
                |half_edge| &mut half_edge.radial_next_half_edge_id,
            )?;
        }
        WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeUsesEdge) => {
            ensure_relation_types(
                kind,
                source_kind,
                WorthEntityKind::Topology(WorthTopologyEntityKind::HalfEdge),
                target_kind,
                WorthEntityKind::Topology(WorthTopologyEntityKind::Edge),
            )?;
            set_optional_parent(
                &mut view.half_edges,
                relation.source,
                relation.target,
                |half_edge| &mut half_edge.edge_id,
            )?;
        }
        WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeStartsAtVertex) => {
            ensure_relation_types(
                kind,
                source_kind,
                WorthEntityKind::Topology(WorthTopologyEntityKind::HalfEdge),
                target_kind,
                WorthEntityKind::Topology(WorthTopologyEntityKind::Vertex),
            )?;
            set_optional_parent(
                &mut view.half_edges,
                relation.source,
                relation.target,
                |half_edge| &mut half_edge.origin_vertex_id,
            )?;
        }
        WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeEndsAtVertex) => {
            ensure_relation_types(
                kind,
                source_kind,
                WorthEntityKind::Topology(WorthTopologyEntityKind::HalfEdge),
                target_kind,
                WorthEntityKind::Topology(WorthTopologyEntityKind::Vertex),
            )?;
            set_optional_parent(
                &mut view.half_edges,
                relation.source,
                relation.target,
                |half_edge| &mut half_edge.target_vertex_id,
            )?;
        }
        WorthRelationKind::Geometry(
            WorthGeometryRelationKind::FaceUsesSurfaceBinding
            | WorthGeometryRelationKind::EdgeUsesCurveBinding
            | WorthGeometryRelationKind::HalfEdgeUsesCoedgeBinding
            | WorthGeometryRelationKind::VertexUsesGeometryBinding,
        ) => {}
        WorthRelationKind::Naming(_) | WorthRelationKind::Diagnostics(_) => {}
    }

    Ok(())
}

pub fn finalize_topology_membership(
    view: &mut WorthTopologyView,
) -> Result<(), WorthTopologyMaterializationError> {
    let loop_memberships: Vec<(EntityId, Vec<EntityId>, Vec<EntityId>)> = view
        .loops
        .iter()
        .map(|loop_record| {
            (
                loop_record.entity_id,
                loop_record.face_ids.clone(),
                loop_record.half_edge_ids.clone(),
            )
        })
        .collect();

    for (loop_id, face_ids, half_edge_ids) in loop_memberships {
        for half_edge_id in half_edge_ids {
            set_optional_parent(&mut view.half_edges, half_edge_id, loop_id, |half_edge| {
                &mut half_edge.loop_id
            })?;
            for face_id in &face_ids {
                set_optional_parent(&mut view.half_edges, half_edge_id, *face_id, |half_edge| {
                    &mut half_edge.face_id
                })?;
                push_child_to_parent(&mut view.faces, *face_id, half_edge_id, |face| {
                    &mut face.boundary_half_edge_ids
                })?;
            }
        }
    }

    Ok(())
}

fn ensure_relation_types(
    relation_kind: WorthRelationKind,
    actual_source: WorthEntityKind,
    expected_source: WorthEntityKind,
    actual_target: WorthEntityKind,
    expected_target: WorthEntityKind,
) -> Result<(), WorthTopologyMaterializationError> {
    if actual_source != expected_source || actual_target != expected_target {
        return Err(WorthTopologyMaterializationError::new(format!(
            "worth relation `{}` expected {} -> {} but saw {} -> {}",
            relation_kind.kind_name(),
            expected_source.kind_name(),
            expected_target.kind_name(),
            actual_source.kind_name(),
            actual_target.kind_name(),
        )));
    }

    Ok(())
}

fn push_child_to_parent<T, F>(
    records: &mut [T],
    entity_id: EntityId,
    child_id: EntityId,
    children: F,
) -> Result<(), WorthTopologyMaterializationError>
where
    F: Fn(&mut T) -> &mut Vec<EntityId>,
    T: HasEntityId,
{
    let record = find_record_mut(records, entity_id)?;
    let targets = children(record);
    if !targets.contains(&child_id) {
        targets.push(child_id);
    }
    Ok(())
}

fn set_optional_parent<T, F>(
    records: &mut [T],
    entity_id: EntityId,
    parent_id: EntityId,
    field: F,
) -> Result<(), WorthTopologyMaterializationError>
where
    F: Fn(&mut T) -> &mut Option<EntityId>,
    T: HasEntityId,
{
    let record = find_record_mut(records, entity_id)?;
    *field(record) = Some(parent_id);
    Ok(())
}

fn find_record_mut<T>(
    records: &mut [T],
    entity_id: EntityId,
) -> Result<&mut T, WorthTopologyMaterializationError>
where
    T: HasEntityId,
{
    records
        .iter_mut()
        .find(|record| record.entity_id() == entity_id)
        .ok_or_else(|| {
            WorthTopologyMaterializationError::new(format!(
                "worth topology materialization could not find entity {:?} while wiring structure",
                entity_id
            ))
        })
}
