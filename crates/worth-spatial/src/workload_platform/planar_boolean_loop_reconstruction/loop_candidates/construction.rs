use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanWalkOutcomeKind, PlanarBooleanWalkOutcomeRow,
};

use super::counters::PlanarBooleanLoopCandidateCounters;
use super::identity::{
    denied_loop_candidate_identity, denied_loop_candidate_set_identity, loop_candidate_identity,
    loop_candidate_set_identity,
};
use super::input::PlanarBooleanLoopCandidateBoundaryInput;
use super::product::{
    PlanarBooleanDeniedLoopCandidateSet, PlanarBooleanLoopCandidateBoundary,
    PlanarBooleanLoopCandidateSet,
};
use super::row::{
    PlanarBooleanDeniedLoopCandidate, PlanarBooleanDeniedLoopCandidateKind,
    PlanarBooleanLoopCandidate,
};

pub(crate) fn promote_loop_candidates(
    input: PlanarBooleanLoopCandidateBoundaryInput<'_>,
) -> PlanarBooleanLoopCandidateBoundary {
    let request_identity = input.walk_outcomes().request_identity().to_string();
    let walk_outcome_set_identity = input
        .walk_outcomes()
        .walk_outcome_set_identity()
        .to_string();
    let mut counters = PlanarBooleanLoopCandidateCounters::default();
    let mut candidates = Vec::new();
    let mut denied = Vec::new();

    for row in input.walk_outcomes().closed_rows() {
        counters.considered_closed_walk();
        match promote_closed_walk(&request_identity, row) {
            Ok(candidate) => {
                counters.promoted_loop_candidate();
                candidates.push(candidate);
            }
            Err(denied_candidate) => {
                counters.emitted_denied_loop_candidate();
                denied.push(denied_candidate);
            }
        }
    }

    candidates.sort_by(|left, right| {
        left.source_loop_identity()
            .cmp(right.source_loop_identity())
            .then_with(|| {
                left.loop_candidate_identity()
                    .cmp(right.loop_candidate_identity())
            })
    });
    denied.sort_by(|left, right| {
        left.source_loop_identity()
            .cmp(right.source_loop_identity())
            .then_with(|| {
                left.denied_loop_candidate_identity()
                    .cmp(right.denied_loop_candidate_identity())
            })
    });

    let candidate_set = PlanarBooleanLoopCandidateSet::new(
        loop_candidate_set_identity(&request_identity, &walk_outcome_set_identity, &candidates),
        request_identity.clone(),
        walk_outcome_set_identity.clone(),
        candidates,
    );
    let denied_set = PlanarBooleanDeniedLoopCandidateSet::new(
        denied_loop_candidate_set_identity(&request_identity, &walk_outcome_set_identity, &denied),
        request_identity,
        walk_outcome_set_identity,
        denied,
    );
    PlanarBooleanLoopCandidateBoundary::new(candidate_set, denied_set, counters)
}

fn promote_closed_walk(
    request_identity: &str,
    row: &PlanarBooleanWalkOutcomeRow,
) -> Result<PlanarBooleanLoopCandidate, PlanarBooleanDeniedLoopCandidate> {
    debug_assert_eq!(row.kind(), PlanarBooleanWalkOutcomeKind::Closed);
    if row.fragment_identities().len() < 2 {
        return Err(PlanarBooleanDeniedLoopCandidate::new(
            denied_loop_candidate_identity(
                request_identity,
                row.walk_outcome_identity(),
                row.source_loop_identity(),
                denied_kind_name(PlanarBooleanDeniedLoopCandidateKind::InsufficientCardinality),
                row.fragment_identities(),
            ),
            row.walk_outcome_identity().to_string(),
            row.source_loop_identity().to_string(),
            PlanarBooleanDeniedLoopCandidateKind::InsufficientCardinality,
            row.fragment_identities().to_vec(),
            row.split_vertex_identities().to_vec(),
            "loop candidate promotion requires at least two fragments in a closure-proven walk"
                .to_string(),
        ));
    }
    if row.source_face_identities().len() != 1
        || row.local_frame_identities().len() != 1
        || row.precision_basis_identities().len() != 1
    {
        return Err(PlanarBooleanDeniedLoopCandidate::new(
            denied_loop_candidate_identity(
                request_identity,
                row.walk_outcome_identity(),
                row.source_loop_identity(),
                denied_kind_name(PlanarBooleanDeniedLoopCandidateKind::LineageContradiction),
                row.fragment_identities(),
            ),
            row.walk_outcome_identity().to_string(),
            row.source_loop_identity().to_string(),
            PlanarBooleanDeniedLoopCandidateKind::LineageContradiction,
            row.fragment_identities().to_vec(),
            row.split_vertex_identities().to_vec(),
            "loop candidate promotion requires a single source face, local frame, and precision basis after closure proof"
                .to_string(),
        ));
    }
    Ok(PlanarBooleanLoopCandidate::new(
        loop_candidate_identity(
            request_identity,
            row.walk_outcome_identity(),
            row.source_loop_identity(),
            row.fragment_identities(),
        ),
        row.walk_outcome_identity().to_string(),
        row.source_loop_identity().to_string(),
        row.source_face_identities()[0].clone(),
        row.local_frame_identities()[0].clone(),
        row.precision_basis_identities()[0].clone(),
        row.source_senses().to_vec(),
        row.fragment_identities().to_vec(),
        row.split_vertex_identities().to_vec(),
    ))
}

fn denied_kind_name(kind: PlanarBooleanDeniedLoopCandidateKind) -> &'static str {
    match kind {
        PlanarBooleanDeniedLoopCandidateKind::LineageContradiction => "lineage-contradiction",
        PlanarBooleanDeniedLoopCandidateKind::InsufficientCardinality => "insufficient-cardinality",
    }
}
