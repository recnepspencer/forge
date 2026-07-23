use super::*;

pub(super) fn writeback_causality_match_count(
    loop_prevention: &BridgeWritebackLoopPreventionReport,
) -> usize {
    usize::from(
        loop_prevention
            .incoming_feedback_causality_digest()
            .is_some(),
    )
}

pub(super) struct WritebackExecutionCounterEvidence<'a> {
    pub(super) loop_prevention: &'a BridgeWritebackLoopPreventionReport,
    pub(super) outcome: Option<&'a BridgeWritebackAuthorityOutcome>,
    pub(super) error_kind: Option<BridgeWritebackErrorKind>,
    pub(super) candidate_present: bool,
    pub(super) mapper_record_present: bool,
    pub(super) request_present: bool,
    pub(super) receipt_present: bool,
    pub(super) replay_bundle_present: bool,
}

pub(super) fn writeback_execution_counters(
    evidence: WritebackExecutionCounterEvidence<'_>,
) -> BridgeWritebackCounters {
    let WritebackExecutionCounterEvidence {
        loop_prevention,
        outcome,
        error_kind,
        candidate_present,
        mapper_record_present,
        request_present,
        receipt_present,
        replay_bundle_present,
    } = evidence;
    debug_assert!(
        !receipt_present || request_present,
        "receipts should only exist for emitted requests"
    );
    debug_assert!(
        !replay_bundle_present || outcome.is_some(),
        "replay bundles should only exist for lowered outcomes"
    );
    let authority_boundary_observed = request_present || receipt_present;
    let (noop_count, commit_count) = classify_outcome_counts(loop_prevention, outcome);
    let (strategy_rejection_count, validation_rejection_count) =
        classify_rejection_counts(error_kind);

    BridgeWritebackCounters::new(
        1,
        1,
        usize::from(candidate_present),
        1 + usize::from(mapper_record_present),
        usize::from(authority_boundary_observed),
        1,
        1,
        strategy_rejection_count,
        1,
        writeback_causality_match_count(loop_prevention),
        1,
        usize::from(
            loop_prevention.disposition() == BridgeWritebackLoopDisposition::RejectAsUnsafeFeedback,
        ),
        noop_count,
        commit_count,
        usize::from(error_kind.is_some()),
        usize::from(matches!(
            error_kind,
            Some(BridgeWritebackErrorKind::AuthorityDenied)
        )),
        validation_rejection_count,
        0,
        0,
    )
}

fn classify_outcome_counts(
    loop_prevention: &BridgeWritebackLoopPreventionReport,
    outcome: Option<&BridgeWritebackAuthorityOutcome>,
) -> (usize, usize) {
    let noop = loop_prevention.disposition() == BridgeWritebackLoopDisposition::CanonicalNoop
        || outcome.is_some_and(|value| {
            value.outcome_class() == BridgeWritebackOutcomeClass::CanonicalNoop
        });
    let commit = outcome.is_some_and(|value| {
        value.outcome_class() == BridgeWritebackOutcomeClass::AuthoritativeCommit
    });
    (usize::from(noop), usize::from(commit))
}

fn classify_rejection_counts(error_kind: Option<BridgeWritebackErrorKind>) -> (usize, usize) {
    let strategy = matches!(
        error_kind,
        Some(
            BridgeWritebackErrorKind::StrategyUnavailable
                | BridgeWritebackErrorKind::FamilyBindingMismatch
                | BridgeWritebackErrorKind::StrategyDescriptorMismatch
        )
    );
    let validation = matches!(
        error_kind,
        Some(
            BridgeWritebackErrorKind::WritebackNotRequested
                | BridgeWritebackErrorKind::PolicyRejected
                | BridgeWritebackErrorKind::IdempotenceBasisMismatch
                | BridgeWritebackErrorKind::InvariantRejected
                | BridgeWritebackErrorKind::PreviewWritebackRejected
        )
    );
    (usize::from(strategy), usize::from(validation))
}

pub(super) fn writeback_replay_validation_counters(mismatch: bool) -> BridgeWritebackCounters {
    BridgeWritebackCounters::new(
        1,
        1,
        0,
        1,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        usize::from(mismatch),
        0,
        0,
        1,
        usize::from(mismatch),
    )
}
