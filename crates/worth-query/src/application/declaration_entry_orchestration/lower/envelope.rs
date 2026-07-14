use crate::application::{
    WorthQueryDeclarationEntryOrchestrationDeferred, WorthQueryDeclarationEntryOrchestrationDenied,
    WorthQueryDeclarationEntryOrchestrationFailed, WorthQueryDeclarationEntryOrchestrationOutcome,
    WorthQueryDeclarationEntryOrchestrationPlan, WorthQueryDeclarationEntryOrchestrationStage,
    WorthQueryDeclarationEntryOrchestrationStageRecord, WorthQueryDeclarationEnvelopeChecked,
    WorthQueryDeclarationEnvelopeInput, WorthQueryDeclarationInput, WorthQueryDeclarationReceipt,
    WorthQueryDeclarationReceiptDeferred, WorthQueryDeclarationReceiptDenied,
    WorthQueryDeclarationReceiptFailed, WorthQueryDomainEntryMarker,
};

pub(super) fn lower_from_issued_envelope<
    D: WorthQueryDomainEntryMarker,
    C: crate::application::WorthQueryDomainOperatingContext<D>,
    I: WorthQueryDeclarationInput<D>,
>(
    handle: &crate::application::WorthQueryAdmittedConfiguredDomainHandle<D, C>,
    plan: &WorthQueryDeclarationEntryOrchestrationPlan<D, I>,
    step_records: &mut Vec<WorthQueryDeclarationEntryOrchestrationStageRecord>,
    receipt: WorthQueryDeclarationReceipt<D, I>,
) -> WorthQueryDeclarationEntryOrchestrationOutcome<D, I> {
    match handle.envelope_routes_checked(WorthQueryDeclarationEnvelopeInput::issued(receipt)) {
        WorthQueryDeclarationEnvelopeChecked::Enveloped(envelope) => {
            step_records.push(
                WorthQueryDeclarationEntryOrchestrationStageRecord::terminal_success(
                    WorthQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed,
                    Some(super::super::artifacts::canonical_digest_token(
                        envelope.envelope_digest(),
                    )),
                )
                .with_materialization_tier(plan.envelope_materialization_tier()),
            );
            WorthQueryDeclarationEntryOrchestrationOutcome::Enveloped(envelope)
        }
        _ => panic!("issued receipts should always lower into covered envelopes"),
    }
}

pub(super) fn lower_from_deferred_envelope<
    D: WorthQueryDomainEntryMarker,
    C: crate::application::WorthQueryDomainOperatingContext<D>,
    I: WorthQueryDeclarationInput<D>,
>(
    handle: &crate::application::WorthQueryAdmittedConfiguredDomainHandle<D, C>,
    receipt: WorthQueryDeclarationReceiptDeferred<D, I>,
) -> WorthQueryDeclarationEntryOrchestrationOutcome<D, I> {
    match handle.envelope_routes_checked(WorthQueryDeclarationEnvelopeInput::deferred(receipt)) {
        WorthQueryDeclarationEnvelopeChecked::Deferred(envelope) => {
            WorthQueryDeclarationEntryOrchestrationOutcome::Deferred(
                WorthQueryDeclarationEntryOrchestrationDeferred::new(
                    envelope.envelope().declaration_family_key(),
                    WorthQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
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
    D: WorthQueryDomainEntryMarker,
    C: crate::application::WorthQueryDomainOperatingContext<D>,
    I: WorthQueryDeclarationInput<D>,
>(
    handle: &crate::application::WorthQueryAdmittedConfiguredDomainHandle<D, C>,
    receipt: WorthQueryDeclarationReceiptDenied<D, I>,
) -> WorthQueryDeclarationEntryOrchestrationOutcome<D, I> {
    match handle.envelope_routes_checked(WorthQueryDeclarationEnvelopeInput::denied(receipt)) {
        WorthQueryDeclarationEnvelopeChecked::Denied(envelope) => {
            WorthQueryDeclarationEntryOrchestrationOutcome::Denied(
                WorthQueryDeclarationEntryOrchestrationDenied::new(
                    envelope.envelope().declaration_family_key(),
                    WorthQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
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
    D: WorthQueryDomainEntryMarker,
    C: crate::application::WorthQueryDomainOperatingContext<D>,
    I: WorthQueryDeclarationInput<D>,
>(
    handle: &crate::application::WorthQueryAdmittedConfiguredDomainHandle<D, C>,
    receipt: WorthQueryDeclarationReceiptFailed<D, I>,
) -> WorthQueryDeclarationEntryOrchestrationOutcome<D, I> {
    match handle.envelope_routes_checked(WorthQueryDeclarationEnvelopeInput::failed(receipt)) {
        WorthQueryDeclarationEnvelopeChecked::Failed(envelope) => {
            WorthQueryDeclarationEntryOrchestrationOutcome::Failed(
                WorthQueryDeclarationEntryOrchestrationFailed::new(
                    envelope.envelope().declaration_family_key(),
                    WorthQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
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
