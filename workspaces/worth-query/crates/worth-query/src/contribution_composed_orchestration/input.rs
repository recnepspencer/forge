use std::marker::PhantomData;

use worth_foundational::FoundationalProfileSet;

use crate::application::{WorthQueryDeclarationInput, WorthQueryDomainEntryMarker};
use crate::domain_capabilities::{
    WorthQueryAdmissionContributionAuthoring, WorthQueryContinuityContributionAuthoring,
    WorthQueryExplanationContributionAuthoring, WorthQuerySupportContributionAuthoring,
    WorthQueryWorkflowContributionAuthoring,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryContributionIntent {
    Admission(WorthQueryAdmissionContributionAuthoring),
    Support(WorthQuerySupportContributionAuthoring),
    Explanation(WorthQueryExplanationContributionAuthoring),
    Workflow(WorthQueryWorkflowContributionAuthoring),
    Continuity(WorthQueryContinuityContributionAuthoring),
}

impl WorthQueryContributionIntent {
    pub fn admission(value: WorthQueryAdmissionContributionAuthoring) -> Self {
        Self::Admission(value)
    }

    pub fn support(value: WorthQuerySupportContributionAuthoring) -> Self {
        Self::Support(value)
    }

    pub fn explanation(value: WorthQueryExplanationContributionAuthoring) -> Self {
        Self::Explanation(value)
    }

    pub fn workflow(value: WorthQueryWorkflowContributionAuthoring) -> Self {
        Self::Workflow(value)
    }

    pub fn continuity(value: WorthQueryContinuityContributionAuthoring) -> Self {
        Self::Continuity(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub enum WorthQueryContributionComposedMaterializationPolicy {
    #[default]
    None,
    Summary(FoundationalProfileSet),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryContributionComposedOrchestrationInput<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    declaration_input: I,
    contributions: Vec<WorthQueryContributionIntent>,
    materialization_policy: WorthQueryContributionComposedMaterializationPolicy,
    _marker: PhantomData<D>,
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryContributionComposedOrchestrationInput<D, I>
{
    pub fn new(declaration_input: I) -> Self {
        Self {
            declaration_input,
            contributions: Vec::new(),
            materialization_policy: WorthQueryContributionComposedMaterializationPolicy::None,
            _marker: PhantomData,
        }
    }

    pub fn with_contribution(mut self, contribution: WorthQueryContributionIntent) -> Self {
        self.contributions.push(contribution);
        self
    }

    pub fn with_contributions(
        mut self,
        contributions: impl IntoIterator<Item = WorthQueryContributionIntent>,
    ) -> Self {
        self.contributions.extend(contributions);
        self
    }

    pub fn materialize_summaries_with_profile(mut self, profile: FoundationalProfileSet) -> Self {
        self.materialization_policy =
            WorthQueryContributionComposedMaterializationPolicy::Summary(profile);
        self
    }

    pub fn declaration_input(&self) -> &I {
        &self.declaration_input
    }

    pub fn contributions(&self) -> &[WorthQueryContributionIntent] {
        &self.contributions
    }

    pub fn materialization_policy(&self) -> &WorthQueryContributionComposedMaterializationPolicy {
        &self.materialization_policy
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        I,
        Vec<WorthQueryContributionIntent>,
        WorthQueryContributionComposedMaterializationPolicy,
    ) {
        (
            self.declaration_input,
            self.contributions,
            self.materialization_policy,
        )
    }
}
