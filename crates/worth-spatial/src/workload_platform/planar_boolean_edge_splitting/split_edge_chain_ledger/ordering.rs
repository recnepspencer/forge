use std::collections::BTreeMap;

use super::input::PlanarBooleanSplitEdgeChainLedgerInput;

type EdgeKey = (String, String);

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SplitLedgerScheduleBindings {
    endpoint_boundary_by_edge: BTreeMap<EdgeKey, String>,
    interval_subdivision_by_edge: BTreeMap<EdgeKey, String>,
    vertex_by_edge: BTreeMap<EdgeKey, String>,
    fragment_by_edge: BTreeMap<EdgeKey, String>,
}

impl SplitLedgerScheduleBindings {
    pub(crate) fn from_input(input: &PlanarBooleanSplitEdgeChainLedgerInput<'_>) -> Self {
        let mut endpoint_boundary_by_edge = BTreeMap::new();
        for schedule in input.endpoint_boundary_schedules().schedules() {
            endpoint_boundary_by_edge.insert(
                edge_key(schedule.source_edge_identity(), schedule.carrier_identity()),
                schedule.schedule_identity().to_string(),
            );
        }

        let mut interval_subdivision_by_edge = BTreeMap::new();
        for schedule in input.interval_subdivision_schedules().schedules() {
            interval_subdivision_by_edge.insert(
                edge_key(schedule.source_edge_identity(), schedule.carrier_identity()),
                schedule.schedule_identity().to_string(),
            );
        }

        let mut vertex_by_edge = BTreeMap::new();
        for schedule in input.split_vertices().schedules() {
            vertex_by_edge.insert(
                edge_key(schedule.source_edge_identity(), schedule.carrier_identity()),
                schedule.schedule_identity().to_string(),
            );
        }

        let mut fragment_by_edge = BTreeMap::new();
        for schedule in input.split_fragments().schedules() {
            fragment_by_edge.insert(
                edge_key(schedule.source_edge_identity(), schedule.carrier_identity()),
                schedule.schedule_identity().to_string(),
            );
        }

        Self {
            endpoint_boundary_by_edge,
            interval_subdivision_by_edge,
            vertex_by_edge,
            fragment_by_edge,
        }
    }

    pub(crate) fn endpoint_boundary_schedule_identity(&self, key: &EdgeKey) -> String {
        self.endpoint_boundary_by_edge
            .get(key)
            .cloned()
            .unwrap_or_default()
    }
    pub(crate) fn interval_subdivision_schedule_identity(&self, key: &EdgeKey) -> String {
        self.interval_subdivision_by_edge
            .get(key)
            .cloned()
            .unwrap_or_default()
    }
    pub(crate) fn vertex_schedule_identity(&self, key: &EdgeKey) -> String {
        self.vertex_by_edge.get(key).cloned().unwrap_or_default()
    }
    pub(crate) fn fragment_schedule_identity(&self, key: &EdgeKey) -> String {
        self.fragment_by_edge.get(key).cloned().unwrap_or_default()
    }
}

fn edge_key(source_edge_identity: &str, carrier_identity: &str) -> EdgeKey {
    (
        source_edge_identity.to_string(),
        carrier_identity.to_string(),
    )
}
