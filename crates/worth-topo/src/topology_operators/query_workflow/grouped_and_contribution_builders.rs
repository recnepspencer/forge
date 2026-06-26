use forge_query::facade::{
    ForgeQueryContributionComposedOrchestrationInput, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationSupportsNeighborhoodGrouping, ForgeQueryGroupedDeclarationInput,
};

use crate::query_domain::TopologyQueryDomain;
use crate::topology_operators::application::TopologyDeclarationMutationPayload;
use crate::topology_operators::TopologyOperatorContributionDeclaration;

use super::semantic_contribution_codec::{
    topology_fallback_policy_contribution, topology_naming_row_contributions,
};
use super::TopologyOperatorContributionInput;
use super::TopologyOperatorContributionIntent;
use super::TopologyOperatorGroupedInput;

pub fn topology_grouped_operator_neighborhood<I>(seed: I) -> TopologyOperatorGroupedInput<I>
where
    I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
    I::Family: ForgeQueryDeclarationSupportsNeighborhoodGrouping<TopologyQueryDomain>,
{
    ForgeQueryGroupedDeclarationInput::local_neighborhood(seed)
}

pub fn topology_operator_contribution_workflow<I>(
    declaration: I,
) -> TopologyOperatorContributionInput<I>
where
    I: TopologyOperatorContributionDeclaration,
{
    ForgeQueryContributionComposedOrchestrationInput::new(declaration.clone())
        .with_contributions(declaration.topology_semantic_contributions())
}

pub(crate) fn topology_semantic_contributions<I>(
    declaration: &I,
) -> Vec<TopologyOperatorContributionIntent>
where
    I: TopologyDeclarationMutationPayload,
{
    let fallback_policy = declaration.strictest_fallback_policy();
    let mut contributions = topology_naming_row_contributions(declaration);

    contributions.push(topology_fallback_policy_contribution::<I>(fallback_policy));

    contributions
}
