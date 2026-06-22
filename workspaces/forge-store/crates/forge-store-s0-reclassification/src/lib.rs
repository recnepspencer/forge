#![forbid(unsafe_code)]

use forge_store_claim_boundaries::StoreCapabilityTier;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackendReclassification {
    admitted_tier: StoreCapabilityTier,
}

impl BackendReclassification {
    pub const fn new(admitted_tier: StoreCapabilityTier) -> Self {
        Self { admitted_tier }
    }

    pub const fn admitted_tier(&self) -> StoreCapabilityTier {
        self.admitted_tier
    }
}
