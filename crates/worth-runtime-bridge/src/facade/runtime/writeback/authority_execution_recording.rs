use super::authority_candidate::PreparedWritebackAuthorityCandidate;
use super::authority_failure_mapping::{
    map_writeback_error_kind_to_failure_class, writeback_failure_digest,
};
use super::execution_counters::{writeback_execution_counters, WritebackExecutionCounterEvidence};
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

    pub(super) fn contract(&self) -> &AdmittedBridgeWritebackContract {
        self.contract
    }

    pub(super) fn effect(&self) -> &BridgeDerivedWritebackEffect {
        self.effect
    }

    pub(super) fn idempotence(&self) -> &BridgeWritebackIdempotenceBasis {
        self.idempotence
    }

    pub(super) fn loop_prevention(&self) -> &BridgeWritebackLoopPreventionReport {
        self.loop_prevention
    }

    pub(super) fn strategy_coherence(&self) -> &BridgeWritebackStrategyCoherenceReport {
        self.strategy_coherence
    }
}

pub(super) struct WritebackAuthorityAttempt<'a> {
    execution: &'a WritebackAuthorityExecutionContext<'a>,
    prepared: &'a PreparedWritebackAuthorityCandidate,
    request: &'a TruthWritebackRequest,
}

impl<'a> WritebackAuthorityAttempt<'a> {
    pub(super) fn new(
        execution: &'a WritebackAuthorityExecutionContext<'a>,
        prepared: &'a PreparedWritebackAuthorityCandidate,
        request: &'a TruthWritebackRequest,
    ) -> Self {
        Self {
            execution,
            prepared,
            request,
        }
    }

    pub(super) fn execution(&self) -> &WritebackAuthorityExecutionContext<'_> {
        self.execution
    }

    pub(super) fn mapper_record(&self) -> &BridgeWritebackMapperRecord {
        self.prepared.mapper_record()
    }

    pub(super) fn candidate(&self) -> &BridgeValidatedWritebackCandidate {
        self.prepared.candidate()
    }

    pub(super) fn request(&self) -> &TruthWritebackRequest {
        self.request
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
        writeback_execution_counters(WritebackExecutionCounterEvidence {
            loop_prevention: context.loop_prevention,
            outcome: Some(outcome),
            error_kind: None,
            candidate_present: false,
            mapper_record_present: false,
            request_present: false,
            receipt_present: false,
            replay_bundle_present: true,
        }),
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
        writeback_execution_counters(WritebackExecutionCounterEvidence {
            loop_prevention: context.loop_prevention,
            outcome: None,
            error_kind: Some(error.kind()),
            candidate_present: false,
            mapper_record_present: false,
            request_present: false,
            receipt_present: false,
            replay_bundle_present: false,
        }),
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
        writeback_execution_counters(WritebackExecutionCounterEvidence {
            loop_prevention: context.loop_prevention,
            outcome: None,
            error_kind: Some(error.kind()),
            candidate_present: true,
            mapper_record_present: true,
            request_present: false,
            receipt_present: false,
            replay_bundle_present: false,
        }),
    )
}

pub(super) fn request_dispatch_failure_record(
    attempt: &WritebackAuthorityAttempt<'_>,
    error: &BridgeWritebackError,
) -> BridgeWritebackExecutionRecord {
    let context = attempt.execution();
    BridgeWritebackExecutionRecord::new(
        context.contract,
        context.effect,
        context.idempotence,
        context.loop_prevention,
        context.strategy_coherence,
        Some(attempt.mapper_record()),
        Some(attempt.candidate()),
        None,
        None,
        Some(attempt.request()),
        None,
        None::<std::sync::Arc<str>>,
        Some(map_writeback_error_kind_to_failure_class(error.kind())),
        Some(writeback_failure_digest(
            error,
            context.contract,
            context.effect,
            context.idempotence,
        )),
        writeback_execution_counters(WritebackExecutionCounterEvidence {
            loop_prevention: context.loop_prevention,
            outcome: None,
            error_kind: Some(error.kind()),
            candidate_present: true,
            mapper_record_present: true,
            request_present: true,
            receipt_present: false,
            replay_bundle_present: false,
        }),
    )
}

pub(super) fn validated_receipt_failure_record(
    attempt: &WritebackAuthorityAttempt<'_>,
    receipt: &TruthWritebackReceipt,
    error: &BridgeWritebackError,
) -> BridgeWritebackExecutionRecord {
    let context = attempt.execution();
    BridgeWritebackExecutionRecord::new(
        context.contract,
        context.effect,
        context.idempotence,
        context.loop_prevention,
        context.strategy_coherence,
        Some(attempt.mapper_record()),
        Some(attempt.candidate()),
        None,
        None,
        Some(attempt.request()),
        Some(receipt),
        None::<std::sync::Arc<str>>,
        Some(map_writeback_error_kind_to_failure_class(error.kind())),
        Some(writeback_failure_digest(
            error,
            context.contract,
            context.effect,
            context.idempotence,
        )),
        writeback_execution_counters(WritebackExecutionCounterEvidence {
            loop_prevention: context.loop_prevention,
            outcome: None,
            error_kind: Some(error.kind()),
            candidate_present: true,
            mapper_record_present: true,
            request_present: true,
            receipt_present: true,
            replay_bundle_present: false,
        }),
    )
}

pub(super) fn rejected_receipt_record(
    attempt: &WritebackAuthorityAttempt<'_>,
    receipt: &TruthWritebackReceipt,
    failure_class: BridgeWritebackFailureClass,
    error: &BridgeWritebackError,
) -> BridgeWritebackExecutionRecord {
    let context = attempt.execution();
    BridgeWritebackExecutionRecord::new(
        context.contract,
        context.effect,
        context.idempotence,
        context.loop_prevention,
        context.strategy_coherence,
        Some(attempt.mapper_record()),
        Some(attempt.candidate()),
        None,
        None,
        Some(attempt.request()),
        Some(receipt),
        None::<std::sync::Arc<str>>,
        Some(failure_class),
        Some(writeback_failure_digest(
            error,
            context.contract,
            context.effect,
            context.idempotence,
        )),
        writeback_execution_counters(WritebackExecutionCounterEvidence {
            loop_prevention: context.loop_prevention,
            outcome: None,
            error_kind: Some(error.kind()),
            candidate_present: true,
            mapper_record_present: true,
            request_present: true,
            receipt_present: true,
            replay_bundle_present: false,
        }),
    )
}

pub(super) fn successful_authority_record(
    attempt: &WritebackAuthorityAttempt<'_>,
    outcome: &BridgeWritebackAuthorityOutcome,
    replay_bundle: &BridgeWritebackReplayBundle,
    receipt: &TruthWritebackReceipt,
) -> BridgeWritebackExecutionRecord {
    let context = attempt.execution();
    BridgeWritebackExecutionRecord::new(
        context.contract,
        context.effect,
        context.idempotence,
        context.loop_prevention,
        context.strategy_coherence,
        Some(attempt.mapper_record()),
        Some(attempt.candidate()),
        Some(outcome),
        Some(replay_bundle),
        Some(attempt.request()),
        Some(receipt),
        None::<std::sync::Arc<str>>,
        None,
        None::<std::sync::Arc<str>>,
        writeback_execution_counters(WritebackExecutionCounterEvidence {
            loop_prevention: context.loop_prevention,
            outcome: Some(outcome),
            error_kind: None,
            candidate_present: true,
            mapper_record_present: true,
            request_present: true,
            receipt_present: true,
            replay_bundle_present: true,
        }),
    )
}
