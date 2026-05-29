use std::collections::BTreeMap;

use forge_relational::facade::identity::EntityId;
use schema::facade::platform::entities::{EntityKind, TopologyEntityKind};
use schema::facade::platform::relations::{
    GeometryRelationKind, RelationKind, TopologyRelationKind,
};

use crate::brep::topology_graph::TopologyView;
use crate::derived_topology::materialized_graph::errors::TopologyMaterializationError;
use crate::derived_topology::materialized_graph::input_rows::MaterializationRelationRow;
use crate::derived_topology::materialized_graph::relation_wiring_support::{
    ensure_relation_types, push_child_to_parent, set_optional_parent,
};

pub fn apply_relation(
    view: &mut TopologyView,
    entity_kind_map: &BTreeMap<EntityId, EntityKind>,
    relation: &MaterializationRelationRow,
) -> Result<(), TopologyMaterializationError> {
    let kind = relation.kind;
    let Some(source_kind) = entity_kind_map.get(&relation.source).copied() else {
        return Err(TopologyMaterializationError::new(format!(
            " relation `{}` references missing source entity {:?}",
            kind.kind_name(),
            relation.source
        )));
    };
    let Some(target_kind) = entity_kind_map.get(&relation.target).copied() else {
        return Err(TopologyMaterializationError::new(format!(
            " relation `{}` references missing target entity {:?}",
            kind.kind_name(),
            relation.target
        )));
    };

    match kind {
        RelationKind::Topology(TopologyRelationKind::ModelOwnsBody) => {
            ensure_relation_types(
                kind,
                source_kind,
                EntityKind::Topology(TopologyEntityKind::Model),
                target_kind,
                EntityKind::Topology(TopologyEntityKind::Body),
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
        RelationKind::Topology(TopologyRelationKind::BodyOwnsLump) => {
            ensure_relation_types(
                kind,
                source_kind,
                EntityKind::Topology(TopologyEntityKind::Body),
                target_kind,
                EntityKind::Topology(TopologyEntityKind::Lump),
            )?;
            push_child_to_parent(&mut view.bodies, relation.source, relation.target, |body| {
                &mut body.lump_ids
            })?;
            set_optional_parent(&mut view.lumps, relation.target, relation.source, |lump| {
                &mut lump.body_id
            })?;
        }
        RelationKind::Topology(TopologyRelationKind::LumpOwnsRegion) => {
            ensure_relation_types(
                kind,
                source_kind,
                EntityKind::Topology(TopologyEntityKind::Lump),
                target_kind,
                EntityKind::Topology(TopologyEntityKind::Region),
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
        RelationKind::Topology(TopologyRelationKind::RegionOwnsShell) => {
            ensure_relation_types(
                kind,
                source_kind,
                EntityKind::Topology(TopologyEntityKind::Region),
                target_kind,
                EntityKind::Topology(TopologyEntityKind::Shell),
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
        RelationKind::Topology(TopologyRelationKind::ShellOwnsFace) => {
            ensure_relation_types(
                kind,
                source_kind,
                EntityKind::Topology(TopologyEntityKind::Shell),
                target_kind,
                EntityKind::Topology(TopologyEntityKind::Face),
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
        RelationKind::Topology(TopologyRelationKind::FaceOuterLoop) => {
            ensure_relation_types(
                kind,
                source_kind,
                EntityKind::Topology(TopologyEntityKind::Face),
                target_kind,
                EntityKind::Topology(TopologyEntityKind::Loop),
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
        RelationKind::Topology(TopologyRelationKind::FaceInnerLoop) => {
            ensure_relation_types(
                kind,
                source_kind,
                EntityKind::Topology(TopologyEntityKind::Face),
                target_kind,
                EntityKind::Topology(TopologyEntityKind::Loop),
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
        RelationKind::Topology(TopologyRelationKind::LoopOwnsHalfEdge) => {
            ensure_relation_types(
                kind,
                source_kind,
                EntityKind::Topology(TopologyEntityKind::Loop),
                target_kind,
                EntityKind::Topology(TopologyEntityKind::HalfEdge),
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
        RelationKind::Topology(TopologyRelationKind::WireOwnsHalfEdge) => {
            ensure_relation_types(
                kind,
                source_kind,
                EntityKind::Topology(TopologyEntityKind::Wire),
                target_kind,
                EntityKind::Topology(TopologyEntityKind::HalfEdge),
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
        RelationKind::Topology(TopologyRelationKind::HalfEdgeNext) => {
            ensure_relation_types(
                kind,
                source_kind,
                EntityKind::Topology(TopologyEntityKind::HalfEdge),
                target_kind,
                EntityKind::Topology(TopologyEntityKind::HalfEdge),
            )?;
            set_optional_parent(
                &mut view.half_edges,
                relation.source,
                relation.target,
                |half_edge| &mut half_edge.next_half_edge_id,
            )?;
        }
        RelationKind::Topology(TopologyRelationKind::HalfEdgePrev) => {
            ensure_relation_types(
                kind,
                source_kind,
                EntityKind::Topology(TopologyEntityKind::HalfEdge),
                target_kind,
                EntityKind::Topology(TopologyEntityKind::HalfEdge),
            )?;
            set_optional_parent(
                &mut view.half_edges,
                relation.source,
                relation.target,
                |half_edge| &mut half_edge.prev_half_edge_id,
            )?;
        }
        RelationKind::Topology(TopologyRelationKind::HalfEdgeRadialNext) => {
            ensure_relation_types(
                kind,
                source_kind,
                EntityKind::Topology(TopologyEntityKind::HalfEdge),
                target_kind,
                EntityKind::Topology(TopologyEntityKind::HalfEdge),
            )?;
            set_optional_parent(
                &mut view.half_edges,
                relation.source,
                relation.target,
                |half_edge| &mut half_edge.radial_next_half_edge_id,
            )?;
        }
        RelationKind::Topology(TopologyRelationKind::HalfEdgeUsesEdge) => {
            ensure_relation_types(
                kind,
                source_kind,
                EntityKind::Topology(TopologyEntityKind::HalfEdge),
                target_kind,
                EntityKind::Topology(TopologyEntityKind::Edge),
            )?;
            set_optional_parent(
                &mut view.half_edges,
                relation.source,
                relation.target,
                |half_edge| &mut half_edge.edge_id,
            )?;
        }
        RelationKind::Topology(TopologyRelationKind::HalfEdgeStartsAtVertex) => {
            ensure_relation_types(
                kind,
                source_kind,
                EntityKind::Topology(TopologyEntityKind::HalfEdge),
                target_kind,
                EntityKind::Topology(TopologyEntityKind::Vertex),
            )?;
            set_optional_parent(
                &mut view.half_edges,
                relation.source,
                relation.target,
                |half_edge| &mut half_edge.origin_vertex_id,
            )?;
        }
        RelationKind::Topology(TopologyRelationKind::HalfEdgeEndsAtVertex) => {
            ensure_relation_types(
                kind,
                source_kind,
                EntityKind::Topology(TopologyEntityKind::HalfEdge),
                target_kind,
                EntityKind::Topology(TopologyEntityKind::Vertex),
            )?;
            set_optional_parent(
                &mut view.half_edges,
                relation.source,
                relation.target,
                |half_edge| &mut half_edge.target_vertex_id,
            )?;
        }
        RelationKind::Geometry(
            GeometryRelationKind::FaceUsesSurfaceBinding
            | GeometryRelationKind::EdgeUsesCurveBinding
            | GeometryRelationKind::HalfEdgeUsesCoedgeBinding
            | GeometryRelationKind::VertexUsesGeometryBinding,
        ) => {}
        RelationKind::Naming(_) | RelationKind::Diagnostics(_) => {}
    }

    Ok(())
}

pub fn finalize_topology_membership(
    view: &mut TopologyView,
) -> Result<(), TopologyMaterializationError> {
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




