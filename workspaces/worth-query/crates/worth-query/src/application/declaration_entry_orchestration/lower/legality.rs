use crate::application::{
    WorthQueryDeclarationEntryOrchestrationPlan, WorthQueryDeclarationInput,
    WorthQueryDeclarationLegalityChecked, WorthQueryDeclarationLegalityDenial,
    WorthQueryDomainEntryMarker,
};

use super::super::sequencing::{
    WorthQueryDeclarationEntryOrchestrationAutomationContext,
    WorthQueryDeclarationEntryOrchestrationAutomationRefusal,
    WorthQueryDeclarationEntryOrchestrationAutomationRefusalClass,
};
use super::progression::lower_from_progression_checked;
use super::reason::legality_denial_reason;
use crate::application::{
    WorthQueryDeclarationEntryOrchestrationDeferred, WorthQueryDeclarationEntryOrchestrationDenied,
    WorthQueryDeclarationEntryOrchestrationOutcome, WorthQueryDeclarationEntryOrchestrationRefusal,
    WorthQueryDeclarationEntryOrchestrationStage,
    WorthQueryDeclarationEntryOrchestrationStageRecord,
};

#[cfg(test)]
pub(super) fn lower_from_legality_checked<
    D: WorthQueryDomainEntryMarker,
    C: crate::application::WorthQueryDomainOperatingContext<D>,
    I: WorthQueryDeclarationInput<D>,
>(
    handle: &crate::application::WorthQueryInstalledDomainDeclarationContext<D, C>,
    plan: &WorthQueryDeclarationEntryOrchestrationPlan<D, I>,
    automation_context: &WorthQueryDeclarationEntryOrchestrationAutomationContext<'_>,
    step_records: &mut Vec<WorthQueryDeclarationEntryOrchestrationStageRecord>,
    checked: WorthQueryDeclarationLegalityChecked<D, I>,
) -> WorthQueryDeclarationEntryOrchestrationOutcome<D, I> {
    match checked {
        WorthQueryDeclarationLegalityChecked::Legal(legal) => {
            step_records.push(
                WorthQueryDeclarationEntryOrchestrationStageRecord::automated(
                    WorthQueryDeclarationEntryOrchestrationStage::LegalityEstablished,
                    Some(legal.legality_digest().to_string()),
                ),
            );
            lower_from_progression_checked(
                handle,
                plan,
                automation_context,
                step_records,
                handle.progress_declaration_checked(legal),
            )
        }
        WorthQueryDeclarationLegalityChecked::Illegal(denial) => {
            lower_from_legality_denial(automation_context, step_records, denial)
        }
    }
}

#[cfg(test)]
fn lower_from_legality_denial<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>(
    automation_context: &WorthQueryDeclarationEntryOrchestrationAutomationContext<'_>,
    step_records: &mut Vec<WorthQueryDeclarationEntryOrchestrationStageRecord>,
    denial: WorthQueryDeclarationLegalityDenial<D, I>,
) -> WorthQueryDeclarationEntryOrchestrationOutcome<D, I> {
    let family = denial.declaration_family_key();
    let retained = Some(denial.declaration_digest());
    let reason = legality_denial_reason(&denial);
    match denial {
        WorthQueryDeclarationLegalityDenial::DeferredByLegalityBoundary { .. } => {
            step_records.push(
                WorthQueryDeclarationEntryOrchestrationStageRecord::deferred(
                    WorthQueryDeclarationEntryOrchestrationStage::LegalityEstablished,
                    retained.clone(),
                    reason,
                ),
            );
            WorthQueryDeclarationEntryOrchestrationOutcome::Deferred(
                WorthQueryDeclarationEntryOrchestrationDeferred::new(
                    family,
                    WorthQueryDeclarationEntryOrchestrationStage::LegalityEstablished,
                    reason,
                    retained,
                ),
            )
        }
        WorthQueryDeclarationLegalityDenial::TemporalProjectionUnsupported { kind, .. }
            if kind.is_deferred() =>
        {
            step_records.push(
                WorthQueryDeclarationEntryOrchestrationStageRecord::deferred(
                    WorthQueryDeclarationEntryOrchestrationStage::LegalityEstablished,
                    retained.clone(),
                    reason,
                ),
            );
            WorthQueryDeclarationEntryOrchestrationOutcome::Deferred(
                WorthQueryDeclarationEntryOrchestrationDeferred::new(
                    family,
                    WorthQueryDeclarationEntryOrchestrationStage::LegalityEstablished,
                    reason,
                    retained,
                ),
            )
        }
        WorthQueryDeclarationLegalityDenial::AsyncProjectionUnsupported { kind, .. }
            if kind.is_deferred() =>
        {
            step_records.push(
                WorthQueryDeclarationEntryOrchestrationStageRecord::deferred(
                    WorthQueryDeclarationEntryOrchestrationStage::LegalityEstablished,
                    retained.clone(),
                    reason,
                ),
            );
            WorthQueryDeclarationEntryOrchestrationOutcome::Deferred(
                WorthQueryDeclarationEntryOrchestrationDeferred::new(
                    family,
                    WorthQueryDeclarationEntryOrchestrationStage::LegalityEstablished,
                    reason,
                    retained,
                ),
            )
        }
        WorthQueryDeclarationLegalityDenial::UnsupportedLegalityClass { .. } => {
            step_records.push(WorthQueryDeclarationEntryOrchestrationStageRecord::refused(
                WorthQueryDeclarationEntryOrchestrationStage::LegalityEstablished,
                retained.clone(),
                reason,
            ));
            WorthQueryDeclarationEntryOrchestrationOutcome::Refused(
                WorthQueryDeclarationEntryOrchestrationRefusal::from_automation(
                    WorthQueryDeclarationEntryOrchestrationAutomationRefusal::new(
                    WorthQueryDeclarationEntryOrchestrationAutomationRefusalClass::UnsupportedAutomation,
                    WorthQueryDeclarationEntryOrchestrationStage::LegalityEstablished,
                    reason,
                    family,
                    retained,
                    automation_context.orchestration_identity_digest(),
                    automation_context.automation_boundary(),
                    ),
                    WorthQueryDeclarationEntryOrchestrationStage::LegalityEstablished,
                ),
            )
        }
        _ => {
            step_records.push(WorthQueryDeclarationEntryOrchestrationStageRecord::denied(
                WorthQueryDeclarationEntryOrchestrationStage::LegalityEstablished,
                retained.clone(),
                reason,
            ));
            WorthQueryDeclarationEntryOrchestrationOutcome::Denied(
                WorthQueryDeclarationEntryOrchestrationDenied::new(
                    family,
                    WorthQueryDeclarationEntryOrchestrationStage::LegalityEstablished,
                    reason,
                    retained,
                ),
            )
        }
    }
}
