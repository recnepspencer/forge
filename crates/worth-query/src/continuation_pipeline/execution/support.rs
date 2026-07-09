use crate::application::{
    WorthQueryAdmittedConfiguredDomainHandle, WorthQueryDeclarationBridgeContinuationRequest,
    WorthQueryDeclarationBridgeRoutingInput, WorthQueryDeclarationFamilyMarker,
    WorthQueryDeclarationInput, WorthQueryDeclarationSignalCompatibilityChecked,
    WorthQueryDeclarationSignalExecutionFamily, WorthQueryDomainEntryMarker,
    WorthQueryDomainOperatingContext,
};
use crate::basis_lifecycle::BasisFamily;
use crate::binding_pipeline::WorthQueryBindingLinkedArtifacts;
use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceScope, WorthQueryEvidenceTag,
};

use super::digest_projection::{hex_byte, terminal_declaration_aspect_projection_for_digest};
use crate::continuation_pipeline::artifacts::WorthQueryPreparedContinuationSignalPosture;
use crate::continuation_pipeline::WorthQueryPreparedContinuation;

pub(super) fn required_capability_families_for_prepared<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    bridge_request: &WorthQueryDeclarationBridgeContinuationRequest,
    signal_execution_family: Option<WorthQueryDeclarationSignalExecutionFamily>,
) -> Vec<crate::application::WorthQueryCapabilityFamily> {
    let mut required = Vec::new();

    if let Some(contract) = I::Family::bridge_continuation_contract() {
        required.extend_from_slice(contract.required_capability_families());
    }

    if let Some(family) = signal_execution_family {
        let signal_required = match family {
            WorthQueryDeclarationSignalExecutionFamily::RuntimeDerivedExecution
            | WorthQueryDeclarationSignalExecutionFamily::MixedDerivedExecution => {
                &[crate::application::WorthQueryCapabilityFamily::QueryComposition][..]
            }
            WorthQueryDeclarationSignalExecutionFamily::HistoricalDerivedExecution => &[
                crate::application::WorthQueryCapabilityFamily::QueryComposition,
                crate::application::WorthQueryCapabilityFamily::HistoricalEvaluation,
            ][..],
            WorthQueryDeclarationSignalExecutionFamily::PreviewDerivedExecution => &[
                crate::application::WorthQueryCapabilityFamily::QueryComposition,
                crate::application::WorthQueryCapabilityFamily::PreviewSession,
            ][..],
        };
        required.extend_from_slice(signal_required);
    }

    if matches!(
        bridge_request.truth_context(),
        crate::application::WorthQueryDeclarationBridgeTruthContext::Historical
    ) {
        required.push(crate::application::WorthQueryCapabilityFamily::HistoricalEvaluation);
    }

    required.sort();
    required.dedup();
    required
}

pub(super) struct ResolvedSignalContinuationTruth {
    pub(super) posture: WorthQueryPreparedContinuationSignalPosture,
    pub(super) execution_family: Option<WorthQueryDeclarationSignalExecutionFamily>,
    pub(super) basis_families: Vec<BasisFamily>,
    pub(super) digest: Option<String>,
    pub(super) reason: &'static str,
}

pub(super) fn linked_from_subject<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    subject: &WorthQueryDeclarationBridgeRoutingInput<D, I>,
) -> WorthQueryBindingLinkedArtifacts {
    let envelope = match subject {
        WorthQueryDeclarationBridgeRoutingInput::Enveloped(envelope) => envelope,
        WorthQueryDeclarationBridgeRoutingInput::Deferred(envelope) => envelope.envelope(),
        WorthQueryDeclarationBridgeRoutingInput::Denied(envelope) => envelope.envelope(),
        WorthQueryDeclarationBridgeRoutingInput::Failed(envelope) => envelope.envelope(),
        WorthQueryDeclarationBridgeRoutingInput::EnvelopeChecked(checked) => match checked {
            crate::application::WorthQueryDeclarationEnvelopeChecked::Enveloped(envelope) => {
                envelope
            }
            crate::application::WorthQueryDeclarationEnvelopeChecked::Deferred(envelope) => {
                envelope.envelope()
            }
            crate::application::WorthQueryDeclarationEnvelopeChecked::Denied(envelope) => {
                envelope.envelope()
            }
            crate::application::WorthQueryDeclarationEnvelopeChecked::Failed(envelope) => {
                envelope.envelope()
            }
        },
    };

    let mut linked = WorthQueryBindingLinkedArtifacts::new()
        .with_declaration_digest(envelope.declaration_digest())
        .with_receipt_digest(canonical_derived_digest_identity(
            "declaration-receipt",
            envelope.receipt_digest(),
        ))
        .with_envelope_digest(canonical_derived_digest_identity(
            "declaration-envelope",
            envelope.envelope_digest(),
        ));
    if let Some(progression_digest) = envelope.progression_digest() {
        linked = linked.with_progression_digest(progression_digest);
    }
    if let Some(route_plan_digest) = envelope.route_plan_digest() {
        linked = linked.with_route_plan_digest(route_plan_digest);
    }
    linked
}

