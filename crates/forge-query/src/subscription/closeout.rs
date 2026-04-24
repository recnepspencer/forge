use crate::identity::hash_parts;

use super::active_budget::ActiveSubscriptionAllocationPosture;
use super::active_counters::ActiveSubscriptionCounters;
use super::active_digest::ActiveSubscriptionLaneDigest;
use super::attachment::SubscriptionConsumerAttachment;
use super::attachment_digest::SubscriptionConsumerAttachmentDigest;
use super::delivery_density::ActiveDeliveryDensityPosture;
use super::performance_receipt::SubscriptionPerformanceReceipt;
use super::preview_isolation::{
    PreviewSubscriptionDiscardCloseout, PreviewSubscriptionPromotionHandoff,
};
use super::support::QuerySubscriptionSupportProfile;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SubscriptionLifecycleCloseoutKind {
    ConsumerDetached,
    ConsumerTerminated,
    PreviewDiscarded,
    PreviewPromoted,
}

impl SubscriptionLifecycleCloseoutKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ConsumerDetached => "consumer_detached",
            Self::ConsumerTerminated => "consumer_terminated",
            Self::PreviewDiscarded => "preview_discarded",
            Self::PreviewPromoted => "preview_promoted",
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum SubscriptionLifecycleCloseRequest {
    DetachConsumer(SubscriptionConsumerAttachment),
    TerminateConsumer(SubscriptionConsumerAttachment),
    PreviewDiscard(PreviewSubscriptionDiscardCloseout),
    PreviewPromotion(PreviewSubscriptionPromotionHandoff),
}

impl SubscriptionLifecycleCloseRequest {
    pub(super) fn lane_digest(&self) -> &ActiveSubscriptionLaneDigest {
        match self {
            Self::DetachConsumer(attachment) | Self::TerminateConsumer(attachment) => {
                attachment.lane_digest()
            }
            Self::PreviewDiscard(closeout) => closeout.active_lane_digest(),
            Self::PreviewPromotion(handoff) => handoff.preview_lane_digest(),
        }
    }

    pub(super) fn attachment_digest(&self) -> &SubscriptionConsumerAttachmentDigest {
        match self {
            Self::DetachConsumer(attachment) | Self::TerminateConsumer(attachment) => {
                attachment.attachment_digest()
            }
            Self::PreviewDiscard(closeout) => closeout.attachment_digest(),
            Self::PreviewPromotion(handoff) => handoff.attachment_digest(),
        }
    }

    pub(super) fn closeout_kind(&self) -> SubscriptionLifecycleCloseoutKind {
        match self {
            Self::DetachConsumer(_) => SubscriptionLifecycleCloseoutKind::ConsumerDetached,
            Self::TerminateConsumer(_) => SubscriptionLifecycleCloseoutKind::ConsumerTerminated,
            Self::PreviewDiscard(_) => SubscriptionLifecycleCloseoutKind::PreviewDiscarded,
            Self::PreviewPromotion(_) => SubscriptionLifecycleCloseoutKind::PreviewPromoted,
        }
    }

