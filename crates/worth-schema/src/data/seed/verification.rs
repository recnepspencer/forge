use forge_relational::facade::history::BranchId;
use forge_relational::facade::runtime::RelationalRuntime;

use crate::data::authority::{
    RawTopologyIntent, TopologyAuthority, TopologyAuthorityError, VerifiedTopologyCommit,
};
use crate::data::tracing::BoundaryFailure;

pub fn verify_topology_intent(
    runtime: &mut RelationalRuntime,
    intent: RawTopologyIntent,
) -> Result<VerifiedTopologyCommit, BoundaryFailure<TopologyAuthorityError>> {
    TopologyAuthority::new(runtime)
        .apply_topology_intent_traced(intent)
        .map(|traced| traced.into_primary_result())
}

pub fn verify_topology_intent_on_branch(
    runtime: &mut RelationalRuntime,
    intent: RawTopologyIntent,
    branch_id: BranchId,
) -> Result<VerifiedTopologyCommit, BoundaryFailure<TopologyAuthorityError>> {
    TopologyAuthority::new(runtime)
        .apply_topology_intent_on_branch_traced(intent, branch_id)
        .map(|traced| traced.into_primary_result())
}
