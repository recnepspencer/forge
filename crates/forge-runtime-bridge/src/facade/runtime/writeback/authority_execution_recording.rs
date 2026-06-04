use super::authority_failure_mapping::{
    map_writeback_error_kind_to_failure_class, writeback_failure_digest,
};
use super::execution_counters::writeback_execution_counters;
use super::*;

pub(super) struct WritebackAuthorityExecutionContext<'a> {
    contract: &'a AdmittedBridgeWritebackContract,
    effect: &'a BridgeDerivedWritebackEffect,
    idempotence: &'a BridgeWritebackIdempotenceBasis,
    loop_prevention: &'a BridgeWritebackLoopPreventionReport,
    strategy_coherence: &'a BridgeWritebackStrategyCoherenceReport,
}

impl<'a> WritebackAuthorityExecutionContext<'a> {
    pub(super) fn new(
        contract: &'a AdmittedBridgeWritebackContract,
        effect: &'a BridgeDerivedWritebackEffect,
        idempotence: &'a BridgeWritebackIdempotenceBasis,
        loop_prevention: &'a BridgeWritebackLoopPreventionReport,
        strategy_coherence: &'a BridgeWritebackStrategyCoherenceReport,
    ) -> Self {
        Self {
            contract,
            effect,
            idempotence,
            loop_prevention,
            strategy_coherence,
        }
    }
}

pub(super) fn canonical_noop_record(
    context: &WritebackAuthorityExecutionContext<'_>,
    outcome: &BridgeWritebackAuthorityOutcome,
    replay_bundle: &BridgeWritebackReplayBundle,
) -> BridgeWritebackExecutionRecord {
    BridgeWritebackExecutionRecord::new(
        context.contract,
        context.effect,
        context.idempotence,
        context.loop_prevention,
        context.strategy_coherence,
        None,
        None,
        Some(outcome),
        Some(replay_bundle),
        None,
        None,
        None::<std::sync::Arc<str>>,
        None,
        None::<std::sync::Arc<str>>,
        writeback_execution_counters(
            context.loop_prevention,
            Some(outcome),
            None,
            false,
            false,
            false,
            false,
            true,
        ),
    )
}

pub(super) fn blocked_before_candidate_record(
    context: &WritebackAuthorityExecutionContext<'_>,
    error: &BridgeWritebackError,
) -> BridgeWritebackExecutionRecord {
    BridgeWritebackExecutionRecord::new(
        context.contract,
        context.effect,
        context.idempotence,
        context.loop_prevention,
        context.strategy_coherence,
        None,
        None,
        None,
        None,
        None,
        None,
        None::<std::sync::Arc<str>>,
        Some(map_writeback_error_kind_to_failure_class(error.kind())),
        Some(writeback_failure_digest(
            error,
            context.contract,
            context.effect,
            context.idempotence,
        )),
        writeback_execution_counters(
            context.loop_prevention,
            None,
            Some(error.kind()),
            false,
            false,
            false,
            false,
            false,
        ),
    )
}

pub(super) fn blocked_before_authority_record(
    context: &WritebackAuthorityExecutionContext<'_>,
    mapper_record: &BridgeWritebackMapperRecord,
    candidate: &BridgeValidatedWritebackCandidate,
    error: &BridgeWritebackError,
) -> BridgeWritebackExecutionRecord {
    BridgeWritebackExecutionRecord::new(
        context.contract,
        context.effect,
        context.idempotence,
        context.loop_prevention,
        context.strategy_coherence,
        Some(mapper_record),
        Some(candidate),
        None,
        None,
        None,
        None,
        None::<std::sync::Arc<str>>,
        Some(map_writeback_error_kind_to_failure_class(error.kind())),
        Some(writeback_failure_digest(
            error,
            context.contract,
            context.effect,
            context.idempotence,
        )),
        writeback_execution_counters(
            context.loop_prevention,
            None,
            Some(error.kind()),
            true,
            true,
            false,
            false,
            false,
        ),
    )
}

