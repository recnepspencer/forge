use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationBridgeContinuationRequest,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationSignalCompatibilityChecked,
    ForgeQueryDeclarationSignalCompatibilityDenialCause, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext,
};
use crate::binding_pipeline::{
    ForgeQueryBindingLinkedArtifacts, ForgeQueryBindingNarrowingDecision,
    ForgeQueryBindingRequestDescriptor, ForgeQueryBindingWitnessCheck,
};
use crate::continuation_pipeline::{
    prepare_continuation_from_signal_checked_on_handle, ForgeQueryPreparedContinuationOutcome,
};

use super::lower::{
    checked_and_linked_from_subject, handle_alignment_outcome, linked_from_orchestration_subject,
    orchestration_digest,
};
use super::{
    ForgeQuerySignalCompatibilityOrchestration, ForgeQuerySignalCompatibilityOrchestrationChecked,
    ForgeQuerySignalCompatibilityOrchestrationInput,
    ForgeQuerySignalCompatibilityOrchestrationOutcome,
};

pub struct ForgeQuerySignalCompatibilityOrchestrationTranscript<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    request: ForgeQueryBindingRequestDescriptor,
    outcome: ForgeQuerySignalCompatibilityOrchestrationOutcome<D, I>,
    witness_checks: Vec<ForgeQueryBindingWitnessCheck>,
    narrowing_decisions: Vec<ForgeQueryBindingNarrowingDecision>,
    orchestration_digest: String,
    linked_artifacts: ForgeQueryBindingLinkedArtifacts,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQuerySignalCompatibilityOrchestrationTranscript<D, I>
{
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        request: ForgeQueryBindingRequestDescriptor,
        outcome: ForgeQuerySignalCompatibilityOrchestrationOutcome<D, I>,
        witness_checks: Vec<ForgeQueryBindingWitnessCheck>,
        narrowing_decisions: Vec<ForgeQueryBindingNarrowingDecision>,
        orchestration_digest: String,
        linked_artifacts: ForgeQueryBindingLinkedArtifacts,
    ) -> Self {
        Self {
            request,
            outcome,
            witness_checks,
            narrowing_decisions,
            orchestration_digest,
            linked_artifacts,
        }
    }

    pub fn request(&self) -> &ForgeQueryBindingRequestDescriptor {
        &self.request
    }

    pub fn outcome(&self) -> &ForgeQuerySignalCompatibilityOrchestrationOutcome<D, I> {
        &self.outcome
    }

    pub fn witness_checks(&self) -> &[ForgeQueryBindingWitnessCheck] {
        &self.witness_checks
    }

    pub fn narrowing_decisions(&self) -> &[ForgeQueryBindingNarrowingDecision] {
        &self.narrowing_decisions
    }

    pub fn orchestration_digest(&self) -> &str {
        &self.orchestration_digest
    }

    pub fn linked_artifacts(&self) -> &ForgeQueryBindingLinkedArtifacts {
        &self.linked_artifacts
    }

    pub fn into_checked(self) -> ForgeQuerySignalCompatibilityOrchestrationChecked<D, I> {
        ForgeQuerySignalCompatibilityOrchestrationChecked::new(
            self.outcome,
            self.orchestration_digest,
            self.linked_artifacts,
        )
    }
}

pub(crate) fn orchestrate_signal_compatibility_on_handle<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    input: ForgeQuerySignalCompatibilityOrchestrationInput<D, I>,
) -> ForgeQuerySignalCompatibilityOrchestrationTranscript<D, I> {
    let (subject, required_contract, bridge_request) = input.into_parts();
    let request = ForgeQueryBindingRequestDescriptor::new(
        I::Family::semantic_family_key(),
        "signal_compatibility_orchestration",
        required_contract.clone(),
    );
    if let Some(outcome) = handle_alignment_outcome(handle, &subject) {
        let linked = linked_from_orchestration_subject(&subject);
        let digest = orchestration_digest("misaligned", &linked, "misaligned");
        return ForgeQuerySignalCompatibilityOrchestrationTranscript::new(
            request,
            outcome,
            vec![ForgeQueryBindingWitnessCheck::failed(
                "handle_alignment",
                "the retained signal subject no longer matches this admitted handle",
            )],
            Vec::new(),
            digest,
            linked,
        );
    }

    let (checked, linked) = checked_and_linked_from_subject(handle, subject);
    let (outcome, witness_checks, narrowing, outcome_token) =
        outcome_from_signal_checked(handle, required_contract, bridge_request, checked);
    let digest = orchestration_digest("signal_orchestration", &linked, &outcome_token);
    ForgeQuerySignalCompatibilityOrchestrationTranscript::new(
        request,
        outcome,
        witness_checks,
        narrowing,
        digest,
        linked,
    )
}

