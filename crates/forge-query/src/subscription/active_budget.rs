use super::active_dimensions::{
    ActiveAllocationScopeWidth, ActiveFanoutWidth, ActiveRegistryLookupWidth,
};
use super::active_posture::ActiveLaneLookupClass;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActiveSubscriptionAllocationPosture {
    LifecycleArena,
    DeliveryWindowArena,
    PatchScratch,
    HeapAllocationDebtExplicit,
    HeapAllocationDenied,
}

pub type ActiveSubscriptionAllocationPolicy = ActiveSubscriptionAllocationPosture;

impl ActiveSubscriptionAllocationPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::LifecycleArena => "lifecycle_arena",
            Self::DeliveryWindowArena => "delivery_window_arena",
            Self::PatchScratch => "patch_scratch",
            Self::HeapAllocationDebtExplicit => "heap_allocation_debt_explicit",
            Self::HeapAllocationDenied => "heap_allocation_denied",
        }
    }

    pub(super) fn admits_lifecycle_phase(self) -> bool {
        matches!(
            self,
            Self::LifecycleArena | Self::HeapAllocationDebtExplicit
        )
    }

    pub(super) fn admits_delivery_window_phase(self) -> bool {
        matches!(
            self,
            Self::DeliveryWindowArena | Self::HeapAllocationDebtExplicit
        )
    }

    pub(super) fn admits_patch_scratch_phase(self) -> bool {
        matches!(self, Self::PatchScratch | Self::HeapAllocationDebtExplicit)
    }

    pub(super) fn is_heap_denied(self) -> bool {
        self == Self::HeapAllocationDenied
    }

    pub(super) fn is_heap_debt(self) -> bool {
        self == Self::HeapAllocationDebtExplicit
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActiveSubscriptionWorkBudget {
    registry_lookup_width: ActiveRegistryLookupWidth,
    fanout_width: ActiveFanoutWidth,
    allocation_scope_width: ActiveAllocationScopeWidth,
    lookup_class: ActiveLaneLookupClass,
    allocation_policy: ActiveSubscriptionAllocationPosture,
    durable_checkpoint_requested: bool,
    store_backed_restart_requested: bool,
}

impl ActiveSubscriptionWorkBudget {
    pub fn admitted(
        registry_lookup_width: ActiveRegistryLookupWidth,
        fanout_width: ActiveFanoutWidth,
        allocation_scope_width: ActiveAllocationScopeWidth,
        allocation_policy: ActiveSubscriptionAllocationPosture,
    ) -> Self {
        Self {
            registry_lookup_width,
            fanout_width,
            allocation_scope_width,
            lookup_class: ActiveLaneLookupClass::EquivalenceIndex,
            allocation_policy,
            durable_checkpoint_requested: false,
            store_backed_restart_requested: false,
        }
    }

    pub fn with_lookup_class(mut self, lookup_class: ActiveLaneLookupClass) -> Self {
        self.lookup_class = lookup_class;
        self
    }

    pub fn with_durable_checkpoint_request(mut self) -> Self {
        self.durable_checkpoint_requested = true;
        self
    }

    pub fn with_store_backed_restart_request(mut self) -> Self {
        self.store_backed_restart_requested = true;
        self
    }

    pub fn registry_lookup_width(&self) -> u64 {
        self.registry_lookup_width.get()
    }

    pub fn fanout_width(&self) -> u64 {
        self.fanout_width.get()
    }

    pub fn allocation_scope_width(&self) -> u64 {
        self.allocation_scope_width.get()
    }

    pub fn lookup_class(&self) -> &ActiveLaneLookupClass {
        &self.lookup_class
    }

    pub fn allocation_policy(&self) -> &ActiveSubscriptionAllocationPosture {
        &self.allocation_policy
    }

    pub fn allocation_posture(&self) -> ActiveSubscriptionAllocationPosture {
        self.allocation_policy
    }

    pub fn durable_checkpoint_requested(&self) -> bool {
        self.durable_checkpoint_requested
    }

    pub fn store_backed_restart_requested(&self) -> bool {
        self.store_backed_restart_requested
    }

    pub(super) fn exceeds_phase_one_budget(&self) -> bool {
        self.registry_lookup_width() < 1
            || self.fanout_width() < 1
            || self.allocation_scope_width() < 1
    }
}
