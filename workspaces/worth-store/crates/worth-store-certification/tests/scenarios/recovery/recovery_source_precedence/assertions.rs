use worth_store_recovery_physics::{
    AdmittedRecoverySource, BackendResidueKind, CheckpointBaseAdmission,
    RecoverySourceApplicationRole, RecoverySourceDecisionKind, RecoverySourceDecisionOutcome,
    WalTailRedoSource,
};

pub(crate) fn assert_checkpoint_plus_tail(
    admitted: &AdmittedRecoverySource,
    checkpoint: &CheckpointBaseAdmission,
    wal_tail: &WalTailRedoSource,
) {
    assert_eq!(
        admitted.trace().kind(),
        RecoverySourceDecisionKind::CheckpointPlusWalTail
    );
    assert_eq!(
        admitted.selected_checkpoint().unwrap().checkpoint_id(),
        checkpoint.checkpoint_id()
    );
    assert_eq!(
        admitted.selected_checkpoint().unwrap().covered_lsn_range(),
        checkpoint.covered_lsn_range()
    );
    assert_eq!(
        admitted.selected_wal_tail().unwrap().checkpoint_id(),
        wal_tail.checkpoint_id()
    );
    assert_eq!(
        admitted.selected_wal_tail().unwrap().lsn_range(),
        wal_tail.lsn_range()
    );
}

pub(crate) fn decision_rows(
    admitted: &AdmittedRecoverySource,
) -> Vec<(
    String,
    u64,
    RecoverySourceApplicationRole,
    RecoverySourceDecisionOutcome,
)> {
    let mut rows = admitted
        .trace()
        .decision_rows()
        .iter()
        .map(|row| {
            (
                row.trace().physical_basis().to_owned(),
                row.trace().discovery_order(),
                row.role(),
                row.outcome(),
            )
        })
        .collect::<Vec<_>>();
    rows.sort();
    rows
}

pub(crate) fn residue_kinds(admitted: &AdmittedRecoverySource) -> Vec<BackendResidueKind> {
    let mut kinds = admitted
        .trace()
        .residue_rejections()
        .iter()
        .map(|rejection| rejection.kind())
        .collect::<Vec<_>>();
    kinds.sort();
    kinds
}

pub(crate) fn count_outcome(
    admitted: &AdmittedRecoverySource,
    outcome: RecoverySourceDecisionOutcome,
) -> usize {
    admitted
        .trace()
        .decision_rows()
        .iter()
        .filter(|row| row.outcome() == outcome)
        .count()
}

pub(crate) fn count_role(
    admitted: &AdmittedRecoverySource,
    role: RecoverySourceApplicationRole,
) -> usize {
    admitted
        .trace()
        .decision_rows()
        .iter()
        .filter(|row| row.role() == role)
        .count()
}
