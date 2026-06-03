use crate::diagnostics::BridgeDiagnosticsFacade;
use crate::writeback::{
    BridgeMappedWritebackFamilyInputIdentity, BridgeWritebackExecutionRecordIdentity,
    BridgeWritebackFamilyAdmissionRecordIdentity, BridgeWritebackMapperEnvelopeIdentity,
    BridgeWritebackMapperRecordIdentity, BridgeWritebackReplayRecordIdentity,
};

use super::super::digest_basis::{retained_mapping_digest, RetainedCausalMappingDigestArtifact};

pub(crate) fn writeback_admission_record_digest(
    facade: &BridgeDiagnosticsFacade,
    reference_identity: &str,
) -> Option<String> {
    facade
        .writeback_admission_record_for_identity(
            &BridgeWritebackFamilyAdmissionRecordIdentity::new(reference_identity),
        )
        .map(|record| {
            let family_kind = format!("{:?}", record.family_kind());
            let effect_class = format!("{:?}", record.effect_class());
            let strategy_class = format!("{:?}", record.strategy_class());
            let diagnostics_tier = format!("{:?}", record.diagnostics_tier());
            let replay_permitted = record.replay_artifacts_permitted().to_string();
            retained_mapping_digest(
                RetainedCausalMappingDigestArtifact::WritebackAdmissionRecord,
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
        })
}

pub(crate) fn writeback_mapper_envelope_digest(
    facade: &BridgeDiagnosticsFacade,
    reference_identity: &str,
) -> Option<String> {
    facade
        .writeback_mapper_envelope_for_identity(&BridgeWritebackMapperEnvelopeIdentity::new(
            reference_identity,
        ))
        .map(|envelope| {
            let family_kind = format!("{:?}", envelope.family_kind());
            let effect_class = format!("{:?}", envelope.effect_class());
            let strategy_class = format!("{:?}", envelope.strategy_class());
            retained_mapping_digest(
                RetainedCausalMappingDigestArtifact::WritebackMapperEnvelope,
                &[
                    envelope.envelope_identity().as_str(),
                    envelope.contract_digest(),
                    family_kind.as_str(),
                    effect_class.as_str(),
                    strategy_class.as_str(),
                    envelope.strategy_descriptor_digest(),
                    envelope.causality_digest(),
                    envelope.effect_intent_digest(),
                    envelope.digest(),
                ],
            )
        })
}

pub(crate) fn writeback_mapped_family_input_digest(
    facade: &BridgeDiagnosticsFacade,
    reference_identity: &str,
) -> Option<String> {
    facade
        .writeback_mapped_family_input_for_identity(&BridgeMappedWritebackFamilyInputIdentity::new(
            reference_identity,
        ))
        .map(|mapped_input| {
            let family_kind = format!("{:?}", mapped_input.family_kind());
            let effect_class = format!("{:?}", mapped_input.effect_class());
            let strategy_class = format!("{:?}", mapped_input.strategy_class());
            retained_mapping_digest(
                RetainedCausalMappingDigestArtifact::WritebackMappedFamilyInput,
                &[
                    mapped_input.mapped_input_identity().as_str(),
                    mapped_input.mapper_envelope_digest(),
                    mapped_input.contract_digest(),
                    family_kind.as_str(),
                    effect_class.as_str(),
                    strategy_class.as_str(),
                    mapped_input.strategy_descriptor_digest(),
                    mapped_input.causality_digest(),
                    mapped_input.effect_intent_digest(),
                    mapped_input.digest(),
                ],
            )
        })
}

pub(crate) fn writeback_mapper_record_digest(
    facade: &BridgeDiagnosticsFacade,
    reference_identity: &str,
) -> Option<String> {
    facade
        .writeback_mapper_record_for_identity(&BridgeWritebackMapperRecordIdentity::new(
            reference_identity,
        ))
        .map(|record| {
            let family_kind = format!("{:?}", record.family_kind());
            let effect_class = format!("{:?}", record.effect_class());
            let strategy_class = format!("{:?}", record.strategy_class());
            retained_mapping_digest(
                RetainedCausalMappingDigestArtifact::WritebackMapperRecord,
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
                    record.effect_intent_digest(),
                    record.digest(),
                ],
            )
        })
}

pub(crate) fn writeback_execution_record_digest(
    facade: &BridgeDiagnosticsFacade,
    reference_identity: &str,
) -> Option<String> {
    facade
        .writeback_execution_record_for_identity(&BridgeWritebackExecutionRecordIdentity::new(
            reference_identity,
        ))
        .map(|record| {
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
            retained_mapping_digest(
                RetainedCausalMappingDigestArtifact::WritebackExecutionRecord,
                &[
                    record.record_identity().as_str(),
                    record.contract_digest(),
                    record.writeback_effect_artifact_digest(),
                    record.effect_intent_digest(),
                    family_kind.as_str(),
                    strategy_class.as_str(),
                    record.causality_digest(),
                    record.idempotence_digest(),
                    record.loop_prevention_digest(),
                    record.strategy_coherence_digest(),
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
        })
}

pub(crate) fn writeback_replay_record_digest(
    facade: &BridgeDiagnosticsFacade,
    reference_identity: &str,
) -> Option<String> {
    facade
        .writeback_replay_record_for_identity(&BridgeWritebackReplayRecordIdentity::new(
            reference_identity,
        ))
        .map(|record| {
            let family_kind = format!("{:?}", record.family_kind());
            let failure_class = record
                .failure_class()
                .map(|value| format!("{value:?}"))
                .unwrap_or_else(|| "none".to_string());
            retained_mapping_digest(
                RetainedCausalMappingDigestArtifact::WritebackReplayRecord,
                &[
                    record.record_identity().as_str(),
                    family_kind.as_str(),
                    record.expected_replay_digest(),
                    record.replayed_replay_digest(),
                    record.expected_semantic_digest(),
                    record.replayed_semantic_digest(),
                    record.expected_effect_intent_digest(),
                    record.replayed_effect_intent_digest(),
                    record.expected_effect_intent_patch_canonical_basis(),
                    record.replayed_effect_intent_patch_canonical_basis(),
                    record.expected_causality_digest(),
                    record.replayed_causality_digest(),
                    failure_class.as_str(),
                    record.counters().digest(),
                    record.digest(),
                ],
            )
        })
}
