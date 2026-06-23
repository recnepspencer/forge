use crate::workload_platform::planar_boolean_events::{
    PlanarBooleanIntervalEventKind, PlanarBooleanSourceIntervalSense,
};

use super::counters::PlanarBooleanIntervalSplitCandidateCounters;

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanIntervalSplitCandidate {
    candidate_identity: String,
    interval_event_identity: String,
    interval_event_kind: PlanarBooleanIntervalEventKind,
    carrier_identity: String,
    source_edge_identity: String,
    segment_identity: String,
    source_interval_identity: String,
    source_parameter_range: [f64; 2],
    source_sense: PlanarBooleanSourceIntervalSense,
    normalized_interval_identity: String,
    normalized_parameter_range: [f64; 2],
    local_frame_identity: String,
    precision_basis_identity: String,
    participation_row_identity: String,
    event_group_identities: Vec<String>,
}

impl PlanarBooleanIntervalSplitCandidate {
    pub(crate) fn new(input: PlanarBooleanIntervalSplitCandidateInput) -> Self {
        Self {
            candidate_identity: input.candidate_identity,
            interval_event_identity: input.interval_event_identity,
            interval_event_kind: input.interval_event_kind,
            carrier_identity: input.carrier_identity,
            source_edge_identity: input.source_edge_identity,
            segment_identity: input.segment_identity,
            source_interval_identity: input.source_interval_identity,
            source_parameter_range: input.source_parameter_range,
            source_sense: input.source_sense,
            normalized_interval_identity: input.normalized_interval_identity,
            normalized_parameter_range: input.normalized_parameter_range,
            local_frame_identity: input.local_frame_identity,
            precision_basis_identity: input.precision_basis_identity,
            participation_row_identity: input.participation_row_identity,
            event_group_identities: input.event_group_identities,
        }
    }

    pub fn candidate_identity(&self) -> &str {
        &self.candidate_identity
    }

    pub fn interval_event_identity(&self) -> &str {
        &self.interval_event_identity
    }

    pub fn interval_event_kind(&self) -> PlanarBooleanIntervalEventKind {
        self.interval_event_kind
    }

    pub fn carrier_identity(&self) -> &str {
        &self.carrier_identity
    }

    pub fn source_edge_identity(&self) -> &str {
        &self.source_edge_identity
    }

    pub fn segment_identity(&self) -> &str {
        &self.segment_identity
    }

    pub fn source_interval_identity(&self) -> &str {
        &self.source_interval_identity
    }

    pub fn source_parameter_range(&self) -> [f64; 2] {
        self.source_parameter_range
    }

    pub fn source_sense(&self) -> PlanarBooleanSourceIntervalSense {
        self.source_sense
    }

    pub fn normalized_interval_identity(&self) -> &str {
        &self.normalized_interval_identity
    }

    pub fn normalized_parameter_range(&self) -> [f64; 2] {
        self.normalized_parameter_range
    }

    pub fn local_frame_identity(&self) -> &str {
        &self.local_frame_identity
    }

    pub fn precision_basis_identity(&self) -> &str {
        &self.precision_basis_identity
    }

    pub fn participation_row_identity(&self) -> &str {
        &self.participation_row_identity
    }

    pub fn event_group_identities(&self) -> &[String] {
        &self.event_group_identities
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanIntervalSplitCandidateSet {
    candidate_set_identity: String,
    participation_index_identity: String,
    candidates: Vec<PlanarBooleanIntervalSplitCandidate>,
    counters: PlanarBooleanIntervalSplitCandidateCounters,
}

impl PlanarBooleanIntervalSplitCandidateSet {
    pub(crate) fn new(
        candidate_set_identity: String,
        participation_index_identity: String,
        candidates: Vec<PlanarBooleanIntervalSplitCandidate>,
        counters: PlanarBooleanIntervalSplitCandidateCounters,
    ) -> Self {
        Self {
            candidate_set_identity,
            participation_index_identity,
            candidates,
            counters,
        }
    }

    pub fn candidate_set_identity(&self) -> &str {
        &self.candidate_set_identity
    }

    pub fn participation_index_identity(&self) -> &str {
        &self.participation_index_identity
    }

    pub fn candidates(&self) -> &[PlanarBooleanIntervalSplitCandidate] {
        &self.candidates
    }

    pub fn counters(&self) -> PlanarBooleanIntervalSplitCandidateCounters {
        self.counters
    }
}

pub(crate) struct PlanarBooleanIntervalSplitCandidateInput {
    pub(crate) candidate_identity: String,
    pub(crate) interval_event_identity: String,
    pub(crate) interval_event_kind: PlanarBooleanIntervalEventKind,
    pub(crate) carrier_identity: String,
    pub(crate) source_edge_identity: String,
    pub(crate) segment_identity: String,
    pub(crate) source_interval_identity: String,
    pub(crate) source_parameter_range: [f64; 2],
    pub(crate) source_sense: PlanarBooleanSourceIntervalSense,
    pub(crate) normalized_interval_identity: String,
    pub(crate) normalized_parameter_range: [f64; 2],
    pub(crate) local_frame_identity: String,
    pub(crate) precision_basis_identity: String,
    pub(crate) participation_row_identity: String,
    pub(crate) event_group_identities: Vec<String>,
}
