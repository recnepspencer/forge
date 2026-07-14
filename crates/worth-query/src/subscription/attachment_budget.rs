use super::active_dimensions::{ActiveAllocationScopeWidth, ActiveFanoutWidth};
use super::attachment_dimensions::ConsumerDeliveryPacingWidth;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeliveryBackpressurePolicy {
    RetainWithinWindow,
    DropWithGapNotice,
    TerminateConsumer,
    DebtExplicit,
}

impl DeliveryBackpressurePolicy {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::RetainWithinWindow => "retain_within_window",
            Self::DropWithGapNotice => "drop_with_gap_notice",
            Self::TerminateConsumer => "terminate_consumer",
            Self::DebtExplicit => "debt_explicit",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubscriptionConsumerAttachmentBudget {
    fanout_width: ActiveFanoutWidth,
    delivery_pacing_width: ConsumerDeliveryPacingWidth,
    allocation_scope_width: ActiveAllocationScopeWidth,
    backpressure_policy: DeliveryBackpressurePolicy,
    backpressure_denial_requested: bool,
}

impl SubscriptionConsumerAttachmentBudget {
    pub fn admitted(
        fanout_width: ActiveFanoutWidth,
        delivery_pacing_width: ConsumerDeliveryPacingWidth,
        allocation_scope_width: ActiveAllocationScopeWidth,
        backpressure_policy: DeliveryBackpressurePolicy,
    ) -> Self {
        Self {
            fanout_width,
            delivery_pacing_width,
            allocation_scope_width,
            backpressure_policy,
            backpressure_denial_requested: false,
        }
    }

    pub fn with_backpressure_denial_request(mut self) -> Self {
        self.backpressure_denial_requested = true;
        self
    }

    pub(super) fn exceeds_phase_two_budget(&self) -> bool {
        self.fanout_width.get() == 0
            || self.delivery_pacing_width.get() == 0
            || self.allocation_scope_width.get() == 0
    }

    pub fn fanout_width(&self) -> u64 {
        self.fanout_width.get()
    }

    pub fn delivery_pacing_width(&self) -> u64 {
        self.delivery_pacing_width.get()
    }

    pub fn allocation_scope_width(&self) -> u64 {
        self.allocation_scope_width.get()
    }

    pub fn backpressure_policy(&self) -> &DeliveryBackpressurePolicy {
        &self.backpressure_policy
    }

    pub(super) fn backpressure_denial_requested(&self) -> bool {
        self.backpressure_denial_requested
    }
}
