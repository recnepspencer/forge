use super::execution_counters::writeback_replay_validation_counters;
use super::*;

impl RuntimeBridge {
    /// Produces the replay bundle for an executed writeback outcome.
    pub fn replay_writeback_bundle(
        &self,
        contract: &AdmittedBridgeWritebackContract,
        effect: &BridgeDerivedWritebackEffect,
        idempotence: &BridgeWritebackIdempotenceBasis,
        outcome: &BridgeWritebackAuthorityOutcome,
    ) -> BridgeWritebackReplayBundle {
        BridgeWritebackReplayBundle::from_canonical_records(contract, effect, idempotence, outcome)
    }

    /// Verifies that a replayed writeback bundle still matches the expected semantics.
    pub fn validate_replayed_writeback_bundle(
        &self,
        expected: &BridgeWritebackReplayBundle,
        replayed: &BridgeWritebackReplayBundle,
    ) -> Result<(), BridgeWritebackError> {
        let mismatch = expected.semantic_digest() != replayed.semantic_digest();
        let failure_class = mismatch.then_some(BridgeWritebackFailureClass::ReplayMismatch);
        let counters = writeback_replay_validation_counters(mismatch);
        let replay_record =
            BridgeWritebackReplayRecord::new(expected, replayed, failure_class, counters);
        self.diagnostics.record_writeback_replay(replay_record);
        if expected.semantic_digest() != replayed.semantic_digest() {
            return Err(BridgeWritebackError::new(
                BridgeWritebackErrorKind::ReplayMismatch,
                format!(
                    "writeback replay semantic mismatch: expected `{}` from effect intent `{}`, replayed `{}` from effect intent `{}`",
                    expected.semantic_digest(),
                    expected.effect_intent_digest(),
                    replayed.semantic_digest(),
                    replayed.effect_intent_digest(),
                ),
            ));
        }

        Ok(())
    }
}
