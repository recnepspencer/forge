use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationBridgeContinuationRequest,
    ForgeQueryDeclarationBridgeRoutingInput, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationSignalCompatibilityChecked,
    ForgeQueryDeclarationSignalExecutionFamily, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext,
};
use crate::basis_lifecycle::BasisFamily;
use crate::binding_pipeline::ForgeQueryBindingLinkedArtifacts;
use crate::identity::hash_parts;

use crate::continuation_pipeline::artifacts::ForgeQueryPreparedContinuationSignalPosture;
use crate::continuation_pipeline::ForgeQueryPreparedContinuation;

pub(super) fn required_capability_families_for_prepared<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    bridge_request: &ForgeQueryDeclarationBridgeContinuationRequest,
    signal_execution_family: Option<ForgeQueryDeclarationSignalExecutionFamily>,
) -> Vec<crate::application::ForgeQueryCapabilityFamily> {
    let mut required = Vec::new();

    if let Some(contract) = I::Family::bridge_continuation_contract() {
        required.extend_from_slice(contract.required_capability_families());
    }

    if let Some(family) = signal_execution_family {
        let signal_required = match family {
            ForgeQueryDeclarationSignalExecutionFamily::RuntimeDerivedExecution
            | ForgeQueryDeclarationSignalExecutionFamily::MixedDerivedExecution => {
                &[crate::application::ForgeQueryCapabilityFamily::QueryComposition][..]
            }
            ForgeQueryDeclarationSignalExecutionFamily::HistoricalDerivedExecution => &[
                crate::application::ForgeQueryCapabilityFamily::QueryComposition,
                crate::application::ForgeQueryCapabilityFamily::HistoricalEvaluation,
            ][..],
            ForgeQueryDeclarationSignalExecutionFamily::PreviewDerivedExecution => &[
                crate::application::ForgeQueryCapabilityFamily::QueryComposition,
                crate::application::ForgeQueryCapabilityFamily::PreviewSession,
            ][..],
        };
        required.extend_from_slice(signal_required);
    }

    if matches!(
        bridge_request.truth_context(),
        crate::application::ForgeQueryDeclarationBridgeTruthContext::Historical
    ) {
        required.push(crate::application::ForgeQueryCapabilityFamily::HistoricalEvaluation);
    }

    required.sort();
    required.dedup();
    required
}

pub(super) struct ResolvedSignalContinuationTruth {
    pub(super) posture: ForgeQueryPreparedContinuationSignalPosture,
    pub(super) execution_family: Option<ForgeQueryDeclarationSignalExecutionFamily>,
    pub(super) basis_families: Vec<BasisFamily>,
    pub(super) digest: Option<String>,
    pub(super) reason: &'static str,
}

pub(super) fn linked_from_subject<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    subject: &ForgeQueryDeclarationBridgeRoutingInput<D, I>,
) -> ForgeQueryBindingLinkedArtifacts {
    let envelope = match subject {
        ForgeQueryDeclarationBridgeRoutingInput::Enveloped(envelope) => envelope,
        ForgeQueryDeclarationBridgeRoutingInput::Deferred(envelope) => envelope.envelope(),
        ForgeQueryDeclarationBridgeRoutingInput::Denied(envelope) => envelope.envelope(),
        ForgeQueryDeclarationBridgeRoutingInput::Failed(envelope) => envelope.envelope(),
        ForgeQueryDeclarationBridgeRoutingInput::EnvelopeChecked(checked) => match checked {
            crate::application::ForgeQueryDeclarationEnvelopeChecked::Enveloped(envelope) => {
                envelope
            }
            crate::application::ForgeQueryDeclarationEnvelopeChecked::Deferred(envelope) => {
                envelope.envelope()
            }
            crate::application::ForgeQueryDeclarationEnvelopeChecked::Denied(envelope) => {
                envelope.envelope()
            }
            crate::application::ForgeQueryDeclarationEnvelopeChecked::Failed(envelope) => {
                envelope.envelope()
            }
        },
    };

    let mut linked = ForgeQueryBindingLinkedArtifacts::new()
        .with_declaration_digest(envelope.declaration_digest())
        .with_receipt_digest(canonical_digest_token(envelope.receipt_digest()))
        .with_envelope_digest(canonical_digest_token(envelope.envelope_digest()));
    if let Some(progression_digest) = envelope.progression_digest() {
        linked = linked.with_progression_digest(progression_digest);
    }
    if let Some(route_plan_digest) = envelope.route_plan_digest() {
        linked = linked.with_route_plan_digest(route_plan_digest);
    }
    linked
}

pub(super) fn linked_from_prepared<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    prepared: &ForgeQueryPreparedContinuation<D, I>,
) -> ForgeQueryBindingLinkedArtifacts {
    let mut linked = ForgeQueryBindingLinkedArtifacts::new()
        .with_declaration_digest(prepared.declaration_digest())
        .with_receipt_digest(canonical_digest_token(prepared.receipt_digest()))
        .with_envelope_digest(canonical_digest_token(prepared.envelope_digest()));
    if let Some(progression_digest) = prepared.progression_digest() {
        linked = linked.with_progression_digest(progression_digest);
    }
    if let Some(route_plan_digest) = prepared.route_plan_digest() {
        linked = linked.with_route_plan_digest(route_plan_digest);
    }
    linked
}

