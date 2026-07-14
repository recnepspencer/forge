use super::{
    AdmittedRecoverySource, CheckpointBaseAdmission, RecoverySourceDecisionKind,
    RecoverySourceDecisionTrace, RecoverySourceReplayBasis, WalTailRedoSource,
};

use super::source_admission_accumulator::RecoverySourceSelectionInput;

pub(super) fn select_admitted_recovery_source(
    profile: String,
    input: RecoverySourceSelectionInput,
) -> AdmittedRecoverySource {
    let RecoverySourceSelectionInput {
        candidate_count,
        mut checkpoint_bases,
        mut wal_tails,
        roles,
        residue_rejections,
        decision_rows,
        recovery_blocked,
    } = input;

    if let Some(damage) = recovery_blocked {
        let trace = RecoverySourceDecisionTrace::new(
            RecoverySourceDecisionKind::RecoveryBlocked,
            profile,
            candidate_count,
            roles,
            residue_rejections,
            decision_rows,
            RecoverySourceReplayBasis::empty(),
        );
        return AdmittedRecoverySource::RecoveryBlocked { damage, trace };
    }

    checkpoint_bases.sort_by(checkpoint_base_order);
    wal_tails.sort_by_key(|tail| tail.lsn_range());

    if let Some(checkpoint) = checkpoint_bases.into_iter().next() {
        return select_checkpoint_based_source(
            profile,
            candidate_count,
            roles,
            residue_rejections,
            decision_rows,
            checkpoint,
            wal_tails,
        );
    }

    if let Some(wal_tail) = wal_tails
        .into_iter()
        .find(|wal_tail| wal_tail.checkpoint_id().is_none())
    {
        let trace = RecoverySourceDecisionTrace::new(
            RecoverySourceDecisionKind::WalOnly,
            profile,
            candidate_count,
            roles,
            residue_rejections,
            decision_rows,
            RecoverySourceReplayBasis::wal_only(wal_tail.lsn_range()),
        );
        return AdmittedRecoverySource::WalOnly { wal_tail, trace };
    }

    no_valid_checkpoint(
        profile,
        candidate_count,
        roles,
        residue_rejections,
        decision_rows,
    )
}

fn select_checkpoint_based_source(
    profile: String,
    candidate_count: usize,
    roles: Vec<super::RecoverySourceApplicationRole>,
    residue_rejections: Vec<super::BackendResidueRejection>,
    decision_rows: Vec<super::RecoverySourceDecisionRow>,
    checkpoint: CheckpointBaseAdmission,
    mut wal_tails: Vec<WalTailRedoSource>,
) -> AdmittedRecoverySource {
    if let Some(wal_tail_index) = matching_wal_tail_index(&checkpoint, &wal_tails) {
        let wal_tail = wal_tails.remove(wal_tail_index);
        let trace = RecoverySourceDecisionTrace::new(
            RecoverySourceDecisionKind::CheckpointPlusWalTail,
            profile,
            candidate_count,
            roles,
            residue_rejections,
            decision_rows,
            RecoverySourceReplayBasis::checkpoint_plus_tail(
                checkpoint.checkpoint_id().clone(),
                wal_tail.lsn_range(),
            ),
        );
        return AdmittedRecoverySource::CheckpointPlusWalTail {
            checkpoint,
            wal_tail,
            trace,
        };
    }

    no_valid_checkpoint(
        profile,
        candidate_count,
        roles,
        residue_rejections,
        decision_rows,
    )
}

fn no_valid_checkpoint(
    profile: String,
    candidate_count: usize,
    roles: Vec<super::RecoverySourceApplicationRole>,
    residue_rejections: Vec<super::BackendResidueRejection>,
    decision_rows: Vec<super::RecoverySourceDecisionRow>,
) -> AdmittedRecoverySource {
    let trace = RecoverySourceDecisionTrace::new(
        RecoverySourceDecisionKind::NoValidCheckpoint,
        profile,
        candidate_count,
        roles,
        residue_rejections,
        decision_rows,
        RecoverySourceReplayBasis::empty(),
    );
    AdmittedRecoverySource::NoValidCheckpoint { trace }
}

fn checkpoint_base_order(
    left: &CheckpointBaseAdmission,
    right: &CheckpointBaseAdmission,
) -> std::cmp::Ordering {
    left.checkpoint_id()
        .digest()
        .as_str()
        .cmp(right.checkpoint_id().digest().as_str())
        .then_with(|| left.covered_lsn_range().cmp(&right.covered_lsn_range()))
}

fn matching_wal_tail_index(
    checkpoint: &CheckpointBaseAdmission,
    wal_tails: &[WalTailRedoSource],
) -> Option<usize> {
    wal_tails
        .iter()
        .position(|wal_tail| wal_tail.checkpoint_id() == Some(checkpoint.checkpoint_id()))
}
