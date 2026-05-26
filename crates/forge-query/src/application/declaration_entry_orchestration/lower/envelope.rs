use crate::application::{
    ForgeQueryDeclarationEntryOrchestrationDeferred, ForgeQueryDeclarationEntryOrchestrationDenied,
    ForgeQueryDeclarationEntryOrchestrationFailed, ForgeQueryDeclarationEntryOrchestrationOutcome,
    ForgeQueryDeclarationEntryOrchestrationStage,
    ForgeQueryDeclarationEntryOrchestrationStageRecord, ForgeQueryDeclarationEnvelopeChecked,
    ForgeQueryDeclarationEnvelopeInput, ForgeQueryDeclarationInput, ForgeQueryDeclarationReceipt,
    ForgeQueryDeclarationReceiptDeferred, ForgeQueryDeclarationReceiptDenied,
    ForgeQueryDeclarationReceiptFailed, ForgeQueryDomainEntryMarker,
};

pub(super) fn lower_from_issued_envelope<
    D: ForgeQueryDomainEntryMarker,
    C: crate::application::ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &crate::application::ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    step_records: &mut Vec<ForgeQueryDeclarationEntryOrchestrationStageRecord>,
    receipt: ForgeQueryDeclarationReceipt<D, I>,
) -> ForgeQueryDeclarationEntryOrchestrationOutcome<D, I> {
    match handle.envelope_routes_checked(ForgeQueryDeclarationEnvelopeInput::issued(receipt)) {
        ForgeQueryDeclarationEnvelopeChecked::Enveloped(envelope) => {
            step_records.push(
                ForgeQueryDeclarationEntryOrchestrationStageRecord::terminal_success(
                    ForgeQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed,
                    Some(super::super::artifacts::canonical_digest_token(
                        envelope.envelope_digest(),
                    )),
                ),
            );
            ForgeQueryDeclarationEntryOrchestrationOutcome::Enveloped(envelope)
        }
        _ => panic!("issued receipts should always lower into covered envelopes"),
    }
}

pub(super) fn lower_from_deferred_envelope<
    D: ForgeQueryDomainEntryMarker,
    C: crate::application::ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &crate::application::ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    receipt: ForgeQueryDeclarationReceiptDeferred<D, I>,
) -> ForgeQueryDeclarationEntryOrchestrationOutcome<D, I> {
    match handle.envelope_routes_checked(ForgeQueryDeclarationEnvelopeInput::deferred(receipt)) {
        ForgeQueryDeclarationEnvelopeChecked::Deferred(envelope) => {
            ForgeQueryDeclarationEntryOrchestrationOutcome::Deferred(
                ForgeQueryDeclarationEntryOrchestrationDeferred::new(
                    envelope.envelope().declaration_family_key(),
                    ForgeQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
                    envelope.reason(),
                    Some(super::super::artifacts::canonical_digest_token(
                        envelope.envelope().envelope_digest(),
                    )),
                ),
            )
        }
        _ => panic!("deferred receipts should lower into deferred envelopes"),
    }
}

pub(super) fn lower_from_denied_envelope<
    D: ForgeQueryDomainEntryMarker,
    C: crate::application::ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &crate::application::ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    receipt: ForgeQueryDeclarationReceiptDenied<D, I>,
) -> ForgeQueryDeclarationEntryOrchestrationOutcome<D, I> {
    match handle.envelope_routes_checked(ForgeQueryDeclarationEnvelopeInput::denied(receipt)) {
        ForgeQueryDeclarationEnvelopeChecked::Denied(envelope) => {
            ForgeQueryDeclarationEntryOrchestrationOutcome::Denied(
                ForgeQueryDeclarationEntryOrchestrationDenied::new(
                    envelope.envelope().declaration_family_key(),
                    ForgeQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
                    envelope.reason(),
                    Some(super::super::artifacts::canonical_digest_token(
                        envelope.envelope().envelope_digest(),
                    )),
                ),
            )
        }
        _ => panic!("denied receipts should lower into denied envelopes"),
    }
}

pub(super) fn lower_from_failed_envelope<
    D: ForgeQueryDomainEntryMarker,
    C: crate::application::ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &crate::application::ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    receipt: ForgeQueryDeclarationReceiptFailed<D, I>,
) -> ForgeQueryDeclarationEntryOrchestrationOutcome<D, I> {
    match handle.envelope_routes_checked(ForgeQueryDeclarationEnvelopeInput::failed(receipt)) {
        ForgeQueryDeclarationEnvelopeChecked::Failed(envelope) => {
            ForgeQueryDeclarationEntryOrchestrationOutcome::Failed(
                ForgeQueryDeclarationEntryOrchestrationFailed::new(
                    envelope.envelope().declaration_family_key(),
                    ForgeQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
                    envelope.reason(),
                    Some(super::super::artifacts::canonical_digest_token(
                        envelope.envelope().envelope_digest(),
                    )),
                ),
            )
        }
        _ => panic!("failed receipts should lower into failed envelopes"),
    }
}