    pub(super) fn source_digest(&self) -> &str {
        match self {
            Self::DetachConsumer(attachment) | Self::TerminateConsumer(attachment) => {
                attachment.attachment_digest().as_str()
            }
            Self::PreviewDiscard(closeout) => closeout.closeout_digest(),
            Self::PreviewPromotion(handoff) => handoff.handoff_digest(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SubscriptionLifecycleCloseDenialKind {
    LaneHandleMismatch,
    AttachmentLaneMismatch,
    AttachmentNotActive,
}

impl SubscriptionLifecycleCloseDenialKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::LaneHandleMismatch => "lane_handle_mismatch",
            Self::AttachmentLaneMismatch => "attachment_lane_mismatch",
            Self::AttachmentNotActive => "attachment_not_active",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubscriptionLifecycleCloseError {
    denial_kind: SubscriptionLifecycleCloseDenialKind,
    message: String,
    source_digest: String,
    counters: ActiveSubscriptionCounters,
}

impl SubscriptionLifecycleCloseError {
    pub(super) fn new(
        denial_kind: SubscriptionLifecycleCloseDenialKind,
        message: impl Into<String>,
        source_digest: impl Into<String>,
        counters: ActiveSubscriptionCounters,
    ) -> Self {
        Self {
            denial_kind,
            message: message.into(),
            source_digest: source_digest.into(),
            counters,
        }
    }

    pub fn denial_kind(&self) -> &SubscriptionLifecycleCloseDenialKind {
        &self.denial_kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }

    pub fn counters(&self) -> &ActiveSubscriptionCounters {
        &self.counters
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubscriptionLifecycleCloseout {
    active_lane_digest: ActiveSubscriptionLaneDigest,
    attachment_digest: SubscriptionConsumerAttachmentDigest,
    closeout_kind: SubscriptionLifecycleCloseoutKind,
    lane_terminal: bool,
    source_digest: String,
    support_profile: QuerySubscriptionSupportProfile,
    performance_receipt: SubscriptionPerformanceReceipt,
    counters: ActiveSubscriptionCounters,
    closeout_digest: String,
}

impl SubscriptionLifecycleCloseout {
    pub(super) fn new(request: SubscriptionLifecycleCloseRequest, lane_terminal: bool) -> Self {
        let closeout_kind = request.closeout_kind();
        let mut counters = ActiveSubscriptionCounters::default();
        counters.consumer_attachment_close_count = 1;
        counters.subscription_lifecycle_closeout_count = 1;
        if lane_terminal {
            counters.active_lane_close_count = 1;
        }
        let source_digest = request.source_digest().to_string();
        let support_profile =
            QuerySubscriptionSupportProfile::active_runtime_admitted(&source_digest);
        let consumed_width = if lane_terminal { 2 } else { 1 };
        let performance_receipt = SubscriptionPerformanceReceipt::new(
            consumed_width,
            2,
            ActiveDeliveryDensityPosture::SparseDelta,
            ActiveSubscriptionAllocationPosture::LifecycleArena,
            &source_digest,
        );
        counters.subscription_performance_receipt_count = 1;
        counters.subscription_budget_consumption_width = performance_receipt.consumed_width();
        counters.subscription_budget_remaining_width = performance_receipt.remaining_width();
        let active_lane_digest = request.lane_digest().clone();
        let attachment_digest = request.attachment_digest().clone();
        let closeout_digest = hash_parts(&[
            "subscription_lifecycle_closeout_v1".to_string(),
            format!("lane:{}", active_lane_digest.as_str()),
            format!("attachment:{}", attachment_digest.as_str()),
            format!("kind:{}", closeout_kind.as_str()),
            format!("lane_terminal:{lane_terminal}"),
            format!("support:{}", support_profile.digest()),
            format!(
                "performance:{}",
                performance_receipt.performance_receipt_digest()
            ),
            format!("counters:{}", counters.digest()),
            format!("source:{source_digest}"),
        ]);
        Self {
            active_lane_digest,
            attachment_digest,
            closeout_kind,
            lane_terminal,
            source_digest,
            support_profile,
            performance_receipt,
            counters,
            closeout_digest,
        }
    }

    pub fn active_lane_digest(&self) -> &ActiveSubscriptionLaneDigest {
        &self.active_lane_digest
    }

    pub fn attachment_digest(&self) -> &SubscriptionConsumerAttachmentDigest {
        &self.attachment_digest
    }

    pub fn closeout_kind(&self) -> &SubscriptionLifecycleCloseoutKind {
        &self.closeout_kind
    }

    pub fn lane_terminal(&self) -> bool {
        self.lane_terminal
    }

    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }

    pub fn support_profile(&self) -> &QuerySubscriptionSupportProfile {
        &self.support_profile
    }

    pub fn performance_receipt(&self) -> &SubscriptionPerformanceReceipt {
        &self.performance_receipt
    }

    pub fn counters(&self) -> &ActiveSubscriptionCounters {
        &self.counters
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }
}
