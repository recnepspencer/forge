use crate::topology_operators::{
    mutation_sequence::naming_mutation_continuity_matrix_from_rows, NamingMutationContinuityMatrix,
    TopologyDeclaredMutationSequence, TopologyMutationDerivedFallbackPolicy,
};

use super::semantic_contribution_codec::{
    fallback_explanation_detail, topology_fallback_policy_from_query_evidence,
    topology_naming_row_from_query_evidence,
};
use super::workflow_artifacts::TopologyOperatorRetainedContributionComposition;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TopologyRetainedContributionSemanticProjection {
    naming_mutation_continuity_matrix: NamingMutationContinuityMatrix,
    derived_fallback_policy: TopologyMutationDerivedFallbackPolicy,
}

impl TopologyRetainedContributionSemanticProjection {
    pub(crate) fn naming_mutation_continuity_matrix(&self) -> &NamingMutationContinuityMatrix {
        &self.naming_mutation_continuity_matrix
    }

    pub(crate) fn derived_fallback_policy(&self) -> TopologyMutationDerivedFallbackPolicy {
        self.derived_fallback_policy
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn fallback_explanation_detail(&self) -> &'static str {
        fallback_explanation_detail(self.derived_fallback_policy)
    }
}

pub(crate) fn validate_topology_retained_contribution_composition(
    composition: &ForgeQueryDeclarationEntryContributionComposition,
    semantic_family_key: &'static str,
    sequence: &TopologyDeclaredMutationSequence,
) -> Result<
    TopologyOperatorRetainedContributionComposition,
    crate::topology_operators::application::TopologyMutationApplicationError,
> {
    let naming_mutation_continuity_matrix = retained_naming_mutation_continuity_matrix(composition);
    let fallback_policy = retained_derived_fallback_policy_opt(composition);
    let expected = sequence.naming_continuity_matrix();
    if naming_mutation_continuity_matrix == *expected
        && fallback_policy == Some(sequence.strictest_fallback_policy())
    {
        return Ok(composition.clone());
    }

    Err(
        crate::topology_operators::application::TopologyMutationApplicationError::RetainedSemanticAftermathMismatch {
            semantic_family_key,
            reason: format!(
                "retained Query semantic aftermath did not match the declared topology mutation sequence: expected {} naming row(s) and fallback policy `{}`, retained {} naming row(s) and fallback policy `{}`",
                expected.rows.len(),
                sequence.strictest_fallback_policy().as_str(),
                naming_mutation_continuity_matrix.rows.len(),
                fallback_policy
                    .map(TopologyMutationDerivedFallbackPolicy::as_str)
                    .unwrap_or("missing")
            ),
        },
    )
}

pub(crate) fn validated_topology_retained_contribution_semantic_projection(
    composition: &ForgeQueryDeclarationEntryContributionComposition,
    semantic_family_key: &'static str,
    sequence: &TopologyDeclaredMutationSequence,
) -> Result<
    TopologyRetainedContributionSemanticProjection,
    crate::topology_operators::application::TopologyMutationApplicationError,
> {
    let composition = validate_topology_retained_contribution_composition(
        composition,
        semantic_family_key,
        sequence,
    )?;
    Ok(topology_retained_contribution_semantic_projection(
        &composition,
    ))
}

pub(crate) fn topology_retained_contribution_semantic_projection(
    composition: &TopologyOperatorRetainedContributionComposition,
) -> TopologyRetainedContributionSemanticProjection {
    topology_retained_contribution_semantic_projection_opt(composition)
        .expect("retained topology contribution composition should preserve one fallback policy")
}

fn topology_retained_contribution_semantic_projection_opt(
    composition: &TopologyOperatorRetainedContributionComposition,
) -> Option<TopologyRetainedContributionSemanticProjection> {
    Some(TopologyRetainedContributionSemanticProjection {
        naming_mutation_continuity_matrix: retained_naming_mutation_continuity_matrix(composition),
        derived_fallback_policy: retained_derived_fallback_policy_opt(composition)?,
    })
}

fn retained_naming_mutation_continuity_matrix(
    composition: &TopologyOperatorRetainedContributionComposition,
) -> NamingMutationContinuityMatrix {
    let naming_rows = composition
        .evidence()
        .iter()
        .filter_map(topology_naming_row_from_query_evidence)
        .collect();
    naming_mutation_continuity_matrix_from_rows(naming_rows)
}

fn retained_derived_fallback_policy_opt(
    composition: &TopologyOperatorRetainedContributionComposition,
) -> Option<TopologyMutationDerivedFallbackPolicy> {
    composition
        .evidence()
        .iter()
        .find_map(topology_fallback_policy_from_query_evidence)
}

type ForgeQueryDeclarationEntryContributionComposition =
    forge_query::facade::ForgeQueryDeclarationEntryContributionComposition;
