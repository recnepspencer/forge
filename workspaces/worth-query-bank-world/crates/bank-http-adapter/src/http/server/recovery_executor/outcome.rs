use bank_server::{BankMutationCommitOutcome, BankRecoveryPosture, BankUndoCorrection};

use super::super::super::protocol::{
    BankHttpCommitDescription, BankHttpDenial, BankHttpDenialKind,
    BankHttpEstateDisbursementOutcome, BankHttpEstateNotificationOutcome, BankHttpNextAction,
    BankHttpRecoveryInspectionOutcome, BankHttpRecoveryPosture, BankHttpRedoProgressionOutcome,
    BankHttpUndoAdmissionOutcome, BankHttpUndoCorrection, BankHttpUndoProgressionOutcome,
};

pub(super) fn commit_description(
    receipt: &bank_server::BankCommitReceipt,
) -> BankHttpCommitDescription {
    BankHttpCommitDescription {
        changed_record_count: receipt.changed_record_count(),
        emitted_effect_count: receipt.emitted_effect_count(),
        expected_version_count: receipt.expected_version_count(),
        expected_fact_count: receipt.expected_fact_count(),
        provider_work_units: None,
    }
}

pub(super) const fn recovery_posture(posture: BankRecoveryPosture) -> BankHttpRecoveryPosture {
    match posture {
        BankRecoveryPosture::Reversible => BankHttpRecoveryPosture::Reversible,
        BankRecoveryPosture::Compensatable => BankHttpRecoveryPosture::Compensatable,
        BankRecoveryPosture::Reconcilable => BankHttpRecoveryPosture::Reconcilable,
        BankRecoveryPosture::Irreversible => BankHttpRecoveryPosture::Irreversible,
    }
}

pub(super) const fn undo_correction(correction: BankUndoCorrection) -> BankHttpUndoCorrection {
    match correction {
        BankUndoCorrection::Compensation => BankHttpUndoCorrection::Compensation,
        BankUndoCorrection::RecordedInverse => BankHttpUndoCorrection::RecordedInverse,
        BankUndoCorrection::Reconciliation => BankHttpUndoCorrection::Reconciliation,
    }
}

pub(super) fn commit_denial(outcome: BankMutationCommitOutcome) -> BankHttpDenial {
    match outcome {
        BankMutationCommitOutcome::Stale { .. } => stale(),
        BankMutationCommitOutcome::Cancelled => {
            BankHttpDenial::new(BankHttpDenialKind::Cancelled, BankHttpNextAction::Retry)
        }
        BankMutationCommitOutcome::Denied { .. } | BankMutationCommitOutcome::Aborted => {
            unavailable()
        }
        BankMutationCommitOutcome::PartialEffect(_)
        | BankMutationCommitOutcome::Indeterminate(_) => BankHttpDenial::new(
            BankHttpDenialKind::Unavailable,
            BankHttpNextAction::ContactOperator,
        ),
        BankMutationCommitOutcome::Committed(_)
        | BankMutationCommitOutcome::AlreadyCommitted(_) => unavailable(),
    }
}

pub(super) use super::super::estate_denial::estate_denial;

pub(super) fn notification_denied(
    request_id: Option<String>,
    denial: BankHttpDenial,
) -> BankHttpEstateNotificationOutcome {
    BankHttpEstateNotificationOutcome::Denied { request_id, denial }
}

pub(super) fn inspection_denied(
    request_id: Option<String>,
    denial: BankHttpDenial,
) -> BankHttpRecoveryInspectionOutcome {
    BankHttpRecoveryInspectionOutcome::Denied { request_id, denial }
}

pub(super) fn undo_denied(
    request_id: Option<String>,
    denial: BankHttpDenial,
) -> BankHttpUndoAdmissionOutcome {
    BankHttpUndoAdmissionOutcome::Denied { request_id, denial }
}

pub(super) fn disbursement_denied(
    request_id: Option<String>,
    denial: BankHttpDenial,
) -> BankHttpEstateDisbursementOutcome {
    BankHttpEstateDisbursementOutcome::Denied { request_id, denial }
}

pub(super) fn undo_progression_denied(
    request_id: Option<String>,
    denial: BankHttpDenial,
) -> BankHttpUndoProgressionOutcome {
    BankHttpUndoProgressionOutcome::Denied { request_id, denial }
}

pub(super) fn redo_progression_denied(
    request_id: Option<String>,
    denial: BankHttpDenial,
) -> BankHttpRedoProgressionOutcome {
    BankHttpRedoProgressionOutcome::Denied { request_id, denial }
}

pub(super) const fn stale() -> BankHttpDenial {
    BankHttpDenial::new(BankHttpDenialKind::Stale, BankHttpNextAction::Refresh)
}

pub(super) const fn saturated() -> BankHttpDenial {
    BankHttpDenial::new(BankHttpDenialKind::Saturated, BankHttpNextAction::Retry)
}

pub(super) const fn unavailable() -> BankHttpDenial {
    BankHttpDenial::new(BankHttpDenialKind::Unavailable, BankHttpNextAction::Retry)
}

pub(super) const fn deadline_exceeded() -> BankHttpDenial {
    BankHttpDenial::new(
        BankHttpDenialKind::DeadlineExceeded,
        BankHttpNextAction::Retry,
    )
}

pub(super) const fn conflicting_idempotency_key() -> BankHttpDenial {
    BankHttpDenial::new(
        BankHttpDenialKind::MalformedRequest,
        BankHttpNextAction::CorrectRequest,
    )
}