pub(super) fn signal_subject_from_bridge_subject<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    subject: ForgeQueryDeclarationBridgeRoutingInput<D, I>,
) -> crate::application::ForgeQueryDeclarationSignalCompatibilityInput<D, I> {
    match subject {
        ForgeQueryDeclarationBridgeRoutingInput::Enveloped(envelope) => {
            crate::application::ForgeQueryDeclarationSignalCompatibilityInput::enveloped(envelope)
        }
        ForgeQueryDeclarationBridgeRoutingInput::Deferred(envelope) => {
            crate::application::ForgeQueryDeclarationSignalCompatibilityInput::deferred(envelope)
        }
        ForgeQueryDeclarationBridgeRoutingInput::Denied(envelope) => {
            crate::application::ForgeQueryDeclarationSignalCompatibilityInput::denied(envelope)
        }
        ForgeQueryDeclarationBridgeRoutingInput::Failed(envelope) => {
            crate::application::ForgeQueryDeclarationSignalCompatibilityInput::failed(envelope)
        }
        ForgeQueryDeclarationBridgeRoutingInput::EnvelopeChecked(checked) => {
            crate::application::ForgeQueryDeclarationSignalCompatibilityInput::envelope_checked(
                checked,
            )
        }
    }
}

pub(super) fn bridge_subject_and_signal_truth<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    checked: ForgeQueryDeclarationSignalCompatibilityChecked<D, I>,
) -> (
    ResolvedSignalContinuationTruth,
    ForgeQueryDeclarationBridgeRoutingInput<D, I>,
) {
    match checked {
        ForgeQueryDeclarationSignalCompatibilityChecked::Compatible(compatibility) => (
            ResolvedSignalContinuationTruth {
                posture: ForgeQueryPreparedContinuationSignalPosture::Compatible,
                execution_family: Some(compatibility.execution_family()),
                basis_families: compatibility.basis_families().to_vec(),
                digest: Some(compatibility.signal_compatibility_digest().to_string()),
                reason: "compatible",
            },
            ForgeQueryDeclarationBridgeRoutingInput::enveloped(compatibility.into_envelope()),
        ),
        ForgeQueryDeclarationSignalCompatibilityChecked::Deferred(deferred) => (
            ResolvedSignalContinuationTruth {
                posture: ForgeQueryPreparedContinuationSignalPosture::Deferred,
                execution_family: None,
                basis_families: I::Family::signal_compatibility_contract()
                    .map(|contract| contract.required_basis_families().to_vec())
                    .unwrap_or_default(),
                digest: None,
                reason: deferred.reason(),
            },
            ForgeQueryDeclarationBridgeRoutingInput::enveloped(deferred.into_envelope()),
        ),
        ForgeQueryDeclarationSignalCompatibilityChecked::Denied(denied) => (
            ResolvedSignalContinuationTruth {
                posture: ForgeQueryPreparedContinuationSignalPosture::Denied,
                execution_family: denied.execution_family(),
                basis_families: denied.basis_families().to_vec(),
                digest: None,
                reason: denied.reason(),
            },
            ForgeQueryDeclarationBridgeRoutingInput::enveloped(denied.into_envelope()),
        ),
        ForgeQueryDeclarationSignalCompatibilityChecked::Failed(failed) => (
            ResolvedSignalContinuationTruth {
                posture: ForgeQueryPreparedContinuationSignalPosture::Failed,
                execution_family: None,
                basis_families: I::Family::signal_compatibility_contract()
                    .map(|contract| contract.required_basis_families().to_vec())
                    .unwrap_or_default(),
                digest: None,
                reason: failed.reason(),
            },
            ForgeQueryDeclarationBridgeRoutingInput::enveloped(failed.into_envelope()),
        ),
    }
}

pub(super) fn prepared_digest<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    bridge_request: &ForgeQueryDeclarationBridgeContinuationRequest,
    required_contract: &crate::application::ForgeQueryDeclarationAspectContract,
    linked: &ForgeQueryBindingLinkedArtifacts,
    signal_truth: &ResolvedSignalContinuationTruth,
    bridge_routing: &crate::application::ForgeQueryDeclarationBridgeRouting<D, I>,
) -> String {
    hash_parts(&[
        "forge_query_prepared_continuation_v1".to_string(),
        handle.handle_identity_digest().to_string(),
        handle.operating_context_identity_digest().to_string(),
        format!("bridge_mode:{}", bridge_request.mode().as_str()),
        format!("truth_context:{}", bridge_request.truth_context().as_str()),
        format!("required_contract:{required_contract:?}"),
        format!("signal_posture:{:?}", signal_truth.posture),
        format!(
            "signal_basis_families:{}",
            signal_truth
                .basis_families
                .iter()
                .map(BasisFamily::as_str)
                .collect::<Vec<_>>()
                .join("|")
        ),
        format!("signal_digest:{:?}", signal_truth.digest),
        format!(
            "future_projection:{}",
            bridge_routing.future_projection().projection_digest()
        ),
        format!(
            "basis_lifecycle_support:{}",
            bridge_routing.basis_lifecycle_support_digest()
        ),
        format!("bridge_routing:{}", bridge_routing.bridge_routing_digest()),
        format!("linked:{linked:?}"),
    ])
}

pub(super) fn transcript_digest(
    kind: &str,
    family: &'static str,
    linked: &ForgeQueryBindingLinkedArtifacts,
    outcome: &str,
) -> String {
    hash_parts(&[
        "forge_query_continuation_pipeline_v1".to_string(),
        kind.to_string(),
        family.to_string(),
        format!("outcome:{outcome}"),
        format!("linked:{linked:?}"),
    ])
}

fn canonical_digest_token(digest: &forge_foundational::facade::CanonicalDerivedDigest) -> String {
    let hex = digest
        .value()
        .bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    format!("{}:{hex}", digest.metadata().algorithm().id().as_str())
}
