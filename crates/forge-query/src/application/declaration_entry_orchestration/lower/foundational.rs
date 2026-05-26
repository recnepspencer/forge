use crate::application::{
    ForgeQueryDeclarationEntryOrchestrationOutcome, ForgeQueryDeclarationEntryOrchestrationPlan,
    ForgeQueryDeclarationEntryOrchestrationStage,
    ForgeQueryDeclarationEntryOrchestrationStageRecord,
    ForgeQueryDeclarationFoundationalEvidenceInput, ForgeQueryDeclarationInput,
    ForgeQueryDomainEntryMarker,
};

use super::super::materialization::foundational_materialization_tier;
use super::super::sequencing::ForgeQueryDeclarationEntryOrchestrationAutomationContext;
use super::route::lower_from_route_checked;

pub(super) fn lower_from_progressed<
    D: ForgeQueryDomainEntryMarker,
    C: crate::application::ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &crate::application::ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    plan: &ForgeQueryDeclarationEntryOrchestrationPlan<D, I>,
    automation_context: &ForgeQueryDeclarationEntryOrchestrationAutomationContext<'_>,
    step_records: &mut Vec<ForgeQueryDeclarationEntryOrchestrationStageRecord>,
    progressed: crate::application::ForgeQueryAdmittedDeclarationProgression<D, I>,
) -> ForgeQueryDeclarationEntryOrchestrationOutcome<D, I> {
    let evidence = match handle.describe_foundational_with_profile(
        ForgeQueryDeclarationFoundationalEvidenceInput::admitted_progression(progressed.clone()),
        plan.foundational_evidence_profile(),
    ) {
        Ok(evidence) => evidence,
        Err(_) => {
            panic!("same-handle admitted progression should always describe foundational evidence")
        }
    };
    step_records.push(
        ForgeQueryDeclarationEntryOrchestrationStageRecord::automated(
            ForgeQueryDeclarationEntryOrchestrationStage::FoundationalDescribed,
            Some(super::super::artifacts::canonical_digest_token(
                evidence.attachment_bundle_digest(),
            )),
        )
        .with_materialization_tier(foundational_materialization_tier(
            plan.foundational_evidence_profile(),
        )),
    );
    let route_checked = handle.plan_routes_checked(
        crate::application::ForgeQueryDeclarationRoutePlanInput::admitted(progressed, evidence),
    );
    lower_from_route_checked(
        handle,
        plan,
        automation_context,
        step_records,
        route_checked,
    )
}
