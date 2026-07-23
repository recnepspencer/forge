use super::authority_execution_artifacts::BridgeWritebackAuthorityExecutionArtifacts;
use super::authority_execution_recording::{
    blocked_before_candidate_record, canonical_noop_record, WritebackAuthorityExecutionContext,
};
use super::*;

impl RuntimeBridge {
    pub(super) fn resolve_terminal_writeback_loop_disposition(
        &self,
        contract: &AdmittedBridgeWritebackContract,
        effect: &BridgeDerivedWritebackEffect,
        idempotence: &BridgeWritebackIdempotenceBasis,
        loop_prevention: BridgeWritebackLoopPreventionReport,
    ) -> Option<Result<BridgeWritebackAuthorityExecutionArtifacts, BridgeWritebackError>> {
        match loop_prevention.disposition() {
            BridgeWritebackLoopDisposition::AllowAuthoritativeAttempt => None,
            BridgeWritebackLoopDisposition::CanonicalNoop => Some(Ok(self
                .record_canonical_writeback_noop(contract, effect, idempotence, loop_prevention))),
            BridgeWritebackLoopDisposition::RejectAsUnsafeFeedback => Some(Err(self
                .record_unsafe_writeback_feedback(
                    contract,
                    effect,
                    idempotence,
                    &loop_prevention,
                ))),
        }
    }

    fn record_canonical_writeback_noop(
        &self,
        contract: &AdmittedBridgeWritebackContract,
        effect: &BridgeDerivedWritebackEffect,
        idempotence: &BridgeWritebackIdempotenceBasis,
        loop_prevention: BridgeWritebackLoopPreventionReport,
    ) -> BridgeWritebackAuthorityExecutionArtifacts {
        let outcome = BridgeWritebackAuthorityOutcome::canonical_noop(idempotence);
        let strategy = self.classify_writeback_strategy_coherence(contract, effect, idempotence);
        let replay = self.replay_writeback_bundle(contract, effect, idempotence, &outcome);
        let context = WritebackAuthorityExecutionContext::new(
            contract,
            effect,
            idempotence,
            &loop_prevention,
            &strategy,
        );
        let record = canonical_noop_record(&context, &outcome, &replay);
        self.diagnostics.record_writeback_execution(record.clone());
        BridgeWritebackAuthorityExecutionArtifacts::new(loop_prevention, outcome, None, record)
    }

    fn record_unsafe_writeback_feedback(
        &self,
        contract: &AdmittedBridgeWritebackContract,
        effect: &BridgeDerivedWritebackEffect,
        idempotence: &BridgeWritebackIdempotenceBasis,
        loop_prevention: &BridgeWritebackLoopPreventionReport,
    ) -> BridgeWritebackError {
        let error = BridgeWritebackError::new(
            BridgeWritebackErrorKind::InvariantRejected,
            format!(
                "unsafe bridge feedback suppressed before authority execution: {}",
                loop_prevention.digest()
            ),
        );
        let strategy = self.classify_writeback_strategy_coherence(contract, effect, idempotence);
        let context = WritebackAuthorityExecutionContext::new(
            contract,
            effect,
            idempotence,
            loop_prevention,
            &strategy,
        );
        self.diagnostics
            .record_writeback_execution(blocked_before_candidate_record(&context, &error));
        error
    }
}
