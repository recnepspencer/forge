use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationBridgeRoutingChecked,
    ForgeQueryDeclarationBridgeRoutingDenialCause, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
};
use crate::binding_pipeline::{
    ForgeQueryBindingNarrowingDecision, ForgeQueryBindingRequestDescriptor,
    ForgeQueryBindingTranscript, ForgeQueryBindingWitnessCheck, ForgeQueryContinuationBindingInput,
    ForgeQueryContinuationBindingRequest, ForgeQueryResolveContinuationFromTargetRequest,
};

use super::readmission::prepared_execution_readmission_from_routing;
use super::support::{
    bridge_subject_and_signal_truth, linked_from_subject, prepared_digest,
    required_capability_families_for_prepared, signal_basis_families_from_family,
    signal_subject_from_bridge_subject, transcript_digest,
};
use crate::continuation_pipeline::artifacts::{
    basis_posture_for_families, family_for_mode, runtime_contract_for_mode, truth_context_for_mode,
    workspace_contract_for_mode, ForgeQueryPreparedContinuationExecutionMode,
    ForgeQueryPreparedContinuationSignalPosture,
};
use crate::continuation_pipeline::request::ForgeQueryPreparedContinuationRequest;
use crate::continuation_pipeline::transcript::ForgeQueryPreparedContinuationTranscript;
use crate::continuation_pipeline::{
    ForgeQueryPreparedContinuation, ForgeQueryPreparedContinuationOutcome,
};

pub(crate) fn prepare_continuation_from_target_on_handle<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    request: ForgeQueryResolveContinuationFromTargetRequest<D, I>,
) -> ForgeQueryPreparedContinuationTranscript<D, I> {
    let binding = handle.bind_continuation_from_target_proof(request);
    prepare_from_binding_transcript(handle, binding)
}

pub(crate) fn prepare_continuation_from_context_on_handle<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    request: ForgeQueryContinuationBindingRequest<D, I>,
) -> ForgeQueryPreparedContinuationTranscript<D, I> {
    let binding = handle.bind_continuation_request_from_context_proof(request);
    prepare_from_binding_transcript(handle, binding)
}

pub(crate) fn prepare_continuation_from_signal_checked_on_handle<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    checked: crate::application::ForgeQueryDeclarationSignalCompatibilityChecked<D, I>,
    bridge_request: crate::application::ForgeQueryDeclarationBridgeContinuationRequest,
    required_contract: crate::application::ForgeQueryDeclarationAspectContract,
) -> ForgeQueryPreparedContinuationTranscript<D, I> {
    let request = ForgeQueryBindingRequestDescriptor::new(
        I::Family::semantic_family_key(),
        "prepared_continuation",
        required_contract.clone(),
    );
    let (signal_truth, subject) = bridge_subject_and_signal_truth(checked);
    let linked = linked_from_subject(&subject);
    prepare_from_resolved_signal_truth(
        handle,
        request,
        linked,
        signal_truth,
        subject,
        bridge_request,
        required_contract,
    )
}

fn prepare_continuation_from_input_on_handle<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    request: ForgeQueryPreparedContinuationRequest<D, I>,
) -> ForgeQueryPreparedContinuationTranscript<D, I> {
    let (input, request_contract) = request.into_parts();
    let (bridge_request, subject) = input.into_bridge_parts();
    let required_contract = request_contract.unwrap_or_else(|| I::Family::aspect_contract());
    let request_descriptor = ForgeQueryBindingRequestDescriptor::new(
        I::Family::semantic_family_key(),
        "prepared_continuation",
        required_contract.clone(),
    );
    let signal_checked =
        handle.signal_compatibility_checked(signal_subject_from_bridge_subject(subject));
    let (signal_truth, subject) = bridge_subject_and_signal_truth(signal_checked);
    let linked = linked_from_subject(&subject);
    prepare_from_resolved_signal_truth(
        handle,
        request_descriptor,
        linked,
        signal_truth,
        subject,
        bridge_request,
        required_contract,
    )
}

