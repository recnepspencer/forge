use std::collections::BTreeSet;

use forge_relational::facade::identity::EntityId;

use super::VertexDiskReadStageCounters;
use crate::brep::topology_graph::{TopologyHalfEdge, TopologyView};
use crate::derived_topology::invalidation_plan::migrated_products::vertex_disks::VertexDiskBoundarySourceRow;
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationTouchedClosure;

pub(super) struct VertexDiskTouchedTopologySelection {
    rows: Vec<VertexDiskBoundarySourceRow>,
    counters: VertexDiskReadStageCounters,
}

impl VertexDiskTouchedTopologySelection {
    pub(super) fn from_touched_closure_and_topology(
        touched_closure: &DerivedInvalidationTouchedClosure,
        topology: &TopologyView,
    ) -> Self {
        let touched_entities = touched_vertex_disk_anchor_entity_set(touched_closure);
        let selected = selected_vertex_disk_rows_from_touched_topology(topology, &touched_entities);
        let selected_source_row_count = selected.rows.len();
        let counters = VertexDiskReadStageCounters::new(
            selected.touched_vertex_count,
            selected.half_edge_lookup_count,
            selected_source_row_count,
            selected_source_row_count,
            selected_source_row_count,
            selected.touched_incident_half_edge_count,
            selected.touched_incident_edge_count,
            topology
                .half_edges
                .len()
                .saturating_sub(selected.touched_incident_half_edge_count),
            0,
        );
        Self {
            rows: selected.rows,
            counters,
        }
    }

    pub(super) fn into_rows_and_counters(
        self,
    ) -> (
        Vec<VertexDiskBoundarySourceRow>,
        VertexDiskReadStageCounters,
    ) {
        (self.rows, self.counters)
    }
}

struct SelectedVertexDiskRows {
    rows: Vec<VertexDiskBoundarySourceRow>,
    half_edge_lookup_count: usize,
    touched_vertex_count: usize,
    touched_incident_half_edge_count: usize,
    touched_incident_edge_count: usize,
}

fn selected_vertex_disk_rows_from_touched_topology(
    topology: &TopologyView,
    touched_entities: &BTreeSet<EntityId>,
) -> SelectedVertexDiskRows {
    let mut rows = Vec::new();
    let mut half_edge_lookup_count = 0;
    let mut touched_vertex_count = 0;
    let mut touched_incident_half_edge_count = 0;
    let mut touched_incident_edge_count = 0;
    for touched_entity in touched_entities {
        half_edge_lookup_count += 1;
        let Some(half_edge) = topology
            .half_edges
            .iter()
            .find(|half_edge| half_edge.entity_id == *touched_entity)
        else {
            continue;
        };
        let touched_vertices = incident_vertices(half_edge, topology)
            .into_iter()
            .map(entity_identity)
            .collect::<Vec<_>>();
        let incident_half_edges = incident_half_edge_identities(topology, half_edge);
        let incident_different_edge_half_edges =
            incident_different_edge_half_edge_identities(topology, half_edge);
        let touched_incident_edges = touched_incident_edge_identities(topology, half_edge);
        touched_vertex_count += touched_vertices.len();
        touched_incident_half_edge_count += incident_half_edges.len();
        touched_incident_edge_count += touched_incident_edges.len();
        rows.push(VertexDiskBoundarySourceRow::new(
            touched_vertices,
            entity_identity(half_edge.entity_id),
            entity_identity(half_edge.entity_id),
            half_edge.edge_id.map(entity_identity).unwrap_or_default(),
            incident_half_edges,
            incident_different_edge_half_edges,
            touched_incident_edges,
        ));
    }
    SelectedVertexDiskRows {
        rows,
        half_edge_lookup_count,
        touched_vertex_count,
        touched_incident_half_edge_count,
        touched_incident_edge_count,
    }
}

fn incident_half_edge_identities(
    topology: &TopologyView,
    source: &TopologyHalfEdge,
) -> Vec<String> {
    let vertices = incident_vertices(source, topology);
    topology
        .half_edges
        .iter()
        .filter(|half_edge| {
            let candidate_vertices = incident_vertices(half_edge, topology);
            vertices
                .iter()
                .any(|vertex| candidate_vertices.contains(vertex))
        })
        .map(|half_edge| entity_identity(half_edge.entity_id))
        .collect()
}

fn incident_different_edge_half_edge_identities(
    topology: &TopologyView,
    source: &TopologyHalfEdge,
) -> Vec<String> {
    let vertices = incident_vertices(source, topology);
    topology
        .half_edges
        .iter()
        .filter(|half_edge| half_edge.edge_id != source.edge_id)
        .filter(|half_edge| {
            let candidate_vertices = incident_vertices(half_edge, topology);
            vertices
                .iter()
                .any(|vertex| candidate_vertices.contains(vertex))
        })
        .map(|half_edge| entity_identity(half_edge.entity_id))
        .collect()
}

fn touched_incident_edge_identities(
    topology: &TopologyView,
    source: &TopologyHalfEdge,
) -> Vec<String> {
    let vertices = incident_vertices(source, topology);
    let mut edge_identities = topology
        .half_edges
        .iter()
        .filter(|half_edge| {
            let candidate_vertices = incident_vertices(half_edge, topology);
            vertices
                .iter()
                .any(|vertex| candidate_vertices.contains(vertex))
        })
        .filter_map(|half_edge| half_edge.edge_id)
        .map(entity_identity)
        .collect::<Vec<_>>();
    edge_identities.sort();
    edge_identities.dedup();
    edge_identities
}

fn incident_vertices(half_edge: &TopologyHalfEdge, topology: &TopologyView) -> BTreeSet<EntityId> {
    let mut vertices = BTreeSet::new();
    if let Some(origin) = half_edge.origin_vertex_id {
        vertices.insert(origin);
    }
    if let Some(target) = half_edge.target_vertex_id {
        vertices.insert(target);
        return vertices;
    }
    if let Some(next_id) = half_edge.next_half_edge_id {
        if let Some(next) = topology
            .half_edges
            .iter()
            .find(|candidate| candidate.entity_id == next_id)
        {
            if let Some(target) = next.origin_vertex_id {
                vertices.insert(target);
            }
        }
    }
    vertices
}

fn entity_identity(entity_id: EntityId) -> String {
    format!(
        "entity:{}:{}:{}",
        entity_id.partition_id.0, entity_id.local_slot.0, entity_id.generation.0
    )
}

fn touched_vertex_disk_anchor_entity_set(
    touched_closure: &DerivedInvalidationTouchedClosure,
) -> BTreeSet<EntityId> {
    touched_closure
        .basis()
        .entities()
        .iter()
        .map(|entity| entity.entity_id())
        .collect()
}
