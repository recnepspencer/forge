use crate::workload_platform::planar_boolean_loop_reconstruction::test_support::{
    prepared_loop_continuation_subject, source_provenance_without_first_source_loop_carrier,
    LoopFixtureEntryOrder,
};
use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanClosedWalkCandidateAssembly, PlanarBooleanClosedWalkCandidateSetInput,
    PlanarBooleanFragmentContinuationIndex, PlanarBooleanFragmentContinuationIndexInput,
    PlanarBooleanLoopCandidateBoundary, PlanarBooleanLoopCandidateBoundaryInput,
    PlanarBooleanLoopContainmentEvidencePostureKind, PlanarBooleanLoopIslandPartition,
    PlanarBooleanLoopIslandPartitionInput, PlanarBooleanLoopRoleOutcomeBoundary,
    PlanarBooleanLoopRoleOutcomeBoundaryInput, PlanarBooleanLoopRoleOutcomeKind,
    PlanarBooleanReconstructedLoopBoundary, PlanarBooleanReconstructedLoopBoundaryInput,
    PlanarBooleanSourceLoopSplitAttribution, PlanarBooleanSourceLoopSplitAttributionInput,
    PlanarBooleanWalkOutcomeSet, PlanarBooleanWalkOutcomeSetInput,
};

#[test]
fn loop_role_outcomes_are_replay_stable_for_real_loop_products() {
    let canonical = prepared_loop_continuation_subject(LoopFixtureEntryOrder::Canonical);
    let replayed = prepared_loop_continuation_subject(LoopFixtureEntryOrder::Replayed);

    let canonical_role_boundary = role_products(&canonical);
    let replayed_role_boundary = role_products(&replayed);

    assert_eq!(
        canonical_role_boundary.role_outcomes().rows(),
        replayed_role_boundary.role_outcomes().rows()
    );
    assert_eq!(
        canonical_role_boundary
            .containment_evidence_postures()
            .rows(),
        replayed_role_boundary
            .containment_evidence_postures()
            .rows()
    );
}

#[test]
fn born_loops_emit_typed_role_outcomes_before_degeneracy_when_present() {
    let prepared = prepared_loop_continuation_subject(LoopFixtureEntryOrder::Canonical);
    let reconstructed = reconstructed_products(&prepared);
    let role_boundary = PlanarBooleanLoopRoleOutcomeBoundary::classify(
        PlanarBooleanLoopRoleOutcomeBoundaryInput::from_reconstructed_loop_products_and_provenance(
            &reconstructed.boundary,
            &reconstructed.partition,
            &reconstructed.split_attribution,
            &prepared.source_provenance,
        ),
    );

    assert_eq!(
        role_boundary.role_outcomes().rows().len(),
        reconstructed.boundary.reconstructed_loops().rows().len()
            + reconstructed.boundary.born_loops().rows().len()
    );

    for born_loop in reconstructed.boundary.born_loops().rows() {
        let born_role_outcome = role_boundary
            .role_outcomes()
            .rows()
            .iter()
            .find(|row| row.loop_identity() == born_loop.born_loop_identity())
            .expect("every born loop should receive a typed role outcome");
        if born_loop.source_loop_identities().len() > 1 {
            assert_eq!(
                born_role_outcome.kind(),
                PlanarBooleanLoopRoleOutcomeKind::BornLoopRoleAmbiguous
            );
            assert!(born_role_outcome.preserved_source_role().is_none());
        }

        let born_containment_posture = role_boundary
            .containment_evidence_postures()
            .rows()
            .iter()
            .find(|row| row.loop_identity() == born_loop.born_loop_identity())
            .expect("every born loop should receive containment posture");
        if born_loop.source_loop_identities().len() > 1 {
            assert_eq!(
                born_containment_posture.kind(),
                PlanarBooleanLoopContainmentEvidencePostureKind::MultiSourceBornLoopContainmentEvidence
            );
        }
    }
}

#[test]
fn missing_source_role_evidence_becomes_typed_role_outcome() {
    let prepared = prepared_loop_continuation_subject(LoopFixtureEntryOrder::Canonical);
    let reconstructed = reconstructed_products(&prepared);
    let malformed_provenance =
        source_provenance_without_first_source_loop_carrier(&prepared.source_provenance);

    let role_boundary = PlanarBooleanLoopRoleOutcomeBoundary::classify(
        PlanarBooleanLoopRoleOutcomeBoundaryInput::from_reconstructed_loop_products_and_provenance(
            &reconstructed.boundary,
            &reconstructed.partition,
            &reconstructed.split_attribution,
            &malformed_provenance,
        ),
    );

    assert!(role_boundary
        .role_outcomes()
        .rows()
        .iter()
        .any(|row| row.kind() == PlanarBooleanLoopRoleOutcomeKind::MissingSourceRoleEvidence));
    assert!(role_boundary
        .containment_evidence_postures()
        .rows()
        .iter()
        .any(|row| {
            row.kind()
                == PlanarBooleanLoopContainmentEvidencePostureKind::MissingSourceContainmentEvidence
        }));
}

struct ReconstructedProducts {
    boundary: PlanarBooleanReconstructedLoopBoundary,
    partition: PlanarBooleanLoopIslandPartition,
    split_attribution: PlanarBooleanSourceLoopSplitAttribution,
}

fn role_products(
    prepared: &crate::workload_platform::planar_boolean_loop_reconstruction::test_support::PreparedLoopContinuationIndexSubject,
) -> PlanarBooleanLoopRoleOutcomeBoundary {
    let reconstructed = reconstructed_products(prepared);
    PlanarBooleanLoopRoleOutcomeBoundary::classify(
        PlanarBooleanLoopRoleOutcomeBoundaryInput::from_reconstructed_loop_products_and_provenance(
            &reconstructed.boundary,
            &reconstructed.partition,
            &reconstructed.split_attribution,
            &prepared.source_provenance,
        ),
    )
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
