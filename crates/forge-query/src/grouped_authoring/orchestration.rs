use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationEntryOrchestrationOutcome,
    ForgeQueryDeclarationEntryOrchestrationProof, ForgeQueryDeclarationInput,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
};
use crate::identity::hash_parts;

use super::artifact::ForgeQueryGroupedDeclarationArtifact;

mod ordinary;

pub(crate) use ordinary::ordinary_outcome_from_grouped_orchestration_checked;

pub struct ForgeQueryGroupedEnvelopeMember<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    member_index: usize,
    envelope: crate::application::ForgeQueryDeclarationEnvelope<D, I>,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryGroupedEnvelopeMember<D, I>
{
    fn new(
        member_index: usize,
        envelope: crate::application::ForgeQueryDeclarationEnvelope<D, I>,
    ) -> Self {
        Self {
            member_index,
            envelope,
        }
    }

    pub fn member_index(&self) -> usize {
        self.member_index
    }

    pub fn envelope(&self) -> &crate::application::ForgeQueryDeclarationEnvelope<D, I> {
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
    fn new(
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
                        "{}:{:?}",
                        member.member_index(),
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
    member_outcome: ForgeQueryDeclarationEntryOrchestrationOutcome<D, I>,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryGroupedMemberOrchestrationStop<D, I>
{
    fn new(
        declaration: ForgeQueryGroupedDeclarationArtifact<D, I>,
        member_index: usize,
        member_outcome: ForgeQueryDeclarationEntryOrchestrationOutcome<D, I>,
    ) -> Self {
        Self {
            declaration,
            member_index,
            member_outcome,
        }
    }

    pub fn declaration(&self) -> &ForgeQueryGroupedDeclarationArtifact<D, I> {
        &self.declaration
    }

    pub fn member_index(&self) -> usize {
        self.member_index
    }

    pub fn member_outcome(&self) -> &ForgeQueryDeclarationEntryOrchestrationOutcome<D, I> {
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
    fn new(
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
    member_transcripts: Vec<ForgeQueryDeclarationEntryOrchestrationProof<D, I>>,
    outcome: ForgeQueryGroupedOrchestrationChecked<D, I>,
    orchestration_digest: String,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryGroupedOrchestrationProof<D, I>
{
    fn new(
        declaration: ForgeQueryGroupedDeclarationArtifact<D, I>,
        member_transcripts: Vec<ForgeQueryDeclarationEntryOrchestrationProof<D, I>>,
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

    pub fn member_transcripts(&self) -> &[ForgeQueryDeclarationEntryOrchestrationProof<D, I>] {
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
    if declaration.operating_context_identity_digest() != handle.operating_context_identity_digest()
    {
        return ForgeQueryGroupedOrchestrationChecked::WrongWorld(
            ForgeQueryGroupedOrchestrationAlignmentStop::new(
                declaration,
                "the grouped declaration was admitted in a different operating context",
            ),
        );
    }
    if declaration.handle_identity_digest() != handle.handle_identity_digest() {
        return ForgeQueryGroupedOrchestrationChecked::WrongHandle(
            ForgeQueryGroupedOrchestrationAlignmentStop::new(
                declaration,
                "the grouped declaration was admitted on a different configured domain handle",
            ),
        );
    }
    let mut member_envelopes = Vec::with_capacity(declaration.members().len());
    for member in declaration.members() {
        let checked = handle.orchestrate_declaration_entry_checked(member.input().clone());
        match checked {
            ForgeQueryDeclarationEntryOrchestrationOutcome::Enveloped(envelope) => {
                member_envelopes.push(ForgeQueryGroupedEnvelopeMember::new(
                    member.member_index(),
                    envelope,
                ));
            }
            terminal => {
                let stop_declaration = declaration.clone();
                return ForgeQueryGroupedOrchestrationChecked::MemberStopped(
                    ForgeQueryGroupedMemberOrchestrationStop::new(
                        stop_declaration,
                        member.member_index(),
                        terminal,
                    ),
                );
            }
        }
    }
    ForgeQueryGroupedOrchestrationChecked::Bound(ForgeQueryGroupedOrchestration::new(
        declaration,
        member_envelopes,
    ))
}

pub(crate) fn forge_query_grouped_orchestration_proof_on_handle<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D> + Clone,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    declaration: ForgeQueryGroupedDeclarationArtifact<D, I>,
) -> ForgeQueryGroupedOrchestrationTranscript<D, I> {
    let mut member_transcripts = Vec::with_capacity(declaration.members().len());
    for member in declaration.members() {
        let transcript = handle.orchestrate_declaration_entry_proof(member.input().clone());
        let failed = !matches!(
            transcript.outcome(),
            ForgeQueryDeclarationEntryOrchestrationOutcome::Enveloped(_)
        );
        member_transcripts.push(transcript);
        if failed {
            let checked =
                forge_query_grouped_orchestration_checked_on_handle(handle, declaration.clone());
            return ForgeQueryGroupedOrchestrationProof::new(
                declaration,
                member_transcripts,
                checked,
            );
        }
    }
    let checked = forge_query_grouped_orchestration_checked_on_handle(handle, declaration.clone());
    ForgeQueryGroupedOrchestrationProof::new(declaration, member_transcripts, checked)
}
