use crate::application::{
    WorthQueryDeclarationEntryOrchestrationOutcome, WorthQueryDeclarationEntryOrchestrationPlan,
    WorthQueryDeclarationEntryOrchestrationStage,
    WorthQueryDeclarationEntryOrchestrationStageRecord,
    WorthQueryDeclarationFoundationalEvidenceInput, WorthQueryDeclarationInput,
    WorthQueryDomainEntryMarker,
};

use super::super::materialization::foundational_materialization_tier;
use super::super::sequencing::WorthQueryDeclarationEntryOrchestrationAutomationContext;
use super::route::lower_from_route_checked;

#[cfg(test)]
pub(super) fn lower_from_progressed<
    D: WorthQueryDomainEntryMarker,
    C: crate::application::WorthQueryDomainOperatingContext<D>,
    I: WorthQueryDeclarationInput<D>,
>(
    handle: &crate::application::WorthQueryInstalledDomainDeclarationContext<D, C>,
    plan: &WorthQueryDeclarationEntryOrchestrationPlan<D, I>,
    automation_context: &WorthQueryDeclarationEntryOrchestrationAutomationContext<'_>,
    step_records: &mut Vec<WorthQueryDeclarationEntryOrchestrationStageRecord>,
    progressed: crate::application::WorthQueryAdmittedDeclarationProgression<D, I>,
) -> WorthQueryDeclarationEntryOrchestrationOutcome<D, I> {
    let evidence = match handle.describe_foundational_with_profile(
        WorthQueryDeclarationFoundationalEvidenceInput::admitted_progression(progressed.clone()),
        plan.foundational_evidence_profile(),
    ) {
        Ok(evidence) => evidence,
        Err(_) => {
            panic!("same-handle admitted progression should always describe foundational evidence")
        }
    };
    step_records.push(
        WorthQueryDeclarationEntryOrchestrationStageRecord::automated(
            WorthQueryDeclarationEntryOrchestrationStage::FoundationalDescribed,
            Some(super::super::artifacts::canonical_digest_token(
                evidence.attachment_bundle_digest(),
            )),
        )
        .with_materialization_tier(foundational_materialization_tier(
            plan.foundational_evidence_profile(),
        )),
    );
    let route_checked = handle.plan_routes_checked(
        crate::application::WorthQueryDeclarationRoutePlanInput::admitted(progressed, evidence),
    );
    lower_from_route_checked(
        handle,
        plan,
        automation_context,
        step_records,
        route_checked,
    )
}
