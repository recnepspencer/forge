use std::collections::BTreeSet;

use crate::workload_platform::planar_boolean_edge_splitting::event_participation_index::PlanarBooleanSplitEventParticipationIndex;

use super::candidate::{
    PlanarBooleanIntervalSplitCandidate, PlanarBooleanIntervalSplitCandidateInput,
    PlanarBooleanIntervalSplitCandidateSet,
};
use super::counters::PlanarBooleanIntervalSplitCandidateCounters;
use super::denial::PlanarBooleanIntervalSplitCandidateDenial;
use super::identity::{interval_candidate_identity, interval_candidate_set_identity};
use super::source_interval_binding::BoundIntervalSourceRange;

impl PlanarBooleanSplitEventParticipationIndex {
    pub fn extract_interval_split_candidates(
        &self,
    ) -> Result<PlanarBooleanIntervalSplitCandidateSet, PlanarBooleanIntervalSplitCandidateDenial>
    {
        let mut candidates = Vec::new();
        let mut inspected_interval_events = BTreeSet::new();
        for row in self.rows() {
            for interval_event_identity in row.interval_event_identities() {
                let event = self.interval_event(interval_event_identity).ok_or_else(|| {
                    PlanarBooleanIntervalSplitCandidateDenial::missing_index_owned_interval_event(
                        interval_event_identity,
                        "interval split candidate requires an indexed interval event",
                    )
                })?;
                inspected_interval_events.insert(event.event_identity().to_string());
                let source_range = BoundIntervalSourceRange::bind(row, event)?;
                candidates.push(candidate_from_source_interval(
                    self.index_identity(),
                    &source_range,
                ));
            }
        }
        candidates.sort_by(|left, right| {
            left.candidate_identity()
                .cmp(right.candidate_identity())
                .then_with(|| left.carrier_identity().cmp(right.carrier_identity()))
        });
        let counters = PlanarBooleanIntervalSplitCandidateCounters::new(
            inspected_interval_events.len(),
            candidates.len(),
            0,
        );
        let candidate_set_identity =
            interval_candidate_set_identity(self.index_identity(), &candidates);
        Ok(PlanarBooleanIntervalSplitCandidateSet::new(
            candidate_set_identity,
            self.index_identity().to_string(),
            candidates,
            counters,
        ))
    }
}

fn candidate_from_source_interval(
    participation_index_identity: &str,
    source_range: &BoundIntervalSourceRange<'_>,
) -> PlanarBooleanIntervalSplitCandidate {
    let source_interval = source_range.source_interval();
    let candidate_identity = interval_candidate_identity(
        participation_index_identity,
        source_range.interval_event_identity(),
        source_interval.carrier_identity(),
        source_interval.source_interval_identity(),
    );
    PlanarBooleanIntervalSplitCandidate::new(PlanarBooleanIntervalSplitCandidateInput {
        candidate_identity,
        interval_event_identity: source_range.interval_event_identity().to_string(),
        interval_event_kind: source_range.interval_event_kind(),
        carrier_identity: source_interval.carrier_identity().to_string(),
        source_edge_identity: source_range.source_edge_identity().to_string(),
        segment_identity: source_interval.segment_identity().to_string(),
        source_interval_identity: source_interval.source_interval_identity().to_string(),
        source_parameter_range: source_interval.source_parameter_range(),
        source_sense: source_interval.sense(),
        normalized_interval_identity: source_range
            .normalized_interval()
            .normalized_interval_identity()
            .to_string(),
        normalized_parameter_range: source_range.normalized_interval().parameter_range(),
        local_frame_identity: source_range.local_frame_identity().to_string(),
        precision_basis_identity: source_range.precision_basis_identity().to_string(),
        participation_row_identity: source_range.participation_row_identity().to_string(),
        event_group_identities: source_range.event_group_identities().to_vec(),
    })
}
