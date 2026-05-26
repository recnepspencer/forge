use crate::application::{
    ForgeQueryDeclarationEnvelopeChecked, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationReceiptChecked, ForgeQueryDeclarationReceiptDenialCause,
    ForgeQueryDeclarationReceiptInput, ForgeQueryDeclarationRoutePlanChecked,
    ForgeQueryDeclarationRoutePlanDenialCause, ForgeQueryDomainEntryMarker,
};

use super::super::checked::{
    ForgeQueryDeclarationEntryOrchestrationChecked,
    ForgeQueryDeclarationEntryOrchestrationDeferred, ForgeQueryDeclarationEntryOrchestrationDenied,
    ForgeQueryDeclarationEntryOrchestrationFailed,
};
use super::super::proof::{
    ForgeQueryDeclarationEntryOrchestrationStage,
    ForgeQueryDeclarationEntryOrchestrationStageRecord,
};
use super::super::refusal::{
    ForgeQueryDeclarationEntryOrchestrationRefusal,
    ForgeQueryDeclarationEntryOrchestrationRefusalClass,
};
use super::canonical_digest_token;

pub(super) fn lower_from_route_checked<
    D: ForgeQueryDomainEntryMarker,
    C: crate::application::ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &crate::application::ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    stage_records: &mut Vec<ForgeQueryDeclarationEntryOrchestrationStageRecord>,
    checked: ForgeQueryDeclarationRoutePlanChecked<D, I>,
) -> ForgeQueryDeclarationEntryOrchestrationChecked<D, I> {
    match checked {
        ForgeQueryDeclarationRoutePlanChecked::Planned(plan) => {
            stage_records.push(ForgeQueryDeclarationEntryOrchestrationStageRecord::reached(
                ForgeQueryDeclarationEntryOrchestrationStage::RoutePlanned,
                Some(plan.route_plan_digest().to_string()),
            ));
            let receipt_checked =
                handle.receipt_routes_checked(ForgeQueryDeclarationReceiptInput::planned(plan));
            lower_from_receipt_checked(handle, stage_records, receipt_checked, true)
        }
        ForgeQueryDeclarationRoutePlanChecked::Deferred(plan) => {
            stage_records.push(ForgeQueryDeclarationEntryOrchestrationStageRecord::reached(
                ForgeQueryDeclarationEntryOrchestrationStage::RoutePlanned,
                None,
            ));
            let receipt_checked =
                handle.receipt_routes_checked(ForgeQueryDeclarationReceiptInput::deferred(plan));
            lower_from_receipt_checked(handle, stage_records, receipt_checked, false)
        }
        ForgeQueryDeclarationRoutePlanChecked::Denied(plan) => {
            if plan.cause() == ForgeQueryDeclarationRoutePlanDenialCause::IntentRequired {
                stage_records.push(ForgeQueryDeclarationEntryOrchestrationStageRecord::stopped(
                    ForgeQueryDeclarationEntryOrchestrationStage::RoutePlanned,
                    None,
                ));
                return ForgeQueryDeclarationEntryOrchestrationChecked::Refused(
                    ForgeQueryDeclarationEntryOrchestrationRefusal::new(
                        plan.declaration_family_key(),
                        ForgeQueryDeclarationEntryOrchestrationRefusalClass::ExplicitIntentRequired,
                        ForgeQueryDeclarationEntryOrchestrationStage::RoutePlanned,
                        plan.reason(),
                    ),
                );
            }
            stage_records.push(ForgeQueryDeclarationEntryOrchestrationStageRecord::reached(
                ForgeQueryDeclarationEntryOrchestrationStage::RoutePlanned,
                None,
            ));
            let receipt_checked =
                handle.receipt_routes_checked(ForgeQueryDeclarationReceiptInput::denied(plan));
            lower_from_receipt_checked(handle, stage_records, receipt_checked, false)
        }
        ForgeQueryDeclarationRoutePlanChecked::Failed(plan) => {
            stage_records.push(ForgeQueryDeclarationEntryOrchestrationStageRecord::reached(
                ForgeQueryDeclarationEntryOrchestrationStage::RoutePlanned,
                None,
            ));
            let receipt_checked =
                handle.receipt_routes_checked(ForgeQueryDeclarationReceiptInput::failed(plan));
            lower_from_receipt_checked(handle, stage_records, receipt_checked, false)
        }
    }
}

