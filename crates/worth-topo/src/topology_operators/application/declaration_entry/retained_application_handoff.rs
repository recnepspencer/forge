#[cfg(test)]
use forge_query::facade::ForgeQueryDeclarationReceipt;
use forge_query::facade::{ForgeQueryDeclarationEnvelope, ForgeQueryDeclarationInput};

use crate::query_domain::TopologyQueryDomain;
use crate::topology_operators::{
    validated_topology_retained_contribution_semantic_projection,
    TopologyOperatorContributionArtifact, TopologyRetainedContributionSemanticProjection,
};

use super::super::TopologyMutationApplicationError;

pub(crate) struct TopologyRetainedApplicationHandoff<I>
where
    I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
{
    contribution_artifact: TopologyOperatorContributionArtifact<I>,
}

impl<I> TopologyRetainedApplicationHandoff<I>
where
    I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
{
    pub(crate) fn new(contribution_artifact: TopologyOperatorContributionArtifact<I>) -> Self {
        Self {
            contribution_artifact,
        }
    }

    pub(crate) fn declaration_family_key(&self) -> &'static str {
        self.declaration_envelope().declaration_family_key()
    }

    #[cfg(test)]
    pub(crate) fn progression_digest(&self) -> &str {
        self.declaration_envelope()
            .progression_digest()
            .expect("retained topology application handoff should preserve a progression digest")
    }

    #[cfg(test)]
    pub(crate) fn declaration_receipt(
        &self,
    ) -> &ForgeQueryDeclarationReceipt<TopologyQueryDomain, I> {
        self.contribution_artifact.envelope().receipt()
    }

    #[cfg(test)]
    pub(crate) fn route_plan_digest(&self) -> &str {
        self.declaration_envelope()
            .route_plan_digest()
            .expect("retained topology application handoff should preserve a route-plan digest")
    }

    pub(crate) fn declaration_envelope(
        &self,
    ) -> &ForgeQueryDeclarationEnvelope<TopologyQueryDomain, I> {
        self.contribution_artifact.envelope()
    }

    #[cfg(test)]
    pub(crate) fn contribution_digest(&self) -> &str {
        self.contribution_artifact
            .contribution_composition()
            .contribution_digest()
    }

    pub(crate) fn retain_accepted_query_contribution_semantic_projection(
        &self,
        semantic_family_key: &'static str,
        sequence: &crate::topology_operators::TopologyDeclaredMutationSequence,
    ) -> Result<TopologyRetainedContributionSemanticProjection, TopologyMutationApplicationError>
    {
        validated_topology_retained_contribution_semantic_projection(
            self.contribution_artifact.contribution_composition(),
            semantic_family_key,
            sequence,
        )
    }
}
