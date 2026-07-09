#![allow(dead_code)]

use serde::Serialize;

use super::{
    ColdRecallTierPath, RecallCoalescingKey, RecallCompletionWitness, RetainedReadPlacementPath,
    TierMissOutcome,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum RecallExecutionDisposition {
    Executed,
    CoalescedJoin,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CoalescedRecallReport {
    coalescing_key: RecallCoalescingKey,
    disposition: RecallExecutionDisposition,
    artifact_key: String,
    placement_path: RetainedReadPlacementPath,
    verification_label: String,
    completion_witness: Option<RecallCompletionWitness>,
}

impl CoalescedRecallReport {
    pub(crate) fn new(
        coalescing_key: RecallCoalescingKey,
        disposition: RecallExecutionDisposition,
        artifact_key: impl Into<String>,
        placement_path: RetainedReadPlacementPath,
        verification_label: impl Into<String>,
        completion_witness: Option<RecallCompletionWitness>,
    ) -> Self {
        Self {
            coalescing_key,
            disposition,
            artifact_key: artifact_key.into(),
            placement_path,
            verification_label: verification_label.into(),
            completion_witness,
        }
    }

    pub fn coalescing_key(&self) -> &RecallCoalescingKey {
        &self.coalescing_key
    }

    pub fn disposition(&self) -> RecallExecutionDisposition {
        self.disposition
    }

    pub fn completion_witness(&self) -> Option<&RecallCompletionWitness> {
        self.completion_witness.as_ref()
    }

    pub fn artifact_key(&self) -> &str {
        &self.artifact_key
    }

    pub fn placement_path(&self) -> RetainedReadPlacementPath {
        self.placement_path
    }

    pub fn resolved_path(&self) -> ColdRecallTierPath {
        self.placement_path.into()
    }

    pub fn tier_miss_outcome(&self) -> TierMissOutcome {
        self.placement_path.tier_miss_outcome()
    }

    pub fn verification_label(&self) -> &str {
        &self.verification_label
    }
}