#[allow(clippy::too_many_arguments)]
fn prepare_from_resolved_signal_truth<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    request_descriptor: ForgeQueryBindingRequestDescriptor,
    linked: crate::binding_pipeline::ForgeQueryBindingLinkedArtifacts,
    signal_truth: super::support::ResolvedSignalContinuationTruth,
    subject: crate::application::ForgeQueryDeclarationBridgeRoutingInput<D, I>,
    bridge_request: crate::application::ForgeQueryDeclarationBridgeContinuationRequest,
    required_contract: crate::application::ForgeQueryDeclarationAspectContract,
) -> ForgeQueryPreparedContinuationTranscript<D, I> {
    let mut witness_checks = vec![
        ForgeQueryBindingWitnessCheck::passed("continuation_binding"),
        ForgeQueryBindingWitnessCheck::passed("signal_compatibility"),
    ];
    let mut narrowing = vec![ForgeQueryBindingNarrowingDecision::new(
        "prepared continuation reuses retained continuation binding and lower-authority continuation truth",
    )];
    if !matches!(
        signal_truth.posture,
        ForgeQueryPreparedContinuationSignalPosture::Compatible
    ) {
        witness_checks[1] =
            ForgeQueryBindingWitnessCheck::failed("signal_compatibility", signal_truth.reason);
        narrowing.push(ForgeQueryBindingNarrowingDecision::new(
            "prepared continuation carries forward target-specific signal compatibility posture",
        ));
    }
    let bridge_checked = handle.route_bridge_continuation_checked(subject);
    let outcome = prepared_outcome_from_bridge_checked(
        handle,
        &bridge_request,
        &required_contract,
        &linked,
        &signal_truth,
        &mut witness_checks,
        bridge_checked,
    );
    let digest = transcript_digest(
        "prepared_continuation",
        I::Family::semantic_family_key(),
        &linked,
        prepared_outcome_token(&outcome),
    );
    ForgeQueryPreparedContinuationTranscript::new(
        request_descriptor,
        outcome,
        witness_checks,
        narrowing,
        digest,
        linked,
    )
}

#[allow(clippy::too_many_arguments)]
fn prepared_outcome_from_bridge_checked<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    bridge_request: &crate::application::ForgeQueryDeclarationBridgeContinuationRequest,
    required_contract: &crate::application::ForgeQueryDeclarationAspectContract,
    linked: &crate::binding_pipeline::ForgeQueryBindingLinkedArtifacts,
    signal_truth: &super::support::ResolvedSignalContinuationTruth,
    witness_checks: &mut Vec<ForgeQueryBindingWitnessCheck>,
    bridge_checked: ForgeQueryDeclarationBridgeRoutingChecked<D, I>,
) -> ForgeQueryPreparedContinuationOutcome<D, I> {
    match bridge_checked {
        ForgeQueryDeclarationBridgeRoutingChecked::Routed(routing) => {
            witness_checks.push(ForgeQueryBindingWitnessCheck::passed("bridge_routing"));
            let basis_families = signal_basis_families_from_family::<D, I>();
            let required_capability_families = required_capability_families_for_prepared::<D, I>(
                bridge_request,
                signal_truth.execution_family,
            );
            let execution_readmission = prepared_execution_readmission_from_routing(
                bridge_request,
                &routing,
                required_capability_families,
            );
            let prepared = ForgeQueryPreparedContinuation::new(
                family_for_mode(bridge_request.mode()),
                truth_context_for_mode(routing.truth_context()),
                basis_posture_for_families(&basis_families),
                workspace_contract_for_mode(bridge_request.mode()),
                runtime_contract_for_mode(bridge_request.mode()),
                ForgeQueryPreparedContinuationExecutionMode::ExplicitBridgeLowering,
                basis_families,
                execution_readmission,
                routing,
                signal_truth.posture,
                signal_truth.execution_family,
                signal_truth.digest.clone(),
                prepared_digest::<D, C, I>(
                    handle,
                    bridge_request,
                    required_contract,
                    linked,
                    signal_truth,
                ),
            );
            ForgeQueryPreparedContinuationOutcome::Prepared(prepared)
        }
        ForgeQueryDeclarationBridgeRoutingChecked::Deferred(deferred) => {
            witness_checks.push(ForgeQueryBindingWitnessCheck::failed(
                "bridge_routing",
                deferred.reason(),
            ));
            ForgeQueryPreparedContinuationOutcome::Deferred(deferred.reason().to_string())
        }
        ForgeQueryDeclarationBridgeRoutingChecked::Denied(denied) => {
            witness_checks.push(ForgeQueryBindingWitnessCheck::failed(
                "bridge_routing",
                denied.reason(),
            ));
            prepared_outcome_from_bridge_denial_cause(denied.cause(), denied.reason())
        }
        ForgeQueryDeclarationBridgeRoutingChecked::Failed(failed) => {
            witness_checks.push(ForgeQueryBindingWitnessCheck::failed(
                "bridge_routing",
                failed.reason(),
            ));
            ForgeQueryPreparedContinuationOutcome::Failed(failed.reason().to_string())
        }
    }
}

