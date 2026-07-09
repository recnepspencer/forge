use std::marker::PhantomData;

use worth_foundational::FoundationalProfileSet;

use crate::application::{WorthQueryDeclarationInput, WorthQueryDomainEntryMarker};
use crate::contribution_composed_orchestration::{
    WorthQueryContributionComposedOrchestrationInput, WorthQueryContributionIntent,
};
use crate::domain_capabilities::{
    WorthQueryExplanationContributionAuthoring, WorthQuerySupportContributionAuthoring,
    WorthQueryWorkflowContributionAuthoring,
};

use super::WorthQueryGeometryMaterialAttachmentHelperFamily;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGeometryMaterialAttachmentInput<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    declaration_input: I,
    contributions: Vec<WorthQueryContributionIntent>,
    materialization_profile: Option<FoundationalProfileSet>,
    _marker: PhantomData<D>,
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryGeometryMaterialAttachmentInput<D, I>
where
    I::Family: WorthQueryGeometryMaterialAttachmentHelperFamily<D>,
{
    pub fn new(declaration_input: I) -> Self {
        Self {
            declaration_input,
            contributions: Vec::new(),
            materialization_profile: None,
            _marker: PhantomData,
        }
    }

    pub fn with_contribution(mut self, contribution: WorthQueryContributionIntent) -> Self {
        self.contributions.push(contribution);
        self
    }

    pub fn with_support_contribution(
        self,
        contribution: WorthQuerySupportContributionAuthoring,
    ) -> Self {
        self.with_contribution(WorthQueryContributionIntent::support(contribution))
    }

    pub fn with_explanation_contribution(
        self,
        contribution: WorthQueryExplanationContributionAuthoring,
    ) -> Self {
        self.with_contribution(WorthQueryContributionIntent::explanation(contribution))
    }

    pub fn with_workflow_contribution(
        self,
        contribution: WorthQueryWorkflowContributionAuthoring,
    ) -> Self {
        self.with_contribution(WorthQueryContributionIntent::workflow(contribution))
    }

    pub fn materialize_summaries_with_profile(mut self, profile: FoundationalProfileSet) -> Self {
        self.materialization_profile = Some(profile);
        self
    }

    pub(crate) fn into_composed_input(
        self,
    ) -> WorthQueryContributionComposedOrchestrationInput<D, I> {
        let mut input =
            WorthQueryContributionComposedOrchestrationInput::new(self.declaration_input)
                .with_contributions(self.contributions);
        if let Some(profile) = self.materialization_profile {
            input = input.materialize_summaries_with_profile(profile);
        }
        input
    }
}
