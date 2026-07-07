use crate::topology_operators::{
    NamingMutationContinuityMatrix, TopologyDeclaredMutationSequence,
    TopologyMutationDerivedFallbackPolicy, TopologyMutationDigest, TopologyMutationFamily,
    TopologyRetainedContributionSemanticProjection,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TopologyAcceptedMutationProjection {
    semantic_family_key: &'static str,
    mutation_families: Vec<TopologyMutationFamily>,
    topology_mutation_digest: TopologyMutationDigest,
    naming_mutation_continuity_matrix: NamingMutationContinuityMatrix,
    derived_fallback_policy: TopologyMutationDerivedFallbackPolicy,
    fallback_explanation_detail: &'static str,
}

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

const _: () = {
    let _ = std::mem::size_of::<TopologyAcceptedMutationProjection>();
    let _ = TopologyAcceptedMutationProjection::from_sequence_and_semantic_projection;
    let _ = TopologyAcceptedMutationProjection::semantic_family_key;
    let _ = TopologyAcceptedMutationProjection::mutation_families;
    let _ = TopologyAcceptedMutationProjection::topology_mutation_digest;
    let _ = TopologyAcceptedMutationProjection::naming_mutation_continuity_matrix;
    let _ = TopologyAcceptedMutationProjection::derived_fallback_policy;
    let _ = TopologyAcceptedMutationProjection::fallback_explanation_detail;
};
