use worth_foundational::FoundationalProfileSet;

use crate::application::{WorthQueryDeclarationInput, WorthQueryDomainEntryMarker};
use crate::contribution_composed_orchestration::WorthQueryContributionIntent;
use crate::domain_capabilities::{
    WorthQueryExplanationContributionAuthoring, WorthQuerySupportContributionAuthoring,
    WorthQueryWorkflowContributionAuthoring,
};

use super::artifact::WorthQueryGroupedDeclarationArtifact;
use super::input::WorthQueryGroupedDeclarationInput;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGroupedContributionAssignment {
    member_index: usize,
    contribution: WorthQueryContributionIntent,
}

impl WorthQueryGroupedContributionAssignment {
    pub(crate) fn new(member_index: usize, contribution: WorthQueryContributionIntent) -> Self {
        Self {
            member_index,
            contribution,
        }
    }

    pub fn member_index(&self) -> usize {
        self.member_index
    }

    pub fn contribution(&self) -> &WorthQueryContributionIntent {
        &self.contribution
    }
}

#[derive(Clone)]
pub(crate) enum WorthQueryGroupedContributionSource<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    DeclarationInput(WorthQueryGroupedDeclarationInput<D, I>),
    DeclarationArtifact(WorthQueryGroupedDeclarationArtifact<D, I>),
}

#[derive(Clone)]
pub struct WorthQueryGroupedContributionInput<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    source: WorthQueryGroupedContributionSource<D, I>,
    shared_contributions: Vec<WorthQueryContributionIntent>,
    member_contributions: Vec<WorthQueryGroupedContributionAssignment>,
    materialization_profile: Option<FoundationalProfileSet>,
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryGroupedContributionInput<D, I>
{
    pub fn new(declaration_input: WorthQueryGroupedDeclarationInput<D, I>) -> Self {
        Self {
            source: WorthQueryGroupedContributionSource::DeclarationInput(declaration_input),
            shared_contributions: Vec::new(),
            member_contributions: Vec::new(),
            materialization_profile: None,
        }
    }

    pub fn from_declaration(declaration: WorthQueryGroupedDeclarationArtifact<D, I>) -> Self {
        Self {
            source: WorthQueryGroupedContributionSource::DeclarationArtifact(declaration),
            shared_contributions: Vec::new(),
            member_contributions: Vec::new(),
            materialization_profile: None,
        }
    }

    pub fn with_shared_contribution(mut self, contribution: WorthQueryContributionIntent) -> Self {
        self.shared_contributions.push(contribution);
        self
    }

    pub fn with_shared_support_contribution(
        self,
        contribution: WorthQuerySupportContributionAuthoring,
    ) -> Self {
        self.with_shared_contribution(WorthQueryContributionIntent::support(contribution))
    }

    pub fn with_shared_explanation_contribution(
        self,
        contribution: WorthQueryExplanationContributionAuthoring,
    ) -> Self {
        self.with_shared_contribution(WorthQueryContributionIntent::explanation(contribution))
    }

    pub fn with_shared_workflow_contribution(
        self,
        contribution: WorthQueryWorkflowContributionAuthoring,
    ) -> Self {
        self.with_shared_contribution(WorthQueryContributionIntent::workflow(contribution))
    }

    pub fn with_member_contribution(
        mut self,
        member_index: usize,
        contribution: WorthQueryContributionIntent,
    ) -> Self {
        self.member_contributions
            .push(WorthQueryGroupedContributionAssignment::new(
                member_index,
                contribution,
            ));
        self
    }

    pub fn materialize_summaries_with_profile(mut self, profile: FoundationalProfileSet) -> Self {
        self.materialization_profile = Some(profile);
        self
    }

    pub fn declaration_input(&self) -> Option<&WorthQueryGroupedDeclarationInput<D, I>> {
        match &self.source {
            WorthQueryGroupedContributionSource::DeclarationInput(input) => Some(input),
            WorthQueryGroupedContributionSource::DeclarationArtifact(_) => None,
        }
    }

    pub fn declaration(&self) -> Option<&WorthQueryGroupedDeclarationArtifact<D, I>> {
        match &self.source {
            WorthQueryGroupedContributionSource::DeclarationInput(_) => None,
            WorthQueryGroupedContributionSource::DeclarationArtifact(declaration) => {
                Some(declaration)
            }
        }
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        WorthQueryGroupedContributionSource<D, I>,
        Vec<WorthQueryContributionIntent>,
        Vec<WorthQueryGroupedContributionAssignment>,
        Option<FoundationalProfileSet>,
    ) {
        (
            self.source,
            self.shared_contributions,
            self.member_contributions,
            self.materialization_profile,
        )
    }
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryGroupedDeclarationInput<D, I>
{
    pub fn with_shared_contribution(
        self,
        contribution: WorthQueryContributionIntent,
    ) -> WorthQueryGroupedContributionInput<D, I> {
        WorthQueryGroupedContributionInput::new(self).with_shared_contribution(contribution)
    }

    pub fn with_shared_explanation_contribution(
        self,
        contribution: WorthQueryExplanationContributionAuthoring,
    ) -> WorthQueryGroupedContributionInput<D, I> {
        WorthQueryGroupedContributionInput::new(self)
            .with_shared_explanation_contribution(contribution)
    }

    pub fn with_shared_support_contribution(
        self,
        contribution: WorthQuerySupportContributionAuthoring,
    ) -> WorthQueryGroupedContributionInput<D, I> {
        WorthQueryGroupedContributionInput::new(self).with_shared_support_contribution(contribution)
    }

    pub fn with_member_contribution(
        self,
        member_index: usize,
        contribution: WorthQueryContributionIntent,
    ) -> WorthQueryGroupedContributionInput<D, I> {
        WorthQueryGroupedContributionInput::new(self)
            .with_member_contribution(member_index, contribution)
    }
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryGroupedDeclarationArtifact<D, I>
{
    pub fn with_shared_contribution(
        self,
        contribution: WorthQueryContributionIntent,
    ) -> WorthQueryGroupedContributionInput<D, I> {
        WorthQueryGroupedContributionInput::from_declaration(self)
            .with_shared_contribution(contribution)
    }

    pub fn with_shared_explanation_contribution(
        self,
        contribution: WorthQueryExplanationContributionAuthoring,
    ) -> WorthQueryGroupedContributionInput<D, I> {
        WorthQueryGroupedContributionInput::from_declaration(self)
            .with_shared_explanation_contribution(contribution)
    }

    pub fn with_shared_support_contribution(
        self,
        contribution: WorthQuerySupportContributionAuthoring,
    ) -> WorthQueryGroupedContributionInput<D, I> {
        WorthQueryGroupedContributionInput::from_declaration(self)
            .with_shared_support_contribution(contribution)
    }

    pub fn with_member_contribution(
        self,
        member_index: usize,
        contribution: WorthQueryContributionIntent,
    ) -> WorthQueryGroupedContributionInput<D, I> {
        WorthQueryGroupedContributionInput::from_declaration(self)
            .with_member_contribution(member_index, contribution)
    }
}
