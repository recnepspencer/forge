use forge_runtime_bridge::facade::{
    BridgeCausalEvidenceFamily, BridgeCausalEvidenceOwner, BridgeCausalExplanationEnvelope,
};

use super::policy::{
    CausalInspectionMaterializationError, CausalInspectionMaterializationErrorKind,
    CausalInspectionMaterializationPolicy,
};
use crate::runtime::inspection::causal::inventory::CausalEvidenceFamily;

pub(super) fn validate_materialization_contract(
    query_observation_digest: &str,
    requested_families: &[CausalEvidenceFamily],
    envelope: &BridgeCausalExplanationEnvelope,
    materialization_policy: CausalInspectionMaterializationPolicy,
) -> Result<(), CausalInspectionMaterializationError> {
    if envelope.bindings().is_empty() {
        return Err(CausalInspectionMaterializationError::new(
            CausalInspectionMaterializationErrorKind::MaterializationPolicyOverclaim,
            &[
                format!("policy:{}", materialization_policy.as_str()),
                format!("envelope:{}", envelope.envelope_digest()),
                "bindings:0".to_string(),
            ],
        ));
    }
    validate_query_observation_binding(query_observation_digest, envelope)?;
    validate_requested_replay_posture(requested_families, envelope)
}

fn validate_query_observation_binding(
    query_observation_digest: &str,
    envelope: &BridgeCausalExplanationEnvelope,
) -> Result<(), CausalInspectionMaterializationError> {
    let mut query_observation_binding_count = 0;
    let mut query_observation_matches_subject = false;
    for binding in envelope.bindings() {
        if binding.owner() != BridgeCausalEvidenceOwner::Query
            || binding.family() != BridgeCausalEvidenceFamily::QueryObservation
        {
            continue;
        }
        query_observation_binding_count += 1;
        query_observation_matches_subject |=
            binding.reference_identity() == query_observation_digest;
    }

    match (
        query_observation_binding_count,
        query_observation_matches_subject,
    ) {
        (0, _) => Err(query_observation_binding_error(
            CausalInspectionMaterializationErrorKind::QueryObservationBindingMissing,
            query_observation_digest,
            envelope,
            0,
        )),
        (1, true) => Ok(()),
        (1, false) => Err(query_observation_binding_error(
            CausalInspectionMaterializationErrorKind::QueryObservationBindingMismatch,
            query_observation_digest,
            envelope,
            1,
        )),
        (binding_count, _) => Err(query_observation_binding_error(
            CausalInspectionMaterializationErrorKind::QueryObservationBindingOverclaim,
            query_observation_digest,
            envelope,
            binding_count,
        )),
    }
}

fn query_observation_binding_error(
    kind: CausalInspectionMaterializationErrorKind,
    query_observation_digest: &str,
    envelope: &BridgeCausalExplanationEnvelope,
    query_observation_binding_count: usize,
) -> CausalInspectionMaterializationError {
    CausalInspectionMaterializationError::new(
        kind,
        &[
            format!("expected-query-observation:{query_observation_digest}"),
            format!("query-observation-binding-count:{query_observation_binding_count}"),
            format!("envelope:{}", envelope.envelope_digest()),
        ],
    )
}

fn validate_requested_replay_posture(
    requested_families: &[CausalEvidenceFamily],
    envelope: &BridgeCausalExplanationEnvelope,
) -> Result<(), CausalInspectionMaterializationError> {
    let bridge_replay_requested = requested_families
        .iter()
        .any(|family| *family == CausalEvidenceFamily::BridgeReplay);
    let signal_cursor_requested = requested_families
        .iter()
        .any(|family| *family == CausalEvidenceFamily::SignalReplayCursor);
    if !bridge_replay_requested && !signal_cursor_requested {
        return Ok(());
    }

    let bridge_replay_bound = envelope.bindings().iter().any(|binding| {
        binding.owner() == BridgeCausalEvidenceOwner::RuntimeBridge
            && matches!(
                binding.family(),
                BridgeCausalEvidenceFamily::BridgeStreamReplay
                    | BridgeCausalEvidenceFamily::BridgeWritebackReplay
            )
    });
    let signal_cursor_bound = envelope.bindings().iter().any(|binding| {
        binding.owner() == BridgeCausalEvidenceOwner::Signal
            && binding.family() == BridgeCausalEvidenceFamily::SignalReplayCursor
    });
    let bridge_replay_satisfied = !bridge_replay_requested || bridge_replay_bound;
    let signal_cursor_satisfied = !signal_cursor_requested || signal_cursor_bound;
    if bridge_replay_satisfied && signal_cursor_satisfied {
        return Ok(());
    }

    Err(CausalInspectionMaterializationError::new(
        CausalInspectionMaterializationErrorKind::ReplayPostureUnsupported,
        &[
            format!("envelope:{}", envelope.envelope_digest()),
            format!("bridge-replay-requested:{bridge_replay_requested}"),
            format!("bridge-replay-bound:{bridge_replay_bound}"),
            format!("signal-cursor-requested:{signal_cursor_requested}"),
            format!("signal-cursor-bound:{signal_cursor_bound}"),
        ],
    ))
}
