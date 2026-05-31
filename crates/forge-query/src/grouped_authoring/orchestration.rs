use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationEnvelope,
    ForgeQueryDeclarationEnvelopeOrchestrationTranscript, ForgeQueryDeclarationInput,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
};
use crate::identity::hash_parts;

use super::artifact::{
    ForgeQueryGroupedDeclarationArtifact, ForgeQueryGroupedDeclarationAspectRecord,
};
use super::member_lowering::lower_grouped_members_on_handle;
use super::posture::ForgeQueryGroupedMemberRole;

mod ordinary;

pub(crate) use ordinary::ordinary_outcome_from_grouped_orchestration_checked;

pub struct ForgeQueryGroupedEnvelopeMember<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    member_index: usize,
    role: ForgeQueryGroupedMemberRole,
    aspect_record: ForgeQueryGroupedDeclarationAspectRecord,
    envelope: ForgeQueryDeclarationEnvelope<D, I>,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryGroupedEnvelopeMember<D, I>
{
    pub(crate) fn new(
        member_index: usize,
        role: ForgeQueryGroupedMemberRole,
        aspect_record: ForgeQueryGroupedDeclarationAspectRecord,
        envelope: ForgeQueryDeclarationEnvelope<D, I>,
    ) -> Self {
        Self {
            member_index,
            role,
            aspect_record,
            envelope,
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

    pub fn envelope(&self) -> &ForgeQueryDeclarationEnvelope<D, I> {
        &self.envelope
    }
}

pub struct ForgeQueryGroupedOrchestration<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    declaration: ForgeQueryGroupedDeclarationArtifact<D, I>,
    member_envelopes: Vec<ForgeQueryGroupedEnvelopeMember<D, I>>,
    orchestration_digest: String,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryGroupedOrchestration<D, I>
{
    pub(crate) fn new(
        declaration: ForgeQueryGroupedDeclarationArtifact<D, I>,
        member_envelopes: Vec<ForgeQueryGroupedEnvelopeMember<D, I>>,
    ) -> Self {
        let orchestration_digest = hash_parts(&[
            format!("group:{}", declaration.group_digest()),
            format!(
                "member_envelopes:{}",
                member_envelopes
                    .iter()
                    .map(|member| format!(
                        "{}:{:?}:{}:{:?}",
                        member.member_index(),
                        member.role(),
                        format!("{:?}", member.aspect_record().coverage_basis()),
                        member.envelope().envelope_digest()
                    ))
                    .collect::<Vec<_>>()
                    .join("|")
            ),
        ]);
        Self {
            declaration,
            member_envelopes,
            orchestration_digest,
        }
    }

    pub fn declaration(&self) -> &ForgeQueryGroupedDeclarationArtifact<D, I> {
        &self.declaration
    }

    pub fn member_envelopes(&self) -> &[ForgeQueryGroupedEnvelopeMember<D, I>] {
        &self.member_envelopes
    }

    pub fn orchestration_digest(&self) -> &str {
        &self.orchestration_digest
    }
}

pub struct ForgeQueryGroupedMemberOrchestrationStop<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    declaration: ForgeQueryGroupedDeclarationArtifact<D, I>,
    member_index: usize,
    member_role: ForgeQueryGroupedMemberRole,
    member_aspect_record: ForgeQueryGroupedDeclarationAspectRecord,
    member_outcome: crate::application::ForgeQueryDeclarationEntryOrchestrationOutcome<D, I>,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryGroupedMemberOrchestrationStop<D, I>
{
    pub(crate) fn new(
        declaration: ForgeQueryGroupedDeclarationArtifact<D, I>,
        member_index: usize,
        member_role: ForgeQueryGroupedMemberRole,
        member_aspect_record: ForgeQueryGroupedDeclarationAspectRecord,
        member_outcome: crate::application::ForgeQueryDeclarationEntryOrchestrationOutcome<D, I>,
    ) -> Self {
        Self {
            declaration,
            member_index,
            member_role,
            member_aspect_record,
            member_outcome,
        }
    }

    pub fn declaration(&self) -> &ForgeQueryGroupedDeclarationArtifact<D, I> {
        &self.declaration
    }

    pub fn member_index(&self) -> usize {
        self.member_index
    }

    pub fn member_role(&self) -> ForgeQueryGroupedMemberRole {
        self.member_role
    }

    pub fn member_aspect_record(&self) -> &ForgeQueryGroupedDeclarationAspectRecord {
        &self.member_aspect_record
    }

    pub fn member_outcome(
        &self,
    ) -> &crate::application::ForgeQueryDeclarationEntryOrchestrationOutcome<D, I> {
        &self.member_outcome
    }
}

pub struct ForgeQueryGroupedOrchestrationAlignmentStop<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    declaration: ForgeQueryGroupedDeclarationArtifact<D, I>,
    reason: String,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryGroupedOrchestrationAlignmentStop<D, I>
{
    pub(crate) fn new(
        declaration: ForgeQueryGroupedDeclarationArtifact<D, I>,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            declaration,
            reason: reason.into(),
        }
    }

    pub fn declaration(&self) -> &ForgeQueryGroupedDeclarationArtifact<D, I> {
        &self.declaration
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}

pub enum ForgeQueryGroupedOrchestrationStop<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    MemberStopped(ForgeQueryGroupedMemberOrchestrationStop<D, I>),
    WrongWorld(ForgeQueryGroupedOrchestrationAlignmentStop<D, I>),
    WrongHandle(ForgeQueryGroupedOrchestrationAlignmentStop<D, I>),
}

pub enum ForgeQueryGroupedOrchestrationChecked<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    Bound(ForgeQueryGroupedOrchestration<D, I>),
    MemberStopped(ForgeQueryGroupedMemberOrchestrationStop<D, I>),
    WrongWorld(ForgeQueryGroupedOrchestrationAlignmentStop<D, I>),
    WrongHandle(ForgeQueryGroupedOrchestrationAlignmentStop<D, I>),
}

