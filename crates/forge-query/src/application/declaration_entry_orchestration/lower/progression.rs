use crate::application::{
    ForgeQueryDeclarationEntryOrchestrationDeferred, ForgeQueryDeclarationEntryOrchestrationDenied,
    ForgeQueryDeclarationEntryOrchestrationFailed, ForgeQueryDeclarationEntryOrchestrationOutcome,
    ForgeQueryDeclarationEntryOrchestrationPlan,
    ForgeQueryDeclarationEntryOrchestrationRebindRequired,
    ForgeQueryDeclarationEntryOrchestrationStage,
    ForgeQueryDeclarationEntryOrchestrationStageRecord,
    ForgeQueryDeclarationEntryOrchestrationStale, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationProgressionChecked, ForgeQueryDomainEntryMarker,
};

use super::super::sequencing::ForgeQueryDeclarationEntryOrchestrationAutomationContext;
use super::foundational::lower_from_progressed;

pub(super) fn lower_from_progression_checked<
    D: ForgeQueryDomainEntryMarker,
    C: crate::application::ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &crate::application::ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    plan: &ForgeQueryDeclarationEntryOrchestrationPlan<D, I>,
    automation_context: &ForgeQueryDeclarationEntryOrchestrationAutomationContext<'_>,
    step_records: &mut Vec<ForgeQueryDeclarationEntryOrchestrationStageRecord>,
    checked: ForgeQueryDeclarationProgressionChecked<D, I>,
) -> ForgeQueryDeclarationEntryOrchestrationOutcome<D, I> {
    match checked {
        ForgeQueryDeclarationProgressionChecked::Admitted(progressed) => {
            step_records.push(
                ForgeQueryDeclarationEntryOrchestrationStageRecord::automated(
                    ForgeQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
                    Some(progressed.progression_digest().to_string()),
                ),
            );
            lower_from_progressed(handle, plan, automation_context, step_records, progressed)
        }
        ForgeQueryDeclarationProgressionChecked::Deferred(progress) => {
            let digest = Some(progress.progression_digest().to_string());
            let reason = progress.progression_contract().reason();
            step_records.push(
                ForgeQueryDeclarationEntryOrchestrationStageRecord::deferred(
                    ForgeQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
                    digest.clone(),
                    reason,
                ),
            );
            ForgeQueryDeclarationEntryOrchestrationOutcome::Deferred(
                ForgeQueryDeclarationEntryOrchestrationDeferred::new(
                    progress.declaration_family_key(),
                    ForgeQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
                    reason,
                    digest,
                ),
            )
        }
        ForgeQueryDeclarationProgressionChecked::Denied(progress) => {
            let digest = Some(progress.progression_digest().to_string());
            let reason = progress.progression_contract().reason();
            step_records.push(ForgeQueryDeclarationEntryOrchestrationStageRecord::denied(
                ForgeQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
                digest.clone(),
                reason,
            ));
            ForgeQueryDeclarationEntryOrchestrationOutcome::Denied(
                ForgeQueryDeclarationEntryOrchestrationDenied::new(
                    progress.declaration_family_key(),
                    ForgeQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
                    reason,
                    digest,
                ),
            )
        }
        ForgeQueryDeclarationProgressionChecked::Stale(progress) => {
            let digest = Some(progress.progression_digest().to_string());
            let reason = "declaration progression requires stale-readable review before admission";
            step_records.push(ForgeQueryDeclarationEntryOrchestrationStageRecord::denied(
                ForgeQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
                digest.clone(),
                reason,
            ));
            ForgeQueryDeclarationEntryOrchestrationOutcome::Stale(
                ForgeQueryDeclarationEntryOrchestrationStale::new(
                    progress.declaration_family_key(),
                    ForgeQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
                    reason,
                    digest,
                ),
            )
        }
        ForgeQueryDeclarationProgressionChecked::RebindRequired(progress) => {
            let digest = Some(progress.progression_digest().to_string());
            let reason = "declaration progression requires explicit rebind before lowering";
            step_records.push(ForgeQueryDeclarationEntryOrchestrationStageRecord::denied(
                ForgeQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
                digest.clone(),
                reason,
            ));
            ForgeQueryDeclarationEntryOrchestrationOutcome::RebindRequired(
                ForgeQueryDeclarationEntryOrchestrationRebindRequired::new(
                    progress.declaration_family_key(),
                    ForgeQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
                    reason,
                    digest,
                ),
            )
        }
        ForgeQueryDeclarationProgressionChecked::Failed(progress) => {
            let digest = Some(progress.progression_digest().to_string());
            let reason = progress.progression_contract().reason();
            step_records.push(ForgeQueryDeclarationEntryOrchestrationStageRecord::failed(
                ForgeQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
                digest.clone(),
                reason,
            ));
            ForgeQueryDeclarationEntryOrchestrationOutcome::Failed(
                ForgeQueryDeclarationEntryOrchestrationFailed::new(
                    progress.declaration_family_key(),
                    ForgeQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
                    reason,
                    digest,
                ),
            )
        }
    }
}
