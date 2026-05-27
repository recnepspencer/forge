use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryAdmittedDeclarationProgression,
    ForgeQueryDeclarationBridgeContinuationMode, ForgeQueryDeclarationBridgeContinuationRequest,
    ForgeQueryDeclarationBridgeTruthContext, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
};
use crate::ordinary_outcome::ForgeQueryOrdinaryOutcome;
use crate::signal_compatibility_orchestration::{
    ForgeQuerySignalCompatibilityOrchestration, ForgeQuerySignalCompatibilityOrchestrationChecked,
    ForgeQuerySignalCompatibilityOrchestrationInput,
    ForgeQuerySignalCompatibilityOrchestrationOutcome,
    ForgeQuerySignalCompatibilityOrchestrationTranscript,
};

pub(super) fn prepare_preview_for_active_face_selection<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    progression: ForgeQueryAdmittedDeclarationProgression<D, I>,
) -> ForgeQuerySignalCompatibilityOrchestrationOutcome<D, I> {
    handle.orchestrate_signal_compatibility(continuation_input(
        progression,
        preview_session_request(),
    ))
}

pub(super) fn prepare_preview_for_active_face_selection_outcome<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    progression: ForgeQueryAdmittedDeclarationProgression<D, I>,
) -> ForgeQueryOrdinaryOutcome<ForgeQuerySignalCompatibilityOrchestration<D, I>> {
    handle.orchestrate_signal_compatibility_outcome(continuation_input(
        progression,
        preview_session_request(),
    ))
}

pub(super) fn prepare_preview_for_active_face_selection_checked<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    progression: ForgeQueryAdmittedDeclarationProgression<D, I>,
) -> ForgeQuerySignalCompatibilityOrchestrationChecked<D, I> {
    handle.orchestrate_signal_compatibility_checked(continuation_input(
        progression,
        preview_session_request(),
    ))
}

pub(super) fn prepare_preview_for_active_face_selection_proof<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    progression: ForgeQueryAdmittedDeclarationProgression<D, I>,
) -> ForgeQuerySignalCompatibilityOrchestrationTranscript<D, I> {
    handle.orchestrate_signal_compatibility_proof(continuation_input(
        progression,
        preview_session_request(),
    ))
}

pub(super) fn prepare_runtime_route_for_active_face_selection<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    progression: ForgeQueryAdmittedDeclarationProgression<D, I>,
) -> ForgeQuerySignalCompatibilityOrchestrationOutcome<D, I> {
    handle
        .orchestrate_signal_compatibility(continuation_input(progression, runtime_route_request()))
}

pub(super) fn prepare_runtime_route_for_active_face_selection_outcome<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    progression: ForgeQueryAdmittedDeclarationProgression<D, I>,
) -> ForgeQueryOrdinaryOutcome<ForgeQuerySignalCompatibilityOrchestration<D, I>> {
    handle.orchestrate_signal_compatibility_outcome(continuation_input(
        progression,
        runtime_route_request(),
    ))
}

pub(super) fn prepare_runtime_route_for_active_face_selection_checked<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    progression: ForgeQueryAdmittedDeclarationProgression<D, I>,
) -> ForgeQuerySignalCompatibilityOrchestrationChecked<D, I> {
    handle.orchestrate_signal_compatibility_checked(continuation_input(
        progression,
        runtime_route_request(),
    ))
}

pub(super) fn prepare_runtime_route_for_active_face_selection_proof<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    progression: ForgeQueryAdmittedDeclarationProgression<D, I>,
) -> ForgeQuerySignalCompatibilityOrchestrationTranscript<D, I> {
    handle.orchestrate_signal_compatibility_proof(continuation_input(
        progression,
        runtime_route_request(),
    ))
}

pub(super) fn prepare_current_truth_view_for_active_face_selection<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    progression: ForgeQueryAdmittedDeclarationProgression<D, I>,
) -> ForgeQuerySignalCompatibilityOrchestrationOutcome<D, I> {
    handle.orchestrate_signal_compatibility(continuation_input(
        progression,
        truth_view_request(ForgeQueryDeclarationBridgeTruthContext::Current),
    ))
}

