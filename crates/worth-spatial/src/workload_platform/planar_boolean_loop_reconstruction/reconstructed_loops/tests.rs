use crate::workload_platform::planar_boolean_loop_reconstruction::test_support::{
    prepared_loop_continuation_subject, source_provenance_with_missing_fragment_membership,
    LoopFixtureEntryOrder,
};
use crate::workload_platform::planar_boolean_loop_reconstruction::{
    ComparePlanarBooleanLoopReconstructionReplay, PlanarBooleanClosedWalkCandidateAssembly,
    PlanarBooleanClosedWalkCandidateSetInput, PlanarBooleanFragmentContinuationIndex,
    PlanarBooleanFragmentContinuationIndexInput, PlanarBooleanLoopCandidateBoundary,
    PlanarBooleanLoopCandidateBoundaryInput, PlanarBooleanLoopIslandPartition,
    PlanarBooleanLoopIslandPartitionInput, PlanarBooleanLoopReconstructionReplayInput,
    PlanarBooleanReconstructedLoopBoundary, PlanarBooleanReconstructedLoopBoundaryDenialKind,
    PlanarBooleanReconstructedLoopBoundaryInput, PlanarBooleanSourceLoopSplitAttribution,
    PlanarBooleanSourceLoopSplitAttributionInput, PlanarBooleanWalkOutcomeSet,
    PlanarBooleanWalkOutcomeSetInput,
};

#[test]
fn reconstructed_loop_boundary_is_replay_stable_for_real_loop_candidates() {
    let canonical = prepared_loop_continuation_subject(LoopFixtureEntryOrder::Canonical);
    let replayed = prepared_loop_continuation_subject(LoopFixtureEntryOrder::Replayed);

    let canonical_products = reconstructed_products(&canonical);
    let replayed_products = reconstructed_products(&replayed);

    assert_eq!(
        canonical_products.boundary.reconstructed_loops().rows(),
        replayed_products.boundary.reconstructed_loops().rows()
    );
    assert_eq!(
        canonical_products.boundary.born_loops().rows(),
        replayed_products.boundary.born_loops().rows()
    );
    assert_eq!(
        canonical_products.partition.rows(),
        replayed_products.partition.rows()
    );
    assert_eq!(
        canonical_products.split_attribution.rows(),
        replayed_products.split_attribution.rows()
    );

    let replay = ComparePlanarBooleanLoopReconstructionReplay::compare(
        PlanarBooleanLoopReconstructionReplayInput::from_boundaries(
            &canonical_products.boundary,
            &replayed_products.boundary,
            &canonical_products.partition,
            &replayed_products.partition,
            &canonical_products.split_attribution,
            &replayed_products.split_attribution,
        ),
    )
    .expect("real loop reconstruction products should compare across replay");
    assert!(!replay.replay_identity().is_empty());
}

#[test]
fn reconstructed_loop_boundary_denies_missing_fragment_membership_before_islanding() {
    let prepared = prepared_loop_continuation_subject(LoopFixtureEntryOrder::Canonical);
    let boundary = loop_candidate_boundary(&prepared);
    let malformed_provenance =
        source_provenance_with_missing_fragment_membership(&prepared.source_provenance);

    let denial = PlanarBooleanReconstructedLoopBoundary::admit(
        PlanarBooleanReconstructedLoopBoundaryInput::from_loop_candidates_and_provenance(
            boundary.loop_candidates(),
            &malformed_provenance,
        ),
    )
    .expect_err("missing fragment memberships should deny reconstruction");

    assert_eq!(
        denial.kind(),
        PlanarBooleanReconstructedLoopBoundaryDenialKind::UntrackedBornLoopEmergence
    );
}

struct ReconstructedProducts {
    boundary: PlanarBooleanReconstructedLoopBoundary,
    partition: PlanarBooleanLoopIslandPartition,
    split_attribution: PlanarBooleanSourceLoopSplitAttribution,
}

fn reconstructed_products(
    prepared: &crate::workload_platform::planar_boolean_loop_reconstruction::test_support::PreparedLoopContinuationIndexSubject,
) -> ReconstructedProducts {
    let boundary = loop_candidate_boundary(prepared);
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
    let split_attribution = PlanarBooleanSourceLoopSplitAttribution::attribute(
        PlanarBooleanSourceLoopSplitAttributionInput::from_island_partition(&partition),
    );
    ReconstructedProducts {
        boundary: reconstructed,
        partition,
        split_attribution,
    }
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
