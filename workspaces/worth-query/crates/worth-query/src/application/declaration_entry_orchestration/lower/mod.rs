#[cfg(test)]
use crate::application::{
    WorthQueryDeclarationFamilyMarker, WorthQueryDeclarationInput, WorthQueryDomainEntryMarker,
};

#[cfg(test)]
use super::artifacts::{
    WorthQueryDeclarationEntryOrchestrationArtifactPolicy,
    WorthQueryDeclarationEntryOrchestrationExposureLevel,
    WorthQueryDeclarationEntryOrchestrationInput, WorthQueryDeclarationEntryOrchestrationOutcome,
    WorthQueryDeclarationEntryOrchestrationPlan, WorthQueryDeclarationEntryOrchestrationStage,
    WorthQueryDeclarationEntryOrchestrationStageRecord,
};
#[cfg(test)]
use super::sequencing::WorthQueryDeclarationEntryOrchestrationAutomationContext;

#[cfg(test)]
mod entry;
#[cfg(test)]
mod envelope;
#[cfg(test)]
mod foundational;
#[cfg(test)]
mod legality;
mod product;
#[cfg(test)]
mod progression;
#[cfg(test)]
mod reason;
#[cfg(test)]
mod receipt;
#[cfg(test)]
mod route;

#[cfg(test)]
pub(crate) struct WorthQueryLoweredDeclarationEntryOrchestration<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    pub(crate) plan: WorthQueryDeclarationEntryOrchestrationPlan<D, I>,
    pub(crate) outcome: WorthQueryDeclarationEntryOrchestrationOutcome<D, I>,
    pub(crate) step_records: Vec<WorthQueryDeclarationEntryOrchestrationStageRecord>,
}

#[cfg(test)]
pub(crate) fn worth_query_lower_declaration_entry_orchestration_on_handle<
    D: WorthQueryDomainEntryMarker,
    C: crate::application::WorthQueryDomainOperatingContext<D>,
    I: WorthQueryDeclarationInput<D>,
>(
    handle: &crate::application::WorthQueryInstalledDomainDeclarationContext<D, C>,
    input: I,
    exposure_level: WorthQueryDeclarationEntryOrchestrationExposureLevel,
    artifact_policy: WorthQueryDeclarationEntryOrchestrationArtifactPolicy,
) -> WorthQueryLoweredDeclarationEntryOrchestration<D, I> {
    let orchestration_input = WorthQueryDeclarationEntryOrchestrationInput::new(
        handle.retained_world_basis(),
        I::Family::aspect_contract(),
        I::Family::aspect_coverage(),
        crate::application::WorthQueryDeclarationAspectCoverageBasis::DeclaredFamilyCoverage,
        exposure_level,
        artifact_policy,
    );
    let plan = WorthQueryDeclarationEntryOrchestrationPlan::new(orchestration_input.clone());
    let automation_context = WorthQueryDeclarationEntryOrchestrationAutomationContext::new(
        plan.orchestration_identity_digest(),
        plan.automation_boundary(),
    );
    let mut step_records = vec![
        WorthQueryDeclarationEntryOrchestrationStageRecord::admitted(
            WorthQueryDeclarationEntryOrchestrationStage::AdmittedHandle,
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
    WorthQueryLoweredDeclarationEntryOrchestration {
        plan,
        outcome,
        step_records,
    }
}

#[cfg(test)]
pub(crate) fn worth_query_checked_declaration_entry_orchestration_on_handle<
    D: WorthQueryDomainEntryMarker,
    C: crate::application::WorthQueryDomainOperatingContext<D>,
    I: WorthQueryDeclarationInput<D>,
>(
    handle: &crate::application::WorthQueryInstalledDomainDeclarationContext<D, C>,
    input: I,
) -> WorthQueryDeclarationEntryOrchestrationOutcome<D, I> {
    worth_query_lower_declaration_entry_orchestration_on_handle(
        handle,
        input,
        WorthQueryDeclarationEntryOrchestrationExposureLevel::Checked,
        WorthQueryDeclarationEntryOrchestrationArtifactPolicy::CheckedOutcomeOnly,
    )
    .outcome
}

pub(crate) use product::{
    worth_query_lower_declaration_entry_product_orchestration_from_progressed_on_handle,
    WorthQueryDeclarationEntryProductChecked,
};
