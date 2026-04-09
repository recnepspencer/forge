use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::counters::BridgeSpeculationCounters;
use super::discard::BridgePreviewDiscardRecord;
use super::execution::BridgePreviewExecutionRecord;
use super::promotion::BridgePreviewPromotionRecord;
use super::taxonomy::BridgePreviewLifecycleStateKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgePreviewReplayBundle {
    preview_execution_record: BridgePreviewExecutionRecord,
    preview_discard_record: Option<BridgePreviewDiscardRecord>,
    preview_promotion_record: Option<BridgePreviewPromotionRecord>,
    lifecycle_outcome: BridgePreviewLifecycleStateKind,
    counters: BridgeSpeculationCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgePreviewReplayBundle {
    pub fn new(
        preview_execution_record: BridgePreviewExecutionRecord,
        preview_discard_record: Option<BridgePreviewDiscardRecord>,
        preview_promotion_record: Option<BridgePreviewPromotionRecord>,
    ) -> Self {
        let lifecycle_outcome = if preview_promotion_record.is_some() {
            BridgePreviewLifecycleStateKind::Promoted
        } else if preview_discard_record.is_some() {
            BridgePreviewLifecycleStateKind::Discarded
        } else {
            BridgePreviewLifecycleStateKind::Active
        };
        let replay_bundle_width = 1
            + usize::from(preview_discard_record.is_some())
            + usize::from(preview_promotion_record.is_some());
        let counters = BridgeSpeculationCounters::for_replay(1, replay_bundle_width);
        let canonical_basis = Arc::<str>::from(format!(
            "preview-replay-bundle|session={}|execution-record={}|discard-record={}|promotion-record={}|outcome:{lifecycle_outcome:?}|bundle-width={}",
            preview_execution_record.preview_session_identity(),
            preview_execution_record.record_identity().as_str(),
            preview_discard_record
                .as_ref()
                .map(|record| record.record_identity().as_str())
                .unwrap_or("none"),
            preview_promotion_record
                .as_ref()
                .map(|record| record.record_identity().as_str())
                .unwrap_or("none"),
            replay_bundle_width,
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Self {
            preview_execution_record,
            preview_discard_record,
            preview_promotion_record,
            lifecycle_outcome,
            counters,
            canonical_basis,
            digest: Arc::from(format!("preview-replay-bundle:sha256:{digest:x}")),
        }
    }

    pub fn preview_execution_record(&self) -> &BridgePreviewExecutionRecord {
        &self.preview_execution_record
    }

    pub fn preview_discard_record(&self) -> Option<&BridgePreviewDiscardRecord> {
        self.preview_discard_record.as_ref()
    }

    pub fn preview_promotion_record(&self) -> Option<&BridgePreviewPromotionRecord> {
        self.preview_promotion_record.as_ref()
    }

    pub fn lifecycle_outcome(&self) -> BridgePreviewLifecycleStateKind {
        self.lifecycle_outcome
    }

    pub fn counters(&self) -> &BridgeSpeculationCounters {
        &self.counters
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
