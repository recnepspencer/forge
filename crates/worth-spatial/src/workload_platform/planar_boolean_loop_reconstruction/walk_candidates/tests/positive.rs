use crate::workload_platform::planar_boolean_loop_reconstruction::test_support::{
    prepared_loop_continuation_subject, LoopFixtureEntryOrder,
};
use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanClosedWalkCandidateAssembly, PlanarBooleanClosedWalkCandidateSetInput,
    PlanarBooleanFragmentContinuationIndex, PlanarBooleanFragmentContinuationIndexInput,
};

#[test]
fn closed_walk_candidate_assembly_preserves_connected_components_and_proof_replay_stability() {
    let canonical = prepared_loop_continuation_subject(LoopFixtureEntryOrder::Canonical);
    let replayed = prepared_loop_continuation_subject(LoopFixtureEntryOrder::Replayed);

    let canonical_assembly = assemble(&canonical);
    let replayed_assembly = assemble(&replayed);

    assert_eq!(
        canonical_assembly
            .closed_walk_candidates()
            .closed_walk_candidate_set_identity(),
        replayed_assembly
            .closed_walk_candidates()
            .closed_walk_candidate_set_identity()
    );
    assert_eq!(
        canonical_assembly.closed_walk_candidates().rows(),
        replayed_assembly.closed_walk_candidates().rows()
    );
    assert_eq!(
        canonical_assembly.fragment_consumption_proof(),
        replayed_assembly.fragment_consumption_proof()
    );
    assert_eq!(
        canonical_assembly
            .closed_walk_candidates()
            .counters()
            .walk_candidates_assembled(),
        1
    );
}

fn assemble(
    prepared: &crate::workload_platform::planar_boolean_loop_reconstruction::test_support::PreparedLoopContinuationIndexSubject,
) -> PlanarBooleanClosedWalkCandidateAssembly {
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
    PlanarBooleanClosedWalkCandidateAssembly::assemble(
        PlanarBooleanClosedWalkCandidateSetInput::from_continuation_index(&index),
    )
}
