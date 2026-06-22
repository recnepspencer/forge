use crate::workload_platform::planar_boolean_loop_reconstruction::test_support::{
    prepared_loop_continuation_subject, LoopFixtureEntryOrder,
};
use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanClosedWalkCandidateAssembly, PlanarBooleanClosedWalkCandidateSetInput,
    PlanarBooleanFragmentContinuationIndex, PlanarBooleanFragmentContinuationIndexInput,
    PlanarBooleanLoopCandidateBoundary, PlanarBooleanLoopCandidateBoundaryInput,
    PlanarBooleanWalkOutcomeSet, PlanarBooleanWalkOutcomeSetInput,
};

#[test]
fn closed_walks_promote_into_replay_stable_loop_candidates() {
    let canonical = prepared_loop_continuation_subject(LoopFixtureEntryOrder::Canonical);
    let replayed = prepared_loop_continuation_subject(LoopFixtureEntryOrder::Replayed);

    let canonical_boundary = promote(&canonical);
    let replayed_boundary = promote(&replayed);

    assert_eq!(
        canonical_boundary
            .loop_candidates()
            .loop_candidate_set_identity(),
        replayed_boundary
            .loop_candidates()
            .loop_candidate_set_identity()
    );
    assert_eq!(
        canonical_boundary.loop_candidates().rows(),
        replayed_boundary.loop_candidates().rows()
    );
    assert_eq!(
        canonical_boundary.denied_loop_candidates().rows(),
        replayed_boundary.denied_loop_candidates().rows()
    );
    assert_eq!(canonical_boundary.counters().closed_walks_considered(), 1);
    assert_eq!(canonical_boundary.counters().loop_candidates_promoted(), 1);
    assert_eq!(
        canonical_boundary
            .counters()
            .denied_loop_candidates_emitted(),
        0
    );

    let row = canonical_boundary
        .loop_candidates()
        .rows()
        .first()
        .expect("prepared fixture should promote one loop candidate");
    assert!(!row.fragment_identities().is_empty());
    assert!(!row.split_vertex_identities().is_empty());
}

fn promote(
    prepared: &crate::workload_platform::planar_boolean_loop_reconstruction::test_support::PreparedLoopContinuationIndexSubject,
) -> PlanarBooleanLoopCandidateBoundary {
    let index = PlanarBooleanFragmentContinuationIndex::admit(
        PlanarBooleanFragmentContinuationIndexInput::from_request_and_provenance(
            &prepared.request,
            &prepared.source_provenance,
            &prepared.subject.vertices,
            &prepared.subject.fragments,
            &prepared.subject.overlap_chains,
        ),
    )
    .expect("prepared continuation subject should admit");
    let assembly = PlanarBooleanClosedWalkCandidateAssembly::assemble(
        PlanarBooleanClosedWalkCandidateSetInput::from_continuation_index(&index),
    );
    let outcomes = PlanarBooleanWalkOutcomeSet::classify(
        PlanarBooleanWalkOutcomeSetInput::from_closed_walk_candidates(
            assembly.closed_walk_candidates(),
            assembly.fragment_consumption_proof(),
        ),
    );
    PlanarBooleanLoopCandidateBoundary::promote(
        PlanarBooleanLoopCandidateBoundaryInput::from_walk_outcomes(&outcomes),
    )
}
