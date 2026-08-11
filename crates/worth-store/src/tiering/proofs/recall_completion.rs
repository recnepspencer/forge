use serde::Serialize;

use super::recall_path::{ColdRecallTierPath, RetainedReadPlacementPath, TierMissOutcome};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RecallCompletionWitness {
    artifact_key: String,
    placement_path: RetainedReadPlacementPath,
    tier_miss_outcome: TierMissOutcome,
    verification_label: String,
}

impl RecallCompletionWitness {
    pub(crate) fn new(
        artifact_key: impl Into<String>,
        placement_path: RetainedReadPlacementPath,
        verification_label: impl Into<String>,
    ) -> Self {
        Self {
            artifact_key: artifact_key.into(),
            tier_miss_outcome: placement_path.tier_miss_outcome(),
            placement_path,
            verification_label: verification_label.into(),
        }
    }

    pub fn artifact_key(&self) -> &str {
        &self.artifact_key
    }

    pub fn placement_path(&self) -> RetainedReadPlacementPath {
        self.placement_path
    }

    pub fn tier_miss_outcome(&self) -> TierMissOutcome {
        self.tier_miss_outcome
    }

    pub fn resolved_path(&self) -> ColdRecallTierPath {
        self.placement_path.into()
    }

    pub fn verification_label(&self) -> &str {
        &self.verification_label
    }
}
