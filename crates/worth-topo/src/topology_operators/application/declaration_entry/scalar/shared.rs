use crate::topology_operators::declaration_entry::{
    TopologyDetachBoundaryMembershipDeclaration, TopologyDetachRadialAdjacencyDeclaration,
    TopologyDetachShellOrWireMembershipDeclaration, TopologyRetireTopologyEntityDeclaration,
    TopologyRewireLoopEndpointDeclaration, TopologySpliceRadialAdjacencyDeclaration,
};

use super::super::super::{
    TopologyDeclaredMutationArtifact, TopologyMutationApplicationError,
    TopologyMutationApplicationMode, TopologyMutationApplicationRunner, TopologyMutationFamily,
    TopologyQueryBindingIndex,
};
use super::super::execution_finalize::{finalize_lowered_mutations, lower_mutation_sequence};
use super::super::mutation_payload::TopologyDeclarationMutationPayload;
use super::super::orchestration_boundary::orchestrate_topology_declaration_entry;

pub(super) trait ScalarDeclarationFamily {
    const FAMILY: TopologyMutationFamily;
}

impl ScalarDeclarationFamily for TopologyRetireTopologyEntityDeclaration {
    const FAMILY: TopologyMutationFamily = TopologyMutationFamily::RetireTopologyEntity;
}

impl ScalarDeclarationFamily for TopologyDetachBoundaryMembershipDeclaration {
    const FAMILY: TopologyMutationFamily = TopologyMutationFamily::DetachBoundaryMembership;
}

impl ScalarDeclarationFamily for TopologyRewireLoopEndpointDeclaration {
    const FAMILY: TopologyMutationFamily = TopologyMutationFamily::RewireLoopEndpoint;
}

impl ScalarDeclarationFamily for TopologyDetachShellOrWireMembershipDeclaration {
    const FAMILY: TopologyMutationFamily = TopologyMutationFamily::DetachShellOrWireMembership;
}

impl ScalarDeclarationFamily for TopologySpliceRadialAdjacencyDeclaration {
    const FAMILY: TopologyMutationFamily = TopologyMutationFamily::SpliceRadialAdjacency;
}

impl ScalarDeclarationFamily for TopologyDetachRadialAdjacencyDeclaration {
    const FAMILY: TopologyMutationFamily = TopologyMutationFamily::DetachRadialAdjacency;
}

pub(super) fn apply_scalar_declaration<D>(
    runner: &mut TopologyMutationApplicationRunner<'_, '_>,
    declaration: D,
    bindings: &TopologyQueryBindingIndex,
    mode: TopologyMutationApplicationMode,
) -> Result<TopologyDeclaredMutationArtifact, TopologyMutationApplicationError>
where
    D: ScalarDeclarationFamily
        + TopologyDeclarationMutationPayload
        + forge_query::facade::ForgeQueryDeclarationInput<crate::query_domain::TopologyQueryDomain>
        + Clone,
{
    orchestrate_topology_declaration_entry(D::FAMILY, declaration.clone())?;
    let sequence = declaration.clone().into_mutation_sequence();
    let lowered_mutations =
        lower_mutation_sequence(runner, &sequence, bindings, &Default::default())?;
    finalize_lowered_mutations(
        runner,
        lowered_mutations,
        D::SEMANTIC_FAMILY_KEY,
        mode,
        &sequence,
    )
}
