use worth_store_physical_backend::SimulatedStrictDurableProfile;
use worth_store_physical_integrity::{
    ManifestIntegrityAuthority, ManifestIntegrityInspectionRequest,
};
use worth_store_recovery_physics::{
    CheckpointArtifactDurabilityCommitment, CheckpointCandidate,
    CheckpointCandidateDiscoverySource, CheckpointCutoverReceipt,
    CheckpointCutoverRecoverySelection, CheckpointCutoverRecoverySelectionKind,
    CheckpointDurabilityEvidenceSet, CheckpointManifest, CheckpointPublicationPlan,
    CheckpointRootPosture, CheckpointValidation, CheckpointValidationDenialKind,
    ContiguousWalTailProof, IntegrityDamageMap, RecoveredCheckpointCutoverState,
    RecoveryBlockedByIntegrityDamage, SharpCheckpointCertificationMode,
};

use worth_store_test_support::harness::recovery::checkpoint_basis as checkpoint_basis_fixture;
use worth_store_test_support::harness::recovery::checkpoint_durability as checkpoint_durability_fixture;

use checkpoint_basis_fixture::{
    covered_range, frontier, manifest, page_cell, redo_boundary, wal_range,
};
use checkpoint_durability_fixture::{
    checkpoint_durability, durable_ack_for_digest, locate, recovered_locator, validate,
};

#[test]
fn equivalent_checkpoint_manifests_validate_to_same_checkpoint_identity() {
    let first = manifest(10, 20, 19);
    let second = manifest(10, 20, 19);

    let first_validation = validate(first);
    let second_validation = validate(second);

    assert_eq!(
        first_validation.checkpoint_id(),
        second_validation.checkpoint_id()
    );
}

#[test]
fn torn_missing_stale_and_recovery_blocking_checkpoint_inputs_deny() {
    assert_denial(
        CheckpointManifest::torn_manifest(),
        CheckpointValidationDenialKind::TornManifest,
    );
    assert_denial(
        CheckpointManifest::sharp(
            CheckpointRootPosture::MissingRoot,
            frontier(19),
            covered_range(10, 20),
            redo_boundary(19),
            SharpCheckpointCertificationMode::certified(),
        ),
        CheckpointValidationDenialKind::MissingRoot,
    );
    assert_denial(
        CheckpointManifest::sharp(
            CheckpointRootPosture::root_present(checkpoint_basis_fixture::root_record_reference()),
            frontier(12),
            covered_range(10, 20),
            redo_boundary(19),
            SharpCheckpointCertificationMode::certified(),
        ),
        CheckpointValidationDenialKind::StalePageLsnFrontier,
    );

    let located = locate(manifest(10, 20, 19));
    let manifest_damage = ManifestIntegrityAuthority::new()
        .inspect_manifest(ManifestIntegrityInspectionRequest::damaged_root(
            page_cell().owner(),
        ))
        .unwrap_err();
    let damage_map = IntegrityDamageMap::new()
        .with_manifest_root_damage(RecoveryBlockedByIntegrityDamage::damaged_manifest_root(
            &manifest_damage,
        ))
        .unwrap();
    let denial =
        CheckpointValidation::validate_located_checkpoint(located, &damage_map).unwrap_err();

    assert_eq!(
        denial.kind(),
        CheckpointValidationDenialKind::RecoveryBlockingIntegrityDamage
    );
}

#[test]
fn discovered_candidates_require_store_owned_locator() {
    for source in [
        CheckpointCandidateDiscoverySource::DirectoryListing,
        CheckpointCandidateDiscoverySource::BackendResidue,
        CheckpointCandidateDiscoverySource::OrphanedManifest,
    ] {
        let candidate = CheckpointCandidate::from_manifest(manifest(10, 20, 19), source);
        let denial = CheckpointValidation::require_locator(candidate).unwrap_err();
        assert_eq!(
            denial.kind(),
            CheckpointValidationDenialKind::MissingCheckpointLocator
        );
    }
}

#[test]
fn cutover_crash_selection_is_deterministic() {
    let validation = validate(manifest(10, 20, 19));
    let durability = checkpoint_durability(&validation);
    let plan = CheckpointPublicationPlan::<SimulatedStrictDurableProfile>::plan_cutover(
        validation,
        durability.clone(),
    )
    .unwrap();
    let receipt = CheckpointCutoverReceipt::publish(plan);
    let wrong_root = RecoveredCheckpointCutoverState::admit_selected_during_cutover(
        receipt.clone(),
        recovered_locator(manifest(10, 20, 19)),
        durability.manifest(),
        durability.manifest(),
        durability.page_lsn_frontier(),
    )
    .unwrap_err();
    let wrong_locator = RecoveredCheckpointCutoverState::admit_selected_during_cutover(
        receipt.clone(),
        recovered_locator(manifest(20, 30, 29)),
        durability.root(),
        durability.manifest(),
        durability.page_lsn_frontier(),
    )
    .unwrap_err();

    let before = CheckpointCutoverRecoverySelection::from_recovered_state(
        RecoveredCheckpointCutoverState::before_cutover(),
    );
    let during_without_selector = CheckpointCutoverRecoverySelection::from_recovered_state(
        RecoveredCheckpointCutoverState::during_cutover_without_durable_selector(),
    );
    let during = CheckpointCutoverRecoverySelection::from_recovered_state(
        RecoveredCheckpointCutoverState::admit_selected_during_cutover(
            receipt.clone(),
            recovered_locator(manifest(10, 20, 19)),
            durability.root(),
            durability.manifest(),
            durability.page_lsn_frontier(),
        )
        .unwrap(),
    );
    let after = CheckpointCutoverRecoverySelection::from_recovered_state(
        RecoveredCheckpointCutoverState::admit_selected_after_cutover(
            receipt.clone(),
            recovered_locator(manifest(10, 20, 19)),
            durability.root(),
            durability.manifest(),
            durability.page_lsn_frontier(),
        )
        .unwrap(),
    );

    assert_eq!(
        before.kind(),
        CheckpointCutoverRecoverySelectionKind::NoValidCheckpoint
    );
    assert_eq!(
        during_without_selector.kind(),
        CheckpointCutoverRecoverySelectionKind::NoValidCheckpoint
    );
    assert_eq!(
        wrong_root.kind(),
        CheckpointValidationDenialKind::RecoveredCheckpointEvidenceMismatch
    );
    assert_eq!(
        wrong_locator.kind(),
        CheckpointValidationDenialKind::RecoveredCheckpointEvidenceMismatch
    );
    assert_eq!(during.checkpoint_id(), Some(receipt.checkpoint_id()));
    assert_eq!(after.checkpoint_id(), Some(receipt.checkpoint_id()));
}

