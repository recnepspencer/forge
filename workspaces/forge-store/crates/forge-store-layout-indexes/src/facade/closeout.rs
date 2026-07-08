pub use crate::handoff::{S8LayoutHazardInventory, StorageFoundationS9LayoutHandoff};
pub use crate::skeleton::{
    S8CratePrimaryRole, S8CrateResponsibilityMap, S8CrateResponsibilityRow,
    S8CrossCrateAuthorityFlowReport, S8DomainSkeletonInventory, S8ForbiddenAuthoritySource,
    S8PhaseSkeletonObligation, S8PhaseSkeletonObligationRow, S8ProjectionOutputPosture,
    S8SubsystemTopologyCloseout,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutCloseoutFacade;

impl LayoutCloseoutFacade {
    pub const fn responsibility_map(&self) -> S8CrateResponsibilityMap {
        S8CrateResponsibilityMap::current()
    }

    pub const fn inventory(&self) -> S8DomainSkeletonInventory {
        S8DomainSkeletonInventory::current()
    }

    pub const fn topology(&self) -> S8SubsystemTopologyCloseout {
        S8SubsystemTopologyCloseout::current()
    }

    pub const fn authority_flow(&self) -> S8CrossCrateAuthorityFlowReport {
        S8CrossCrateAuthorityFlowReport::current()
    }
}

pub const fn layout_closeout() -> LayoutCloseoutFacade {
    LayoutCloseoutFacade
}
