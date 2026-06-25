use forge_store_contracts::RoadmapScope;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S2PhysicalSubstrateReadiness {
    scope: RoadmapScope,
    sealed: bool,
}

impl S2PhysicalSubstrateReadiness {
    pub(crate) const fn from_admitted_physical_substrate_closeout(scope: RoadmapScope) -> Self {
        Self {
            scope,
            sealed: true,
        }
    }

    pub const fn scope(&self) -> RoadmapScope {
        self.scope
    }

    pub const fn is_sealed(&self) -> bool {
        self.sealed
    }
}
