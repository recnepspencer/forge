use crate::facade::{BridgeWritebackError, BridgeWritebackErrorKind, BridgeWritebackReplayBundle};
use crate::writeback::BridgeWritebackReplayRecord;

pub(in crate::harness::adapter::adapter_impl) struct FamilyExtensionChangedCausalityIsolation {
    projected_bundle: BridgeWritebackReplayBundle,
    changed_projected_bundle: BridgeWritebackReplayBundle,
    error: BridgeWritebackError,
    replay_record: BridgeWritebackReplayRecord,
}

impl FamilyExtensionChangedCausalityIsolation {
    pub(in crate::harness::adapter::adapter_impl::writeback_certification::family_extension) fn from_changed_causality(
        projected_bundle: &BridgeWritebackReplayBundle,
        changed_projected_bundle: &BridgeWritebackReplayBundle,
        error: &BridgeWritebackError,
        replay_record: &BridgeWritebackReplayRecord,
    ) -> Self {
        Self {
            projected_bundle: projected_bundle.clone(),
            changed_projected_bundle: changed_projected_bundle.clone(),
            error: error.clone(),
            replay_record: replay_record.clone(),
        }
    }

    pub(in crate::harness::adapter::adapter_impl) fn projected_bundle(
        &self,
    ) -> &BridgeWritebackReplayBundle {
        &self.projected_bundle
    }

    pub(in crate::harness::adapter::adapter_impl) fn changed_projected_bundle(
        &self,
    ) -> &BridgeWritebackReplayBundle {
        &self.changed_projected_bundle
    }

    pub(in crate::harness::adapter::adapter_impl) fn error(&self) -> &BridgeWritebackError {
        &self.error
    }

    pub(in crate::harness::adapter::adapter_impl) fn replay_record(
        &self,
    ) -> &BridgeWritebackReplayRecord {
        &self.replay_record
    }

    pub(in crate::harness::adapter::adapter_impl) fn causality_digest_separated(&self) -> bool {
        self.projected_bundle.causality_digest() != self.changed_projected_bundle.causality_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn semantic_digest_separated(&self) -> bool {
        self.projected_bundle.semantic_digest() != self.changed_projected_bundle.semantic_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn bundle_digest_separated(&self) -> bool {
        self.projected_bundle.digest() != self.changed_projected_bundle.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn failure_kind(
        &self,
    ) -> BridgeWritebackErrorKind {
        self.error.kind()
    }

    pub(in crate::harness::adapter::adapter_impl) fn family_replay_record_digest(&self) -> &str {
        self.replay_record.digest()
    }
}