fn prepared_outcome_from_bridge_denial_cause<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    cause: ForgeQueryDeclarationBridgeRoutingDenialCause,
    reason: &str,
) -> ForgeQueryPreparedContinuationOutcome<D, I> {
    match cause {
        ForgeQueryDeclarationBridgeRoutingDenialCause::BridgeEnvelopeMismatch => {
            ForgeQueryPreparedContinuationOutcome::WrongHandle(reason.to_string())
        }
        ForgeQueryDeclarationBridgeRoutingDenialCause::BridgeAuthorityUnavailable
        | ForgeQueryDeclarationBridgeRoutingDenialCause::AuthorityAspectGap
        | ForgeQueryDeclarationBridgeRoutingDenialCause::AuthorityAspectAmbiguity => {
            ForgeQueryPreparedContinuationOutcome::AuthorityMismatch(reason.to_string())
        }
        ForgeQueryDeclarationBridgeRoutingDenialCause::BasisLifecycleMismatch => {
            ForgeQueryPreparedContinuationOutcome::BasisMismatch(reason.to_string())
        }
        ForgeQueryDeclarationBridgeRoutingDenialCause::UnsupportedContinuationMode
        | ForgeQueryDeclarationBridgeRoutingDenialCause::UnsupportedTruthContext => {
            ForgeQueryPreparedContinuationOutcome::Unsupported(reason.to_string())
        }
        _ => ForgeQueryPreparedContinuationOutcome::Denied(reason.to_string()),
    }
}

fn prepare_from_binding_transcript<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    binding: ForgeQueryBindingTranscript<ForgeQueryContinuationBindingInput<D, I>>,
) -> ForgeQueryPreparedContinuationTranscript<D, I> {
    let required_contract = binding.request().required_aspect_contract().clone();
    let checked = binding.into_checked();
    let linked = checked.linked_artifacts().clone();
    let request = ForgeQueryBindingRequestDescriptor::new(
        I::Family::semantic_family_key(),
        "prepared_continuation",
        required_contract.clone(),
    );

    match checked.into_parts() {
        (crate::binding_pipeline::ForgeQueryBindingOutcome::Bound(input), _, _) => {
            prepare_continuation_from_input_on_handle(
                handle,
                ForgeQueryPreparedContinuationRequest::new(input)
                    .with_required_aspect_contract(required_contract),
            )
        }
        (outcome, digest, _) => {
            let (prepared_outcome, witness_name, narrowing) =
                prepared_outcome_from_binding_outcome(outcome);
            ForgeQueryPreparedContinuationTranscript::new(
                request,
                prepared_outcome,
                vec![ForgeQueryBindingWitnessCheck::failed(
                    witness_name,
                    narrowing.reason(),
                )],
                vec![narrowing],
                digest,
                linked,
            )
        }
    }
}

