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
    /// Admits only the complete explicit S.9 grammar. Actual proof-lane
    /// completion is certified separately by each lane's real producer.
    pub fn admit_s9_layout_handoff(
        &self,
    ) -> Result<StorageFoundationS9LayoutHandoff, crate::handoff::S9LayoutHandoffDenial> {
        crate::handoff::admit_s9_layout_handoff(S8LayoutHazardInventory::canonical())
    }
    /// Grammar inspection is deliberately distinct from the admitted S.9
    /// handoff. This facade cannot promote a catalogue or copied counters into
    /// lower-owner proof authority.
    pub(crate) const fn hazard_grammar(&self) -> S8LayoutHazardInventory {
        S8LayoutHazardInventory::canonical()
    }
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
