use crate::evidence_identity::ForgeQueryEvidenceIdentity;

use super::active_counters::ActiveSubscriptionCounters;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SubscriptionConsumerAttachmentDenialKind {
    AttachmentBudgetExceeded,
    BackpressureDenied,
    LaneHandleMismatch,
    AcknowledgementReceiptMismatch,
    AcknowledgementSequenceRegression,
}

impl SubscriptionConsumerAttachmentDenialKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AttachmentBudgetExceeded => "attachment_budget_exceeded",
            Self::BackpressureDenied => "backpressure_denied",
            Self::LaneHandleMismatch => "lane_handle_mismatch",
            Self::AcknowledgementReceiptMismatch => "acknowledgement_receipt_mismatch",
            Self::AcknowledgementSequenceRegression => "acknowledgement_sequence_regression",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubscriptionConsumerAttachmentError {
    denial_kind: SubscriptionConsumerAttachmentDenialKind,
    message: String,
    pub(in crate::subscription) source_identity: ForgeQueryEvidenceIdentity,
    counters: ActiveSubscriptionCounters,
}

impl SubscriptionConsumerAttachmentError {
    pub(super) fn new(
        denial_kind: SubscriptionConsumerAttachmentDenialKind,
        message: impl Into<String>,
        source_identity: ForgeQueryEvidenceIdentity,
        counters: ActiveSubscriptionCounters,
    ) -> Self {
        Self {
            denial_kind,
            message: message.into(),
            source_identity,
            counters,
        }
    }

    pub fn denial_kind(&self) -> &SubscriptionConsumerAttachmentDenialKind {
        &self.denial_kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn counters(&self) -> &ActiveSubscriptionCounters {
        &self.counters
    }
}