pub(super) fn request_dispatch_failure_record(
    context: &WritebackAuthorityExecutionContext<'_>,
    mapper_record: &BridgeWritebackMapperRecord,
    candidate: &BridgeValidatedWritebackCandidate,
    request: &TruthWritebackRequest,
    error: &BridgeWritebackError,
) -> BridgeWritebackExecutionRecord {
    BridgeWritebackExecutionRecord::new(
        context.contract,
        context.effect,
        context.idempotence,
        context.loop_prevention,
        context.strategy_coherence,
        Some(mapper_record),
        Some(candidate),
        None,
        None,
        Some(request),
        None,
        None::<std::sync::Arc<str>>,
        Some(map_writeback_error_kind_to_failure_class(error.kind())),
        Some(writeback_failure_digest(
            error,
            context.contract,
            context.effect,
            context.idempotence,
        )),
        writeback_execution_counters(
            context.loop_prevention,
            None,
            Some(error.kind()),
            true,
            true,
            true,
            false,
            false,
        ),
    )
}

pub(super) fn validated_receipt_failure_record(
    context: &WritebackAuthorityExecutionContext<'_>,
    mapper_record: &BridgeWritebackMapperRecord,
    candidate: &BridgeValidatedWritebackCandidate,
    request: &TruthWritebackRequest,
    receipt: &TruthWritebackReceipt,
    error: &BridgeWritebackError,
) -> BridgeWritebackExecutionRecord {
    BridgeWritebackExecutionRecord::new(
        context.contract,
        context.effect,
        context.idempotence,
        context.loop_prevention,
        context.strategy_coherence,
        Some(mapper_record),
        Some(candidate),
        None,
        None,
        Some(request),
        Some(receipt),
        None::<std::sync::Arc<str>>,
        Some(map_writeback_error_kind_to_failure_class(error.kind())),
        Some(writeback_failure_digest(
            error,
            context.contract,
            context.effect,
            context.idempotence,
        )),
        writeback_execution_counters(
            context.loop_prevention,
            None,
            Some(error.kind()),
            true,
            true,
            true,
            true,
            false,
        ),
    )
}

pub(super) fn rejected_receipt_record(
    context: &WritebackAuthorityExecutionContext<'_>,
    mapper_record: &BridgeWritebackMapperRecord,
    candidate: &BridgeValidatedWritebackCandidate,
    request: &TruthWritebackRequest,
    receipt: &TruthWritebackReceipt,
    failure_class: BridgeWritebackFailureClass,
    error: &BridgeWritebackError,
) -> BridgeWritebackExecutionRecord {
    BridgeWritebackExecutionRecord::new(
        context.contract,
        context.effect,
        context.idempotence,
        context.loop_prevention,
        context.strategy_coherence,
        Some(mapper_record),
        Some(candidate),
        None,
        None,
        Some(request),
        Some(receipt),
        None::<std::sync::Arc<str>>,
        Some(failure_class),
        Some(writeback_failure_digest(
            error,
            context.contract,
            context.effect,
            context.idempotence,
        )),
        writeback_execution_counters(
            context.loop_prevention,
            None,
            Some(error.kind()),
            true,
            true,
            true,
            true,
            false,
        ),
    )
}

pub(super) fn successful_authority_record(
    context: &WritebackAuthorityExecutionContext<'_>,
    mapper_record: &BridgeWritebackMapperRecord,
    candidate: &BridgeValidatedWritebackCandidate,
    outcome: &BridgeWritebackAuthorityOutcome,
    replay_bundle: &BridgeWritebackReplayBundle,
    request: &TruthWritebackRequest,
    receipt: &TruthWritebackReceipt,
) -> BridgeWritebackExecutionRecord {
    BridgeWritebackExecutionRecord::new(
        context.contract,
        context.effect,
        context.idempotence,
        context.loop_prevention,
        context.strategy_coherence,
        Some(mapper_record),
        Some(candidate),
        Some(outcome),
        Some(replay_bundle),
        Some(request),
        Some(receipt),
        None::<std::sync::Arc<str>>,
        None,
        None::<std::sync::Arc<str>>,
        writeback_execution_counters(
            context.loop_prevention,
            Some(outcome),
            None,
            true,
            true,
            true,
            true,
            true,
        ),
    )
}