pub struct ForgeQueryGroupedOrchestrationProof<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    declaration: ForgeQueryGroupedDeclarationArtifact<D, I>,
    member_transcripts: Vec<ForgeQueryDeclarationEnvelopeOrchestrationTranscript<D, I>>,
    outcome: ForgeQueryGroupedOrchestrationChecked<D, I>,
    orchestration_digest: String,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryGroupedOrchestrationProof<D, I>
{
    pub(crate) fn new(
        declaration: ForgeQueryGroupedDeclarationArtifact<D, I>,
        member_transcripts: Vec<ForgeQueryDeclarationEnvelopeOrchestrationTranscript<D, I>>,
        outcome: ForgeQueryGroupedOrchestrationChecked<D, I>,
    ) -> Self {
        let orchestration_digest = hash_parts(&[
            format!("group:{}", declaration.group_digest()),
            format!(
                "member_proofs:{}",
                member_transcripts
                    .iter()
                    .map(|member| member.orchestration_digest().to_string())
                    .collect::<Vec<_>>()
                    .join("|")
            ),
        ]);
        Self {
            declaration,
            member_transcripts,
            outcome,
            orchestration_digest,
        }
    }

    pub fn declaration(&self) -> &ForgeQueryGroupedDeclarationArtifact<D, I> {
        &self.declaration
    }

    pub fn member_transcripts(
        &self,
    ) -> &[ForgeQueryDeclarationEnvelopeOrchestrationTranscript<D, I>] {
        &self.member_transcripts
    }

    pub fn outcome(&self) -> &ForgeQueryGroupedOrchestrationChecked<D, I> {
        &self.outcome
    }

    pub fn orchestration_digest(&self) -> &str {
        &self.orchestration_digest
    }

    pub fn into_checked(self) -> ForgeQueryGroupedOrchestrationChecked<D, I> {
        self.outcome
    }
}

pub type ForgeQueryGroupedOrchestrationTranscript<D, I> = ForgeQueryGroupedOrchestrationProof<D, I>;

pub(crate) fn forge_query_grouped_orchestration_checked_on_handle<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D> + Clone,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    declaration: ForgeQueryGroupedDeclarationArtifact<D, I>,
) -> ForgeQueryGroupedOrchestrationChecked<D, I> {
    lower_grouped_members_on_handle(handle, declaration, false).checked()
}

pub(crate) fn forge_query_grouped_orchestration_proof_on_handle<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D> + Clone,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    declaration: ForgeQueryGroupedDeclarationArtifact<D, I>,
) -> ForgeQueryGroupedOrchestrationTranscript<D, I> {
    let (checked, member_transcripts) =
        lower_grouped_members_on_handle(handle, declaration.clone(), true).into_parts();
    ForgeQueryGroupedOrchestrationProof::new(declaration, member_transcripts, checked)
}
