use crate::workload_platform::planar_boolean_loop_reconstruction::test_support::{
    prepared_loop_continuation_subject, LoopFixtureEntryOrder, PreparedLoopContinuationIndexSubject,
};
use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanClosedWalkCandidateAssembly, PlanarBooleanClosedWalkCandidateSetInput,
    PlanarBooleanDegenerateLoopOutcomeBoundary, PlanarBooleanDegenerateLoopOutcomeBoundaryInput,
    PlanarBooleanDegenerateLoopOutcomeKind, PlanarBooleanDeniedLoopCandidateSet,
    PlanarBooleanFragmentContinuationIndex, PlanarBooleanFragmentContinuationIndexInput,
    PlanarBooleanLoopCandidateBoundary, PlanarBooleanLoopCandidateBoundaryInput,
    PlanarBooleanLoopIdentityBoundary, PlanarBooleanLoopIdentityMintingInput,
    PlanarBooleanLoopIslandPartition, PlanarBooleanLoopIslandPartitionInput,
    PlanarBooleanLoopNamingAuthoritySupport, PlanarBooleanLoopRoleOutcomeBoundary,
    PlanarBooleanLoopRoleOutcomeBoundaryInput, PlanarBooleanReconstructedLoopBoundary,
    PlanarBooleanReconstructedLoopBoundaryInput, PlanarBooleanSourceLoopSplitAttribution,
    PlanarBooleanSourceLoopSplitAttributionInput, PlanarBooleanWalkOutcomeSet,
    PlanarBooleanWalkOutcomeSetInput,
};

#[test]
fn loop_identity_boundary_is_replay_stable_for_real_phase_twelve_products() {
    let canonical = prepared_loop_continuation_subject(LoopFixtureEntryOrder::Canonical);
    let replayed = prepared_loop_continuation_subject(LoopFixtureEntryOrder::Replayed);

    let canonical_products = real_phase_twelve_products(&canonical);
    let replayed_products = real_phase_twelve_products(&replayed);
    let canonical_support =
        PlanarBooleanLoopNamingAuthoritySupport::admit_from_split_receipt_and_provenance(
            &canonical.subject.naming,
            &canonical.source_provenance,
            &canonical_products.split_attribution,
        )
        .expect("real fixture should lower naming authority support");
    let replayed_support =
        PlanarBooleanLoopNamingAuthoritySupport::admit_from_split_receipt_and_provenance(
            &replayed.subject.naming,
            &replayed.source_provenance,
            &replayed_products.split_attribution,
        )
        .expect("replayed fixture should lower naming authority support");

    let canonical_boundary = PlanarBooleanLoopIdentityBoundary::mint(
        PlanarBooleanLoopIdentityMintingInput::from_phase_twelve_products_and_naming_support(
            canonical_products.reconstructed.reconstructed_loops(),
            canonical_products.reconstructed.born_loops(),
            canonical_products.role_boundary.role_outcomes(),
            canonical_products.degenerate_boundary.outcomes(),
            &canonical_products.denied_loop_candidates,
            &canonical_support,
            &canonical_products.split_attribution,
        ),
    )
    .expect("real phase-twelve products should lower into phase thirteen");
    let replayed_boundary = PlanarBooleanLoopIdentityBoundary::mint(
        PlanarBooleanLoopIdentityMintingInput::from_phase_twelve_products_and_naming_support(
            replayed_products.reconstructed.reconstructed_loops(),
            replayed_products.reconstructed.born_loops(),
            replayed_products.role_boundary.role_outcomes(),
            replayed_products.degenerate_boundary.outcomes(),
            &replayed_products.denied_loop_candidates,
            &replayed_support,
            &replayed_products.split_attribution,
        ),
    )
    .expect("replayed phase-twelve products should lower into phase thirteen");

    assert_eq!(
        canonical_boundary.loop_identity_map().rows(),
        replayed_boundary.loop_identity_map().rows()
    );
    assert_eq!(
        canonical_boundary.persistent_name_propagation_map().rows(),
        replayed_boundary.persistent_name_propagation_map().rows()
    );
    assert_eq!(
        canonical_boundary.subshape_signature_map().rows(),
        replayed_boundary.subshape_signature_map().rows()
    );
    assert_eq!(canonical_boundary.loop_identity_map().rows().len(), 0);
    assert_eq!(
        canonical_boundary.counters().loop_identities_minted(),
        admitted_for_identity_minting_count(&canonical_products.degenerate_boundary)
    );
}

struct RealPhaseTwelveProducts {
    reconstructed: PlanarBooleanReconstructedLoopBoundary,
    role_boundary: PlanarBooleanLoopRoleOutcomeBoundary,
    degenerate_boundary: PlanarBooleanDegenerateLoopOutcomeBoundary,
    split_attribution: PlanarBooleanSourceLoopSplitAttribution,
    denied_loop_candidates: PlanarBooleanDeniedLoopCandidateSet,
}

fn real_phase_twelve_products(
    prepared: &PreparedLoopContinuationIndexSubject,
) -> RealPhaseTwelveProducts {
    let candidate_boundary = loop_candidate_boundary(prepared);
    let reconstructed = PlanarBooleanReconstructedLoopBoundary::admit(
        PlanarBooleanReconstructedLoopBoundaryInput::from_loop_candidates_and_provenance(
            candidate_boundary.loop_candidates(),
            &prepared.source_provenance,
        ),
    )
    .expect("real fixture should reconstruct loops");
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
    let degenerate_boundary = PlanarBooleanDegenerateLoopOutcomeBoundary::classify(
        PlanarBooleanDegenerateLoopOutcomeBoundaryInput::from_reconstructed_products_and_role_evidence(
            reconstructed.reconstructed_loops(),
            reconstructed.born_loops(),
            role_boundary.role_outcomes(),
            role_boundary.containment_evidence_postures(),
            prepared.source_provenance.source_loop_carriers(),
            &prepared.subject.fragments,
        ),
    );
    RealPhaseTwelveProducts {
        reconstructed,
        role_boundary,
        degenerate_boundary,
        split_attribution,
        denied_loop_candidates: candidate_boundary.denied_loop_candidates().clone(),
    }
}

fn loop_candidate_boundary(
    prepared: &PreparedLoopContinuationIndexSubject,
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

fn admitted_for_identity_minting_count(
    boundary: &PlanarBooleanDegenerateLoopOutcomeBoundary,
) -> usize {
    boundary
        .outcomes()
        .rows()
        .iter()
        .filter(|row| {
            row.kind() == PlanarBooleanDegenerateLoopOutcomeKind::AdmittedForIdentityMinting
        })
        .count()
}
