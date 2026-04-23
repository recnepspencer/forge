use crate::identity::hash_parts;

use super::active_budget::ActiveSubscriptionAllocationPosture;
use super::delivery_density::ActiveDeliveryDensityPosture;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubscriptionPerformanceReceipt {
    consumed_width: u64,
    budgeted_width: u64,
    remaining_width: u64,
    density_posture: ActiveDeliveryDensityPosture,
    allocation_posture: ActiveSubscriptionAllocationPosture,
    performance_receipt_digest: String,
}

impl SubscriptionPerformanceReceipt {
    pub(super) fn new(
        consumed_width: u64,
        budgeted_width: u64,
        density_posture: ActiveDeliveryDensityPosture,
        allocation_posture: ActiveSubscriptionAllocationPosture,
        source_digest: &str,
    ) -> Self {
        let remaining_width = budgeted_width.saturating_sub(consumed_width);
        let performance_receipt_digest = hash_parts(&[
            "subscription_performance_receipt_v1".to_string(),
            format!("source:{}", source_digest),
            format!("consumed:{}", consumed_width),
            format!("budgeted:{}", budgeted_width),
            format!("remaining:{}", remaining_width),
            format!("density:{}", density_posture.as_str()),
            format!("allocation:{}", allocation_posture.as_str()),
        ]);
        Self {
            consumed_width,
            budgeted_width,
            remaining_width,
            density_posture,
            allocation_posture,
            performance_receipt_digest,
        }
    }

    pub fn consumed_width(&self) -> u64 {
        self.consumed_width
    }

    pub fn budgeted_width(&self) -> u64 {
        self.budgeted_width
    }

    pub fn remaining_width(&self) -> u64 {
        self.remaining_width
    }

    pub fn density_posture(&self) -> ActiveDeliveryDensityPosture {
        self.density_posture
    }

    pub fn allocation_posture(&self) -> ActiveSubscriptionAllocationPosture {
        self.allocation_posture
    }

    pub fn performance_receipt_digest(&self) -> &str {
        &self.performance_receipt_digest
    }
}
