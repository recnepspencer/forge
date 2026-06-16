use std::collections::BTreeSet;

use crate::workload_platform::planar_boolean_edge_splitting::event_participation_index::PlanarBooleanSplitEventParticipationIndex;

use super::candidate::{
    PlanarBooleanPointSplitCandidate, PlanarBooleanPointSplitCandidateInput,
    PlanarBooleanPointSplitCandidateSet,
};
use super::counters::PlanarBooleanPointSplitCandidateCounters;
use super::denial::{
    PlanarBooleanPointSplitCandidateDenial, PlanarBooleanPointSplitCandidateDenialKind,
};
use super::identity::{point_candidate_identity, point_candidate_set_identity};
use super::parameter_binding::{
    bind_point_event_to_source_edge_parameter, PointEventCarrierParameterBinding,
};

impl PlanarBooleanSplitEventParticipationIndex {
    pub fn extract_point_split_candidates(
        &self,
    ) -> Result<PlanarBooleanPointSplitCandidateSet, PlanarBooleanPointSplitCandidateDenial> {
        let mut candidates = Vec::new();
        let mut inspected_point_events = BTreeSet::new();
        for row in self.rows() {
            for point_event_identity in row.point_event_identities() {
                let event = self.point_event(point_event_identity).ok_or_else(|| {
                    denial(
                        PlanarBooleanPointSplitCandidateDenialKind::MissingParticipationRow,
                        point_event_identity,
                        "point split candidate requires an indexed point event",
                    )
                })?;
                inspected_point_events.insert(event.event_identity().to_string());
                let binding =
                    bind_point_event_to_source_edge_parameter(event, row.carrier_identity())?;
                if let PointEventCarrierParameterBinding::Bound(parameter) = binding {
                    candidates.push(point_split_candidate_from_binding(
                        self.index_identity(),
                        event,
                        row,
                        parameter,
                    ));
                }
            }
        }
        canonicalize_point_split_candidates(&mut candidates);
        let counters = PlanarBooleanPointSplitCandidateCounters::new(
            inspected_point_events.len(),
            candidates.len(),
            0,
        );
        let candidate_set_identity =
            point_candidate_set_identity(self.index_identity(), &candidates);
        Ok(PlanarBooleanPointSplitCandidateSet::new(
            candidate_set_identity,
            self.index_identity().to_string(),
            candidates,
            counters,
        ))
    }
}

fn point_split_candidate_from_binding(
    participation_index_identity: &str,
    event: &crate::workload_platform::planar_boolean_events::PlanarBooleanPointEvent,
    row: &crate::workload_platform::planar_boolean_edge_splitting::event_participation_index::PlanarBooleanSplitEventParticipationRow,
    parameter: &crate::workload_platform::planar_boolean_events::PlanarBooleanPointEventSegmentParameterFact,
) -> PlanarBooleanPointSplitCandidate {
    let candidate_identity = point_candidate_identity(
        participation_index_identity,
        event.event_identity(),
        parameter.carrier_identity(),
        parameter.parameter_fact_identity(),
    );
    PlanarBooleanPointSplitCandidate::new(PlanarBooleanPointSplitCandidateInput {
        candidate_identity,
        point_event_identity: event.event_identity().to_string(),
        point_event_kind: event.kind(),
        carrier_identity: parameter.carrier_identity().to_string(),
        source_edge_identity: row.source_edge_identity().to_string(),
        segment_identity: parameter.segment_identity().to_string(),
        coordinate_fact: event.coordinate_fact().clone(),
        parameter_fact_identity: parameter.parameter_fact_identity().to_string(),
        parameter: parameter.parameter(),
        participation_row_identity: row.participation_row_identity().to_string(),
        event_group_identities: row.event_group_identities().to_vec(),
        segment_pair_identities: event.segment_pair_identities().to_vec(),
        participating_carrier_identities: event.participating_carrier_identities().to_vec(),
        event_endpoint_source_identities: event.endpoint_source_identities().to_vec(),
        event_endpoint_projection_fact_digests: event.endpoint_projection_fact_digests().to_vec(),
        predicate_receipt_identities: event.predicate_receipt_identities().to_vec(),
        shared_endpoint_source_identities: event
            .shared_endpoint_event()
            .map(|shared| shared.source_endpoint_identities().to_vec())
            .unwrap_or_default(),
        shared_endpoint_projection_fact_digests: event
            .shared_endpoint_event()
            .map(|shared| shared.endpoint_projection_fact_digests().to_vec())
            .unwrap_or_default(),
        start_source_endpoint_identity: row.start_source_endpoint_identity().to_string(),
        start_projected_endpoint_fact_identity: row
            .start_projected_endpoint_fact_identity()
            .to_string(),
        end_source_endpoint_identity: row.end_source_endpoint_identity().to_string(),
        end_projected_endpoint_fact_identity: row
            .end_projected_endpoint_fact_identity()
            .to_string(),
    })
}

fn canonicalize_point_split_candidates(candidates: &mut [PlanarBooleanPointSplitCandidate]) {
    candidates.sort_by(|left, right| {
        left.candidate_identity()
            .cmp(right.candidate_identity())
            .then_with(|| left.carrier_identity().cmp(right.carrier_identity()))
    });
}

fn denial(
    kind: PlanarBooleanPointSplitCandidateDenialKind,
    evidence_identity: impl Into<String>,
    human_reason: impl Into<String>,
) -> PlanarBooleanPointSplitCandidateDenial {
    PlanarBooleanPointSplitCandidateDenial::new(kind, evidence_identity, human_reason)
}
