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

pub(super) fn writeback_execution_counters(
    loop_prevention: &BridgeWritebackLoopPreventionReport,
    outcome: Option<&BridgeWritebackAuthorityOutcome>,
    error_kind: Option<BridgeWritebackErrorKind>,
    candidate_present: bool,
    mapper_record_present: bool,
    request_present: bool,
    receipt_present: bool,
    replay_bundle_present: bool,
) -> BridgeWritebackCounters {
    debug_assert!(
        !receipt_present || request_present,
        "receipts should only exist for emitted requests"
    );
    debug_assert!(
        !replay_bundle_present || outcome.is_some(),
        "replay bundles should only exist for lowered outcomes"
    );
    let authority_boundary_observed = request_present || receipt_present;
    let noop_count = usize::from(
        loop_prevention.disposition() == BridgeWritebackLoopDisposition::CanonicalNoop
            || outcome.is_some_and(|value| {
                value.outcome_class() == BridgeWritebackOutcomeClass::CanonicalNoop
            }),
    );
    let commit_count = usize::from(outcome.is_some_and(|value| {
        value.outcome_class() == BridgeWritebackOutcomeClass::AuthoritativeCommit
    }));
    let strategy_rejection_count = usize::from(matches!(
        error_kind,
        Some(
            BridgeWritebackErrorKind::StrategyUnavailable
                | BridgeWritebackErrorKind::FamilyBindingMismatch
                | BridgeWritebackErrorKind::StrategyDescriptorMismatch
        )
    ));
    let validation_rejection_count = usize::from(matches!(
        error_kind,
        Some(
            BridgeWritebackErrorKind::WritebackNotRequested
                | BridgeWritebackErrorKind::PolicyRejected
                | BridgeWritebackErrorKind::IdempotenceBasisMismatch
                | BridgeWritebackErrorKind::InvariantRejected
                | BridgeWritebackErrorKind::PreviewWritebackRejected
        )
    ));

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
