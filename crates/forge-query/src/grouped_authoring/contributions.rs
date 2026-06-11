use crate::application::{ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker};
use crate::contribution_composed_orchestration::{
    orchestrate_progressed_declaration_with_contributions_checked_on_handle,
    ForgeQueryContributionComposedMaterializationPolicy,
    ForgeQueryContributionComposedOrchestration,
    ForgeQueryContributionComposedOrchestrationChecked, ForgeQueryContributionIntent,
};
use forge_foundational::FoundationalProfileSet;

use super::artifact::{
    ForgeQueryGroupedDeclarationArtifact, ForgeQueryGroupedDeclarationAspectRecord,
};
use super::contribution_input::{
    ForgeQueryGroupedContributionAssignment, ForgeQueryGroupedContributionInput,
    ForgeQueryGroupedContributionSource,
};
use super::declaration::{
    forge_query_grouped_declaration_checked_on_handle, ForgeQueryGroupedDeclarationChecked,
    ForgeQueryGroupedDeclarationStop,
};
use super::orchestration::ForgeQueryGroupedOrchestrationAlignmentStop;
use super::posture::ForgeQueryGroupedMemberRole;

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
    WrongWorld(ForgeQueryGroupedOrchestrationAlignmentStop<D, I>),
    WrongHandle(ForgeQueryGroupedOrchestrationAlignmentStop<D, I>),
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
    let (source, shared_contributions, member_contributions, materialization_profile) =
        input.into_parts();
    let declaration = match source {
        ForgeQueryGroupedContributionSource::DeclarationInput(declaration_input) => {
            match forge_query_grouped_declaration_checked_on_handle(handle, declaration_input) {
                ForgeQueryGroupedDeclarationChecked::Bound(value) => value,
                ForgeQueryGroupedDeclarationChecked::MemberStopped(stop) => {
                    return Err(ForgeQueryGroupedContributionStop::DeclarationStopped(stop));
                }
            }
        }
        ForgeQueryGroupedContributionSource::DeclarationArtifact(declaration) => declaration,
    };
    if declaration.operating_context_identity_digest() != handle.operating_context_identity_digest()
    {
        return Err(ForgeQueryGroupedContributionStop::WrongWorld(
            ForgeQueryGroupedOrchestrationAlignmentStop::new(
                declaration,
                "the grouped declaration was admitted in a different operating context",
            ),
        ));
    }
    if declaration.handle_identity_digest() != handle.handle_identity_digest() {
        return Err(ForgeQueryGroupedContributionStop::WrongHandle(
            ForgeQueryGroupedOrchestrationAlignmentStop::new(
                declaration,
                "the grouped declaration was admitted on a different configured domain handle",
            ),
        ));
    }
    lower_grouped_contributions_on_handle(
        handle,
        declaration,
        shared_contributions,
        member_contributions,
        materialization_profile,
    )
}

fn lower_grouped_contributions_on_handle<
    D: ForgeQueryDomainEntryMarker,
    C: crate::application::ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D> + Clone,
>(
    handle: &crate::application::ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    declaration: ForgeQueryGroupedDeclarationArtifact<D, I>,
    shared_contributions: Vec<ForgeQueryContributionIntent>,
    member_contributions: Vec<ForgeQueryGroupedContributionAssignment>,
    materialization_profile: Option<FoundationalProfileSet>,
) -> Result<ForgeQueryGroupedContributionComposition<D, I>, ForgeQueryGroupedContributionStop<D, I>>
{
    let mut members = Vec::with_capacity(declaration.members().len());
    for member in declaration.members() {
        let member_specific = member_contributions
            .iter()
            .filter(|value| value.member_index() == member.member_index())
            .map(|value| value.contribution().clone())
            .collect::<Vec<_>>();
        let checked = orchestrate_progressed_declaration_with_contributions_checked_on_handle(
            handle,
            member.progression().clone(),
            member_contributions_for_member(&shared_contributions, &member_specific),
            materialization_policy(materialization_profile.clone()),
        );
        let context = ForgeQueryGroupedContributionMemberContext::new(
            member.member_index(),
            member.role(),
            member.aspect_record().clone(),
            shared_contributions.len(),
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

fn member_contributions_for_member(
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
