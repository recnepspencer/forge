#![forbid(unsafe_code)]

use forge_store_contracts::RoadmapScope;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum StoreCapabilityTier {
    Bootstrap,
    SemanticCertification,
    Compatibility,
    PhysicalFoundation,
    PlatformGrade,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClaimBoundary {
    tier: StoreCapabilityTier,
    scope: RoadmapScope,
}

impl ClaimBoundary {
    pub const fn new(tier: StoreCapabilityTier, scope: RoadmapScope) -> Self {
        Self { tier, scope }
    }

    pub const fn tier(&self) -> StoreCapabilityTier {
        self.tier
    }

    pub const fn scope(&self) -> RoadmapScope {
        self.scope
    }
}
