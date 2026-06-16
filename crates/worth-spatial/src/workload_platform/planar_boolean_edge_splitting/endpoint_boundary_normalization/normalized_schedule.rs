use crate::workload_platform::planar_boolean_edge_splitting::duplicate_split_normalization::{
    PlanarBooleanNormalizedSplitCut, PlanarBooleanRetainedIntervalSplitEntry,
};

use super::counters::PlanarBooleanEndpointBoundaryNormalizationCounters;
use super::decision_record::PlanarBooleanEndpointContactDecision;

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanEndpointBoundaryNormalizedSplitSchedule {
    schedule_identity: String,
    normalized_schedule_identity: String,
    source_edge_identity: String,
    carrier_identity: String,
    fragment_cuts: Vec<PlanarBooleanNormalizedSplitCut>,
    endpoint_contact_decisions: Vec<PlanarBooleanEndpointContactDecision>,
    retained_interval_entries: Vec<PlanarBooleanRetainedIntervalSplitEntry>,
    retained_interval_entry_identities: Vec<String>,
}

impl PlanarBooleanEndpointBoundaryNormalizedSplitSchedule {
    pub(crate) fn new(
        schedule_identity: String,
        normalized_schedule_identity: String,
        source_edge_identity: String,
        carrier_identity: String,
        fragment_cuts: Vec<PlanarBooleanNormalizedSplitCut>,
        endpoint_contact_decisions: Vec<PlanarBooleanEndpointContactDecision>,
        retained_interval_entries: Vec<PlanarBooleanRetainedIntervalSplitEntry>,
    ) -> Self {
        let retained_interval_entry_identities = retained_interval_entries
            .iter()
            .map(|entry| entry.entry_identity().to_string())
            .collect();
        Self {
            schedule_identity,
            normalized_schedule_identity,
            source_edge_identity,
            carrier_identity,
            fragment_cuts,
            endpoint_contact_decisions,
            retained_interval_entries,
            retained_interval_entry_identities,
        }
    }

    pub fn schedule_identity(&self) -> &str {
        &self.schedule_identity
    }
    pub fn normalized_schedule_identity(&self) -> &str {
        &self.normalized_schedule_identity
    }
    pub fn source_edge_identity(&self) -> &str {
        &self.source_edge_identity
    }
    pub fn carrier_identity(&self) -> &str {
        &self.carrier_identity
    }
    pub fn fragment_cuts(&self) -> &[PlanarBooleanNormalizedSplitCut] {
        &self.fragment_cuts
    }
    pub fn endpoint_contact_decisions(&self) -> &[PlanarBooleanEndpointContactDecision] {
        &self.endpoint_contact_decisions
    }
    pub fn retained_interval_entries(&self) -> &[PlanarBooleanRetainedIntervalSplitEntry] {
        &self.retained_interval_entries
    }
    pub fn retained_interval_entry_identities(&self) -> &[String] {
        &self.retained_interval_entry_identities
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet {
    schedule_set_identity: String,
    normalized_schedule_set_identity: String,
    schedules: Vec<PlanarBooleanEndpointBoundaryNormalizedSplitSchedule>,
    counters: PlanarBooleanEndpointBoundaryNormalizationCounters,
}

impl PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet {
    pub(crate) fn new(
        schedule_set_identity: String,
        normalized_schedule_set_identity: String,
        schedules: Vec<PlanarBooleanEndpointBoundaryNormalizedSplitSchedule>,
        counters: PlanarBooleanEndpointBoundaryNormalizationCounters,
    ) -> Self {
        Self {
            schedule_set_identity,
            normalized_schedule_set_identity,
            schedules,
            counters,
        }
    }

    pub fn schedule_set_identity(&self) -> &str {
        &self.schedule_set_identity
    }
    pub fn normalized_schedule_set_identity(&self) -> &str {
        &self.normalized_schedule_set_identity
    }
    pub fn schedules(&self) -> &[PlanarBooleanEndpointBoundaryNormalizedSplitSchedule] {
        &self.schedules
    }
    pub fn counters(&self) -> PlanarBooleanEndpointBoundaryNormalizationCounters {
        self.counters
    }
    pub fn endpoint_contact_decisions(
        &self,
    ) -> impl Iterator<Item = &PlanarBooleanEndpointContactDecision> {
        self.schedules
            .iter()
            .flat_map(|schedule| schedule.endpoint_contact_decisions())
    }
}
