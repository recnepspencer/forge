use crate::topology_operators::{
    NamingMutationContinuityMatrix, TopologyDeclaredMutationSequence,
    TopologyMutationDerivedFallbackPolicy, TopologyMutationDigest, TopologyMutationFamily,
    TopologyRetainedContributionSemanticProjection,
};

#[cfg_attr(not(test), allow(dead_code))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TopologyAcceptedMutationProjection {
    semantic_family_key: &'static str,
    mutation_families: Vec<TopologyMutationFamily>,
    topology_mutation_digest: TopologyMutationDigest,
    naming_mutation_continuity_matrix: NamingMutationContinuityMatrix,
    derived_fallback_policy: TopologyMutationDerivedFallbackPolicy,
    fallback_explanation_detail: &'static str,
}

#[cfg_attr(not(test), allow(dead_code))]
impl TopologyAcceptedMutationProjection {
    pub(crate) fn from_sequence_and_semantic_projection(
        semantic_family_key: &'static str,
        sequence: &TopologyDeclaredMutationSequence,
        semantic_projection: &TopologyRetainedContributionSemanticProjection,
    ) -> Self {
        Self {
            semantic_family_key,
            mutation_families: sequence.families().to_vec(),
            topology_mutation_digest: sequence.topology_mutation_digest().clone(),
            naming_mutation_continuity_matrix: semantic_projection
                .naming_mutation_continuity_matrix()
                .clone(),
            derived_fallback_policy: semantic_projection.derived_fallback_policy(),
            fallback_explanation_detail: semantic_projection.fallback_explanation_detail(),
        }
    }

    pub(crate) fn semantic_family_key(&self) -> &'static str {
        self.semantic_family_key
    }

    pub(crate) fn mutation_families(&self) -> &[TopologyMutationFamily] {
        &self.mutation_families
    }

    pub(crate) fn topology_mutation_digest(&self) -> &TopologyMutationDigest {
        &self.topology_mutation_digest
    }

    pub(crate) fn naming_mutation_continuity_matrix(&self) -> &NamingMutationContinuityMatrix {
        &self.naming_mutation_continuity_matrix
    }

    pub(crate) fn derived_fallback_policy(&self) -> TopologyMutationDerivedFallbackPolicy {
        self.derived_fallback_policy
    }

    pub(crate) fn fallback_explanation_detail(&self) -> &'static str {
        self.fallback_explanation_detail
    }
}