fn prepared_outcome_from_binding_outcome<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    outcome: crate::binding_pipeline::ForgeQueryBindingOutcome<
        ForgeQueryContinuationBindingInput<D, I>,
    >,
) -> (
    ForgeQueryPreparedContinuationOutcome<D, I>,
    &'static str,
    ForgeQueryBindingNarrowingDecision,
) {
    match outcome {
        crate::binding_pipeline::ForgeQueryBindingOutcome::Ambiguous(reason) => (
            ForgeQueryPreparedContinuationOutcome::Ambiguous(reason.reason().to_string()),
            "continuation_binding",
            ForgeQueryBindingNarrowingDecision::new(
                "prepared continuation stopped because continuation binding remained ambiguous",
            ),
        ),
        crate::binding_pipeline::ForgeQueryBindingOutcome::Unavailable(reason) => (
            ForgeQueryPreparedContinuationOutcome::Unavailable(reason.reason().to_string()),
            "continuation_binding",
            ForgeQueryBindingNarrowingDecision::new(reason.reason()),
        ),
        crate::binding_pipeline::ForgeQueryBindingOutcome::WrongWorld(reason) => (
            ForgeQueryPreparedContinuationOutcome::WrongWorld(reason.reason().to_string()),
            "world_alignment",
            ForgeQueryBindingNarrowingDecision::new(reason.reason()),
        ),
        crate::binding_pipeline::ForgeQueryBindingOutcome::WrongHandle(reason) => (
            ForgeQueryPreparedContinuationOutcome::WrongHandle(reason.reason().to_string()),
            "handle_alignment",
            ForgeQueryBindingNarrowingDecision::new(reason.reason()),
        ),
        crate::binding_pipeline::ForgeQueryBindingOutcome::Stale(reason) => (
            ForgeQueryPreparedContinuationOutcome::Stale(reason.reason().to_string()),
            "basis_freshness",
            ForgeQueryBindingNarrowingDecision::new(reason.reason()),
        ),
        crate::binding_pipeline::ForgeQueryBindingOutcome::RebindRequired(reason) => (
            ForgeQueryPreparedContinuationOutcome::RebindRequired(reason.reason().to_string()),
            "continuation_binding",
            ForgeQueryBindingNarrowingDecision::new(reason.reason()),
        ),
        crate::binding_pipeline::ForgeQueryBindingOutcome::AuthorityMismatch(reason) => (
            ForgeQueryPreparedContinuationOutcome::AuthorityMismatch(reason.reason().to_string()),
            "authority_alignment",
            ForgeQueryBindingNarrowingDecision::new(reason.reason()),
        ),
        crate::binding_pipeline::ForgeQueryBindingOutcome::BasisMismatch(reason) => (
            ForgeQueryPreparedContinuationOutcome::BasisMismatch(reason.reason().to_string()),
            "basis_alignment",
            ForgeQueryBindingNarrowingDecision::new(reason.reason()),
        ),
        crate::binding_pipeline::ForgeQueryBindingOutcome::MissingRequiredAspect(reason) => (
            ForgeQueryPreparedContinuationOutcome::Denied(reason.reason().to_string()),
            "aspect_fit",
            ForgeQueryBindingNarrowingDecision::new(reason.reason()),
        ),
        crate::binding_pipeline::ForgeQueryBindingOutcome::AspectConflict(reason) => (
            ForgeQueryPreparedContinuationOutcome::Denied(reason.reason().to_string()),
            "aspect_fit",
            ForgeQueryBindingNarrowingDecision::new(reason.reason()),
        ),
        crate::binding_pipeline::ForgeQueryBindingOutcome::ExplicitNarrowingRequired(reason) => (
            ForgeQueryPreparedContinuationOutcome::RebindRequired(reason.reason().to_string()),
            "continuation_binding",
            ForgeQueryBindingNarrowingDecision::new(reason.reason()),
        ),
        crate::binding_pipeline::ForgeQueryBindingOutcome::Unsupported(reason) => (
            ForgeQueryPreparedContinuationOutcome::Unsupported(reason.reason().to_string()),
            "continuation_binding",
            ForgeQueryBindingNarrowingDecision::new(reason.reason()),
        ),
        crate::binding_pipeline::ForgeQueryBindingOutcome::Bound(_) => unreachable!(),
    }
}

fn prepared_outcome_token<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    outcome: &ForgeQueryPreparedContinuationOutcome<D, I>,
) -> &str {
    match outcome {
        ForgeQueryPreparedContinuationOutcome::Prepared(prepared) => prepared.prepared_digest(),
        ForgeQueryPreparedContinuationOutcome::Ambiguous(_) => "ambiguous",
        ForgeQueryPreparedContinuationOutcome::Unavailable(_) => "unavailable",
        ForgeQueryPreparedContinuationOutcome::WrongWorld(_) => "wrong_world",
        ForgeQueryPreparedContinuationOutcome::WrongHandle(_) => "wrong_handle",
        ForgeQueryPreparedContinuationOutcome::Stale(_) => "stale",
        ForgeQueryPreparedContinuationOutcome::RebindRequired(_) => "rebind_required",
        ForgeQueryPreparedContinuationOutcome::AuthorityMismatch(_) => "authority_mismatch",
        ForgeQueryPreparedContinuationOutcome::BasisMismatch(_) => "basis_mismatch",
        ForgeQueryPreparedContinuationOutcome::Unsupported(_) => "unsupported",
        ForgeQueryPreparedContinuationOutcome::Deferred(_) => "deferred",
        ForgeQueryPreparedContinuationOutcome::Denied(_) => "denied",
        ForgeQueryPreparedContinuationOutcome::Failed(_) => "failed",
    }
}
