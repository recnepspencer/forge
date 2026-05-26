use crate::application::{
    ForgeQueryDeclarationEntryOrchestrationOutcome, ForgeQueryDeclarationEntryOrchestrationPlan,
    ForgeQueryDeclarationEntryOrchestrationRefusal, ForgeQueryDeclarationEntryOrchestrationStage,
    ForgeQueryDeclarationEntryOrchestrationStageRecord, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationReceiptChecked, ForgeQueryDeclarationReceiptDenialCause,
    ForgeQueryDomainEntryMarker,
};

use super::super::sequencing::{
    ForgeQueryDeclarationEntryOrchestrationAutomationContext,
    ForgeQueryDeclarationEntryOrchestrationAutomationRefusal,
    ForgeQueryDeclarationEntryOrchestrationAutomationRefusalClass,
};
use super::envelope::{
    lower_from_deferred_envelope, lower_from_denied_envelope, lower_from_failed_envelope,
    lower_from_issued_envelope,
};

pub(super) fn lower_from_receipt_checked<
    D: ForgeQueryDomainEntryMarker,
    C: crate::application::ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &crate::application::ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    plan: &ForgeQueryDeclarationEntryOrchestrationPlan<D, I>,
    automation_context: &ForgeQueryDeclarationEntryOrchestrationAutomationContext<'_>,
    step_records: &mut Vec<ForgeQueryDeclarationEntryOrchestrationStageRecord>,
    checked: ForgeQueryDeclarationReceiptChecked<D, I>,
) -> ForgeQueryDeclarationEntryOrchestrationOutcome<D, I> {
    match checked {
        ForgeQueryDeclarationReceiptChecked::Issued(receipt) => {
            let digest = super::super::artifacts::canonical_digest_token(receipt.receipt_digest());
            step_records.push(
                ForgeQueryDeclarationEntryOrchestrationStageRecord::automated(
                    ForgeQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
                    Some(digest),
                )
                .with_materialization_tier(plan.receipt_materialization_tier()),
            );
            lower_from_issued_envelope(handle, plan, step_records, receipt)
        }
        ForgeQueryDeclarationReceiptChecked::Deferred(receipt) => {
            let digest =
                super::super::artifacts::canonical_digest_token(receipt.receipt().receipt_digest());
            step_records.push(
                ForgeQueryDeclarationEntryOrchestrationStageRecord::deferred(
                    ForgeQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
                    Some(digest.clone()),
                    receipt.reason(),
                )
                .with_materialization_tier(plan.receipt_materialization_tier()),
            );
            lower_from_deferred_envelope(handle, receipt)
        }
        ForgeQueryDeclarationReceiptChecked::Denied(receipt) => {
            let receipt_stop = ForgeQueryDeclarationEntryOrchestrationStage::ReceiptIssued;
            let digest =
                super::super::artifacts::canonical_digest_token(receipt.receipt().receipt_digest());

            if receipt.receipt_cause()
                == Some(ForgeQueryDeclarationReceiptDenialCause::UnsupportedReceiptKind)
            {
                step_records.push(
                    ForgeQueryDeclarationEntryOrchestrationStageRecord::refused(
                        receipt_stop,
                        Some(digest.clone()),
                        receipt.reason(),
                    )
                    .with_materialization_tier(plan.receipt_materialization_tier()),
                );
                return ForgeQueryDeclarationEntryOrchestrationOutcome::Refused(
                    ForgeQueryDeclarationEntryOrchestrationRefusal::from_automation(
                        ForgeQueryDeclarationEntryOrchestrationAutomationRefusal::new(
                            ForgeQueryDeclarationEntryOrchestrationAutomationRefusalClass::UnsupportedAutomation,
                            receipt_stop,
                            receipt.reason(),
                            receipt.receipt().declaration_family_key(),
                            Some(digest),
                            automation_context.orchestration_identity_digest(),
                            automation_context.automation_boundary(),
                        ),
                        receipt_stop,
                    ),
                );
            }

            step_records.push(
                ForgeQueryDeclarationEntryOrchestrationStageRecord::denied(
                    receipt_stop,
                    Some(digest),
                    receipt.reason(),
                )
                .with_materialization_tier(plan.receipt_materialization_tier()),
            );
            lower_from_denied_envelope(handle, receipt)
        }
        ForgeQueryDeclarationReceiptChecked::Failed(receipt) => {
            let digest =
                super::super::artifacts::canonical_digest_token(receipt.receipt().receipt_digest());
            step_records.push(
                ForgeQueryDeclarationEntryOrchestrationStageRecord::failed(
                    ForgeQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
                    Some(digest),
                    receipt.reason(),
                )
                .with_materialization_tier(plan.receipt_materialization_tier()),
            );
            lower_from_failed_envelope(handle, receipt)
        }
    }
}
