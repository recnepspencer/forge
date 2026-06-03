use crate::application::{
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker,
};

use super::artifacts::{
    ForgeQueryDeclarationEntryOrchestrationArtifactPolicy,
    ForgeQueryDeclarationEntryOrchestrationExposureLevel,
    ForgeQueryDeclarationEntryOrchestrationInput, ForgeQueryDeclarationEntryOrchestrationOutcome,
    ForgeQueryDeclarationEntryOrchestrationPlan, ForgeQueryDeclarationEntryOrchestrationStage,
    ForgeQueryDeclarationEntryOrchestrationStageRecord,
};
use super::sequencing::ForgeQueryDeclarationEntryOrchestrationAutomationContext;

mod entry;
mod envelope;
mod foundational;
mod legality;
mod product;
mod progression;
mod reason;
mod receipt;
mod route;

pub(crate) struct ForgeQueryLoweredDeclarationEntryOrchestration<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    #[allow(dead_code)]
    pub(crate) input: ForgeQueryDeclarationEntryOrchestrationInput<D, I>,
    pub(crate) plan: ForgeQueryDeclarationEntryOrchestrationPlan<D, I>,
    pub(crate) outcome: ForgeQueryDeclarationEntryOrchestrationOutcome<D, I>,
    pub(crate) step_records: Vec<ForgeQueryDeclarationEntryOrchestrationStageRecord>,
}

pub(crate) fn forge_query_lower_declaration_entry_orchestration_on_handle<
    D: ForgeQueryDomainEntryMarker,
    C: crate::application::ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &crate::application::ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    input: I,
    exposure_level: ForgeQueryDeclarationEntryOrchestrationExposureLevel,
    artifact_policy: ForgeQueryDeclarationEntryOrchestrationArtifactPolicy,
) -> ForgeQueryLoweredDeclarationEntryOrchestration<D, I> {
    let orchestration_input = ForgeQueryDeclarationEntryOrchestrationInput::new(
        handle.retained_world_basis(),
        I::Family::aspect_contract(),
        I::Family::aspect_coverage(),
        crate::application::ForgeQueryDeclarationAspectCoverageBasis::DeclaredFamilyCoverage,
        exposure_level,
        artifact_policy,
    );
    let plan = ForgeQueryDeclarationEntryOrchestrationPlan::new(orchestration_input.clone());
    let automation_context = ForgeQueryDeclarationEntryOrchestrationAutomationContext::new(
        plan.orchestration_identity_digest(),
        plan.automation_boundary(),
    );
    let mut step_records = vec![
        ForgeQueryDeclarationEntryOrchestrationStageRecord::admitted(
            ForgeQueryDeclarationEntryOrchestrationStage::AdmittedHandle,
            Some(handle.handle_identity_digest().to_string()),
        ),
    ];
    let outcome = entry::lower_from_declaration_checked(
        handle,
        &plan,
        &automation_context,
        &mut step_records,
        input,
    );
    ForgeQueryLoweredDeclarationEntryOrchestration {
        input: orchestration_input,
        plan,
        outcome,
        step_records,
    }
}

pub(crate) fn forge_query_checked_declaration_entry_orchestration_on_handle<
    D: ForgeQueryDomainEntryMarker,
    C: crate::application::ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &crate::application::ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    input: I,
) -> ForgeQueryDeclarationEntryOrchestrationOutcome<D, I> {
    forge_query_lower_declaration_entry_orchestration_on_handle(
        handle,
        input,
        ForgeQueryDeclarationEntryOrchestrationExposureLevel::Checked,
        ForgeQueryDeclarationEntryOrchestrationArtifactPolicy::CheckedOutcomeOnly,
    )
    .outcome
}

pub(crate) use product::{
    forge_query_lower_declaration_entry_product_orchestration_from_progressed_on_handle,
    ForgeQueryDeclarationEntryProductChecked,
};
