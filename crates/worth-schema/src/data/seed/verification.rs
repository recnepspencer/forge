use forge_relational::facade::history::BranchId;
use forge_relational::facade::runtime::RelationalRuntime;

use crate::data::authority::{
    RawWorthTopologyIntent, VerifiedTopologyCommit, WorthTopologyAuthority,
    WorthTopologyAuthorityError,
};
use crate::data::tracing::WorthBoundaryFailure;

pub fn verify_topology_intent(
    runtime: &mut RelationalRuntime,
    intent: RawWorthTopologyIntent,
) -> Result<VerifiedTopologyCommit, WorthBoundaryFailure<WorthTopologyAuthorityError>> {
    WorthTopologyAuthority::new(runtime)
        .apply_topology_intent_traced(intent)
        .map(|traced| traced.into_primary_result())
}

pub fn verify_topology_intent_on_branch(
    runtime: &mut RelationalRuntime,
    intent: RawWorthTopologyIntent,
    branch_id: BranchId,
) -> Result<VerifiedTopologyCommit, WorthBoundaryFailure<WorthTopologyAuthorityError>> {
    WorthTopologyAuthority::new(runtime)
        .apply_topology_intent_on_branch_traced(intent, branch_id)
        .map(|traced| traced.into_primary_result())
}
