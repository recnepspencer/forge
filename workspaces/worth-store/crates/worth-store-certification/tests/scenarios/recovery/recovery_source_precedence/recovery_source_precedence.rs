use worth_store_physical_format::{
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId, PhysicalSegmentId,
};
use worth_store_physical_integrity::{
    ManifestIntegrityAuthority, ManifestIntegrityInspectionRequest, WalTailIntegrityPosture,
};
use worth_store_recovery_physics::{
    BackendResidueKind, BackendResidueRejection, CompactionArtifactResidueReason,
    CompactionCutoverRecoveryPosture, CompactionGenerationVisibility,
    CompactionVisibleProductEvidenceDenial, PageLsnSkipApplyDecision,
    RecoveryBlockedByIntegrityDamage, RecoverySourceApplicationRole, RecoverySourceCandidate,
    RecoverySourceDecisionKind, RecoverySourceDecisionOutcome, RecoverySourcePrecedenceGraph,
    WalOnlyTailProofDenial,
};

#[path = "assertions.rs"]
mod assertions;
use worth_store_test_support::harness::recovery::source_precedence as source_precedence_fixture;

use assertions::{
    assert_checkpoint_plus_tail, count_outcome, count_role, decision_rows, residue_kinds,
};
use source_precedence_fixture::{
    admitted_compaction_cutover_for_generation, checkpoint_base,
    compaction_cutover_basis_mismatch_denial, compaction_durability_artifact_mismatch_denial,
    compaction_durability_range_mismatch_denial, compaction_generation_mismatch_denial,
    compaction_visible_product_evidence, page_lsn, trace, wal_only_tail,
    wal_only_tail_denial_from_torn_frame, wal_tail_for_checkpoint,
};

#[test]
fn same_persisted_bytes_select_same_sources_under_same_profile() {
    let (checkpoint, receipt) = checkpoint_base(10, 20, 19, 1);
    let wal_tail = wal_tail_for_checkpoint(&receipt, 30, 2);
    let residue = residue(BackendResidueKind::BackendDirectoryResidue, "residue", 3);

    let first = RecoverySourcePrecedenceGraph::new("strict-test-profile")
        .discover(RecoverySourceCandidate::backend_residue(residue.clone()))
        .discover(RecoverySourceCandidate::wal_tail(wal_tail.clone()))
        .discover(RecoverySourceCandidate::checkpoint_base(checkpoint.clone()))
        .admit_sources();
    let second = RecoverySourcePrecedenceGraph::new("strict-test-profile")
        .discover(RecoverySourceCandidate::checkpoint_base(checkpoint.clone()))
        .discover(RecoverySourceCandidate::backend_residue(residue))
        .discover(RecoverySourceCandidate::wal_tail(wal_tail.clone()))
        .admit_sources();

    assert_eq!(first.trace().kind(), second.trace().kind());
    assert_checkpoint_plus_tail(&first, &checkpoint, &wal_tail);
    assert_checkpoint_plus_tail(&second, &checkpoint, &wal_tail);
    assert_eq!(first.trace().profile(), "strict-test-profile");
    assert_eq!(first.trace().candidate_count(), 3);
    assert_eq!(decision_rows(&first), decision_rows(&second));
    assert_eq!(residue_kinds(&first), residue_kinds(&second));
}

#[test]
fn stale_residue_orphans_invalid_compaction_and_physical_integrity_blocked_records_deny() {
    let invalid_compaction = CompactionCutoverRecoveryPosture::missing_generation_identity(trace(
        "invalid-compaction",
        3,
    ));
    let residue_only = RecoverySourcePrecedenceGraph::new("strict-test-profile")
        .discover(RecoverySourceCandidate::backend_residue(residue(
            BackendResidueKind::StalePageImage,
            "stale-page",
            1,
        )))
        .discover(RecoverySourceCandidate::orphaned_checkpoint_manifest(
            residue(
                BackendResidueKind::OrphanedCheckpointManifest,
                "orphaned-manifest",
                2,
            ),
        ))
        .discover(RecoverySourceCandidate::compaction_product(
            invalid_compaction,
            trace("invalid-compaction", 3),
        ))
        .admit_sources();

    assert_eq!(
        residue_only.trace().kind(),
        RecoverySourceDecisionKind::NoValidCheckpoint
    );
    assert_eq!(residue_only.trace().residue_rejections().len(), 3);
    assert_eq!(
        count_outcome(&residue_only, RecoverySourceDecisionOutcome::DiscoveryOnly),
        2
    );
    assert_eq!(
        count_outcome(
            &residue_only,
            RecoverySourceDecisionOutcome::RejectedResidue
        ),
        1
    );

    let blocked = RecoverySourcePrecedenceGraph::new("strict-test-profile")
        .discover(RecoverySourceCandidate::recovery_blocked(
            blocked_manifest_damage(),
            trace("new-blocked", 4),
        ))
        .admit_sources();

    assert_eq!(
        blocked.trace().kind(),
        RecoverySourceDecisionKind::RecoveryBlocked
    );
    assert_eq!(
        count_outcome(&blocked, RecoverySourceDecisionOutcome::RecoveryBlocked),
        1
    );
}

