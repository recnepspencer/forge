use forge_foundational::FoundationalProfileSet;

use crate::application::{ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker};
use crate::contribution_composed_orchestration::ForgeQueryContributionIntent;
use crate::domain_capabilities::{
    ForgeQueryExplanationContributionAuthoring, ForgeQuerySupportContributionAuthoring,
    ForgeQueryWorkflowContributionAuthoring,
};

use super::artifact::ForgeQueryGroupedDeclarationArtifact;
use super::input::ForgeQueryGroupedDeclarationInput;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGroupedContributionAssignment {
    member_index: usize,
    contribution: ForgeQueryContributionIntent,
}

impl ForgeQueryGroupedContributionAssignment {
    pub(crate) fn new(member_index: usize, contribution: ForgeQueryContributionIntent) -> Self {
        Self {
            member_index,
            contribution,
        }
    }

    pub fn member_index(&self) -> usize {
        self.member_index
    }

    pub fn contribution(&self) -> &ForgeQueryContributionIntent {
        &self.contribution
    }
}

#[derive(Clone)]
pub(crate) enum ForgeQueryGroupedContributionSource<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    DeclarationInput(ForgeQueryGroupedDeclarationInput<D, I>),
    DeclarationArtifact(ForgeQueryGroupedDeclarationArtifact<D, I>),
}

#[derive(Clone)]
pub struct ForgeQueryGroupedContributionInput<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    source: ForgeQueryGroupedContributionSource<D, I>,
    shared_contributions: Vec<ForgeQueryContributionIntent>,
    member_contributions: Vec<ForgeQueryGroupedContributionAssignment>,
    materialization_profile: Option<FoundationalProfileSet>,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryGroupedContributionInput<D, I>
{
    pub fn new(declaration_input: ForgeQueryGroupedDeclarationInput<D, I>) -> Self {
        Self {
            source: ForgeQueryGroupedContributionSource::DeclarationInput(declaration_input),
            shared_contributions: Vec::new(),
            member_contributions: Vec::new(),
            materialization_profile: None,
        }
    }

    pub fn from_declaration(declaration: ForgeQueryGroupedDeclarationArtifact<D, I>) -> Self {
        Self {
            source: ForgeQueryGroupedContributionSource::DeclarationArtifact(declaration),
            shared_contributions: Vec::new(),
            member_contributions: Vec::new(),
            materialization_profile: None,
        }
    }

    pub fn with_shared_contribution(mut self, contribution: ForgeQueryContributionIntent) -> Self {
        self.shared_contributions.push(contribution);
        self
    }

    pub fn with_shared_support_contribution(
        self,
        contribution: ForgeQuerySupportContributionAuthoring,
    ) -> Self {
        self.with_shared_contribution(ForgeQueryContributionIntent::support(contribution))
    }

    pub fn with_shared_explanation_contribution(
        self,
        contribution: ForgeQueryExplanationContributionAuthoring,
    ) -> Self {
        self.with_shared_contribution(ForgeQueryContributionIntent::explanation(contribution))
    }

    pub fn with_shared_workflow_contribution(
        self,
        contribution: ForgeQueryWorkflowContributionAuthoring,
    ) -> Self {
        self.with_shared_contribution(ForgeQueryContributionIntent::workflow(contribution))
    }

    pub fn with_member_contribution(
        mut self,
        member_index: usize,
        contribution: ForgeQueryContributionIntent,
    ) -> Self {
        self.member_contributions
            .push(ForgeQueryGroupedContributionAssignment::new(
                member_index,
                contribution,
            ));
        self
    }

    pub fn materialize_summaries_with_profile(mut self, profile: FoundationalProfileSet) -> Self {
        self.materialization_profile = Some(profile);
        self
    }

    pub fn declaration_input(&self) -> Option<&ForgeQueryGroupedDeclarationInput<D, I>> {
        match &self.source {
            ForgeQueryGroupedContributionSource::DeclarationInput(input) => Some(input),
            ForgeQueryGroupedContributionSource::DeclarationArtifact(_) => None,
        }
    }

    pub fn declaration(&self) -> Option<&ForgeQueryGroupedDeclarationArtifact<D, I>> {
        match &self.source {
            ForgeQueryGroupedContributionSource::DeclarationInput(_) => None,
            ForgeQueryGroupedContributionSource::DeclarationArtifact(declaration) => {
                Some(declaration)
            }
        }
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        ForgeQueryGroupedContributionSource<D, I>,
        Vec<ForgeQueryContributionIntent>,
        Vec<ForgeQueryGroupedContributionAssignment>,
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

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryGroupedDeclarationInput<D, I>
{
    pub fn with_shared_contribution(
        self,
        contribution: ForgeQueryContributionIntent,
    ) -> ForgeQueryGroupedContributionInput<D, I> {
        ForgeQueryGroupedContributionInput::new(self).with_shared_contribution(contribution)
    }

    pub fn with_shared_explanation_contribution(
        self,
        contribution: ForgeQueryExplanationContributionAuthoring,
    ) -> ForgeQueryGroupedContributionInput<D, I> {
        ForgeQueryGroupedContributionInput::new(self)
            .with_shared_explanation_contribution(contribution)
    }

    pub fn with_shared_support_contribution(
        self,
        contribution: ForgeQuerySupportContributionAuthoring,
    ) -> ForgeQueryGroupedContributionInput<D, I> {
        ForgeQueryGroupedContributionInput::new(self).with_shared_support_contribution(contribution)
    }

    pub fn with_member_contribution(
        self,
        member_index: usize,
        contribution: ForgeQueryContributionIntent,
    ) -> ForgeQueryGroupedContributionInput<D, I> {
        ForgeQueryGroupedContributionInput::new(self)
            .with_member_contribution(member_index, contribution)
    }
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryGroupedDeclarationArtifact<D, I>
{
    pub fn with_shared_contribution(
        self,
        contribution: ForgeQueryContributionIntent,
    ) -> ForgeQueryGroupedContributionInput<D, I> {
        ForgeQueryGroupedContributionInput::from_declaration(self)
            .with_shared_contribution(contribution)
    }

    pub fn with_shared_explanation_contribution(
        self,
        contribution: ForgeQueryExplanationContributionAuthoring,
    ) -> ForgeQueryGroupedContributionInput<D, I> {
        ForgeQueryGroupedContributionInput::from_declaration(self)
            .with_shared_explanation_contribution(contribution)
    }

    pub fn with_shared_support_contribution(
        self,
        contribution: ForgeQuerySupportContributionAuthoring,
    ) -> ForgeQueryGroupedContributionInput<D, I> {
        ForgeQueryGroupedContributionInput::from_declaration(self)
            .with_shared_support_contribution(contribution)
    }

    pub fn with_member_contribution(
        self,
        member_index: usize,
        contribution: ForgeQueryContributionIntent,
    ) -> ForgeQueryGroupedContributionInput<D, I> {
        ForgeQueryGroupedContributionInput::from_declaration(self)
            .with_member_contribution(member_index, contribution)
    }
}
