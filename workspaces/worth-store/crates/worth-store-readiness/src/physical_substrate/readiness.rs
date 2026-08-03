use super::PhysicalSubstrateReadinessFacts;
use worth_store_contracts::RoadmapScope;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalSubstrateReadiness {
    scope: RoadmapScope,
    facts: PhysicalSubstrateReadinessFacts,
    sealed: bool,
}

impl PhysicalSubstrateReadiness {
    pub const fn scope(&self) -> RoadmapScope {
        self.scope
    }

    pub const fn facts(&self) -> PhysicalSubstrateReadinessFacts {
        self.facts
    }

    pub const fn is_sealed(&self) -> bool {
        self.sealed
    }
}
