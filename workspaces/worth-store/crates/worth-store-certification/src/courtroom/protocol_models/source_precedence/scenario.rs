use worth_store_formal_models::{map_recovery_source_decision_trace, SourcePrecedenceAction};
use worth_store_physical_format::{
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId, PhysicalSegmentId,
};
use worth_store_physical_integrity::{
    ManifestIntegrityAuthority, ManifestIntegrityInspectionRequest,
};
use worth_store_recovery_physics::{
    BackendResidueKind, BackendResidueRejection, CompactionCutoverRecoveryPosture,
    PageLsnSkipApplyDecision, RecoveryBlockedByIntegrityDamage, RecoverySourceCandidate,
    RecoverySourcePrecedenceGraph,
};
use worth_store_test_support::harness::recovery::{
    redo_replay::{checkpoint_plus_tail_source, page_lsn},
    source_precedence::{trace, wal_only_tail},
};

pub(in crate::courtroom::protocol_models) fn execute_ordinary_source_precedence(
) -> Vec<SourcePrecedenceAction> {
    execute_ordinary_source_precedence_traces()
        .into_iter()
        .flatten()
        .collect()
}

pub(in crate::courtroom::protocol_models) fn execute_ordinary_source_precedence_traces(
) -> Vec<Vec<SourcePrecedenceAction>> {
    vec![
        map_recovery_source_decision_trace(checkpoint_plus_tail_source(20, 21).trace()),
        map_recovery_source_decision_trace(residue_and_invalid_compaction().trace()),
        map_recovery_source_decision_trace(page_advisory_source().trace()),
        map_recovery_source_decision_trace(blocked_recovery("strict-test-profile").trace()),
        map_recovery_source_decision_trace(
            RecoverySourcePrecedenceGraph::new("strict-test-profile")
                .admit_sources()
                .trace(),
        ),
    ]
}

fn page_advisory_source() -> worth_store_recovery_physics::AdmittedRecoverySource {
    RecoverySourcePrecedenceGraph::new("strict-test-profile")
        .discover(RecoverySourceCandidate::page_image(
            PageLsnSkipApplyDecision::decide(page_lsn(18), page_lsn(19)),
            trace("page-advisory", 1),
        ))
        .discover(RecoverySourceCandidate::wal_tail(wal_only_tail(1, 10, 2)))
        .admit_sources()
}

fn residue_and_invalid_compaction() -> worth_store_recovery_physics::AdmittedRecoverySource {
    RecoverySourcePrecedenceGraph::new("strict-test-profile")
        .discover(RecoverySourceCandidate::backend_residue(
            BackendResidueRejection::new(
                BackendResidueKind::StalePageImage,
                trace("stale-page", 1),
            ),
        ))
        .discover(RecoverySourceCandidate::compaction_product(
            CompactionCutoverRecoveryPosture::missing_generation_identity(trace(
                "invalid-compaction",
                2,
            )),
            trace("invalid-compaction", 2),
        ))
        .discover(RecoverySourceCandidate::wal_tail(wal_only_tail(1, 10, 3)))
        .admit_sources()
}

pub(in crate::courtroom::protocol_models) fn replay_quarantined_source_guard(
    seed: u64,
) -> Vec<SourcePrecedenceAction> {
    let profile = format!("counterexample-replay-{seed}");
    let blocked = blocked_recovery(&profile);
    assert!(matches!(
        blocked,
        worth_store_recovery_physics::AdmittedRecoverySource::RecoveryBlocked { .. }
    ));
    assert!(blocked.selected_checkpoint().is_none());
    assert!(blocked.selected_wal_tail().is_none());
    map_recovery_source_decision_trace(blocked.trace())
}

fn blocked_recovery(profile: &str) -> worth_store_recovery_physics::AdmittedRecoverySource {
    RecoverySourcePrecedenceGraph::new(profile)
        .discover(RecoverySourceCandidate::recovery_blocked(
            blocked_manifest_damage(),
            trace("blocked", 1),
        ))
        .admit_sources()
}

fn blocked_manifest_damage() -> RecoveryBlockedByIntegrityDamage {
    let denial = ManifestIntegrityAuthority::new()
        .inspect_manifest(ManifestIntegrityInspectionRequest::damaged_root(
            PhysicalGenerationAuthority::for_canonical_physical_format()
                .page_cell(
                    PhysicalSegmentId::from_raw(1).unwrap(),
                    PhysicalPageId::from_raw(1).unwrap(),
                )
                .with_page_generation(PhysicalGeneration::from_raw(1).unwrap())
                .owner(),
        ))
        .unwrap_err();
    RecoveryBlockedByIntegrityDamage::damaged_manifest_root(&denial)
}
