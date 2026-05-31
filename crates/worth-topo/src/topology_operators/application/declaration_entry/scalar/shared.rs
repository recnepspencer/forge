use crate::topology_operators::declaration_entry::{
    TopologyDetachBoundaryMembershipDeclaration, TopologyDetachRadialAdjacencyDeclaration,
    TopologyDetachShellOrWireMembershipDeclaration, TopologyRetireTopologyEntityDeclaration,
    TopologyRewireLoopEndpointDeclaration, TopologySpliceRadialAdjacencyDeclaration,
};

use super::super::super::{
    TopologyEditApplicationMode, TopologyEditFamily, TopologyOperatorExecution,
    TopologyOperatorExecutionError, TopologyOperatorExecutionPath, TopologyOperatorRunner,
    TopologyQueryBindingIndex,
};
use super::super::contract_payload::TopologyDeclarationContractPayload;
use super::super::execution_finalize::{finalize_lowered_batch, lower_contracts};
use super::super::orchestration_boundary::orchestrate_topology_declaration_entry;

pub(super) trait ScalarDeclarationBatch {
    const FAMILY: TopologyEditFamily;
}

impl ScalarDeclarationBatch for TopologyRetireTopologyEntityDeclaration {
    const FAMILY: TopologyEditFamily = TopologyEditFamily::RetireTopologyEntity;
}

impl ScalarDeclarationBatch for TopologyDetachBoundaryMembershipDeclaration {
    const FAMILY: TopologyEditFamily = TopologyEditFamily::DetachBoundaryMembership;
}

impl ScalarDeclarationBatch for TopologyRewireLoopEndpointDeclaration {
    const FAMILY: TopologyEditFamily = TopologyEditFamily::RewireLoopEndpoint;
}

impl ScalarDeclarationBatch for TopologyDetachShellOrWireMembershipDeclaration {
    const FAMILY: TopologyEditFamily = TopologyEditFamily::DetachShellOrWireMembership;
}

impl ScalarDeclarationBatch for TopologySpliceRadialAdjacencyDeclaration {
    const FAMILY: TopologyEditFamily = TopologyEditFamily::SpliceRadialAdjacency;
}

impl ScalarDeclarationBatch for TopologyDetachRadialAdjacencyDeclaration {
    const FAMILY: TopologyEditFamily = TopologyEditFamily::DetachRadialAdjacency;
}

pub(super) fn apply_scalar_declaration<D>(
    runner: &mut TopologyOperatorRunner<'_, '_>,
    declaration: D,
    bindings: &TopologyQueryBindingIndex,
    mode: TopologyEditApplicationMode,
) -> Result<TopologyOperatorExecution, TopologyOperatorExecutionError>
where
    D: ScalarDeclarationBatch
        + TopologyDeclarationContractPayload
        + forge_query::facade::ForgeQueryDeclarationInput<crate::facade::TopologyQueryDomain>
        + Clone,
{
    orchestrate_topology_declaration_entry(D::FAMILY, declaration.clone())?;
    let contracts = declaration.clone().into_contracts();
    let lowered_batch = lower_contracts(runner, &contracts, bindings, &Default::default())?;
    finalize_lowered_batch(
        runner,
        lowered_batch,
        TopologyOperatorExecutionPath::DeclarationEntry {
            semantic_family_key: D::SEMANTIC_FAMILY_KEY,
        },
        mode,
        declaration.semantic_families(),
        declaration.topology_edit_digest(),
        declaration.naming_continuity_matrix(),
        declaration.naming_report(),
    )
}
