use crate::application::{
    WorthQueryDeclarationFamilyMarker, WorthQueryDeclarationInput, WorthQueryDomainEntryMarker,
};

use super::super::sequencing::{
    WorthQueryDeclarationEntryOrchestrationAutomationContext,
    WorthQueryDeclarationEntryOrchestrationAutomationRefusal,
    WorthQueryDeclarationEntryOrchestrationAutomationRefusalClass,
};
use super::legality::lower_from_legality_checked;
use super::reason::{canonicalization_reason, declare_row_reason};
use crate::application::{
    WorthQueryDeclarationEntryOrchestrationDeferred, WorthQueryDeclarationEntryOrchestrationFailed,
    WorthQueryDeclarationEntryOrchestrationOutcome, WorthQueryDeclarationEntryOrchestrationPlan,
    WorthQueryDeclarationEntryOrchestrationRefusal, WorthQueryDeclarationEntryOrchestrationStage,
    WorthQueryDeclarationEntryOrchestrationStageRecord, WorthQueryDeclaredFamilyChecked,
};

pub(super) fn lower_from_declaration_checked<
    D: WorthQueryDomainEntryMarker,
    C: crate::application::WorthQueryDomainOperatingContext<D>,
    I: WorthQueryDeclarationInput<D>,
>(
    handle: &crate::application::WorthQueryAdmittedConfiguredDomainHandle<D, C>,
    plan: &WorthQueryDeclarationEntryOrchestrationPlan<D, I>,
    automation_context: &WorthQueryDeclarationEntryOrchestrationAutomationContext<'_>,
    step_records: &mut Vec<WorthQueryDeclarationEntryOrchestrationStageRecord>,
    input: I,
) -> WorthQueryDeclarationEntryOrchestrationOutcome<D, I> {
    match handle.declare_checked(input) {
        WorthQueryDeclaredFamilyChecked::Admitted(declaration) => {
            step_records.push(
                WorthQueryDeclarationEntryOrchestrationStageRecord::automated(
                    WorthQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
                    Some(super::super::artifacts::canonical_digest_token(
                        declaration.declaration_digest(),
                    )),
                ),
            );
            lower_from_legality_checked(
                handle,
                plan,
                automation_context,
                step_records,
                handle.review_legality_checked(declaration),
            )
        }
        WorthQueryDeclaredFamilyChecked::Deferred(denial) => {
            let digest = Some(denial.support_report().support_digest().to_string());
            let reason = declare_row_reason(denial.support_report());
            step_records.push(
                WorthQueryDeclarationEntryOrchestrationStageRecord::deferred(
                    WorthQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
                    digest.clone(),
                    reason,
                ),
            );
            WorthQueryDeclarationEntryOrchestrationOutcome::Deferred(
                WorthQueryDeclarationEntryOrchestrationDeferred::new(
                    denial.support_report().declaration_family_key(),
                    WorthQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
                    reason,
                    digest,
                ),
            )
        }
        WorthQueryDeclaredFamilyChecked::AsyncDeferred(denial) => {
            let digest = Some(denial.support_report().support_digest().to_string());
            let reason = "async declaration support is deferred for this family";
            step_records.push(
                WorthQueryDeclarationEntryOrchestrationStageRecord::deferred(
                    WorthQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
                    digest.clone(),
                    reason,
                ),
            );
            WorthQueryDeclarationEntryOrchestrationOutcome::Deferred(
                WorthQueryDeclarationEntryOrchestrationDeferred::new(
                    denial.support_report().declaration_family_key(),
                    WorthQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
                    reason,
                    digest,
                ),
            )
        }
        WorthQueryDeclaredFamilyChecked::TemporalDeferred(denial) => {
            let digest = Some(denial.support_report().support_digest().to_string());
            let reason = "temporal declaration support is deferred for this family";
            step_records.push(
                WorthQueryDeclarationEntryOrchestrationStageRecord::deferred(
                    WorthQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
                    digest.clone(),
                    reason,
                ),
            );
            WorthQueryDeclarationEntryOrchestrationOutcome::Deferred(
                WorthQueryDeclarationEntryOrchestrationDeferred::new(
                    denial.support_report().declaration_family_key(),
                    WorthQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
                    reason,
                    digest,
                ),
            )
        }
        WorthQueryDeclaredFamilyChecked::Unsupported(denial)
        | WorthQueryDeclaredFamilyChecked::InvalidContext(denial) => {
            let digest = Some(denial.support_report().support_digest().to_string());
            let reason = declare_row_reason(denial.support_report());
            step_records.push(WorthQueryDeclarationEntryOrchestrationStageRecord::refused(
                WorthQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
                digest.clone(),
                reason,
            ));
            WorthQueryDeclarationEntryOrchestrationOutcome::Refused(
                WorthQueryDeclarationEntryOrchestrationRefusal::from_automation(
                    WorthQueryDeclarationEntryOrchestrationAutomationRefusal::new(
                    WorthQueryDeclarationEntryOrchestrationAutomationRefusalClass::UnsupportedAutomation,
                    WorthQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
                    reason,
                    denial.support_report().declaration_family_key(),
                    digest,
                    automation_context.orchestration_identity_digest(),
                    automation_context.automation_boundary(),
                    ),
                    WorthQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
                ),
            )
        }
        WorthQueryDeclaredFamilyChecked::AsyncUnsupported(denial) => {
            let digest = Some(denial.support_report().support_digest().to_string());
            let reason = "async declaration clauses are unsupported for this family";
            step_records.push(WorthQueryDeclarationEntryOrchestrationStageRecord::refused(
                WorthQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
                digest.clone(),
                reason,
            ));
            WorthQueryDeclarationEntryOrchestrationOutcome::Refused(
                WorthQueryDeclarationEntryOrchestrationRefusal::from_automation(
                    WorthQueryDeclarationEntryOrchestrationAutomationRefusal::new(
                        WorthQueryDeclarationEntryOrchestrationAutomationRefusalClass::UnsupportedAutomation,
                        WorthQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
                        reason,
                        denial.support_report().declaration_family_key(),
                        digest,
                        automation_context.orchestration_identity_digest(),
                        automation_context.automation_boundary(),
                    ),
                    WorthQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
                ),
            )
        }
        WorthQueryDeclaredFamilyChecked::TemporalUnsupported(denial) => {
            let digest = Some(denial.support_report().support_digest().to_string());
            let reason = "temporal declaration clauses are unsupported for this family";
            step_records.push(WorthQueryDeclarationEntryOrchestrationStageRecord::refused(
                WorthQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
                digest.clone(),
                reason,
            ));
            WorthQueryDeclarationEntryOrchestrationOutcome::Refused(
                WorthQueryDeclarationEntryOrchestrationRefusal::from_automation(
                    WorthQueryDeclarationEntryOrchestrationAutomationRefusal::new(
                        WorthQueryDeclarationEntryOrchestrationAutomationRefusalClass::UnsupportedAutomation,
                        WorthQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
                        reason,
                        denial.support_report().declaration_family_key(),
                        digest,
                        automation_context.orchestration_identity_digest(),
                        automation_context.automation_boundary(),
                    ),
                    WorthQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
                ),
            )
        }
        WorthQueryDeclaredFamilyChecked::Canonicalization(error) => {
            let reason = canonicalization_reason(&error);
            step_records.push(WorthQueryDeclarationEntryOrchestrationStageRecord::failed(
                WorthQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
                None,
                reason,
            ));
            WorthQueryDeclarationEntryOrchestrationOutcome::Failed(
                WorthQueryDeclarationEntryOrchestrationFailed::new(
                    I::Family::semantic_family_key(),
                    WorthQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
                    reason,
                    None,
                ),
            )
        }
    }
}
