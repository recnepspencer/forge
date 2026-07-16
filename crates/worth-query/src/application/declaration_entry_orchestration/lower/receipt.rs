use crate::application::{
    WorthQueryDeclarationEntryOrchestrationOutcome, WorthQueryDeclarationEntryOrchestrationPlan,
    WorthQueryDeclarationEntryOrchestrationRefusal, WorthQueryDeclarationEntryOrchestrationStage,
    WorthQueryDeclarationEntryOrchestrationStageRecord, WorthQueryDeclarationInput,
    WorthQueryDeclarationReceiptChecked, WorthQueryDeclarationReceiptDenialCause,
    WorthQueryDomainEntryMarker,
};

use super::super::sequencing::{
    WorthQueryDeclarationEntryOrchestrationAutomationContext,
    WorthQueryDeclarationEntryOrchestrationAutomationRefusal,
    WorthQueryDeclarationEntryOrchestrationAutomationRefusalClass,
};
use super::envelope::{
    lower_from_deferred_envelope, lower_from_denied_envelope, lower_from_failed_envelope,
    lower_from_issued_envelope,
};

#[cfg(test)]
pub(super) fn lower_from_receipt_checked<
    D: WorthQueryDomainEntryMarker,
    C: crate::application::WorthQueryDomainOperatingContext<D>,
    I: WorthQueryDeclarationInput<D>,
>(
    handle: &crate::application::WorthQueryInstalledDomainDeclarationContext<D, C>,
    plan: &WorthQueryDeclarationEntryOrchestrationPlan<D, I>,
    automation_context: &WorthQueryDeclarationEntryOrchestrationAutomationContext<'_>,
    step_records: &mut Vec<WorthQueryDeclarationEntryOrchestrationStageRecord>,
    checked: WorthQueryDeclarationReceiptChecked<D, I>,
) -> WorthQueryDeclarationEntryOrchestrationOutcome<D, I> {
    match checked {
        WorthQueryDeclarationReceiptChecked::Issued(receipt) => {
            let digest = super::super::artifacts::canonical_digest_token(receipt.receipt_digest());
            step_records.push(
                WorthQueryDeclarationEntryOrchestrationStageRecord::automated(
                    WorthQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
                    Some(digest),
                )
                .with_materialization_tier(plan.receipt_materialization_tier()),
            );
            lower_from_issued_envelope(handle, plan, step_records, receipt)
        }
        WorthQueryDeclarationReceiptChecked::Deferred(receipt) => {
            let digest =
                super::super::artifacts::canonical_digest_token(receipt.receipt().receipt_digest());
            step_records.push(
                WorthQueryDeclarationEntryOrchestrationStageRecord::deferred(
                    WorthQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
                    Some(digest.clone()),
                    receipt.reason(),
                )
                .with_materialization_tier(plan.receipt_materialization_tier()),
            );
            lower_from_deferred_envelope(handle, receipt)
        }
        WorthQueryDeclarationReceiptChecked::Denied(receipt) => {
            let receipt_stop = WorthQueryDeclarationEntryOrchestrationStage::ReceiptIssued;
            let digest =
                super::super::artifacts::canonical_digest_token(receipt.receipt().receipt_digest());

            if receipt.receipt_cause()
                == Some(WorthQueryDeclarationReceiptDenialCause::UnsupportedReceiptKind)
            {
                step_records.push(
                    WorthQueryDeclarationEntryOrchestrationStageRecord::refused(
                        receipt_stop,
                        Some(digest.clone()),
                        receipt.reason(),
                    )
                    .with_materialization_tier(plan.receipt_materialization_tier()),
                );
                return WorthQueryDeclarationEntryOrchestrationOutcome::Refused(
                    WorthQueryDeclarationEntryOrchestrationRefusal::from_automation(
                        WorthQueryDeclarationEntryOrchestrationAutomationRefusal::new(
                            WorthQueryDeclarationEntryOrchestrationAutomationRefusalClass::UnsupportedAutomation,
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
                WorthQueryDeclarationEntryOrchestrationStageRecord::denied(
                    receipt_stop,
                    Some(digest),
                    receipt.reason(),
                )
                .with_materialization_tier(plan.receipt_materialization_tier()),
            );
            lower_from_denied_envelope(handle, receipt)
        }
        WorthQueryDeclarationReceiptChecked::Failed(receipt) => {
            let digest =
                super::super::artifacts::canonical_digest_token(receipt.receipt().receipt_digest());
            step_records.push(
                WorthQueryDeclarationEntryOrchestrationStageRecord::failed(
                    WorthQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
                    Some(digest),
                    receipt.reason(),
                )
                .with_materialization_tier(plan.receipt_materialization_tier()),
            );
            lower_from_failed_envelope(handle, receipt)
        }
    }
}