fn lower_from_receipt_checked<
    D: ForgeQueryDomainEntryMarker,
    C: crate::application::ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &crate::application::ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    stage_records: &mut Vec<ForgeQueryDeclarationEntryOrchestrationStageRecord>,
    checked: ForgeQueryDeclarationReceiptChecked<D, I>,
    record_receipt_stage: bool,
) -> ForgeQueryDeclarationEntryOrchestrationChecked<D, I> {
    match checked {
        ForgeQueryDeclarationReceiptChecked::Issued(receipt) => {
            stage_records.push(ForgeQueryDeclarationEntryOrchestrationStageRecord::reached(
                ForgeQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
                Some(canonical_digest_token(receipt.receipt_digest())),
            ));
            match handle.envelope_routes_checked(
                crate::application::ForgeQueryDeclarationEnvelopeInput::issued(receipt),
            ) {
                ForgeQueryDeclarationEnvelopeChecked::Enveloped(envelope) => {
                    stage_records.push(
                        ForgeQueryDeclarationEntryOrchestrationStageRecord::reached(
                            ForgeQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed,
                            Some(canonical_digest_token(envelope.envelope_digest())),
                        ),
                    );
                    ForgeQueryDeclarationEntryOrchestrationChecked::Enveloped(envelope)
                }
                _ => panic!("issued receipts should always lower into covered envelopes"),
            }
        }
        ForgeQueryDeclarationReceiptChecked::Deferred(receipt) => {
            let _ = record_receipt_stage;
            stage_records.push(ForgeQueryDeclarationEntryOrchestrationStageRecord::stopped(
                ForgeQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
                Some(canonical_digest_token(receipt.receipt().receipt_digest())),
            ));
            match handle.envelope_routes_checked(
                crate::application::ForgeQueryDeclarationEnvelopeInput::deferred(receipt),
            ) {
                ForgeQueryDeclarationEnvelopeChecked::Deferred(envelope) => {
                    ForgeQueryDeclarationEntryOrchestrationChecked::Deferred(
                        ForgeQueryDeclarationEntryOrchestrationDeferred::new(
                            envelope.envelope().declaration_family_key(),
                            ForgeQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
                            envelope.reason(),
                            Some(canonical_digest_token(
                                envelope.envelope().envelope_digest(),
                            )),
                        ),
                    )
                }
                _ => panic!("deferred receipts should lower into deferred envelopes"),
            }
        }
        ForgeQueryDeclarationReceiptChecked::Denied(receipt) => {
            if receipt.receipt_cause()
                == Some(ForgeQueryDeclarationReceiptDenialCause::UnsupportedReceiptKind)
            {
                let _ = record_receipt_stage;
                stage_records.push(ForgeQueryDeclarationEntryOrchestrationStageRecord::stopped(
                    ForgeQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
                    Some(canonical_digest_token(receipt.receipt().receipt_digest())),
                ));
                return ForgeQueryDeclarationEntryOrchestrationChecked::Refused(
                    ForgeQueryDeclarationEntryOrchestrationRefusal::new(
                        receipt.receipt().declaration_family_key(),
                        ForgeQueryDeclarationEntryOrchestrationRefusalClass::UnsupportedAutomation,
                        ForgeQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
                        receipt.reason(),
                    ),
                );
            }
            let _ = record_receipt_stage;
            stage_records.push(ForgeQueryDeclarationEntryOrchestrationStageRecord::stopped(
                ForgeQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
                Some(canonical_digest_token(receipt.receipt().receipt_digest())),
            ));
            match handle.envelope_routes_checked(
                crate::application::ForgeQueryDeclarationEnvelopeInput::denied(receipt),
            ) {
                ForgeQueryDeclarationEnvelopeChecked::Denied(envelope) => {
                    ForgeQueryDeclarationEntryOrchestrationChecked::Denied(
                        ForgeQueryDeclarationEntryOrchestrationDenied::new(
                            envelope.envelope().declaration_family_key(),
                            ForgeQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
                            envelope.reason(),
                            Some(canonical_digest_token(
                                envelope.envelope().envelope_digest(),
                            )),
                        ),
                    )
                }
                _ => panic!("denied receipts should lower into denied envelopes"),
            }
        }
        ForgeQueryDeclarationReceiptChecked::Failed(receipt) => {
            let _ = record_receipt_stage;
            stage_records.push(ForgeQueryDeclarationEntryOrchestrationStageRecord::stopped(
                ForgeQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
                Some(canonical_digest_token(receipt.receipt().receipt_digest())),
            ));
            match handle.envelope_routes_checked(
                crate::application::ForgeQueryDeclarationEnvelopeInput::failed(receipt),
            ) {
                ForgeQueryDeclarationEnvelopeChecked::Failed(envelope) => {
                    ForgeQueryDeclarationEntryOrchestrationChecked::Failed(
                        ForgeQueryDeclarationEntryOrchestrationFailed::new(
                            envelope.envelope().declaration_family_key(),
                            ForgeQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
                            envelope.reason(),
                            Some(canonical_digest_token(
                                envelope.envelope().envelope_digest(),
                            )),
                        ),
                    )
                }
                _ => panic!("failed receipts should lower into failed envelopes"),
            }
        }
    }
}
