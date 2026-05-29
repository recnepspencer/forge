use std::marker::PhantomData;

use forge_foundational::FoundationalProfileSet;

use crate::application::{ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker};
use crate::contribution_composed_orchestration::{
    orchestrate_progressed_declaration_with_contributions_checked_on_handle,
    ForgeQueryContributionComposedMaterializationPolicy,
    ForgeQueryContributionComposedOrchestration,
    ForgeQueryContributionComposedOrchestrationChecked, ForgeQueryContributionIntent,
};
use crate::domain_capabilities::{
    ForgeQueryExplanationContributionAuthoring, ForgeQuerySupportContributionAuthoring,
    ForgeQueryWorkflowContributionAuthoring,
};

use super::artifact::{
    ForgeQueryGroupedDeclarationArtifact, ForgeQueryGroupedDeclarationAspectRecord,
};
use super::declaration::{
    forge_query_grouped_declaration_checked_on_handle, ForgeQueryGroupedDeclarationChecked,
    ForgeQueryGroupedDeclarationStop,
};
use super::input::ForgeQueryGroupedDeclarationInput;
use super::posture::ForgeQueryGroupedMemberRole;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGroupedContributionAssignment {
    member_index: usize,
    contribution: ForgeQueryContributionIntent,
}

impl ForgeQueryGroupedContributionAssignment {
    fn new(member_index: usize, contribution: ForgeQueryContributionIntent) -> Self {
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGroupedContributionInput<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    declaration_input: ForgeQueryGroupedDeclarationInput<D, I>,
    shared_contributions: Vec<ForgeQueryContributionIntent>,
    member_contributions: Vec<ForgeQueryGroupedContributionAssignment>,
    materialization_profile: Option<FoundationalProfileSet>,
    _marker: PhantomData<D>,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryGroupedContributionInput<D, I>
{
    pub fn new(declaration_input: ForgeQueryGroupedDeclarationInput<D, I>) -> Self {
        Self {
            declaration_input,
            shared_contributions: Vec::new(),
            member_contributions: Vec::new(),
            materialization_profile: None,
            _marker: PhantomData,
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

    pub fn declaration_input(&self) -> &ForgeQueryGroupedDeclarationInput<D, I> {
        &self.declaration_input
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

#[derive(Clone)]
pub struct ForgeQueryGroupedContributionMemberContext {
    member_index: usize,
    role: ForgeQueryGroupedMemberRole,
    aspect_record: ForgeQueryGroupedDeclarationAspectRecord,
    shared_contribution_count: usize,
    member_contribution_count: usize,
}

impl ForgeQueryGroupedContributionMemberContext {
    fn new(
        member_index: usize,
        role: ForgeQueryGroupedMemberRole,
        aspect_record: ForgeQueryGroupedDeclarationAspectRecord,
        shared_contribution_count: usize,
        member_contribution_count: usize,
    ) -> Self {
        Self {
            member_index,
            role,
            aspect_record,
            shared_contribution_count,
            member_contribution_count,
        }
    }

    pub fn member_index(&self) -> usize {
        self.member_index
    }

    pub fn role(&self) -> ForgeQueryGroupedMemberRole {
        self.role
    }

    pub fn aspect_record(&self) -> &ForgeQueryGroupedDeclarationAspectRecord {
        &self.aspect_record
    }

    pub fn shared_contribution_count(&self) -> usize {
        self.shared_contribution_count
    }

    pub fn member_contribution_count(&self) -> usize {
        self.member_contribution_count
    }
}

pub struct ForgeQueryGroupedContributionComposition<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    declaration: ForgeQueryGroupedDeclarationArtifact<D, I>,
    members: Vec<(
        ForgeQueryGroupedContributionMemberContext,
        ForgeQueryContributionComposedOrchestration<D, I>,
    )>,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryGroupedContributionComposition<D, I>
{
    fn new(
        declaration: ForgeQueryGroupedDeclarationArtifact<D, I>,
        members: Vec<(
            ForgeQueryGroupedContributionMemberContext,
            ForgeQueryContributionComposedOrchestration<D, I>,
        )>,
    ) -> Self {
        Self {
            declaration,
            members,
        }
    }

    pub fn declaration(&self) -> &ForgeQueryGroupedDeclarationArtifact<D, I> {
        &self.declaration
    }

    pub fn members(
        &self,
    ) -> &[(
        ForgeQueryGroupedContributionMemberContext,
        ForgeQueryContributionComposedOrchestration<D, I>,
    )] {
        &self.members
    }
}

pub enum ForgeQueryGroupedContributionStop<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    DeclarationStopped(ForgeQueryGroupedDeclarationStop),
    MemberStopped(
        ForgeQueryGroupedContributionMemberContext,
        ForgeQueryContributionComposedOrchestrationChecked<D, I>,
    ),
}

pub(crate) fn forge_query_grouped_contribution_checked_on_handle<
    D: ForgeQueryDomainEntryMarker,
    C: crate::application::ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D> + Clone,
>(
    handle: &crate::application::ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    input: ForgeQueryGroupedContributionInput<D, I>,
) -> Result<ForgeQueryGroupedContributionComposition<D, I>, ForgeQueryGroupedContributionStop<D, I>>
{
    let declaration = match forge_query_grouped_declaration_checked_on_handle(
        handle,
        input.declaration_input.clone(),
    ) {
        ForgeQueryGroupedDeclarationChecked::Bound(value) => value,
        ForgeQueryGroupedDeclarationChecked::MemberStopped(stop) => {
            return Err(ForgeQueryGroupedContributionStop::DeclarationStopped(stop));
        }
    };
    lower_grouped_contributions_on_handle(handle, declaration, input)
}

fn lower_grouped_contributions_on_handle<
    D: ForgeQueryDomainEntryMarker,
    C: crate::application::ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D> + Clone,
>(
    handle: &crate::application::ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    declaration: ForgeQueryGroupedDeclarationArtifact<D, I>,
    input: ForgeQueryGroupedContributionInput<D, I>,
) -> Result<ForgeQueryGroupedContributionComposition<D, I>, ForgeQueryGroupedContributionStop<D, I>>
{
    let mut members = Vec::with_capacity(declaration.members().len());
    for member in declaration.members() {
        let member_specific = input
            .member_contributions
            .iter()
            .filter(|value| value.member_index() == member.member_index())
            .map(|value| value.contribution().clone())
            .collect::<Vec<_>>();
        let checked = orchestrate_progressed_declaration_with_contributions_checked_on_handle(
            handle,
            member.progression().clone(),
            member_contributions(&input.shared_contributions, &member_specific),
            materialization_policy(input.materialization_profile.clone()),
        );
        let context = ForgeQueryGroupedContributionMemberContext::new(
            member.member_index(),
            member.role(),
            member.aspect_record().clone(),
            input.shared_contributions.len(),
            member_specific.len(),
        );
        match checked {
            ForgeQueryContributionComposedOrchestrationChecked::Bound(bound) => {
                members.push((context, bound));
            }
            stop => {
                return Err(ForgeQueryGroupedContributionStop::MemberStopped(
                    context, stop,
                ))
            }
        }
    }
    Ok(ForgeQueryGroupedContributionComposition::new(
        declaration,
        members,
    ))
}

fn member_contributions(
    shared: &[ForgeQueryContributionIntent],
    member_specific: &[ForgeQueryContributionIntent],
) -> Vec<ForgeQueryContributionIntent> {
    shared
        .iter()
        .cloned()
        .chain(member_specific.iter().cloned())
        .collect()
}

fn materialization_policy(
    profile: Option<FoundationalProfileSet>,
) -> ForgeQueryContributionComposedMaterializationPolicy {
    match profile {
        Some(value) => ForgeQueryContributionComposedMaterializationPolicy::Summary(value),
        None => ForgeQueryContributionComposedMaterializationPolicy::None,
    }
}
