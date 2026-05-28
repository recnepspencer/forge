use schema::facade::platform::entities::{EntityKind, TopologyEntityKind};

use crate::brep::topology_graph::{
    TopologyBody, TopologyEdge, TopologyFace, TopologyHalfEdge, TopologyLoop, TopologyLump,
    TopologyModel, TopologyRegion, TopologyShell, TopologyVertex, TopologyView, TopologyWire,
};
use crate::derived_topology::materialized_graph::input_rows::MaterializationEntityRow;

pub fn push_entity_row(view: &mut TopologyView, record: &MaterializationEntityRow) {
    match record.kind {
        EntityKind::Topology(TopologyEntityKind::Model) => {
            view.models.push(TopologyModel {
                entity_id: record.entity_id,
                label: record.label.clone(),
                body_ids: Vec::new(),
            });
        }
        EntityKind::Topology(TopologyEntityKind::Body) => {
            view.bodies.push(TopologyBody {
                entity_id: record.entity_id,
                label: record.label.clone(),
                model_id: None,
                lump_ids: Vec::new(),
            });
        }
        EntityKind::Topology(TopologyEntityKind::Lump) => {
            view.lumps.push(TopologyLump {
                entity_id: record.entity_id,
                label: record.label.clone(),
                body_id: None,
                region_ids: Vec::new(),
            });
        }
        EntityKind::Topology(TopologyEntityKind::Region) => {
            view.regions.push(TopologyRegion {
                entity_id: record.entity_id,
                label: record.label.clone(),
                lump_id: None,
                shell_ids: Vec::new(),
            });
        }
        EntityKind::Topology(TopologyEntityKind::Shell) => {
            view.shells.push(TopologyShell {
                entity_id: record.entity_id,
                label: record.label.clone(),
                region_id: None,
                face_ids: Vec::new(),
            });
        }
        EntityKind::Topology(TopologyEntityKind::Face) => {
            view.faces.push(TopologyFace {
                entity_id: record.entity_id,
                label: record.label.clone(),
                shell_id: None,
                outer_loop_id: None,
                inner_loop_ids: Vec::new(),
                boundary_half_edge_ids: Vec::new(),
            });
        }
        EntityKind::Topology(TopologyEntityKind::Loop) => {
            view.loops.push(TopologyLoop {
                entity_id: record.entity_id,
                label: record.label.clone(),
                face_ids: Vec::new(),
                half_edge_ids: Vec::new(),
            });
        }
        EntityKind::Topology(TopologyEntityKind::Wire) => {
            view.wires.push(TopologyWire {
                entity_id: record.entity_id,
                label: record.label.clone(),
                half_edge_ids: Vec::new(),
            });
        }
        EntityKind::Topology(TopologyEntityKind::HalfEdge) => {
            view.half_edges.push(TopologyHalfEdge {
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
        EntityKind::Topology(TopologyEntityKind::Edge) => {
            view.edges.push(TopologyEdge {
                entity_id: record.entity_id,
                label: record.label.clone(),
            });
        }
        EntityKind::Topology(TopologyEntityKind::Vertex) => {
            view.vertices.push(TopologyVertex {
                entity_id: record.entity_id,
                label: record.label.clone(),
            });
        }
        EntityKind::Geometry(_) | EntityKind::Naming(_) | EntityKind::Diagnostics(_) => {}
    }
}

pub fn has_topology_content(view: &TopologyView) -> bool {
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




