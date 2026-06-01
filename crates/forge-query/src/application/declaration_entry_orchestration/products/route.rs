use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryAdmittedDeclarationProgression,
    ForgeQueryDeclarationEntryOrchestrationArtifactPolicy,
    ForgeQueryDeclarationEntryOrchestrationExposureLevel,
    ForgeQueryDeclarationEntryOrchestrationProduct, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationRouteIntent, ForgeQueryDeclarationRoutePlan,
    ForgeQueryDeclarationRoutePlanChecked, ForgeQueryDeclarationRoutePlanTerminalError,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
};

use super::common::{route_orchestration_identity, route_terminal_from_checked};
use super::transcript::ForgeQueryDeclarationRouteOrchestrationTranscript;
use crate::application::{
    forge_query_lower_declaration_entry_product_orchestration_from_progressed_on_handle,
    ForgeQueryDeclarationEntryProductChecked,
};

pub(crate) fn forge_query_declaration_route_orchestration_from_progressed_on_handle<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    progressed: ForgeQueryAdmittedDeclarationProgression<D, I>,
    route_intent: Option<ForgeQueryDeclarationRouteIntent>,
) -> Result<ForgeQueryDeclarationRoutePlan<D, I>, ForgeQueryDeclarationRoutePlanTerminalError<D, I>>
{
    match forge_query_checked_declaration_route_orchestration_from_progressed_on_handle(
        handle,
        progressed,
        route_intent,
    ) {
        ForgeQueryDeclarationRoutePlanChecked::Planned(plan) => Ok(plan),
        other => Err(route_terminal_from_checked(other)),
    }
}

pub(crate) fn forge_query_checked_declaration_route_orchestration_from_progressed_on_handle<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    progressed: ForgeQueryAdmittedDeclarationProgression<D, I>,
    route_intent: Option<ForgeQueryDeclarationRouteIntent>,
) -> ForgeQueryDeclarationRoutePlanChecked<D, I> {
    match forge_query_lower_declaration_entry_product_orchestration_from_progressed_on_handle(
        handle,
        progressed,
        ForgeQueryDeclarationEntryOrchestrationExposureLevel::Checked,
        ForgeQueryDeclarationEntryOrchestrationArtifactPolicy::CheckedOutcomeOnly,
        ForgeQueryDeclarationEntryOrchestrationProduct::RoutePlan,
        route_intent,
    )
    .checked
    {
        ForgeQueryDeclarationEntryProductChecked::RoutePlan(checked) => checked,
        _ => panic!("route orchestration must project the route product"),
    }
}

pub(crate) fn forge_query_declaration_route_orchestration_from_progressed_proof_on_handle<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    progressed: ForgeQueryAdmittedDeclarationProgression<D, I>,
    route_intent: Option<ForgeQueryDeclarationRouteIntent>,
) -> ForgeQueryDeclarationRouteOrchestrationTranscript<D, I> {
    let lowered =
        forge_query_lower_declaration_entry_product_orchestration_from_progressed_on_handle(
            handle,
            progressed,
            ForgeQueryDeclarationEntryOrchestrationExposureLevel::ProofVisible,
            ForgeQueryDeclarationEntryOrchestrationArtifactPolicy::ProofVisibleTranscript,
            ForgeQueryDeclarationEntryOrchestrationProduct::RoutePlan,
            route_intent,
        );
    let checked = match lowered.checked {
        ForgeQueryDeclarationEntryProductChecked::RoutePlan(checked) => checked,
        _ => panic!("route orchestration proof must project the route product"),
    };
    let outcome_identity = route_orchestration_identity(&checked);
    ForgeQueryDeclarationRouteOrchestrationTranscript::new(
        lowered.plan,
        checked,
        lowered.step_records,
        outcome_identity,
    )
}