#[test]
fn checkpoint_plus_tail_wal_only_no_checkpoint_and_blocked_stay_distinct() {
    let (checkpoint, receipt) = checkpoint_base(10, 20, 19, 1);
    let checkpoint_plus_tail = RecoverySourcePrecedenceGraph::new("strict-test-profile")
        .discover(RecoverySourceCandidate::checkpoint_base(checkpoint))
        .discover(RecoverySourceCandidate::wal_tail(wal_tail_for_checkpoint(
            &receipt, 30, 2,
        )))
        .admit_sources();
    let wal_only = RecoverySourcePrecedenceGraph::new("strict-test-profile")
        .discover(RecoverySourceCandidate::wal_tail(wal_only_tail(1, 10, 1)))
        .admit_sources();
    let checkpoint_without_matching_tail =
        RecoverySourcePrecedenceGraph::new("strict-test-profile")
            .discover(RecoverySourceCandidate::checkpoint_base(
                checkpoint_base(10, 20, 19, 3).0,
            ))
            .discover(RecoverySourceCandidate::wal_tail(wal_only_tail(21, 30, 4)))
            .admit_sources();
    let no_valid = RecoverySourcePrecedenceGraph::new("strict-test-profile").admit_sources();
    let blocked = RecoverySourcePrecedenceGraph::new("strict-test-profile")
        .discover(RecoverySourceCandidate::recovery_blocked(
            blocked_manifest_damage(),
            trace("blocked", 1),
        ))
        .admit_sources();

    assert_eq!(
        checkpoint_plus_tail.trace().kind(),
        RecoverySourceDecisionKind::CheckpointPlusWalTail
    );
    assert!(checkpoint_plus_tail.selected_checkpoint().is_some());
    assert!(checkpoint_plus_tail.selected_wal_tail().is_some());
    assert_eq!(wal_only.trace().kind(), RecoverySourceDecisionKind::WalOnly);
    assert!(wal_only.selected_checkpoint().is_none());
    assert!(wal_only.selected_wal_tail().is_some());
    assert_eq!(
        no_valid.trace().kind(),
        RecoverySourceDecisionKind::NoValidCheckpoint
    );
    assert_eq!(
        checkpoint_without_matching_tail.trace().kind(),
        RecoverySourceDecisionKind::NoValidCheckpoint
    );
    assert_eq!(
        blocked.trace().kind(),
        RecoverySourceDecisionKind::RecoveryBlocked
    );
}

#[test]
fn roles_remain_separate_across_checkpoint_wal_page_and_residue() {
    let (checkpoint, receipt) = checkpoint_base(10, 20, 19, 1);
    let wal_tail = wal_tail_for_checkpoint(&receipt, 30, 2);
    let apply_decision = PageLsnSkipApplyDecision::decide(page_lsn(18), page_lsn(19));
    let skip_decision = PageLsnSkipApplyDecision::decide(page_lsn(20), page_lsn(19));
    let admitted = RecoverySourcePrecedenceGraph::new("strict-test-profile")
        .discover(RecoverySourceCandidate::orphaned_checkpoint_manifest(
            residue(
                BackendResidueKind::OrphanedCheckpointManifest,
                "orphaned-manifest",
                4,
            ),
        ))
        .discover(RecoverySourceCandidate::page_image(
            apply_decision,
            trace("page-apply", 3),
        ))
        .discover(RecoverySourceCandidate::page_image(
            skip_decision,
            trace("page-skip", 5),
        ))
        .discover(RecoverySourceCandidate::checkpoint_base(checkpoint.clone()))
        .discover(RecoverySourceCandidate::wal_tail(wal_tail.clone()))
        .admit_sources();

    assert_eq!(
        admitted.trace().kind(),
        RecoverySourceDecisionKind::CheckpointPlusWalTail
    );
    assert_checkpoint_plus_tail(&admitted, &checkpoint, &wal_tail);
    assert!(admitted
        .trace()
        .roles()
        .contains(&RecoverySourceApplicationRole::PageSkipApply));
    assert!(admitted
        .trace()
        .roles()
        .contains(&RecoverySourceApplicationRole::ResidueDiscoveryOnly));
    assert_eq!(
        count_role(&admitted, RecoverySourceApplicationRole::PageSkipApply),
        2
    );
    assert!(matches!(
        apply_decision,
        PageLsnSkipApplyDecision::ApplyRedo { .. }
    ));
    assert!(matches!(
        skip_decision,
        PageLsnSkipApplyDecision::SkipAlreadyApplied { .. }
    ));
}