#[cfg(test)]
pub(crate) fn orchestrated_outcome_from_signal_checked_on_handle<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    required_contract: crate::application::ForgeQueryDeclarationAspectContract,
    bridge_request: Option<ForgeQueryDeclarationBridgeContinuationRequest>,
    checked: ForgeQueryDeclarationSignalCompatibilityChecked<D, I>,
) -> ForgeQuerySignalCompatibilityOrchestrationOutcome<D, I> {
    outcome_from_signal_checked(handle, required_contract, bridge_request, checked).0
}

fn outcome_from_signal_checked<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    required_contract: crate::application::ForgeQueryDeclarationAspectContract,
    bridge_request: Option<ForgeQueryDeclarationBridgeContinuationRequest>,
    checked: ForgeQueryDeclarationSignalCompatibilityChecked<D, I>,
) -> (
    ForgeQuerySignalCompatibilityOrchestrationOutcome<D, I>,
    Vec<ForgeQueryBindingWitnessCheck>,
    Vec<ForgeQueryBindingNarrowingDecision>,
    String,
) {
    match checked {
        ForgeQueryDeclarationSignalCompatibilityChecked::Compatible(compatibility) => {
            let mut witness_checks = vec![ForgeQueryBindingWitnessCheck::passed(
                "signal_compatibility",
            )];
            if let Some(bridge_request) = bridge_request {
                let proof = prepare_continuation_from_signal_checked_on_handle(
                    handle,
                    ForgeQueryDeclarationSignalCompatibilityChecked::Compatible(compatibility),
                    bridge_request,
                    required_contract,
                );
                witness_checks.extend(proof.witness_checks().iter().cloned());
                let narrowing = proof.narrowing_decisions().to_vec();
                let checked = proof.into_checked();
                let digest = checked.prepared_digest().to_string();
                return (
                    map_continuation_outcome(checked.into_outcome()),
                    witness_checks,
                    narrowing,
                    digest,
                );
            }
            let digest = compatibility.signal_compatibility_digest().to_string();
            (
                ForgeQuerySignalCompatibilityOrchestrationOutcome::Bound(
                    ForgeQuerySignalCompatibilityOrchestration::Compatible(compatibility),
                ),
                witness_checks,
                vec![ForgeQueryBindingNarrowingDecision::new(
                    "signal compatibility orchestration stopped at retained compatibility because no continuation request was supplied",
                )],
                digest,
            )
        }
        ForgeQueryDeclarationSignalCompatibilityChecked::Deferred(deferred) => (
            ForgeQuerySignalCompatibilityOrchestrationOutcome::Deferred(
                deferred.reason().to_string(),
            ),
            vec![ForgeQueryBindingWitnessCheck::failed(
                "signal_compatibility",
                deferred.reason(),
            )],
            Vec::new(),
            "deferred".to_string(),
        ),
        ForgeQueryDeclarationSignalCompatibilityChecked::Denied(denied) => (
            map_signal_denial(denied.cause(), denied.reason()),
            vec![ForgeQueryBindingWitnessCheck::failed(
                "signal_compatibility",
                denied.reason(),
            )],
            Vec::new(),
            format!("denied:{:?}", denied.cause()),
        ),
        ForgeQueryDeclarationSignalCompatibilityChecked::Failed(failed) => (
            ForgeQuerySignalCompatibilityOrchestrationOutcome::Failed(failed.reason().to_string()),
            vec![ForgeQueryBindingWitnessCheck::failed(
                "signal_compatibility",
                failed.reason(),
            )],
            Vec::new(),
            "failed".to_string(),
        ),
    }
}

