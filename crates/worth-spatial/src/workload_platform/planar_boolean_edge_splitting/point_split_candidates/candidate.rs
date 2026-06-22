use crate::workload_platform::planar_boolean_events::{
    PlanarBooleanPointEventCoordinateFact, PlanarBooleanPointEventKind,
};

use super::counters::PlanarBooleanPointSplitCandidateCounters;

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanPointSplitCandidate {
    candidate_identity: String,
    point_event_identity: String,
    point_event_kind: PlanarBooleanPointEventKind,
    carrier_identity: String,
    source_edge_identity: String,
    segment_identity: String,
    coordinate_fact: PlanarBooleanPointEventCoordinateFact,
    parameter_fact_identity: String,
    parameter: f64,
    participation_row_identity: String,
    event_group_identities: Vec<String>,
    segment_pair_identities: Vec<String>,
    participating_carrier_identities: Vec<String>,
    event_endpoint_source_identities: Vec<String>,
    event_endpoint_projection_fact_digests: Vec<String>,
    predicate_receipt_identities: Vec<String>,
    shared_endpoint_source_identities: Vec<String>,
    shared_endpoint_projection_fact_digests: Vec<String>,
    start_source_endpoint_identity: String,
    start_projected_endpoint_fact_identity: String,
    end_source_endpoint_identity: String,
    end_projected_endpoint_fact_identity: String,
}

impl PlanarBooleanPointSplitCandidate {
    pub(crate) fn new(input: PlanarBooleanPointSplitCandidateInput) -> Self {
        Self {
            candidate_identity: input.candidate_identity,
            point_event_identity: input.point_event_identity,
            point_event_kind: input.point_event_kind,
            carrier_identity: input.carrier_identity,
            source_edge_identity: input.source_edge_identity,
            segment_identity: input.segment_identity,
            coordinate_fact: input.coordinate_fact,
            parameter_fact_identity: input.parameter_fact_identity,
            parameter: input.parameter,
            participation_row_identity: input.participation_row_identity,
            event_group_identities: input.event_group_identities,
            segment_pair_identities: input.segment_pair_identities,
            participating_carrier_identities: input.participating_carrier_identities,
            event_endpoint_source_identities: input.event_endpoint_source_identities,
            event_endpoint_projection_fact_digests: input.event_endpoint_projection_fact_digests,
            predicate_receipt_identities: input.predicate_receipt_identities,
            shared_endpoint_source_identities: input.shared_endpoint_source_identities,
            shared_endpoint_projection_fact_digests: input.shared_endpoint_projection_fact_digests,
            start_source_endpoint_identity: input.start_source_endpoint_identity,
            start_projected_endpoint_fact_identity: input.start_projected_endpoint_fact_identity,
            end_source_endpoint_identity: input.end_source_endpoint_identity,
            end_projected_endpoint_fact_identity: input.end_projected_endpoint_fact_identity,
        }
    }

    pub fn candidate_identity(&self) -> &str {
        &self.candidate_identity
    }

    pub fn point_event_identity(&self) -> &str {
        &self.point_event_identity
    }

    pub fn point_event_kind(&self) -> PlanarBooleanPointEventKind {
        self.point_event_kind
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

    pub fn coordinate_fact(&self) -> &PlanarBooleanPointEventCoordinateFact {
        &self.coordinate_fact
    }

    pub fn parameter_fact_identity(&self) -> &str {
        &self.parameter_fact_identity
    }

    pub fn parameter(&self) -> f64 {
        self.parameter
    }

    pub fn participation_row_identity(&self) -> &str {
        &self.participation_row_identity
    }

    pub fn event_group_identities(&self) -> &[String] {
        &self.event_group_identities
    }

    pub fn segment_pair_identities(&self) -> &[String] {
        &self.segment_pair_identities
    }

    pub fn participating_carrier_identities(&self) -> &[String] {
        &self.participating_carrier_identities
    }

    pub fn event_endpoint_source_identities(&self) -> &[String] {
        &self.event_endpoint_source_identities
    }

    pub fn event_endpoint_projection_fact_digests(&self) -> &[String] {
        &self.event_endpoint_projection_fact_digests
    }

    pub fn predicate_receipt_identities(&self) -> &[String] {
        &self.predicate_receipt_identities
    }

    pub fn shared_endpoint_source_identities(&self) -> &[String] {
        &self.shared_endpoint_source_identities
    }

    pub fn shared_endpoint_projection_fact_digests(&self) -> &[String] {
        &self.shared_endpoint_projection_fact_digests
    }

    pub fn start_source_endpoint_identity(&self) -> &str {
        &self.start_source_endpoint_identity
    }

    pub fn start_projected_endpoint_fact_identity(&self) -> &str {
        &self.start_projected_endpoint_fact_identity
    }

    pub fn end_source_endpoint_identity(&self) -> &str {
        &self.end_source_endpoint_identity
    }

    pub fn end_projected_endpoint_fact_identity(&self) -> &str {
        &self.end_projected_endpoint_fact_identity
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanPointSplitCandidateSet {
    candidate_set_identity: String,
    participation_index_identity: String,
    candidates: Vec<PlanarBooleanPointSplitCandidate>,
    counters: PlanarBooleanPointSplitCandidateCounters,
}

impl PlanarBooleanPointSplitCandidateSet {
    pub(crate) fn new(
        candidate_set_identity: String,
        participation_index_identity: String,
        candidates: Vec<PlanarBooleanPointSplitCandidate>,
        counters: PlanarBooleanPointSplitCandidateCounters,
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

    pub fn candidates(&self) -> &[PlanarBooleanPointSplitCandidate] {
        &self.candidates
    }

    pub fn counters(&self) -> PlanarBooleanPointSplitCandidateCounters {
        self.counters
    }
}

pub(crate) struct PlanarBooleanPointSplitCandidateInput {
    pub(crate) candidate_identity: String,
    pub(crate) point_event_identity: String,
    pub(crate) point_event_kind: PlanarBooleanPointEventKind,
    pub(crate) carrier_identity: String,
    pub(crate) source_edge_identity: String,
    pub(crate) segment_identity: String,
    pub(crate) coordinate_fact: PlanarBooleanPointEventCoordinateFact,
    pub(crate) parameter_fact_identity: String,
    pub(crate) parameter: f64,
    pub(crate) participation_row_identity: String,
    pub(crate) event_group_identities: Vec<String>,
    pub(crate) segment_pair_identities: Vec<String>,
    pub(crate) participating_carrier_identities: Vec<String>,
    pub(crate) event_endpoint_source_identities: Vec<String>,
    pub(crate) event_endpoint_projection_fact_digests: Vec<String>,
    pub(crate) predicate_receipt_identities: Vec<String>,
    pub(crate) shared_endpoint_source_identities: Vec<String>,
    pub(crate) shared_endpoint_projection_fact_digests: Vec<String>,
    pub(crate) start_source_endpoint_identity: String,
    pub(crate) start_projected_endpoint_fact_identity: String,
    pub(crate) end_source_endpoint_identity: String,
    pub(crate) end_projected_endpoint_fact_identity: String,
}