#[test]
fn admitted_checkpoint_and_wal_tail_beat_hostile_residue_candidates() {
    let (checkpoint, receipt) = checkpoint_base(10, 20, 19, 1);
    let wal_tail = wal_tail_for_checkpoint(&receipt, 30, 2);
    let clean = RecoverySourcePrecedenceGraph::new("strict-test-profile")
        .discover(RecoverySourceCandidate::checkpoint_base(checkpoint.clone()))
        .discover(RecoverySourceCandidate::wal_tail(wal_tail.clone()))
        .admit_sources();
    let invalid_compaction = CompactionCutoverRecoveryPosture::missing_generation_identity(trace(
        "invalid-compaction",
        3,
    ));
    let hostile = RecoverySourcePrecedenceGraph::new("strict-test-profile")
        .discover(RecoverySourceCandidate::backend_residue(residue(
            BackendResidueKind::StalePageImage,
            "stale-page",
            4,
        )))
        .discover(RecoverySourceCandidate::orphaned_checkpoint_manifest(
            residue(
                BackendResidueKind::OrphanedCheckpointManifest,
                "orphaned-manifest",
                5,
            ),
        ))
        .discover(RecoverySourceCandidate::compaction_product(
            invalid_compaction,
            trace("invalid-compaction", 3),
        ))
        .discover(RecoverySourceCandidate::checkpoint_base(checkpoint.clone()))
        .discover(RecoverySourceCandidate::wal_tail(wal_tail.clone()))
        .admit_sources();

    assert_checkpoint_plus_tail(&clean, &checkpoint, &wal_tail);
    assert_checkpoint_plus_tail(&hostile, &checkpoint, &wal_tail);
    assert_eq!(hostile.trace().residue_rejections().len(), 3);
    assert_eq!(
        count_outcome(&hostile, RecoverySourceDecisionOutcome::AdmittedCandidate),
        2
    );
    assert_eq!(
        count_outcome(&hostile, RecoverySourceDecisionOutcome::DiscoveryOnly),
        2
    );
    assert_eq!(
        count_outcome(&hostile, RecoverySourceDecisionOutcome::RejectedResidue),
        1
    );
}

#[test]
fn compaction_visibility_requires_generation_cutover_recoverability_and_durability() {
    let evidence = compaction_visible_product_evidence(7);
    let visible = CompactionCutoverRecoveryPosture::admit_visible_product(evidence);
    let (generation, cutover) = admitted_compaction_cutover_for_generation(7);
    let rejected = CompactionCutoverRecoveryPosture::old_generation_not_recoverable(
        generation,
        cutover,
        trace("rejected-compaction", 2),
    );

    assert!(visible.is_visible());
    assert!(matches!(
        rejected.visibility(),
        CompactionGenerationVisibility::ResidueRejected(rejection)
            if rejection.reason() == CompactionArtifactResidueReason::OldGenerationNotRecoverable
    ));
    assert!(matches!(
        compaction_generation_mismatch_denial(),
        CompactionVisibleProductEvidenceDenial::GenerationMismatch { expected, observed }
            if expected.generation() == 7 && observed.generation() == 8
    ));
    assert_eq!(
        compaction_cutover_basis_mismatch_denial(),
        CompactionVisibleProductEvidenceDenial::CutoverBasisMismatch
    );
    assert_eq!(
        compaction_durability_artifact_mismatch_denial(),
        CompactionVisibleProductEvidenceDenial::CutoverDurabilityArtifactMismatch
    );
    assert_eq!(
        compaction_durability_range_mismatch_denial(),
        CompactionVisibleProductEvidenceDenial::CutoverDurabilityRangeMismatch
    );
}

#[test]
fn wal_only_tail_rejects_torn_physical_integrity_tail_posture() {
    let denial = wal_only_tail_denial_from_torn_frame(1, 10);

    assert_eq!(
        denial,
        WalOnlyTailProofDenial::BlockedByWalTailIntegrity {
            posture: WalTailIntegrityPosture::TornTail
        }
    );
}

fn residue(kind: BackendResidueKind, label: &str, order: u64) -> BackendResidueRejection {
    BackendResidueRejection::new(kind, trace(label, order))
}

fn blocked_manifest_damage() -> RecoveryBlockedByIntegrityDamage {
    let denial = ManifestIntegrityAuthority::new()
        .inspect_manifest(ManifestIntegrityInspectionRequest::damaged_root(
            damaged_owner(),
        ))
        .unwrap_err();
    RecoveryBlockedByIntegrityDamage::damaged_manifest_root(&denial)
}

fn damaged_owner() -> worth_store_physical_format::PhysicalGenerationOwner {
    PhysicalGenerationAuthority::for_canonical_physical_format()
        .page_cell(
            PhysicalSegmentId::from_raw(1).unwrap(),
            PhysicalPageId::from_raw(1).unwrap(),
        )
        .with_page_generation(PhysicalGeneration::from_raw(1).unwrap())
        .owner()
}