pub(super) fn linked_from_prepared<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    prepared: &WorthQueryPreparedContinuation<D, I>,
) -> WorthQueryBindingLinkedArtifacts {
    let mut linked = WorthQueryBindingLinkedArtifacts::new()
        .with_declaration_digest(prepared.declaration_digest())
        .with_receipt_digest(canonical_derived_digest_identity(
            "declaration-receipt",
            prepared.receipt_digest(),
        ))
        .with_envelope_digest(canonical_derived_digest_identity(
            "declaration-envelope",
            prepared.envelope_digest(),
        ));
    if let Some(progression_digest) = prepared.progression_digest() {
        linked = linked.with_progression_digest(progression_digest);
    }
    if let Some(route_plan_digest) = prepared.route_plan_digest() {
        linked = linked.with_route_plan_digest(route_plan_digest);
    }
    linked
}

pub(super) fn signal_subject_from_bridge_subject<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    subject: WorthQueryDeclarationBridgeRoutingInput<D, I>,
) -> crate::application::WorthQueryDeclarationSignalCompatibilityInput<D, I> {
    match subject {
        WorthQueryDeclarationBridgeRoutingInput::Enveloped(envelope) => {
            crate::application::WorthQueryDeclarationSignalCompatibilityInput::enveloped(envelope)
        }
        WorthQueryDeclarationBridgeRoutingInput::Deferred(envelope) => {
            crate::application::WorthQueryDeclarationSignalCompatibilityInput::deferred(envelope)
        }
        WorthQueryDeclarationBridgeRoutingInput::Denied(envelope) => {
            crate::application::WorthQueryDeclarationSignalCompatibilityInput::denied(envelope)
        }
        WorthQueryDeclarationBridgeRoutingInput::Failed(envelope) => {
            crate::application::WorthQueryDeclarationSignalCompatibilityInput::failed(envelope)
        }
        WorthQueryDeclarationBridgeRoutingInput::EnvelopeChecked(checked) => {
            crate::application::WorthQueryDeclarationSignalCompatibilityInput::envelope_checked(
                checked,
            )
        }
    }
}

pub(super) fn bridge_subject_and_signal_truth<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    checked: WorthQueryDeclarationSignalCompatibilityChecked<D, I>,
) -> (
    ResolvedSignalContinuationTruth,
    WorthQueryDeclarationBridgeRoutingInput<D, I>,
) {
    match checked {
        WorthQueryDeclarationSignalCompatibilityChecked::Compatible(compatibility) => (
            ResolvedSignalContinuationTruth {
                posture: WorthQueryPreparedContinuationSignalPosture::Compatible,
                execution_family: Some(compatibility.execution_family()),
                basis_families: compatibility.basis_families().to_vec(),
                digest: Some(compatibility.signal_compatibility_digest().to_string()),
                reason: "compatible",
            },
            WorthQueryDeclarationBridgeRoutingInput::enveloped(compatibility.into_envelope()),
        ),
        WorthQueryDeclarationSignalCompatibilityChecked::Deferred(deferred) => (
            ResolvedSignalContinuationTruth {
                posture: WorthQueryPreparedContinuationSignalPosture::Deferred,
                execution_family: None,
                basis_families: I::Family::signal_compatibility_contract()
                    .map(|contract| contract.required_basis_families().to_vec())
                    .unwrap_or_default(),
                digest: None,
                reason: deferred.reason(),
            },
            WorthQueryDeclarationBridgeRoutingInput::enveloped(deferred.into_envelope()),
        ),
        WorthQueryDeclarationSignalCompatibilityChecked::Denied(denied) => (
            ResolvedSignalContinuationTruth {
                posture: WorthQueryPreparedContinuationSignalPosture::Denied,
                execution_family: denied.execution_family(),
                basis_families: denied.basis_families().to_vec(),
                digest: None,
                reason: denied.reason(),
            },
            WorthQueryDeclarationBridgeRoutingInput::enveloped(denied.into_envelope()),
        ),
        WorthQueryDeclarationSignalCompatibilityChecked::Failed(failed) => (
            ResolvedSignalContinuationTruth {
                posture: WorthQueryPreparedContinuationSignalPosture::Failed,
                execution_family: None,
                basis_families: I::Family::signal_compatibility_contract()
                    .map(|contract| contract.required_basis_families().to_vec())
                    .unwrap_or_default(),
                digest: None,
                reason: failed.reason(),
            },
            WorthQueryDeclarationBridgeRoutingInput::enveloped(failed.into_envelope()),
        ),
    }
}

pub(super) fn prepared_digest<
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
    I: WorthQueryDeclarationInput<D>,
