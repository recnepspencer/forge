mod boundary_role;
mod chain_member;
mod chain_row;
mod chain_set;
mod construction;
mod counters;
mod denial;
mod identity;
mod indexed_inputs;
mod validation;

#[cfg(test)]
mod tests;

pub use boundary_role::{PlanarBooleanOverlapChainBoundaryRole, PlanarBooleanOverlapChainPosture};
pub use chain_member::PlanarBooleanOverlapEdgeChainMember;
pub use chain_row::PlanarBooleanOverlapEdgeChain;
pub use chain_set::PlanarBooleanOverlapEdgeChainSet;
pub use counters::PlanarBooleanOverlapEdgeChainCounters;
pub use denial::{PlanarBooleanOverlapEdgeChainDenial, PlanarBooleanOverlapEdgeChainDenialKind};
