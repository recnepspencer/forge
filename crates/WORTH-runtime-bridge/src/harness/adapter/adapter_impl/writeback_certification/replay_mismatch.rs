use crate::error::{BridgeWritebackError, BridgeWritebackErrorKind};
use crate::writeback::BridgeWritebackReplayBundle;

pub(in crate::harness::adapter::adapter_impl) struct WritebackReplayMismatchMatrix {
    expected_bundle: BridgeWritebackReplayBundle,
    replayed_bundle: BridgeWritebackReplayBundle,
    failure_kind: BridgeWritebackErrorKind,
    failure_message: String,
    semantic_mismatch_detected: bool,
    diagnostic_detail_changed: bool,
    restart_replay: WritebackRestartReplayMismatchMatrix,
}

pub(super) struct WritebackRestartReplayMismatchMatrix {
    rebuilt_bundle: BridgeWritebackReplayBundle,
    rebuilt_failure_kind: BridgeWritebackErrorKind,
    rebuilt_failure_message: String,
    restart_mismatch_detected: bool,
}

impl WritebackReplayMismatchMatrix {
    pub(in crate::harness::adapter::adapter_impl) fn from_replay_validation(
        expected_bundle: &BridgeWritebackReplayBundle,
        replayed_bundle: &BridgeWritebackReplayBundle,
        validation_error: &BridgeWritebackError,
        rebuilt_replayed_bundle: &BridgeWritebackReplayBundle,
        rebuilt_validation_error: &BridgeWritebackError,
    ) -> Self {
        Self {
            expected_bundle: expected_bundle.clone(),
            replayed_bundle: replayed_bundle.clone(),
            failure_kind: validation_error.kind(),
            failure_message: validation_error.to_string(),
            semantic_mismatch_detected: expected_bundle.semantic_digest()
                != replayed_bundle.semantic_digest(),
            diagnostic_detail_changed: expected_bundle.digest() != replayed_bundle.digest(),
            restart_replay: WritebackRestartReplayMismatchMatrix::from_rebuilt_replay_validation(
                expected_bundle,
                rebuilt_replayed_bundle,
                rebuilt_validation_error,
            ),
        }
    }

    pub(in crate::harness::adapter::adapter_impl) fn expected_replay_digest(&self) -> &str {
        self.expected_bundle.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn replayed_replay_digest(&self) -> &str {
        self.replayed_bundle.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn expected_semantic_digest(&self) -> &str {
        self.expected_bundle.semantic_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn replayed_semantic_digest(&self) -> &str {
        self.replayed_bundle.semantic_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn expected_causality_digest(&self) -> &str {
        self.expected_bundle.causality_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn replayed_causality_digest(&self) -> &str {
        self.replayed_bundle.causality_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn expected_effect_intent_digest(&self) -> &str {
        self.expected_bundle.effect_intent_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn replayed_effect_intent_digest(&self) -> &str {
        self.replayed_bundle.effect_intent_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn expected_effect_intent_patch_canonical_basis(
        &self,
    ) -> &str {
        self.expected_bundle.effect_intent_patch_canonical_basis()
    }

    pub(in crate::harness::adapter::adapter_impl) fn replayed_effect_intent_patch_canonical_basis(
        &self,
    ) -> &str {
        self.replayed_bundle.effect_intent_patch_canonical_basis()
    }

    pub(in crate::harness::adapter::adapter_impl) fn expected_bundle(
        &self,
    ) -> &BridgeWritebackReplayBundle {
        &self.expected_bundle
    }

    pub(in crate::harness::adapter::adapter_impl) fn replayed_bundle(
        &self,
    ) -> &BridgeWritebackReplayBundle {
        &self.replayed_bundle
    }

    pub(in crate::harness::adapter::adapter_impl) fn failure_kind(
        &self,
    ) -> BridgeWritebackErrorKind {
        self.failure_kind
    }

    pub(super) fn failure_message(&self) -> &str {
        &self.failure_message
    }

    pub(in crate::harness::adapter::adapter_impl) fn semantic_mismatch_detected(&self) -> bool {
        self.semantic_mismatch_detected
    }

    pub(super) fn diagnostic_detail_changed(&self) -> bool {
        self.diagnostic_detail_changed
    }

    pub(in crate::harness::adapter::adapter_impl) fn restart_failure_kind(
        &self,
    ) -> BridgeWritebackErrorKind {
        self.restart_replay.rebuilt_failure_kind
    }

    pub(in crate::harness::adapter::adapter_impl) fn rebuilt_bundle(
        &self,
    ) -> &BridgeWritebackReplayBundle {
        self.restart_replay.rebuilt_bundle()
    }

    pub(in crate::harness::adapter::adapter_impl) fn rebuilt_replay_digest(&self) -> &str {
        self.restart_replay.rebuilt_replay_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn rebuilt_effect_intent_digest(&self) -> &str {
        self.restart_replay.rebuilt_effect_intent_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn rebuilt_effect_intent_patch_canonical_basis(
        &self,
    ) -> &str {
        self.restart_replay
            .rebuilt_effect_intent_patch_canonical_basis()
    }

    pub(super) fn restart_replay(&self) -> &WritebackRestartReplayMismatchMatrix {
        &self.restart_replay
    }
}

impl WritebackRestartReplayMismatchMatrix {
    fn from_rebuilt_replay_validation(
        expected_bundle: &BridgeWritebackReplayBundle,
        rebuilt_replayed_bundle: &BridgeWritebackReplayBundle,
        rebuilt_validation_error: &BridgeWritebackError,
    ) -> Self {
        Self {
            rebuilt_bundle: rebuilt_replayed_bundle.clone(),
            rebuilt_failure_kind: rebuilt_validation_error.kind(),
            rebuilt_failure_message: rebuilt_validation_error.to_string(),
            restart_mismatch_detected: expected_bundle.semantic_digest()
                != rebuilt_replayed_bundle.semantic_digest(),
        }
    }

    pub(super) fn rebuilt_replay_digest(&self) -> &str {
        self.rebuilt_bundle.digest()
    }

    pub(super) fn rebuilt_semantic_digest(&self) -> &str {
        self.rebuilt_bundle.semantic_digest()
    }

    pub(super) fn rebuilt_effect_intent_digest(&self) -> &str {
        self.rebuilt_bundle.effect_intent_digest()
    }

    pub(super) fn rebuilt_effect_intent_patch_canonical_basis(&self) -> &str {
        self.rebuilt_bundle.effect_intent_patch_canonical_basis()
    }

    pub(in crate::harness::adapter::adapter_impl) fn rebuilt_bundle(
        &self,
    ) -> &BridgeWritebackReplayBundle {
        &self.rebuilt_bundle
    }

    pub(super) fn rebuilt_failure_kind(&self) -> BridgeWritebackErrorKind {
        self.rebuilt_failure_kind
    }

    pub(super) fn rebuilt_failure_message(&self) -> &str {
        &self.rebuilt_failure_message
    }

    pub(super) fn restart_mismatch_detected(&self) -> bool {
        self.restart_mismatch_detected
    }
}
