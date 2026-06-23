use super::counters::PlanarBooleanSplitEdgeFragmentCounters;
use super::fragment_row::PlanarBooleanSplitEdgeFragment;

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanSplitEdgeFragmentSchedule {
    schedule_identity: String,
    interval_subdivision_schedule_identity: String,
    split_vertex_identity_schedule_identity: String,
    source_edge_identity: String,
    carrier_identity: String,
    fragments: Vec<PlanarBooleanSplitEdgeFragment>,
}

impl PlanarBooleanSplitEdgeFragmentSchedule {
    pub(crate) fn new(
        schedule_identity: String,
        interval_subdivision_schedule_identity: String,
        split_vertex_identity_schedule_identity: String,
        source_edge_identity: String,
        carrier_identity: String,
        fragments: Vec<PlanarBooleanSplitEdgeFragment>,
    ) -> Self {
        Self {
            schedule_identity,
            interval_subdivision_schedule_identity,
            split_vertex_identity_schedule_identity,
            source_edge_identity,
            carrier_identity,
            fragments,
        }
    }

    pub fn schedule_identity(&self) -> &str {
        &self.schedule_identity
    }
    pub fn interval_subdivision_schedule_identity(&self) -> &str {
        &self.interval_subdivision_schedule_identity
    }
    pub fn split_vertex_identity_schedule_identity(&self) -> &str {
        &self.split_vertex_identity_schedule_identity
    }
    pub fn source_edge_identity(&self) -> &str {
        &self.source_edge_identity
    }
    pub fn carrier_identity(&self) -> &str {
        &self.carrier_identity
    }
    pub fn fragments(&self) -> &[PlanarBooleanSplitEdgeFragment] {
        &self.fragments
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanSplitEdgeFragmentSet {
    fragment_set_identity: String,
    interval_subdivision_schedule_set_identity: String,
    split_vertex_identity_set_identity: String,
    schedules: Vec<PlanarBooleanSplitEdgeFragmentSchedule>,
    counters: PlanarBooleanSplitEdgeFragmentCounters,
}

impl PlanarBooleanSplitEdgeFragmentSet {
    pub(crate) fn new(
        fragment_set_identity: String,
        interval_subdivision_schedule_set_identity: String,
        split_vertex_identity_set_identity: String,
        schedules: Vec<PlanarBooleanSplitEdgeFragmentSchedule>,
        counters: PlanarBooleanSplitEdgeFragmentCounters,
    ) -> Self {
        Self {
            fragment_set_identity,
            interval_subdivision_schedule_set_identity,
            split_vertex_identity_set_identity,
            schedules,
            counters,
        }
    }

    pub fn fragment_set_identity(&self) -> &str {
        &self.fragment_set_identity
    }
    pub fn interval_subdivision_schedule_set_identity(&self) -> &str {
        &self.interval_subdivision_schedule_set_identity
    }
    pub fn split_vertex_identity_set_identity(&self) -> &str {
        &self.split_vertex_identity_set_identity
    }
    pub fn schedules(&self) -> &[PlanarBooleanSplitEdgeFragmentSchedule] {
        &self.schedules
    }
    pub fn counters(&self) -> PlanarBooleanSplitEdgeFragmentCounters {
        self.counters
    }
    pub fn fragments(&self) -> impl Iterator<Item = &PlanarBooleanSplitEdgeFragment> {
        self.schedules
            .iter()
            .flat_map(|schedule| schedule.fragments())
    }
    pub fn certifies_domain_coverage(&self) -> bool {
        self.counters.source_edges_covered() == self.schedules.len()
            && self.counters.coverage_gaps_rejected() == 0
            && self.counters.collapsed_fragments_rejected() == 0
    }
}
