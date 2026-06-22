use forge_relational::facade::identity::{EntityId, PartitionId};

use crate::brep::topology_graph::{
    TopologyBody, TopologyEdge, TopologyFace, TopologyHalfEdge, TopologyLoop, TopologyLump,
    TopologyModel, TopologyRegion, TopologyShell, TopologyVertex, TopologyView, TopologyWire,
};

pub(crate) fn entity(slot: u64) -> EntityId {
    EntityId::new(PartitionId::main(), slot, 1)
}

pub(crate) fn base_container(base: u64, label: &str) -> TopologyView {
    let model_id = entity(base);
    let body_id = entity(base + 1);
    let lump_id = entity(base + 2);
    let region_id = entity(base + 3);

    TopologyView {
        models: vec![TopologyModel {
            entity_id: model_id,
            label: format!("{label} model"),
            body_ids: vec![body_id],
        }],
        bodies: vec![TopologyBody {
            entity_id: body_id,
            label: format!("{label} body"),
            model_id: Some(model_id),
            lump_ids: vec![lump_id],
        }],
        lumps: vec![TopologyLump {
            entity_id: lump_id,
            label: format!("{label} lump"),
            body_id: Some(body_id),
            region_ids: vec![region_id],
        }],
        regions: vec![TopologyRegion {
            entity_id: region_id,
            label: format!("{label} region"),
            lump_id: Some(lump_id),
            shell_ids: Vec::new(),
        }],
        ..TopologyView::default()
    }
}

pub(crate) fn edge(label: impl Into<String>, id: EntityId) -> TopologyEdge {
    TopologyEdge {
        entity_id: id,
        label: label.into(),
    }
}

pub(crate) fn vertex(label: impl Into<String>, id: EntityId) -> TopologyVertex {
    TopologyVertex {
        entity_id: id,
        label: label.into(),
    }
}

pub(crate) fn shell(label: impl Into<String>, id: EntityId, region_id: EntityId) -> TopologyShell {
    TopologyShell {
        entity_id: id,
        label: label.into(),
        region_id: Some(region_id),
        face_ids: Vec::new(),
    }
}

pub(crate) fn face(
    label: impl Into<String>,
    id: EntityId,
    shell_id: Option<EntityId>,
    loop_id: EntityId,
    half_edge_ids: Vec<EntityId>,
) -> TopologyFace {
    TopologyFace {
        entity_id: id,
        label: label.into(),
        shell_id,
        outer_loop_id: Some(loop_id),
        inner_loop_ids: Vec::new(),
        boundary_half_edge_ids: half_edge_ids,
    }
}

pub(crate) fn loop_record(
    label: impl Into<String>,
    id: EntityId,
    face_id: EntityId,
    half_edge_ids: Vec<EntityId>,
) -> TopologyLoop {
    TopologyLoop {
        entity_id: id,
        label: label.into(),
        face_ids: vec![face_id],
        half_edge_ids,
    }
}

pub(crate) fn wire(
    label: impl Into<String>,
    id: EntityId,
    half_edge_ids: Vec<EntityId>,
) -> TopologyWire {
    TopologyWire {
        entity_id: id,
        label: label.into(),
        half_edge_ids,
    }
}

pub(crate) struct HalfEdgeRecordConstruction {
    pub(crate) label: String,
    pub(crate) id: EntityId,
    pub(crate) loop_id: Option<EntityId>,
    pub(crate) wire_id: Option<EntityId>,
    pub(crate) next_id: Option<EntityId>,
    pub(crate) prev_id: Option<EntityId>,
    pub(crate) radial_next_id: Option<EntityId>,
    pub(crate) edge_id: EntityId,
    pub(crate) origin_id: EntityId,
    pub(crate) target_id: EntityId,
    pub(crate) face_id: Option<EntityId>,
}

pub(crate) fn half_edge(construction: HalfEdgeRecordConstruction) -> TopologyHalfEdge {
    TopologyHalfEdge {
        entity_id: construction.id,
        label: construction.label,
        loop_id: construction.loop_id,
        wire_id: construction.wire_id,
        next_half_edge_id: construction.next_id,
        prev_half_edge_id: construction.prev_id,
        radial_next_half_edge_id: construction.radial_next_id,
        edge_id: Some(construction.edge_id),
        origin_vertex_id: Some(construction.origin_id),
        target_vertex_id: Some(construction.target_id),
        face_id: construction.face_id,
    }
}
