use crate::workload_platform::planar_boolean_loop_reconstruction::test_support::{
    prepared_loop_continuation_subject, LoopFixtureEntryOrder,
};
use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanClosedWalkCandidateAssembly, PlanarBooleanClosedWalkCandidateSetInput,
    PlanarBooleanDegenerateLoopOutcomeBoundary, PlanarBooleanDegenerateLoopOutcomeBoundaryInput,
    PlanarBooleanFragmentContinuationIndex, PlanarBooleanFragmentContinuationIndexInput,
    PlanarBooleanLoopCandidateBoundary, PlanarBooleanLoopCandidateBoundaryInput,
    PlanarBooleanLoopIslandPartition, PlanarBooleanLoopIslandPartitionInput,
    PlanarBooleanLoopRoleOutcomeBoundary, PlanarBooleanLoopRoleOutcomeBoundaryInput,
    PlanarBooleanReconstructedLoopBoundary, PlanarBooleanReconstructedLoopBoundaryInput,
    PlanarBooleanSourceLoopSplitAttribution, PlanarBooleanSourceLoopSplitAttributionInput,
    PlanarBooleanWalkOutcomeSet, PlanarBooleanWalkOutcomeSetInput,
};

#[test]
fn degenerate_loop_outcomes_are_replay_stable_for_real_loop_products() {
    let canonical = prepared_loop_continuation_subject(LoopFixtureEntryOrder::Canonical);
    let replayed = prepared_loop_continuation_subject(LoopFixtureEntryOrder::Replayed);

    let canonical_boundary = degenerate_boundary(&canonical);
    let replayed_boundary = degenerate_boundary(&replayed);

    assert_eq!(
        canonical_boundary.outcomes().rows(),
        replayed_boundary.outcomes().rows()
    );
}

#[test]
fn real_loops_receive_typed_degenerate_outcomes_before_identity_minting() {
    let prepared = prepared_loop_continuation_subject(LoopFixtureEntryOrder::Canonical);
    let products = role_products(&prepared);
    let boundary = PlanarBooleanDegenerateLoopOutcomeBoundary::classify(
        PlanarBooleanDegenerateLoopOutcomeBoundaryInput::from_reconstructed_products_and_role_evidence(
            products.reconstructed.reconstructed_loops(),
            products.reconstructed.born_loops(),
            products.role_boundary.role_outcomes(),
            products.role_boundary.containment_evidence_postures(),
            &products.source_loop_carriers,
            &products.split_fragments,
        ),
    );

    assert_eq!(
        boundary.outcomes().rows().len(),
        products.reconstructed.reconstructed_loops().rows().len()
            + products.reconstructed.born_loops().rows().len()
    );
    assert_eq!(
        boundary.counters().loops_consumed(),
        boundary.outcomes().rows().len()
    );
    assert!(boundary.outcomes().rows().iter().all(|row| {
        !row.degenerate_loop_outcome_identity().is_empty()
            && !row.local_frame_identity().is_empty()
            && !row.precision_basis_identity().is_empty()
    }));
}

struct RoleProducts {
    reconstructed: PlanarBooleanReconstructedLoopBoundary,
    role_boundary: PlanarBooleanLoopRoleOutcomeBoundary,
    source_loop_carriers:
        crate::workload_platform::planar_boolean_loop_reconstruction::PlanarBooleanLoopSourceCarrierSet,
    split_fragments:
        crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanSplitEdgeFragmentSet,
}

fn degenerate_boundary(
    prepared: &crate::workload_platform::planar_boolean_loop_reconstruction::test_support::PreparedLoopContinuationIndexSubject,
) -> PlanarBooleanDegenerateLoopOutcomeBoundary {
    let products = role_products(prepared);
    PlanarBooleanDegenerateLoopOutcomeBoundary::classify(
        PlanarBooleanDegenerateLoopOutcomeBoundaryInput::from_reconstructed_products_and_role_evidence(
            products.reconstructed.reconstructed_loops(),
            products.reconstructed.born_loops(),
            products.role_boundary.role_outcomes(),
            products.role_boundary.containment_evidence_postures(),
            &products.source_loop_carriers,
            &products.split_fragments,
        ),
    )
}

fn role_products(
    prepared: &crate::workload_platform::planar_boolean_loop_reconstruction::test_support::PreparedLoopContinuationIndexSubject,
) -> RoleProducts {
    let reconstructed = reconstructed_products(prepared);
    let partition = PlanarBooleanLoopIslandPartition::partition(
        PlanarBooleanLoopIslandPartitionInput::from_reconstructed_loop_boundary(
            reconstructed.reconstructed_loops(),
            reconstructed.born_loops(),
        ),
    );
    let split_attribution = PlanarBooleanSourceLoopSplitAttribution::attribute(
        PlanarBooleanSourceLoopSplitAttributionInput::from_island_partition(&partition),
    );
    let role_boundary = PlanarBooleanLoopRoleOutcomeBoundary::classify(
        PlanarBooleanLoopRoleOutcomeBoundaryInput::from_reconstructed_loop_products_and_provenance(
            &reconstructed,
            &partition,
            &split_attribution,
            &prepared.source_provenance,
        ),
    );
    RoleProducts {
        reconstructed,
        role_boundary,
        source_loop_carriers: prepared.source_provenance.source_loop_carriers().clone(),
        split_fragments: prepared.subject.fragments.clone(),
    }
}

fn reconstructed_products(
    prepared: &crate::workload_platform::planar_boolean_loop_reconstruction::test_support::PreparedLoopContinuationIndexSubject,
) -> PlanarBooleanReconstructedLoopBoundary {
    let boundary = loop_candidate_boundary(prepared);
    PlanarBooleanReconstructedLoopBoundary::admit(
        PlanarBooleanReconstructedLoopBoundaryInput::from_loop_candidates_and_provenance(
            boundary.loop_candidates(),
            &prepared.source_provenance,
        ),
    )
    .expect("prepared loop continuation subject should reconstruct")
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
