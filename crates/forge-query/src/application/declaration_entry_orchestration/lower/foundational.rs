use crate::application::{
    ForgeQueryDeclarationEntryOrchestrationOutcome, ForgeQueryDeclarationEntryOrchestrationStage,
    ForgeQueryDeclarationEntryOrchestrationStageRecord,
    ForgeQueryDeclarationFoundationalEvidenceInput, ForgeQueryDeclarationInput,
    ForgeQueryDomainEntryMarker,
};

use super::super::sequencing::ForgeQueryDeclarationEntryOrchestrationAutomationContext;
use super::route::lower_from_route_checked;

pub(super) fn lower_from_progressed<
    D: ForgeQueryDomainEntryMarker,
    C: crate::application::ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &crate::application::ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    automation_context: &ForgeQueryDeclarationEntryOrchestrationAutomationContext<'_>,
    step_records: &mut Vec<ForgeQueryDeclarationEntryOrchestrationStageRecord>,
    progressed: crate::application::ForgeQueryAdmittedDeclarationProgression<D, I>,
) -> ForgeQueryDeclarationEntryOrchestrationOutcome<D, I> {
    let evidence = match handle.describe_foundational(
        ForgeQueryDeclarationFoundationalEvidenceInput::admitted_progression(progressed.clone()),
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
        ),
    );
    let route_checked = handle.plan_routes_checked(
        crate::application::ForgeQueryDeclarationRoutePlanInput::admitted(progressed, evidence),
    );
    lower_from_route_checked(handle, automation_context, step_records, route_checked)
}
