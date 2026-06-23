use crate::workload_platform::planar_boolean_loop_reconstruction::test_support::{
    prepared_loop_continuation_subject, LoopFixtureEntryOrder,
};
use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanClosedWalkCandidateAssembly, PlanarBooleanClosedWalkCandidateSetInput,
    PlanarBooleanFragmentContinuationIndex, PlanarBooleanFragmentContinuationIndexInput,
    PlanarBooleanWalkOutcomeKind, PlanarBooleanWalkOutcomeSet, PlanarBooleanWalkOutcomeSetInput,
};

#[test]
fn walk_outcomes_classify_real_continuations_as_closed_with_replay_stable_identity() {
    let canonical = prepared_loop_continuation_subject(LoopFixtureEntryOrder::Canonical);
    let replayed = prepared_loop_continuation_subject(LoopFixtureEntryOrder::Replayed);

    let canonical_outcomes = classify(&canonical);
    let replayed_outcomes = classify(&replayed);

    assert_eq!(
        canonical_outcomes.walk_outcome_set_identity(),
        replayed_outcomes.walk_outcome_set_identity()
    );
    assert_eq!(canonical_outcomes.rows(), replayed_outcomes.rows());
    assert_eq!(canonical_outcomes.counters(), replayed_outcomes.counters());
    assert_eq!(canonical_outcomes.counters().walks_classified(), 1);
    assert_eq!(canonical_outcomes.counters().closed_walks(), 1);

    let row = canonical_outcomes
        .rows()
        .first()
        .expect("prepared fixture should classify exactly one source loop");
    assert_eq!(row.kind(), PlanarBooleanWalkOutcomeKind::Closed);
    assert!(!row.fragment_identities().is_empty());
    assert!(!row.split_vertex_identities().is_empty());
    assert!(!row.continuation_identities().is_empty());
    assert_eq!(row.source_face_identities().len(), 1);
    assert_eq!(row.local_frame_identities().len(), 1);
    assert_eq!(row.precision_basis_identities().len(), 1);
}

fn classify(
    prepared: &crate::workload_platform::planar_boolean_loop_reconstruction::test_support::PreparedLoopContinuationIndexSubject,
) -> PlanarBooleanWalkOutcomeSet {
    let index = PlanarBooleanFragmentContinuationIndex::admit(
        PlanarBooleanFragmentContinuationIndexInput::from_request_and_provenance(
            &prepared.request,
            &prepared.source_provenance,
            &prepared.subject.vertices,
            &prepared.subject.fragments,
            &prepared.subject.overlap_chains,
        ),
    )
    .expect("prepared continuation subject should admit a real continuation index");
    let assembly = PlanarBooleanClosedWalkCandidateAssembly::assemble(
        PlanarBooleanClosedWalkCandidateSetInput::from_continuation_index(&index),
    );
    PlanarBooleanWalkOutcomeSet::classify(
        PlanarBooleanWalkOutcomeSetInput::from_closed_walk_candidates(
            assembly.closed_walk_candidates(),
            assembly.fragment_consumption_proof(),
        ),
    )
}
