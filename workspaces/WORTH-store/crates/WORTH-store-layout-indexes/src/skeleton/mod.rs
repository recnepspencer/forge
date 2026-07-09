mod authority_flow;
mod crate_role;
mod phase_obligation;
mod responsibility_map;
#[cfg(test)]
mod tests;
mod topology_closeout;

pub use authority_flow::{
    S8AuthorityFlowEdge, S8CrossCrateAuthorityFlowReport, S8ForbiddenAuthoritySource,
};
pub use crate_role::{S8CratePrimaryRole, S8ProjectionOutputPosture};
pub use phase_obligation::{S8PhaseSkeletonObligation, S8PhaseSkeletonObligationRow};
pub use responsibility_map::{S8CrateResponsibilityMap, S8CrateResponsibilityRow};
pub use topology_closeout::{S8DomainSkeletonInventory, S8SubsystemTopologyCloseout};
