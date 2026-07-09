use crate::facade::{BridgeWritebackError, BridgeWritebackErrorKind, BridgeWritebackReplayBundle};
use crate::writeback::BridgeWritebackReplayRecord;

pub(in crate::harness::adapter::adapter_impl) struct ReplayLoopCrossFamilyIsolation {
    projected_bundle: BridgeWritebackReplayBundle,
    aspect_bundle: BridgeWritebackReplayBundle,
    error: BridgeWritebackError,
    replay_record: BridgeWritebackReplayRecord,
}

impl ReplayLoopCrossFamilyIsolation {
    pub(super) fn from_replay_error(
        projected_bundle: &BridgeWritebackReplayBundle,
        aspect_bundle: &BridgeWritebackReplayBundle,
        error: &BridgeWritebackError,
        replay_record: &BridgeWritebackReplayRecord,
    ) -> Self {
        Self {
            projected_bundle: projected_bundle.clone(),
            aspect_bundle: aspect_bundle.clone(),
            error: error.clone(),
            replay_record: replay_record.clone(),
        }
    }

    pub(in crate::harness::adapter::adapter_impl) fn projected_bundle(
        &self,
    ) -> &BridgeWritebackReplayBundle {
        &self.projected_bundle
    }

    pub(in crate::harness::adapter::adapter_impl) fn aspect_bundle(
        &self,
    ) -> &BridgeWritebackReplayBundle {
        &self.aspect_bundle
    }

    pub(in crate::harness::adapter::adapter_impl) fn error(&self) -> &BridgeWritebackError {
        &self.error
    }

    pub(in crate::harness::adapter::adapter_impl) fn replay_record(
        &self,
    ) -> &BridgeWritebackReplayRecord {
        &self.replay_record
    }

    pub(in crate::harness::adapter::adapter_impl) fn semantic_digest_separated(&self) -> bool {
        self.projected_bundle.semantic_digest() != self.aspect_bundle.semantic_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn bundle_digest_separated(&self) -> bool {
        self.projected_bundle.digest() != self.aspect_bundle.digest()
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
