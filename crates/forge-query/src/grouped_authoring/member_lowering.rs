use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationEntryOrchestrationDeferred,
    ForgeQueryDeclarationEntryOrchestrationDenied, ForgeQueryDeclarationEntryOrchestrationFailed,
    ForgeQueryDeclarationEntryOrchestrationOutcome, ForgeQueryDeclarationEntryOrchestrationStage,
    ForgeQueryDeclarationEnvelopeChecked, ForgeQueryDeclarationEnvelopeOrchestrationTranscript,
    ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
};
use forge_foundational::facade::CanonicalDerivedDigest;

use super::artifact::ForgeQueryGroupedDeclarationArtifact;
use super::orchestration::{
    ForgeQueryGroupedEnvelopeMember, ForgeQueryGroupedMemberOrchestrationStop,
    ForgeQueryGroupedOrchestration, ForgeQueryGroupedOrchestrationAlignmentStop,
    ForgeQueryGroupedOrchestrationChecked,
};

pub(crate) struct ForgeQueryGroupedMemberLowering<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    checked: ForgeQueryGroupedOrchestrationChecked<D, I>,
    member_transcripts: Vec<ForgeQueryDeclarationEnvelopeOrchestrationTranscript<D, I>>,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryGroupedMemberLowering<D, I>
{
    pub(crate) fn checked(self) -> ForgeQueryGroupedOrchestrationChecked<D, I> {
        self.checked
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        ForgeQueryGroupedOrchestrationChecked<D, I>,
        Vec<ForgeQueryDeclarationEnvelopeOrchestrationTranscript<D, I>>,
    ) {
        (self.checked, self.member_transcripts)
    }
}

pub(crate) fn lower_grouped_members_on_handle<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D> + Clone,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    declaration: ForgeQueryGroupedDeclarationArtifact<D, I>,
    capture_proof: bool,
) -> ForgeQueryGroupedMemberLowering<D, I> {
    if declaration.operating_context_identity_digest() != handle.operating_context_identity_digest()
    {
        return ForgeQueryGroupedMemberLowering {
            checked: ForgeQueryGroupedOrchestrationChecked::WrongWorld(
                ForgeQueryGroupedOrchestrationAlignmentStop::new(
                    declaration,
                    "the grouped declaration was admitted in a different operating context",
                ),
            ),
            member_transcripts: Vec::new(),
        };
    }
    if declaration.handle_identity_digest() != handle.handle_identity_digest() {
        return ForgeQueryGroupedMemberLowering {
            checked: ForgeQueryGroupedOrchestrationChecked::WrongHandle(
                ForgeQueryGroupedOrchestrationAlignmentStop::new(
                    declaration,
                    "the grouped declaration was admitted on a different configured domain handle",
                ),
            ),
            member_transcripts: Vec::new(),
        };
    }

    let mut member_envelopes = Vec::with_capacity(declaration.members().len());
    let mut member_transcripts = Vec::with_capacity(declaration.members().len());
    for member in declaration.members() {
        if capture_proof {
            let transcript =
                handle.orchestrate_envelope_from_progressed_proof(member.progression().clone());
            let terminal_outcome = match transcript.outcome() {
                ForgeQueryDeclarationEnvelopeChecked::Enveloped(_) => None,
                terminal => Some(declaration_entry_outcome_from_envelope_checked_ref(
                    terminal,
                )),
            };
            member_transcripts.push(transcript);
            if let Some(outcome) = terminal_outcome {
                return ForgeQueryGroupedMemberLowering {
                    checked: ForgeQueryGroupedOrchestrationChecked::MemberStopped(
                        ForgeQueryGroupedMemberOrchestrationStop::new(
                            declaration.clone(),
                            member.member_index(),
                            member.role(),
                            member.aspect_record().clone(),
                            outcome,
                        ),
                    ),
                    member_transcripts,
                };
            }

            let checked =
                handle.orchestrate_envelope_from_progressed_checked(member.progression().clone());
            if let ForgeQueryDeclarationEnvelopeChecked::Enveloped(envelope) = checked {
                member_envelopes.push(ForgeQueryGroupedEnvelopeMember::new(
                    member.member_index(),
                    member.role(),
                    member.aspect_record().clone(),
                    envelope,
                ));
            }
            continue;
        }

        let checked =
            handle.orchestrate_envelope_from_progressed_checked(member.progression().clone());
        match checked {
            ForgeQueryDeclarationEnvelopeChecked::Enveloped(envelope) => {
                member_envelopes.push(ForgeQueryGroupedEnvelopeMember::new(
                    member.member_index(),
                    member.role(),
                    member.aspect_record().clone(),
                    envelope,
                ));
            }
            terminal => {
                return ForgeQueryGroupedMemberLowering {
                    checked: ForgeQueryGroupedOrchestrationChecked::MemberStopped(
                        ForgeQueryGroupedMemberOrchestrationStop::new(
                            declaration.clone(),
                            member.member_index(),
                            member.role(),
                            member.aspect_record().clone(),
                            declaration_entry_outcome_from_envelope_checked(terminal),
                        ),
                    ),
                    member_transcripts,
                };
            }
        }
    }

    ForgeQueryGroupedMemberLowering {
        checked: ForgeQueryGroupedOrchestrationChecked::Bound(ForgeQueryGroupedOrchestration::new(
            declaration,
            member_envelopes,
        )),
        member_transcripts,
    }
}

