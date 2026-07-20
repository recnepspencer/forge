use super::active_budget::ActiveSubscriptionAllocationPosture;
use super::active_dimensions::ActiveAllocationScopeWidth;
use super::attachment_budget::DeliveryBackpressurePolicy;
use super::delivery_dimensions::{DeliveryWindowWidth, MaintenanceDeltaWidth, PatchGroupWidth};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryDeliveryWindowBudget {
    delivery_window_width: DeliveryWindowWidth,
    patch_group_width: PatchGroupWidth,
    maintenance_delta_width: MaintenanceDeltaWidth,
    allocation_scope_width: ActiveAllocationScopeWidth,
    allocation_posture: ActiveSubscriptionAllocationPosture,
    backpressure_policy: DeliveryBackpressurePolicy,
}

impl QueryDeliveryWindowBudget {
    pub fn admitted(
        delivery_window_width: DeliveryWindowWidth,
        patch_group_width: PatchGroupWidth,
        maintenance_delta_width: MaintenanceDeltaWidth,
        allocation_scope_width: ActiveAllocationScopeWidth,
        allocation_posture: ActiveSubscriptionAllocationPosture,
        backpressure_policy: DeliveryBackpressurePolicy,
    ) -> Self {
        Self {
            delivery_window_width,
            patch_group_width,
            maintenance_delta_width,
            allocation_scope_width,
            allocation_posture,
            backpressure_policy,
        }
    }

    pub(super) fn exceeds_phase_three_budget(&self) -> bool {
        self.delivery_window_width.get() == 0
            || self.patch_group_width.get() == 0
            || self.maintenance_delta_width.get() == 0
            || self.allocation_scope_width.get() == 0
    }

    pub(super) fn forbidden_allocation_posture(&self) -> bool {
        self.allocation_posture.is_heap_denied()
            || !self.allocation_posture.admits_delivery_window_phase()
    }

    pub fn delivery_window_width(&self) -> u64 {
        self.delivery_window_width.get()
    }

    pub fn patch_group_width(&self) -> u64 {
        self.patch_group_width.get()
    }

    pub fn maintenance_delta_width(&self) -> u64 {
        self.maintenance_delta_width.get()
    }

    pub fn allocation_scope_width(&self) -> u64 {
        self.allocation_scope_width.get()
    }

    pub fn allocation_posture(&self) -> ActiveSubscriptionAllocationPosture {
        self.allocation_posture
    }

    pub fn backpressure_policy(&self) -> &DeliveryBackpressurePolicy {
        &self.backpressure_policy
    }
}
