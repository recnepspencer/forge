use forge_relational::facade::history::BranchId;
use worth_schema::facade::{
    RawWorthTopologyIntent, WorthBoundaryFailure, WorthTopologyAuthority,
    WorthTopologyAuthorityError, WorthTracedTopologyCommit,
};

fn _apply_main_contract(
    authority: &mut WorthTopologyAuthority<'_>,
    intent: RawWorthTopologyIntent,
) -> Result<WorthTracedTopologyCommit, WorthBoundaryFailure<WorthTopologyAuthorityError>> {
    authority.apply_topology_intent_traced(intent)
}

fn _apply_branch_contract(
    authority: &mut WorthTopologyAuthority<'_>,
    intent: RawWorthTopologyIntent,
    branch_id: BranchId,
) -> Result<WorthTracedTopologyCommit, WorthBoundaryFailure<WorthTopologyAuthorityError>> {
    authority.apply_topology_intent_on_branch_traced(intent, branch_id)
}

#[test]
fn worth_schema_public_authority_traced_boundaries_compile_with_envelope_contracts() {
    let _ = _apply_main_contract;
    let _ = _apply_branch_contract;
}
