use crate::workload_platform::planar_boolean_loop_reconstruction::test_support::{
    prepared_loop_continuation_subject, LoopFixtureEntryOrder,
};
use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanClosedWalkCandidateAssembly, PlanarBooleanClosedWalkCandidateSetInput,
    PlanarBooleanFragmentContinuationIndex, PlanarBooleanFragmentContinuationIndexInput,
    PlanarBooleanLoopCandidateBoundary, PlanarBooleanLoopCandidateBoundaryInput,
    PlanarBooleanLoopIslandKind, PlanarBooleanLoopIslandPartition,
    PlanarBooleanLoopIslandPartitionInput, PlanarBooleanReconstructedLoopBoundary,
    PlanarBooleanReconstructedLoopBoundaryInput, PlanarBooleanWalkOutcomeSet,
    PlanarBooleanWalkOutcomeSetInput,
};

#[test]
fn island_partition_emits_loop_level_rows_for_reconstructed_products() {
    let prepared = prepared_loop_continuation_subject(LoopFixtureEntryOrder::Canonical);
    let boundary = loop_candidate_boundary(&prepared);
    let reconstructed = PlanarBooleanReconstructedLoopBoundary::admit(
        PlanarBooleanReconstructedLoopBoundaryInput::from_loop_candidates_and_provenance(
            boundary.loop_candidates(),
            &prepared.source_provenance,
        ),
    )
    .expect("prepared loop continuation subject should reconstruct");

    let partition = PlanarBooleanLoopIslandPartition::partition(
        PlanarBooleanLoopIslandPartitionInput::from_reconstructed_loop_boundary(
            reconstructed.reconstructed_loops(),
            reconstructed.born_loops(),
        ),
    );

    assert_eq!(partition.counters().reconstructed_loops_consumed(), 1);
    assert_eq!(partition.counters().island_rows_emitted(), 1);
    let row = partition
        .rows()
        .first()
        .expect("one island row should exist");
    assert_eq!(row.kind(), PlanarBooleanLoopIslandKind::PreservedSourceLoop);
    assert_eq!(row.member_loop_identities().len(), 1);
}

fn loop_candidate_boundary(
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
