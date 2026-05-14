use crate::facade::{
    BridgeCausalEvidenceBinding, BridgeCausalEvidenceFamily, BridgeCausalEvidenceOwner,
    BridgeCausalEvidenceReference,
};

pub(super) fn bridge_reference(
    family: BridgeCausalEvidenceFamily,
    identity: &str,
) -> BridgeCausalEvidenceReference {
    BridgeCausalEvidenceReference::new(BridgeCausalEvidenceOwner::RuntimeBridge, family, identity)
        .expect("bridge reference should be valid")
}

pub(super) fn query_observation_reference(identity: &str) -> BridgeCausalEvidenceReference {
    BridgeCausalEvidenceReference::new(
        BridgeCausalEvidenceOwner::Query,
        BridgeCausalEvidenceFamily::QueryObservation,
        identity,
    )
    .expect("query observation reference should be valid")
}

pub(super) fn binding_for<'a>(
    bindings: &'a [BridgeCausalEvidenceBinding],
    family: BridgeCausalEvidenceFamily,
    reference_identity: &str,
) -> &'a BridgeCausalEvidenceBinding {
    bindings
        .iter()
        .find(|binding| {
            binding.owner() == BridgeCausalEvidenceOwner::RuntimeBridge
                && binding.family() == family
                && binding.reference_identity() == reference_identity
        })
        .expect("expected writeback causal binding should be present")
}

pub(super) fn writeback_admission_digest(
    record: &crate::facade::BridgeWritebackFamilyAdmissionRecord,
) -> String {
    let family_kind = format!("{:?}", record.family_kind());
    let effect_class = format!("{:?}", record.effect_class());
    let strategy_class = format!("{:?}", record.strategy_class());
    let diagnostics_tier = format!("{:?}", record.diagnostics_tier());
    let replay_permitted = record.replay_artifacts_permitted().to_string();
    digest(
        "bridge-causal-retained-writeback-admission-record",
        &[
            record.record_identity().as_str(),
            record.declaration_identity(),
            record.contract_digest(),
            family_kind.as_str(),
            effect_class.as_str(),
            strategy_class.as_str(),
            record.strategy_descriptor_digest(),
            record.family_basis_digest(),
            record.strategy_basis_digest(),
            record.lowered_policy_digest(),
            diagnostics_tier.as_str(),
            replay_permitted.as_str(),
            record.digest(),
        ],
    )
}

pub(super) fn writeback_mapper_envelope_digest(
    envelope: &crate::facade::BridgeWritebackMapperEnvelope,
) -> String {
    let family_kind = format!("{:?}", envelope.family_kind());
    let effect_class = format!("{:?}", envelope.effect_class());
    let strategy_class = format!("{:?}", envelope.strategy_class());
    digest(
        "bridge-causal-retained-writeback-mapper-envelope",
        &[
            envelope.envelope_identity().as_str(),
            envelope.contract_digest(),
            family_kind.as_str(),
            effect_class.as_str(),
            strategy_class.as_str(),
            envelope.strategy_descriptor_digest(),
            envelope.causality_digest(),
            envelope.domain_payload_digest(),
            envelope.domain_evidence_digest(),
            envelope.digest(),
        ],
    )
}

pub(super) fn writeback_mapped_input_digest(
    mapped_input: &crate::facade::BridgeMappedWritebackFamilyInput,
) -> String {
    let family_kind = format!("{:?}", mapped_input.family_kind());
    let effect_class = format!("{:?}", mapped_input.effect_class());
    let strategy_class = format!("{:?}", mapped_input.strategy_class());
    digest(
        "bridge-causal-retained-writeback-mapped-family-input",
        &[
            mapped_input.mapped_input_identity().as_str(),
            mapped_input.mapper_envelope_digest(),
            mapped_input.contract_digest(),
            family_kind.as_str(),
            effect_class.as_str(),
            strategy_class.as_str(),
            mapped_input.strategy_descriptor_digest(),
            mapped_input.causality_digest(),
            mapped_input.domain_payload_digest(),
            mapped_input.domain_evidence_digest(),
            mapped_input.digest(),
        ],
    )
}

pub(super) fn writeback_mapper_record_digest(
    record: &crate::facade::BridgeWritebackMapperRecord,
) -> String {
    let family_kind = format!("{:?}", record.family_kind());
    let effect_class = format!("{:?}", record.effect_class());
    let strategy_class = format!("{:?}", record.strategy_class());
    digest(
        "bridge-causal-retained-writeback-mapper-record",
        &[
            record.record_identity().as_str(),
            record.mapper_envelope_digest(),
            record.mapped_input_digest(),
            record.witness_digest(),
            record.candidate_digest(),
            family_kind.as_str(),
            effect_class.as_str(),
            strategy_class.as_str(),
            record.strategy_descriptor_digest(),
            record.causality_digest(),
            record.proposed_effect_digest(),
            record.digest(),
        ],
    )
}

pub(super) fn writeback_execution_digest(
    record: &crate::facade::BridgeWritebackExecutionRecord,
) -> String {
    let family_kind = format!("{:?}", record.family_kind());
    let strategy_class = format!("{:?}", record.strategy_class());
    let outcome_class = record
        .outcome_class()
        .map(|value| format!("{value:?}"))
        .unwrap_or_else(|| "none".to_string());
    let failure_class = record
        .failure_class()
        .map(|value| format!("{value:?}"))
        .unwrap_or_else(|| "none".to_string());
    digest(
        "bridge-causal-retained-writeback-execution-record",
        &[
            record.record_identity().as_str(),
            record.contract_digest(),
            record.derived_effect_digest(),
            record.proposed_effect_digest(),
            family_kind.as_str(),
            strategy_class.as_str(),
            record.causality_digest(),
            record.idempotence_digest(),
            record.loop_prevention_digest(),
            record.strategy_compatibility_digest(),
            record.mapper_record_digest().unwrap_or("none"),
            record.candidate_digest().unwrap_or("none"),
            record.outcome_digest().unwrap_or("none"),
            outcome_class.as_str(),
            record.replay_bundle_digest().unwrap_or("none"),
            record.request_digest().unwrap_or("none"),
            record.receipt_digest().unwrap_or("none"),
            failure_class.as_str(),
            record.failure_digest().unwrap_or("none"),
            record.counters().digest(),
            record.digest(),
        ],
    )
}

pub(super) fn writeback_replay_digest(
    record: &crate::facade::BridgeWritebackReplayRecord,
) -> String {
    let family_kind = format!("{:?}", record.family_kind());
    let failure_class = record
        .failure_class()
        .map(|value| format!("{value:?}"))
        .unwrap_or_else(|| "none".to_string());
    digest(
        "bridge-causal-retained-writeback-replay-record",
        &[
            record.record_identity().as_str(),
            family_kind.as_str(),
            record.expected_replay_digest(),
            record.replayed_replay_digest(),
            record.expected_semantic_digest(),
            record.replayed_semantic_digest(),
            record.expected_causality_digest(),
            record.replayed_causality_digest(),
            failure_class.as_str(),
            record.counters().digest(),
            record.digest(),
        ],
    )
}

fn digest(label: &str, parts: &[&str]) -> String {
    use sha2::{Digest, Sha256};

    let mut canonical = String::from(label);
    for part in parts {
        canonical.push('|');
        canonical.push_str(part);
    }
    let digest = Sha256::digest(canonical.as_bytes());
    format!("{label}:sha256:{digest:x}")
}
