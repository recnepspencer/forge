use std::marker::PhantomData;

use forge_foundational::FoundationalProfileSet;

use crate::application::{ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker};
use crate::domain_capabilities::{
    ForgeQueryAdmissionContributionAuthoring, ForgeQueryContinuityContributionAuthoring,
    ForgeQueryExplanationContributionAuthoring, ForgeQuerySupportContributionAuthoring,
    ForgeQueryWorkflowContributionAuthoring,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryContributionIntent {
    Admission(ForgeQueryAdmissionContributionAuthoring),
    Support(ForgeQuerySupportContributionAuthoring),
    Explanation(ForgeQueryExplanationContributionAuthoring),
    Workflow(ForgeQueryWorkflowContributionAuthoring),
    Continuity(ForgeQueryContinuityContributionAuthoring),
}

impl ForgeQueryContributionIntent {
    pub fn admission(value: ForgeQueryAdmissionContributionAuthoring) -> Self {
        Self::Admission(value)
    }

    pub fn support(value: ForgeQuerySupportContributionAuthoring) -> Self {
        Self::Support(value)
    }

    pub fn explanation(value: ForgeQueryExplanationContributionAuthoring) -> Self {
        Self::Explanation(value)
    }

    pub fn workflow(value: ForgeQueryWorkflowContributionAuthoring) -> Self {
        Self::Workflow(value)
    }

    pub fn continuity(value: ForgeQueryContinuityContributionAuthoring) -> Self {
        Self::Continuity(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryContributionComposedMaterializationPolicy {
    None,
    Summary(FoundationalProfileSet),
}

impl Default for ForgeQueryContributionComposedMaterializationPolicy {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryContributionComposedOrchestrationInput<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    declaration_input: I,
    contributions: Vec<ForgeQueryContributionIntent>,
    materialization_policy: ForgeQueryContributionComposedMaterializationPolicy,
    _marker: PhantomData<D>,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryContributionComposedOrchestrationInput<D, I>
{
    pub fn new(declaration_input: I) -> Self {
        Self {
            declaration_input,
            contributions: Vec::new(),
            materialization_policy: ForgeQueryContributionComposedMaterializationPolicy::None,
            _marker: PhantomData,
        }
    }

    pub fn with_contribution(mut self, contribution: ForgeQueryContributionIntent) -> Self {
        self.contributions.push(contribution);
        self
    }

    pub fn with_contributions(
        mut self,
        contributions: impl IntoIterator<Item = ForgeQueryContributionIntent>,
    ) -> Self {
        self.contributions.extend(contributions);
        self
    }

    pub fn materialize_summaries_with_profile(mut self, profile: FoundationalProfileSet) -> Self {
        self.materialization_policy =
            ForgeQueryContributionComposedMaterializationPolicy::Summary(profile);
        self
    }

    pub fn declaration_input(&self) -> &I {
        &self.declaration_input
    }

    pub fn contributions(&self) -> &[ForgeQueryContributionIntent] {
        &self.contributions
    }

    pub fn materialization_policy(&self) -> &ForgeQueryContributionComposedMaterializationPolicy {
        &self.materialization_policy
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        I,
        Vec<ForgeQueryContributionIntent>,
        ForgeQueryContributionComposedMaterializationPolicy,
    ) {
        (
            self.declaration_input,
            self.contributions,
            self.materialization_policy,
        )
    }
}
