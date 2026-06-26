use forge_relational::facade::identity::EntityId;
use schema::facade::platform::authority::WireInterpretationClass;

use crate::projection::read_views::domain::TopologyReadRequestReport;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WireViewQueryReadRow {
    request_report: TopologyReadRequestReport,
    wire_id: EntityId,
    class: WireInterpretationClass,
    connected_component_count: usize,
    half_edge_ids: Vec<EntityId>,
    terminal_vertex_ids: Vec<EntityId>,
    branch_vertex_ids: Vec<EntityId>,
}

impl WireViewQueryReadRow {
    #[allow(dead_code)]
    pub(crate) fn new(
        request_report: TopologyReadRequestReport,
        wire_id: EntityId,
        class: WireInterpretationClass,
        connected_component_count: usize,
        half_edge_ids: Vec<EntityId>,
        terminal_vertex_ids: Vec<EntityId>,
        branch_vertex_ids: Vec<EntityId>,
    ) -> Self {
        Self {
            request_report,
            wire_id,
            class,
            connected_component_count,
            half_edge_ids,
            terminal_vertex_ids,
            branch_vertex_ids,
        }
    }

    pub fn request_report(&self) -> &TopologyReadRequestReport {
        &self.request_report
    }

    pub const fn wire_id(&self) -> EntityId {
        self.wire_id
    }

    pub const fn class(&self) -> WireInterpretationClass {
        self.class
    }

    pub const fn connected_component_count(&self) -> usize {
        self.connected_component_count
    }

    pub fn half_edge_ids(&self) -> &[EntityId] {
        &self.half_edge_ids
    }

    pub fn terminal_vertex_ids(&self) -> &[EntityId] {
        &self.terminal_vertex_ids
    }

    pub fn branch_vertex_ids(&self) -> &[EntityId] {
        &self.branch_vertex_ids
    }
}