pub(super) fn prepare_current_truth_view_for_active_face_selection_outcome<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    progression: ForgeQueryAdmittedDeclarationProgression<D, I>,
) -> ForgeQueryOrdinaryOutcome<ForgeQuerySignalCompatibilityOrchestration<D, I>> {
    handle.orchestrate_signal_compatibility_outcome(continuation_input(
        progression,
        truth_view_request(ForgeQueryDeclarationBridgeTruthContext::Current),
    ))
}

pub(super) fn prepare_current_truth_view_for_active_face_selection_checked<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    progression: ForgeQueryAdmittedDeclarationProgression<D, I>,
) -> ForgeQuerySignalCompatibilityOrchestrationChecked<D, I> {
    handle.orchestrate_signal_compatibility_checked(continuation_input(
        progression,
        truth_view_request(ForgeQueryDeclarationBridgeTruthContext::Current),
    ))
}

pub(super) fn prepare_current_truth_view_for_active_face_selection_proof<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    progression: ForgeQueryAdmittedDeclarationProgression<D, I>,
) -> ForgeQuerySignalCompatibilityOrchestrationTranscript<D, I> {
    handle.orchestrate_signal_compatibility_proof(continuation_input(
        progression,
        truth_view_request(ForgeQueryDeclarationBridgeTruthContext::Current),
    ))
}

pub(super) fn prepare_historical_truth_view_for_active_face_selection<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    progression: ForgeQueryAdmittedDeclarationProgression<D, I>,
) -> ForgeQuerySignalCompatibilityOrchestrationOutcome<D, I> {
    handle.orchestrate_signal_compatibility(continuation_input(
        progression,
        truth_view_request(ForgeQueryDeclarationBridgeTruthContext::Historical),
    ))
}

pub(super) fn prepare_historical_truth_view_for_active_face_selection_outcome<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    progression: ForgeQueryAdmittedDeclarationProgression<D, I>,
) -> ForgeQueryOrdinaryOutcome<ForgeQuerySignalCompatibilityOrchestration<D, I>> {
    handle.orchestrate_signal_compatibility_outcome(continuation_input(
        progression,
        truth_view_request(ForgeQueryDeclarationBridgeTruthContext::Historical),
    ))
}

pub(super) fn prepare_historical_truth_view_for_active_face_selection_checked<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    progression: ForgeQueryAdmittedDeclarationProgression<D, I>,
) -> ForgeQuerySignalCompatibilityOrchestrationChecked<D, I> {
    handle.orchestrate_signal_compatibility_checked(continuation_input(
        progression,
        truth_view_request(ForgeQueryDeclarationBridgeTruthContext::Historical),
    ))
}

pub(super) fn prepare_historical_truth_view_for_active_face_selection_proof<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    progression: ForgeQueryAdmittedDeclarationProgression<D, I>,
) -> ForgeQuerySignalCompatibilityOrchestrationTranscript<D, I> {
    handle.orchestrate_signal_compatibility_proof(continuation_input(
        progression,
        truth_view_request(ForgeQueryDeclarationBridgeTruthContext::Historical),
    ))
}

fn continuation_input<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    progression: ForgeQueryAdmittedDeclarationProgression<D, I>,
    bridge_request: ForgeQueryDeclarationBridgeContinuationRequest,
) -> ForgeQuerySignalCompatibilityOrchestrationInput<D, I> {
    ForgeQuerySignalCompatibilityOrchestrationInput::from_progressed(progression)
        .with_required_aspect_contract(I::Family::aspect_contract())
        .with_bridge_request(bridge_request)
}

fn preview_session_request() -> ForgeQueryDeclarationBridgeContinuationRequest {
    ForgeQueryDeclarationBridgeContinuationRequest::new(
        ForgeQueryDeclarationBridgeContinuationMode::PreviewSession,
        ForgeQueryDeclarationBridgeTruthContext::Preview,
    )
}

fn runtime_route_request() -> ForgeQueryDeclarationBridgeContinuationRequest {
    ForgeQueryDeclarationBridgeContinuationRequest::new(
        ForgeQueryDeclarationBridgeContinuationMode::RuntimeRoute,
        ForgeQueryDeclarationBridgeTruthContext::Current,
    )
}

fn truth_view_request(
    truth_context: ForgeQueryDeclarationBridgeTruthContext,
) -> ForgeQueryDeclarationBridgeContinuationRequest {
    ForgeQueryDeclarationBridgeContinuationRequest::new(
        ForgeQueryDeclarationBridgeContinuationMode::TruthView,
        truth_context,
    )
}
