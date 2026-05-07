use worth_schema::facade::{WorthEntityKind, WorthTopologyEntityKind};

use crate::data::topology_view::{
    WorthTopologyBody, WorthTopologyEdge, WorthTopologyFace, WorthTopologyHalfEdge,
    WorthTopologyLoop, WorthTopologyLump, WorthTopologyModel, WorthTopologyRegion,
    WorthTopologyShell, WorthTopologyVertex, WorthTopologyView, WorthTopologyWire,
};
use crate::materialization::input_rows::MaterializationEntityRow;

pub fn push_entity_row(view: &mut WorthTopologyView, record: &MaterializationEntityRow) {
    match record.kind {
        WorthEntityKind::Topology(WorthTopologyEntityKind::Model) => {
            view.models.push(WorthTopologyModel {
                entity_id: record.entity_id,
                label: record.label.clone(),
                body_ids: Vec::new(),
            });
        }
        WorthEntityKind::Topology(WorthTopologyEntityKind::Body) => {
            view.bodies.push(WorthTopologyBody {
                entity_id: record.entity_id,
                label: record.label.clone(),
                model_id: None,
                lump_ids: Vec::new(),
            });
        }
        WorthEntityKind::Topology(WorthTopologyEntityKind::Lump) => {
            view.lumps.push(WorthTopologyLump {
                entity_id: record.entity_id,
                label: record.label.clone(),
                body_id: None,
                region_ids: Vec::new(),
            });
        }
        WorthEntityKind::Topology(WorthTopologyEntityKind::Region) => {
            view.regions.push(WorthTopologyRegion {
                entity_id: record.entity_id,
                label: record.label.clone(),
                lump_id: None,
                shell_ids: Vec::new(),
            });
        }
        WorthEntityKind::Topology(WorthTopologyEntityKind::Shell) => {
            view.shells.push(WorthTopologyShell {
                entity_id: record.entity_id,
                label: record.label.clone(),
                region_id: None,
                face_ids: Vec::new(),
            });
        }
        WorthEntityKind::Topology(WorthTopologyEntityKind::Face) => {
            view.faces.push(WorthTopologyFace {
                entity_id: record.entity_id,
                label: record.label.clone(),
                shell_id: None,
                outer_loop_id: None,
                inner_loop_ids: Vec::new(),
                boundary_half_edge_ids: Vec::new(),
            });
        }
        WorthEntityKind::Topology(WorthTopologyEntityKind::Loop) => {
            view.loops.push(WorthTopologyLoop {
                entity_id: record.entity_id,
                label: record.label.clone(),
                face_ids: Vec::new(),
                half_edge_ids: Vec::new(),
            });
        }
        WorthEntityKind::Topology(WorthTopologyEntityKind::Wire) => {
            view.wires.push(WorthTopologyWire {
                entity_id: record.entity_id,
                label: record.label.clone(),
                half_edge_ids: Vec::new(),
            });
        }
        WorthEntityKind::Topology(WorthTopologyEntityKind::HalfEdge) => {
            view.half_edges.push(WorthTopologyHalfEdge {
                entity_id: record.entity_id,
                label: record.label.clone(),
                loop_id: None,
                wire_id: None,
                next_half_edge_id: None,
                prev_half_edge_id: None,
                radial_next_half_edge_id: None,
                edge_id: None,
                origin_vertex_id: None,
                target_vertex_id: None,
                face_id: None,
            });
        }
        WorthEntityKind::Topology(WorthTopologyEntityKind::Edge) => {
            view.edges.push(WorthTopologyEdge {
                entity_id: record.entity_id,
                label: record.label.clone(),
            });
        }
        WorthEntityKind::Topology(WorthTopologyEntityKind::Vertex) => {
            view.vertices.push(WorthTopologyVertex {
                entity_id: record.entity_id,
                label: record.label.clone(),
            });
        }
        WorthEntityKind::Geometry(_)
        | WorthEntityKind::Naming(_)
        | WorthEntityKind::Diagnostics(_) => {}
    }
}

pub fn has_topology_content(view: &WorthTopologyView) -> bool {
    !view.bodies.is_empty()
        || !view.lumps.is_empty()
        || !view.regions.is_empty()
        || !view.shells.is_empty()
        || !view.faces.is_empty()
        || !view.loops.is_empty()
        || !view.wires.is_empty()
        || !view.half_edges.is_empty()
        || !view.edges.is_empty()
        || !view.vertices.is_empty()
}