>(
    handle: &WorthQueryAdmittedConfiguredDomainHandle<D, C>,
    bridge_request: &WorthQueryDeclarationBridgeContinuationRequest,
    required_contract: &crate::application::WorthQueryDeclarationAspectContract,
    linked: &WorthQueryBindingLinkedArtifacts,
    signal_truth: &ResolvedSignalContinuationTruth,
    bridge_routing: &crate::application::WorthQueryDeclarationBridgeRouting<D, I>,
) -> String {
    worth_query_evidence_identity(WorthQueryEvidenceScope::ContinuationPreparedDigest)
        .field_value(
            WorthQueryEvidenceTag::new("handle"),
            handle.handle_identity_digest(),
        )
        .field_value(
            WorthQueryEvidenceTag::new("operating_context"),
            handle.operating_context_identity_digest(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("bridge_mode"),
            bridge_request.mode().as_str(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("truth_context"),
            bridge_request.truth_context().as_str(),
        )
        .field_value_sequence(
            WorthQueryEvidenceTag::new("required_aspects"),
            required_contract
                .required()
                .iter()
                .map(terminal_declaration_aspect_projection_for_digest),
        )
        .field_value_sequence(
            WorthQueryEvidenceTag::new("preserved_aspects"),
            required_contract
                .preserved()
                .iter()
                .map(terminal_declaration_aspect_projection_for_digest),
        )
        .field_value_sequence(
            WorthQueryEvidenceTag::new("published_aspects"),
            required_contract
                .published()
                .iter()
                .map(terminal_declaration_aspect_projection_for_digest),
        )
        .field_value_sequence(
            WorthQueryEvidenceTag::new("masked_aspects"),
            required_contract
                .masked()
                .iter()
                .map(terminal_declaration_aspect_projection_for_digest),
        )
        .field_value_sequence(
            WorthQueryEvidenceTag::new("incompatible_aspects"),
            required_contract
                .incompatible()
                .iter()
                .map(terminal_declaration_aspect_projection_for_digest),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("signal_posture"),
            signal_truth.posture.as_str(),
        )
        .optional_shape(
            WorthQueryEvidenceTag::new("signal_execution_family"),
            signal_truth.execution_family.map(|family| family.as_str()),
        )
        .field_value_sequence(
            WorthQueryEvidenceTag::new("signal_basis_families"),
            signal_truth.basis_families.iter().map(BasisFamily::as_str),
        )
        .optional_value(
            WorthQueryEvidenceTag::new("signal_digest"),
            signal_truth.digest.as_deref(),
        )
        .field_value(
            WorthQueryEvidenceTag::new("signal_reason"),
            signal_truth.reason,
        )
        .field_value(
            WorthQueryEvidenceTag::new("future_projection"),
            bridge_routing.future_projection().projection_digest(),
        )
        .field_value(
            WorthQueryEvidenceTag::new("basis_lifecycle_support"),
            bridge_routing.basis_lifecycle_support_digest(),
        )
        .field_value(
            WorthQueryEvidenceTag::new("bridge_routing"),
            bridge_routing.bridge_routing_digest(),
        )
        .field_value(
            WorthQueryEvidenceTag::new("linked_artifacts"),
            linked_artifacts_identity(linked),
        )
        .seal()
        .as_str()
        .to_string()
}

pub(super) fn transcript_digest(
    kind: &str,
    family: &'static str,
    linked: &WorthQueryBindingLinkedArtifacts,
    outcome: &str,
) -> String {
    worth_query_evidence_identity(WorthQueryEvidenceScope::ContinuationExecutionTranscript)
        .field_shape(WorthQueryEvidenceTag::new("kind"), kind)
        .field_shape(WorthQueryEvidenceTag::new("family"), family)
        .field_shape(WorthQueryEvidenceTag::new("outcome"), outcome)
        .field_value(
            WorthQueryEvidenceTag::new("linked_artifacts"),
            linked_artifacts_identity(linked),
        )
        .seal()
        .as_str()
        .to_string()
}

pub(super) fn linked_artifacts_identity(linked: &WorthQueryBindingLinkedArtifacts) -> String {
    worth_query_evidence_identity(WorthQueryEvidenceScope::ContinuationLinkedArtifacts)
        .optional_value(
            WorthQueryEvidenceTag::new("declaration"),
            linked.declaration_digest(),
        )
        .optional_value(
            WorthQueryEvidenceTag::new("progression"),
            linked.progression_digest(),
        )
        .optional_value(
            WorthQueryEvidenceTag::new("route_plan"),
            linked.route_plan_digest(),
        )
        .optional_value(
            WorthQueryEvidenceTag::new("receipt"),
            linked.receipt_digest(),
        )
        .optional_value(
            WorthQueryEvidenceTag::new("envelope"),
            linked.envelope_digest(),
        )
        .optional_value(
            WorthQueryEvidenceTag::new("orchestration"),
            linked.orchestration_digest(),
        )
        .seal()
        .as_str()
        .to_string()
}

fn canonical_derived_digest_identity(
    role: &'static str,
    digest: &worth_foundational::facade::CanonicalDerivedDigest,
) -> String {
    worth_query_evidence_identity(WorthQueryEvidenceScope::ContinuationLinkedArtifacts)
        .field_shape(WorthQueryEvidenceTag::new("role"), role)
        .field_shape(
            WorthQueryEvidenceTag::new("algorithm"),
            digest.metadata().algorithm().id().as_str(),
        )
        .field_value_sequence(
            WorthQueryEvidenceTag::new("bytes"),
            digest.value().bytes().iter().map(|byte| hex_byte(*byte)),
        )
        .seal()
        .as_str()
        .to_string()
}