fn map_signal_denial<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    cause: ForgeQueryDeclarationSignalCompatibilityDenialCause,
    reason: &str,
) -> ForgeQuerySignalCompatibilityOrchestrationOutcome<D, I> {
    match cause {
        ForgeQueryDeclarationSignalCompatibilityDenialCause::SignalBasisMismatch => {
            ForgeQuerySignalCompatibilityOrchestrationOutcome::BasisMismatch(reason.to_string())
        }
        ForgeQueryDeclarationSignalCompatibilityDenialCause::SignalFamilyUnsupported
        | ForgeQueryDeclarationSignalCompatibilityDenialCause::SignalExecutionFamilyUnavailable => {
            ForgeQuerySignalCompatibilityOrchestrationOutcome::Unsupported(reason.to_string())
        }
        ForgeQueryDeclarationSignalCompatibilityDenialCause::MissingRequiredAspect => {
            ForgeQuerySignalCompatibilityOrchestrationOutcome::MissingRequiredAspect(
                reason.to_string(),
            )
        }
        ForgeQueryDeclarationSignalCompatibilityDenialCause::AspectConflict => {
            ForgeQuerySignalCompatibilityOrchestrationOutcome::AspectConflict(reason.to_string())
        }
        ForgeQueryDeclarationSignalCompatibilityDenialCause::AuthorityAspectGap => {
            ForgeQuerySignalCompatibilityOrchestrationOutcome::AuthorityMismatch(
                reason.to_string(),
            )
        }
        ForgeQueryDeclarationSignalCompatibilityDenialCause::EnvelopeNotCoveredForSignalCompatibility
        | ForgeQueryDeclarationSignalCompatibilityDenialCause::SignalCompatibilityMismatch => {
            ForgeQuerySignalCompatibilityOrchestrationOutcome::Denied(reason.to_string())
        }
    }
}

fn map_continuation_outcome<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    outcome: ForgeQueryPreparedContinuationOutcome<D, I>,
) -> ForgeQuerySignalCompatibilityOrchestrationOutcome<D, I> {
    match outcome {
        ForgeQueryPreparedContinuationOutcome::Prepared(prepared) => {
            ForgeQuerySignalCompatibilityOrchestrationOutcome::Bound(
                ForgeQuerySignalCompatibilityOrchestration::Prepared(prepared),
            )
        }
        ForgeQueryPreparedContinuationOutcome::Ambiguous(reason) => {
            ForgeQuerySignalCompatibilityOrchestrationOutcome::Ambiguous(reason)
        }
        ForgeQueryPreparedContinuationOutcome::Unavailable(reason) => {
            ForgeQuerySignalCompatibilityOrchestrationOutcome::Unavailable(reason)
        }
        ForgeQueryPreparedContinuationOutcome::WrongWorld(reason) => {
            ForgeQuerySignalCompatibilityOrchestrationOutcome::WrongWorld(reason)
        }
        ForgeQueryPreparedContinuationOutcome::WrongHandle(reason) => {
            ForgeQuerySignalCompatibilityOrchestrationOutcome::WrongHandle(reason)
        }
        ForgeQueryPreparedContinuationOutcome::Stale(reason) => {
            ForgeQuerySignalCompatibilityOrchestrationOutcome::Stale(reason)
        }
        ForgeQueryPreparedContinuationOutcome::RebindRequired(reason) => {
            ForgeQuerySignalCompatibilityOrchestrationOutcome::RebindRequired(reason)
        }
        ForgeQueryPreparedContinuationOutcome::AuthorityMismatch(reason) => {
            ForgeQuerySignalCompatibilityOrchestrationOutcome::AuthorityMismatch(reason)
        }
        ForgeQueryPreparedContinuationOutcome::BasisMismatch(reason) => {
            ForgeQuerySignalCompatibilityOrchestrationOutcome::BasisMismatch(reason)
        }
        ForgeQueryPreparedContinuationOutcome::Unsupported(reason) => {
            ForgeQuerySignalCompatibilityOrchestrationOutcome::Unsupported(reason)
        }
        ForgeQueryPreparedContinuationOutcome::Deferred(reason) => {
            ForgeQuerySignalCompatibilityOrchestrationOutcome::Deferred(reason)
        }
        ForgeQueryPreparedContinuationOutcome::Denied(reason) => {
            ForgeQuerySignalCompatibilityOrchestrationOutcome::Denied(reason)
        }
        ForgeQueryPreparedContinuationOutcome::Failed(reason) => {
            ForgeQuerySignalCompatibilityOrchestrationOutcome::Failed(reason)
        }
    }
}
