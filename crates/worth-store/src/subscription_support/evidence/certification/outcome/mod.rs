mod classification;
mod operations;
mod participation;
mod rejection;
mod verdict;

use super::super::counter_snapshot::SubscriptionSupportCounterSnapshot;
use super::lane::SubscriptionSupportCertificationLaneKind;
use crate::subscription_support::{
    SubscriptionResumeClassification, SubscriptionSupportDriftCause,
    SubscriptionSupportResultCostSurface,
};
use crate::SupportBatchReceiptReuseReport;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportCertificationLaneOutcome {
    pub(super) lane: SubscriptionSupportCertificationLaneKind,
    pub(super) classification: Option<SubscriptionResumeClassification>,
    pub(super) primary_cause: Option<SubscriptionSupportDriftCause>,
    pub(super) suppressed_causes: Vec<SubscriptionSupportDriftCause>,
    pub(super) truth_digest: String,
    pub(super) artifact_digest: String,
    pub(super) subscription_support_digest: String,
    pub(super) replay_digest: String,
    pub(super) diagnostics_digest: String,
    pub(super) counter_digest: String,
    pub(super) cost_surface: Option<SubscriptionSupportResultCostSurface>,
    pub(super) batch_receipt_reuse_report: Option<SupportBatchReceiptReuseReport>,
    pub(super) counter_snapshot: SubscriptionSupportCounterSnapshot,
}

impl SubscriptionSupportCertificationLaneOutcome {
    pub fn lane(&self) -> SubscriptionSupportCertificationLaneKind {
        self.lane
    }

    pub fn classification(&self) -> Option<SubscriptionResumeClassification> {
        self.classification
    }

    pub fn primary_cause(&self) -> Option<SubscriptionSupportDriftCause> {
        self.primary_cause
    }

    pub fn suppressed_causes(&self) -> &[SubscriptionSupportDriftCause] {
        &self.suppressed_causes
    }

    pub fn counter_snapshot(&self) -> &SubscriptionSupportCounterSnapshot {
        &self.counter_snapshot
    }

    pub fn cost_surface(&self) -> Option<SubscriptionSupportResultCostSurface> {
        self.cost_surface
    }

    pub fn batch_receipt_reuse_report(&self) -> Option<&SupportBatchReceiptReuseReport> {
        self.batch_receipt_reuse_report.as_ref()
    }
}