#[test]
fn cutover_durability_requires_distinct_artifact_role_evidence() {
    let validation = validate(manifest(10, 20, 19));
    let range = validation.manifest().covered_lsn_range().range();
    let manifest_ack = durable_ack_for_digest(
        range,
        1,
        CheckpointArtifactDurabilityCommitment::manifest(&validation).digest(),
    );

    let denial = CheckpointDurabilityEvidenceSet::admit(
        &validation,
        &manifest_ack,
        &manifest_ack,
        &manifest_ack,
        &manifest_ack,
    )
    .unwrap_err();

    assert_eq!(
        denial.kind(),
        CheckpointValidationDenialKind::CutoverDurabilityArtifactMismatch
    );

    let wrong_root_ack = durable_ack_for_digest(
        range,
        2,
        CheckpointArtifactDurabilityCommitment::manifest(&validation).digest(),
    );
    let root_denial = CheckpointDurabilityEvidenceSet::admit(
        &validation,
        &manifest_ack,
        &wrong_root_ack,
        &durable_ack_for_digest(
            range,
            3,
            CheckpointArtifactDurabilityCommitment::page_lsn_frontier(&validation).digest(),
        ),
        &durable_ack_for_digest(
            range,
            4,
            CheckpointArtifactDurabilityCommitment::locator(&validation).digest(),
        ),
    )
    .unwrap_err();

    assert_eq!(
        root_denial.kind(),
        CheckpointValidationDenialKind::CutoverDurabilityArtifactMismatch
    );

    let locator_denial = CheckpointDurabilityEvidenceSet::admit(
        &validation,
        &manifest_ack,
        &durable_ack_for_digest(
            range,
            2,
            CheckpointArtifactDurabilityCommitment::root(&validation).digest(),
        ),
        &durable_ack_for_digest(
            range,
            3,
            CheckpointArtifactDurabilityCommitment::page_lsn_frontier(&validation).digest(),
        ),
        &durable_ack_for_digest(
            range,
            4,
            CheckpointArtifactDurabilityCommitment::manifest(&validation).digest(),
        ),
    )
    .unwrap_err();

    assert_eq!(
        locator_denial.kind(),
        CheckpointValidationDenialKind::CutoverDurabilityArtifactMismatch
    );
}

#[test]
fn sharp_mode_certifies_and_fuzzy_attempts_deny_explicitly() {
    let sharp = manifest(10, 20, 19);
    assert_eq!(sharp.redo_boundary(), redo_boundary(19));

    let fuzzy = worth_store_recovery_physics::FuzzyCheckpointCertificationModeDenial::missing_begin_end_records();
    assert_eq!(
        fuzzy.kind(),
        worth_store_recovery_physics::FuzzyCheckpointCertificationModeDenialKind::MissingBeginEndRecords
    );
    assert_eq!(
        CheckpointManifest::fuzzy_checkpoint_attempt(fuzzy)
            .unwrap_err()
            .kind(),
        CheckpointValidationDenialKind::FuzzyCheckpointModeUnsupported
    );
}

#[test]
fn recovery_tail_requires_the_exact_checkpoint_boundary() {
    let validation = validate(manifest(10, 20, 19));
    let durability = checkpoint_durability(&validation);
    let plan = CheckpointPublicationPlan::<SimulatedStrictDurableProfile>::plan_cutover(
        validation, durability,
    )
    .unwrap();
    let receipt = CheckpointCutoverReceipt::publish(plan);
    let tail = ContiguousWalTailProof::prove(&receipt, wal_range(20, 30)).unwrap();
    assert_eq!(tail.tail_range(), wal_range(20, 30));
    assert_eq!(
        ContiguousWalTailProof::prove(&receipt, wal_range(21, 30))
            .unwrap_err()
            .kind(),
        CheckpointValidationDenialKind::WalRetentionWithoutContiguousTail
    );
}

fn assert_denial(
    result: Result<CheckpointManifest, worth_store_recovery_physics::CheckpointValidationDenial>,
    kind: CheckpointValidationDenialKind,
) {
    assert_eq!(result.unwrap_err().kind(), kind);
}
