use crate::application::{
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker,
};

use super::super::sequencing::{
    ForgeQueryDeclarationEntryOrchestrationAutomationContext,
    ForgeQueryDeclarationEntryOrchestrationAutomationRefusal,
    ForgeQueryDeclarationEntryOrchestrationAutomationRefusalClass,
};
use super::legality::lower_from_legality_checked;
use super::reason::{canonicalization_reason, declare_row_reason};
use crate::application::{
    ForgeQueryDeclarationEntryOrchestrationDeferred, ForgeQueryDeclarationEntryOrchestrationFailed,
    ForgeQueryDeclarationEntryOrchestrationOutcome, ForgeQueryDeclarationEntryOrchestrationPlan,
    ForgeQueryDeclarationEntryOrchestrationRefusal, ForgeQueryDeclarationEntryOrchestrationStage,
    ForgeQueryDeclarationEntryOrchestrationStageRecord, ForgeQueryDeclaredFamilyChecked,
};

pub(super) fn lower_from_declaration_checked<
    D: ForgeQueryDomainEntryMarker,
    C: crate::application::ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &crate::application::ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    plan: &ForgeQueryDeclarationEntryOrchestrationPlan<D, I>,
    automation_context: &ForgeQueryDeclarationEntryOrchestrationAutomationContext<'_>,
    step_records: &mut Vec<ForgeQueryDeclarationEntryOrchestrationStageRecord>,
    input: I,
) -> ForgeQueryDeclarationEntryOrchestrationOutcome<D, I> {
    match handle.declare_checked(input) {
        ForgeQueryDeclaredFamilyChecked::Admitted(declaration) => {
            step_records.push(
                ForgeQueryDeclarationEntryOrchestrationStageRecord::automated(
                    ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
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
        ForgeQueryDeclaredFamilyChecked::Deferred(denial) => {
            let digest = Some(denial.support_report().support_digest().to_string());
            let reason = declare_row_reason(denial.support_report());
            step_records.push(
                ForgeQueryDeclarationEntryOrchestrationStageRecord::deferred(
                    ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
                    digest.clone(),
                    reason,
                ),
            );
            ForgeQueryDeclarationEntryOrchestrationOutcome::Deferred(
                ForgeQueryDeclarationEntryOrchestrationDeferred::new(
                    denial.support_report().declaration_family_key(),
                    ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
                    reason,
                    digest,
                ),
            )
        }
        ForgeQueryDeclaredFamilyChecked::AsyncDeferred(denial) => {
            let digest = Some(denial.support_report().support_digest().to_string());
            let reason = "async declaration support is deferred for this family";
            step_records.push(
                ForgeQueryDeclarationEntryOrchestrationStageRecord::deferred(
                    ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
                    digest.clone(),
                    reason,
                ),
            );
            ForgeQueryDeclarationEntryOrchestrationOutcome::Deferred(
                ForgeQueryDeclarationEntryOrchestrationDeferred::new(
                    denial.support_report().declaration_family_key(),
                    ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
                    reason,
                    digest,
                ),
            )
        }
        ForgeQueryDeclaredFamilyChecked::TemporalDeferred(denial) => {
            let digest = Some(denial.support_report().support_digest().to_string());
            let reason = "temporal declaration support is deferred for this family";
            step_records.push(
                ForgeQueryDeclarationEntryOrchestrationStageRecord::deferred(
                    ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
                    digest.clone(),
                    reason,
                ),
            );
            ForgeQueryDeclarationEntryOrchestrationOutcome::Deferred(
                ForgeQueryDeclarationEntryOrchestrationDeferred::new(
                    denial.support_report().declaration_family_key(),
                    ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
                    reason,
                    digest,
                ),
            )
        }
        ForgeQueryDeclaredFamilyChecked::Unsupported(denial)
        | ForgeQueryDeclaredFamilyChecked::InvalidContext(denial) => {
            let digest = Some(denial.support_report().support_digest().to_string());
            let reason = declare_row_reason(denial.support_report());
            step_records.push(ForgeQueryDeclarationEntryOrchestrationStageRecord::refused(
                ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
                digest.clone(),
                reason,
            ));
            ForgeQueryDeclarationEntryOrchestrationOutcome::Refused(
                ForgeQueryDeclarationEntryOrchestrationRefusal::from_automation(
                    ForgeQueryDeclarationEntryOrchestrationAutomationRefusal::new(
                    ForgeQueryDeclarationEntryOrchestrationAutomationRefusalClass::UnsupportedAutomation,
                    ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
                    reason,
                    denial.support_report().declaration_family_key(),
                    digest,
                    automation_context.orchestration_identity_digest(),
                    automation_context.automation_boundary(),
                    ),
                    ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
                ),
            )
        }
        ForgeQueryDeclaredFamilyChecked::AsyncUnsupported(denial) => {
            let digest = Some(denial.support_report().support_digest().to_string());
            let reason = "async declaration clauses are unsupported for this family";
            step_records.push(ForgeQueryDeclarationEntryOrchestrationStageRecord::refused(
                ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
                digest.clone(),
                reason,
            ));
            ForgeQueryDeclarationEntryOrchestrationOutcome::Refused(
                ForgeQueryDeclarationEntryOrchestrationRefusal::from_automation(
                    ForgeQueryDeclarationEntryOrchestrationAutomationRefusal::new(
                        ForgeQueryDeclarationEntryOrchestrationAutomationRefusalClass::UnsupportedAutomation,
                        ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
                        reason,
                        denial.support_report().declaration_family_key(),
                        digest,
                        automation_context.orchestration_identity_digest(),
                        automation_context.automation_boundary(),
                    ),
                    ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
                ),
            )
        }
        ForgeQueryDeclaredFamilyChecked::TemporalUnsupported(denial) => {
            let digest = Some(denial.support_report().support_digest().to_string());
            let reason = "temporal declaration clauses are unsupported for this family";
            step_records.push(ForgeQueryDeclarationEntryOrchestrationStageRecord::refused(
                ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
                digest.clone(),
                reason,
            ));
            ForgeQueryDeclarationEntryOrchestrationOutcome::Refused(
                ForgeQueryDeclarationEntryOrchestrationRefusal::from_automation(
                    ForgeQueryDeclarationEntryOrchestrationAutomationRefusal::new(
                        ForgeQueryDeclarationEntryOrchestrationAutomationRefusalClass::UnsupportedAutomation,
                        ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
                        reason,
                        denial.support_report().declaration_family_key(),
                        digest,
                        automation_context.orchestration_identity_digest(),
                        automation_context.automation_boundary(),
                    ),
                    ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
                ),
            )
        }
        ForgeQueryDeclaredFamilyChecked::Canonicalization(error) => {
            let reason = canonicalization_reason(&error);
            step_records.push(ForgeQueryDeclarationEntryOrchestrationStageRecord::failed(
                ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
                None,
                reason,
            ));
            ForgeQueryDeclarationEntryOrchestrationOutcome::Failed(
                ForgeQueryDeclarationEntryOrchestrationFailed::new(
                    I::Family::semantic_family_key(),
                    ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
                    reason,
                    None,
                ),
            )
        }
    }
}
