use crate::application::{
    ForgeQueryDeclarationInput, ForgeQueryDeclarationLegalityChecked,
    ForgeQueryDeclarationLegalityDenial, ForgeQueryDomainEntryMarker,
};

use super::super::sequencing::{
    ForgeQueryDeclarationEntryOrchestrationAutomationContext,
    ForgeQueryDeclarationEntryOrchestrationAutomationRefusal,
    ForgeQueryDeclarationEntryOrchestrationAutomationRefusalClass,
};
use super::progression::lower_from_progression_checked;
use super::reason::legality_denial_reason;
use crate::application::{
    ForgeQueryDeclarationEntryOrchestrationDeferred, ForgeQueryDeclarationEntryOrchestrationDenied,
    ForgeQueryDeclarationEntryOrchestrationOutcome, ForgeQueryDeclarationEntryOrchestrationRefusal,
    ForgeQueryDeclarationEntryOrchestrationStage,
    ForgeQueryDeclarationEntryOrchestrationStageRecord,
};

pub(super) fn lower_from_legality_checked<
    D: ForgeQueryDomainEntryMarker,
    C: crate::application::ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &crate::application::ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    automation_context: &ForgeQueryDeclarationEntryOrchestrationAutomationContext<'_>,
    step_records: &mut Vec<ForgeQueryDeclarationEntryOrchestrationStageRecord>,
    checked: ForgeQueryDeclarationLegalityChecked<D, I>,
) -> ForgeQueryDeclarationEntryOrchestrationOutcome<D, I> {
    match checked {
        ForgeQueryDeclarationLegalityChecked::Legal(legal) => {
            step_records.push(
                ForgeQueryDeclarationEntryOrchestrationStageRecord::automated(
                    ForgeQueryDeclarationEntryOrchestrationStage::LegalityEstablished,
                    Some(legal.legality_digest().to_string()),
                ),
            );
            lower_from_progression_checked(
                handle,
                automation_context,
                step_records,
                handle.progress_declaration_checked(legal),
            )
        }
        ForgeQueryDeclarationLegalityChecked::Illegal(denial) => {
            lower_from_legality_denial(automation_context, step_records, denial)
        }
    }
}

fn lower_from_legality_denial<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    automation_context: &ForgeQueryDeclarationEntryOrchestrationAutomationContext<'_>,
    step_records: &mut Vec<ForgeQueryDeclarationEntryOrchestrationStageRecord>,
    denial: ForgeQueryDeclarationLegalityDenial<D, I>,
) -> ForgeQueryDeclarationEntryOrchestrationOutcome<D, I> {
    let family = denial.declaration_family_key();
    let retained = Some(denial.declaration_digest());
    let reason = legality_denial_reason(&denial);
    match denial {
        ForgeQueryDeclarationLegalityDenial::DeferredByLegalityBoundary { .. } => {
            step_records.push(
                ForgeQueryDeclarationEntryOrchestrationStageRecord::deferred(
                    ForgeQueryDeclarationEntryOrchestrationStage::LegalityEstablished,
                    retained.clone(),
                    reason,
                ),
            );
            ForgeQueryDeclarationEntryOrchestrationOutcome::Deferred(
                ForgeQueryDeclarationEntryOrchestrationDeferred::new(
                    family,
                    ForgeQueryDeclarationEntryOrchestrationStage::LegalityEstablished,
                    reason,
                    retained,
                ),
            )
        }
        ForgeQueryDeclarationLegalityDenial::UnsupportedLegalityClass { .. } => {
            step_records.push(ForgeQueryDeclarationEntryOrchestrationStageRecord::refused(
                ForgeQueryDeclarationEntryOrchestrationStage::LegalityEstablished,
                retained.clone(),
                reason,
            ));
            ForgeQueryDeclarationEntryOrchestrationOutcome::Refused(
                ForgeQueryDeclarationEntryOrchestrationRefusal::from_automation(
                    ForgeQueryDeclarationEntryOrchestrationAutomationRefusal::new(
                    ForgeQueryDeclarationEntryOrchestrationAutomationRefusalClass::UnsupportedAutomation,
                    ForgeQueryDeclarationEntryOrchestrationStage::LegalityEstablished,
                    reason,
                    family,
                    retained,
                    automation_context.orchestration_identity_digest(),
                    automation_context.automation_boundary(),
                    ),
                    ForgeQueryDeclarationEntryOrchestrationStage::LegalityEstablished,
                ),
            )
        }
        _ => {
            step_records.push(ForgeQueryDeclarationEntryOrchestrationStageRecord::denied(
                ForgeQueryDeclarationEntryOrchestrationStage::LegalityEstablished,
                retained.clone(),
                reason,
            ));
            ForgeQueryDeclarationEntryOrchestrationOutcome::Denied(
                ForgeQueryDeclarationEntryOrchestrationDenied::new(
                    family,
                    ForgeQueryDeclarationEntryOrchestrationStage::LegalityEstablished,
                    reason,
                    retained,
                ),
            )
        }
    }
}
