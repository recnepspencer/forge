use std::marker::PhantomData;

use forge_foundational::FoundationalProfileSet;

use crate::application::{ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker};
use crate::contribution_composed_orchestration::{
    ForgeQueryContributionComposedOrchestrationInput, ForgeQueryContributionIntent,
};
use crate::domain_capabilities::{
    ForgeQueryExplanationContributionAuthoring, ForgeQuerySupportContributionAuthoring,
    ForgeQueryWorkflowContributionAuthoring,
};

use super::ForgeQueryGeometryMaterialAttachmentHelperFamily;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGeometryMaterialAttachmentInput<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    declaration_input: I,
    contributions: Vec<ForgeQueryContributionIntent>,
    materialization_profile: Option<FoundationalProfileSet>,
    _marker: PhantomData<D>,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryGeometryMaterialAttachmentInput<D, I>
where
    I::Family: ForgeQueryGeometryMaterialAttachmentHelperFamily<D>,
{
    pub fn new(declaration_input: I) -> Self {
        Self {
            declaration_input,
            contributions: Vec::new(),
            materialization_profile: None,
            _marker: PhantomData,
        }
    }

    pub fn with_contribution(mut self, contribution: ForgeQueryContributionIntent) -> Self {
        self.contributions.push(contribution);
        self
    }

    pub fn with_support_contribution(
        self,
        contribution: ForgeQuerySupportContributionAuthoring,
    ) -> Self {
        self.with_contribution(ForgeQueryContributionIntent::support(contribution))
    }

    pub fn with_explanation_contribution(
        self,
        contribution: ForgeQueryExplanationContributionAuthoring,
    ) -> Self {
        self.with_contribution(ForgeQueryContributionIntent::explanation(contribution))
    }

    pub fn with_workflow_contribution(
        self,
        contribution: ForgeQueryWorkflowContributionAuthoring,
    ) -> Self {
        self.with_contribution(ForgeQueryContributionIntent::workflow(contribution))
    }

    pub fn materialize_summaries_with_profile(mut self, profile: FoundationalProfileSet) -> Self {
        self.materialization_profile = Some(profile);
        self
    }

    pub(crate) fn into_composed_input(
        self,
    ) -> ForgeQueryContributionComposedOrchestrationInput<D, I> {
        let mut input =
            ForgeQueryContributionComposedOrchestrationInput::new(self.declaration_input)
                .with_contributions(self.contributions);
        if let Some(profile) = self.materialization_profile {
            input = input.materialize_summaries_with_profile(profile);
        }
        input
    }
}
