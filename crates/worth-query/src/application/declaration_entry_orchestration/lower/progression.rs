use crate::application::{
    WorthQueryDeclarationEntryOrchestrationDeferred, WorthQueryDeclarationEntryOrchestrationDenied,
    WorthQueryDeclarationEntryOrchestrationFailed, WorthQueryDeclarationEntryOrchestrationOutcome,
    WorthQueryDeclarationEntryOrchestrationPlan,
    WorthQueryDeclarationEntryOrchestrationRebindRequired,
    WorthQueryDeclarationEntryOrchestrationStage,
    WorthQueryDeclarationEntryOrchestrationStageRecord,
    WorthQueryDeclarationEntryOrchestrationStale, WorthQueryDeclarationInput,
    WorthQueryDeclarationProgressionChecked, WorthQueryDomainEntryMarker,
};

use super::super::sequencing::WorthQueryDeclarationEntryOrchestrationAutomationContext;
use super::foundational::lower_from_progressed;

#[cfg(test)]
pub(super) fn lower_from_progression_checked<
    D: WorthQueryDomainEntryMarker,
    C: crate::application::WorthQueryDomainOperatingContext<D>,
    I: WorthQueryDeclarationInput<D>,
>(
    handle: &crate::application::WorthQueryInstalledDomainDeclarationContext<D, C>,
    plan: &WorthQueryDeclarationEntryOrchestrationPlan<D, I>,
    automation_context: &WorthQueryDeclarationEntryOrchestrationAutomationContext<'_>,
    step_records: &mut Vec<WorthQueryDeclarationEntryOrchestrationStageRecord>,
    checked: WorthQueryDeclarationProgressionChecked<D, I>,
) -> WorthQueryDeclarationEntryOrchestrationOutcome<D, I> {
    match checked {
        WorthQueryDeclarationProgressionChecked::Admitted(progressed) => {
            step_records.push(
                WorthQueryDeclarationEntryOrchestrationStageRecord::automated(
                    WorthQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
                    Some(progressed.progression_digest().to_string()),
                ),
            );
            lower_from_progressed(handle, plan, automation_context, step_records, progressed)
        }
        WorthQueryDeclarationProgressionChecked::Deferred(progress) => {
            let digest = Some(progress.progression_digest().to_string());
            let reason = progress.progression_contract().reason();
            step_records.push(
                WorthQueryDeclarationEntryOrchestrationStageRecord::deferred(
                    WorthQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
                    digest.clone(),
                    reason,
                ),
            );
            WorthQueryDeclarationEntryOrchestrationOutcome::Deferred(
                WorthQueryDeclarationEntryOrchestrationDeferred::new(
                    progress.declaration_family_key(),
                    WorthQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
                    reason,
                    digest,
                ),
            )
        }
        WorthQueryDeclarationProgressionChecked::Denied(progress) => {
            let digest = Some(progress.progression_digest().to_string());
            let reason = progress.progression_contract().reason();
            step_records.push(WorthQueryDeclarationEntryOrchestrationStageRecord::denied(
                WorthQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
                digest.clone(),
                reason,
            ));
            WorthQueryDeclarationEntryOrchestrationOutcome::Denied(
                WorthQueryDeclarationEntryOrchestrationDenied::new(
                    progress.declaration_family_key(),
                    WorthQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
                    reason,
                    digest,
                ),
            )
        }
        WorthQueryDeclarationProgressionChecked::Stale(progress) => {
            let digest = Some(progress.progression_digest().to_string());
            let reason = "declaration progression requires stale-readable review before admission";
            step_records.push(WorthQueryDeclarationEntryOrchestrationStageRecord::denied(
                WorthQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
                digest.clone(),
                reason,
            ));
            WorthQueryDeclarationEntryOrchestrationOutcome::Stale(
                WorthQueryDeclarationEntryOrchestrationStale::new(
                    progress.declaration_family_key(),
                    WorthQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
                    reason,
                    digest,
                ),
            )
        }
        WorthQueryDeclarationProgressionChecked::RebindRequired(progress) => {
            let digest = Some(progress.progression_digest().to_string());
            let reason = "declaration progression requires explicit rebind before lowering";
            step_records.push(WorthQueryDeclarationEntryOrchestrationStageRecord::denied(
                WorthQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
                digest.clone(),
                reason,
            ));
            WorthQueryDeclarationEntryOrchestrationOutcome::RebindRequired(
                WorthQueryDeclarationEntryOrchestrationRebindRequired::new(
                    progress.declaration_family_key(),
                    WorthQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
                    reason,
                    digest,
                ),
            )
        }
        WorthQueryDeclarationProgressionChecked::Failed(progress) => {
            let digest = Some(progress.progression_digest().to_string());
            let reason = progress.progression_contract().reason();
            step_records.push(WorthQueryDeclarationEntryOrchestrationStageRecord::failed(
                WorthQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
                digest.clone(),
                reason,
            ));
            WorthQueryDeclarationEntryOrchestrationOutcome::Failed(
                WorthQueryDeclarationEntryOrchestrationFailed::new(
                    progress.declaration_family_key(),
                    WorthQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
                    reason,
                    digest,
                ),
            )
        }
    }
}
