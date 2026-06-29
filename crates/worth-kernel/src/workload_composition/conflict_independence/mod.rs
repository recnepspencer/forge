mod common;
mod spatial;
mod spatial_lowering;
mod topology;
mod topology_lowering;

pub use common::ConflictIndependenceDisposition;
pub use spatial::{
    prove_spatial_conflict_independence, SpatialConflictIndependenceDenial,
    SpatialConflictIndependenceDenialKind, SpatialConflictIndependenceProof,
    SpatialConflictIndependenceRequest,
};
pub use topology::{
    prove_topology_conflict_independence, TopologyConflictIndependenceDenial,
    TopologyConflictIndependenceDenialKind, TopologyConflictIndependenceProof,
    TopologyConflictIndependenceRequest,
};

#[cfg(test)]
mod test_support;
#[cfg(test)]
pub(crate) use test_support::{
    owner_backed_spatial_closeout_with_evidence_routing_posture,
    owner_backed_topology_closeout_with_aspect_routing_posture,
    owner_backed_topology_closeout_with_replay_prior_proof_posture,
};
#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_hardening;
