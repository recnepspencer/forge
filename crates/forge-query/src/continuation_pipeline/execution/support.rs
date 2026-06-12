use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationBridgeContinuationRequest,
    ForgeQueryDeclarationBridgeRoutingInput, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationSignalCompatibilityChecked,
    ForgeQueryDeclarationSignalExecutionFamily, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext,
};
use crate::basis_lifecycle::BasisFamily;
use crate::binding_pipeline::ForgeQueryBindingLinkedArtifacts;
use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};

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
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    prepared: &ForgeQueryPreparedContinuation<D, I>,
) -> ForgeQueryBindingLinkedArtifacts {
    let mut linked = ForgeQueryBindingLinkedArtifacts::new()
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
    forge_query_evidence_identity(ForgeQueryEvidenceScope::ContinuationPreparedDigest)
        .field_identity(
            ForgeQueryEvidenceTag::new("handle"),
            handle.handle_identity_digest(),
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("operating_context"),
            handle.operating_context_identity_digest(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("bridge_mode"),
            bridge_request.mode().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("truth_context"),
            bridge_request.truth_context().as_str(),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("required_aspects"),
            required_contract.required().iter().map(String::as_str),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("preserved_aspects"),
            required_contract.preserved().iter().map(String::as_str),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("published_aspects"),
            required_contract.published().iter().map(String::as_str),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("masked_aspects"),
            required_contract.masked().iter().map(String::as_str),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("incompatible_aspects"),
            required_contract.incompatible().iter().map(String::as_str),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("signal_posture"),
            signal_truth.posture.as_str(),
        )
        .optional_shape(
            ForgeQueryEvidenceTag::new("signal_execution_family"),
            signal_truth.execution_family.map(|family| family.as_str()),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("signal_basis_families"),
            signal_truth.basis_families.iter().map(BasisFamily::as_str),
        )
        .optional_value(
            ForgeQueryEvidenceTag::new("signal_digest"),
            signal_truth.digest.as_deref(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("signal_reason"),
            signal_truth.reason,
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("future_projection"),
            bridge_routing.future_projection().projection_digest(),
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("basis_lifecycle_support"),
            bridge_routing.basis_lifecycle_support_digest(),
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("bridge_routing"),
            bridge_routing.bridge_routing_digest(),
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("linked_artifacts"),
            linked_artifacts_identity(linked),
        )
        .seal()
        .as_str()
        .to_string()
}

pub(super) fn transcript_digest(
    kind: &str,
    family: &'static str,
    linked: &ForgeQueryBindingLinkedArtifacts,
    outcome: &str,
) -> String {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::ContinuationExecutionTranscript)
        .field_shape(ForgeQueryEvidenceTag::new("kind"), kind)
        .field_shape(ForgeQueryEvidenceTag::new("family"), family)
        .field_shape(ForgeQueryEvidenceTag::new("outcome"), outcome)
        .field_identity(
            ForgeQueryEvidenceTag::new("linked_artifacts"),
            linked_artifacts_identity(linked),
        )
        .seal()
        .as_str()
        .to_string()
}

pub(super) fn linked_artifacts_identity(linked: &ForgeQueryBindingLinkedArtifacts) -> String {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::ContinuationLinkedArtifacts)
        .optional_value(
            ForgeQueryEvidenceTag::new("declaration"),
            linked.declaration_digest(),
        )
        .optional_value(
            ForgeQueryEvidenceTag::new("progression"),
            linked.progression_digest(),
        )
        .optional_value(
            ForgeQueryEvidenceTag::new("route_plan"),
            linked.route_plan_digest(),
        )
        .optional_value(
            ForgeQueryEvidenceTag::new("receipt"),
            linked.receipt_digest(),
        )
        .optional_value(
            ForgeQueryEvidenceTag::new("envelope"),
            linked.envelope_digest(),
        )
        .optional_value(
            ForgeQueryEvidenceTag::new("orchestration"),
            linked.orchestration_digest(),
        )
        .seal()
        .as_str()
        .to_string()
}

fn canonical_derived_digest_identity(
    role: &'static str,
    digest: &forge_foundational::facade::CanonicalDerivedDigest,
) -> String {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::ContinuationLinkedArtifacts)
        .field_shape(ForgeQueryEvidenceTag::new("role"), role)
        .field_shape(
            ForgeQueryEvidenceTag::new("algorithm"),
            digest.metadata().algorithm().id().as_str(),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("bytes"),
            digest.value().bytes().iter().map(|byte| hex_byte(*byte)),
        )
        .seal()
        .as_str()
        .to_string()
}

fn hex_byte(byte: u8) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(2);
    encoded.push(HEX[(byte >> 4) as usize] as char);
    encoded.push(HEX[(byte & 0x0f) as usize] as char);
    encoded
}