fn declaration_entry_outcome_from_envelope_checked_ref<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    checked: &ForgeQueryDeclarationEnvelopeChecked<D, I>,
) -> ForgeQueryDeclarationEntryOrchestrationOutcome<D, I> {
    match checked {
        ForgeQueryDeclarationEnvelopeChecked::Enveloped(_) => {
            panic!("terminal conversion should not receive an enveloped outcome")
        }
        ForgeQueryDeclarationEnvelopeChecked::Deferred(value) => {
            ForgeQueryDeclarationEntryOrchestrationOutcome::Deferred(
                ForgeQueryDeclarationEntryOrchestrationDeferred::new(
                    value.envelope().declaration_family_key(),
                    ForgeQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
                    value.reason(),
                    Some(canonical_digest_token(value.envelope().envelope_digest())),
                ),
            )
        }
        ForgeQueryDeclarationEnvelopeChecked::Denied(value) => {
            ForgeQueryDeclarationEntryOrchestrationOutcome::Denied(
                ForgeQueryDeclarationEntryOrchestrationDenied::new(
                    value.envelope().declaration_family_key(),
                    ForgeQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
                    value.reason(),
                    Some(canonical_digest_token(value.envelope().envelope_digest())),
                ),
            )
        }
        ForgeQueryDeclarationEnvelopeChecked::Failed(value) => {
            ForgeQueryDeclarationEntryOrchestrationOutcome::Failed(
                ForgeQueryDeclarationEntryOrchestrationFailed::new(
                    value.envelope().declaration_family_key(),
                    ForgeQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
                    value.reason(),
                    Some(canonical_digest_token(value.envelope().envelope_digest())),
                ),
            )
        }
    }
}

fn declaration_entry_outcome_from_envelope_checked<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    checked: ForgeQueryDeclarationEnvelopeChecked<D, I>,
) -> ForgeQueryDeclarationEntryOrchestrationOutcome<D, I> {
    match checked {
        ForgeQueryDeclarationEnvelopeChecked::Enveloped(envelope) => {
            ForgeQueryDeclarationEntryOrchestrationOutcome::Enveloped(envelope)
        }
        ForgeQueryDeclarationEnvelopeChecked::Deferred(value) => {
            ForgeQueryDeclarationEntryOrchestrationOutcome::Deferred(
                ForgeQueryDeclarationEntryOrchestrationDeferred::new(
                    value.envelope().declaration_family_key(),
                    ForgeQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
                    value.reason(),
                    Some(canonical_digest_token(value.envelope().envelope_digest())),
                ),
            )
        }
        ForgeQueryDeclarationEnvelopeChecked::Denied(value) => {
            ForgeQueryDeclarationEntryOrchestrationOutcome::Denied(
                ForgeQueryDeclarationEntryOrchestrationDenied::new(
                    value.envelope().declaration_family_key(),
                    ForgeQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
                    value.reason(),
                    Some(canonical_digest_token(value.envelope().envelope_digest())),
                ),
            )
        }
        ForgeQueryDeclarationEnvelopeChecked::Failed(value) => {
            ForgeQueryDeclarationEntryOrchestrationOutcome::Failed(
                ForgeQueryDeclarationEntryOrchestrationFailed::new(
                    value.envelope().declaration_family_key(),
                    ForgeQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
                    value.reason(),
                    Some(canonical_digest_token(value.envelope().envelope_digest())),
                ),
            )
        }
    }
}

fn canonical_digest_token(digest: &CanonicalDerivedDigest) -> String {
    let hex = digest
        .value()
        .bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    format!("{}:{hex}", digest.metadata().algorithm().id().as_str())
}
