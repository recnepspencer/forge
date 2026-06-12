use crate::diagnostics::BridgeDiagnosticsFacade;
use crate::identity::BridgeIdentityEvidence;
use crate::policy::BridgeDiagnosticsTier;
use crate::writeback::{
    BridgeMappedWritebackFamilyInputIdentity, BridgeWritebackEffectClass,
    BridgeWritebackExecutionRecordIdentity, BridgeWritebackFailureClass,
    BridgeWritebackFamilyAdmissionRecordIdentity, BridgeWritebackFamilyKind,
    BridgeWritebackMapperEnvelopeIdentity, BridgeWritebackMapperRecordIdentity,
    BridgeWritebackOutcomeClass, BridgeWritebackReplayRecordIdentity, BridgeWritebackStrategyClass,
};

use super::super::digest_basis::{
    compose_retained_causal_mapping_evidence_identity, retained_mapping_identity_digest_part,
    retained_mapping_shape_part, retained_mapping_value_part, RetainedCausalMappingDigestArtifact,
};

pub(crate) fn writeback_admission_record_digest(
    facade: &BridgeDiagnosticsFacade,
    reference_identity: &str,
) -> Option<BridgeIdentityEvidence> {
    facade
        .writeback_admission_record_for_identity(
            &BridgeWritebackFamilyAdmissionRecordIdentity::new(reference_identity),
        )
        .map(|record| writeback_admission_digest(&record))
}

pub(crate) fn writeback_admission_digest(
    record: &crate::writeback::BridgeWritebackFamilyAdmissionRecord,
) -> BridgeIdentityEvidence {
    let replay_permitted = record.replay_artifacts_permitted().to_string();
    compose_retained_causal_mapping_evidence_identity(
        RetainedCausalMappingDigestArtifact::WritebackAdmissionRecord,
        &[
            retained_mapping_identity_digest_part(record.record_identity().as_str()),
            retained_mapping_identity_digest_part(record.declaration_identity()),
            retained_mapping_identity_digest_part(record.contract_digest()),
            retained_mapping_shape_part(writeback_family_kind_label(record.family_kind())),
            retained_mapping_shape_part(writeback_effect_class_label(record.effect_class())),
            retained_mapping_shape_part(writeback_strategy_class_label(record.strategy_class())),
            retained_mapping_identity_digest_part(record.strategy_descriptor_digest()),
            retained_mapping_identity_digest_part(record.family_basis_digest()),
            retained_mapping_identity_digest_part(record.strategy_basis_digest()),
            retained_mapping_identity_digest_part(record.lowered_policy_digest()),
            retained_mapping_shape_part(diagnostics_tier_label(record.diagnostics_tier())),
            retained_mapping_value_part(replay_permitted.as_str()),
            retained_mapping_identity_digest_part(record.digest()),
        ],
    )
}

pub(crate) fn writeback_mapper_envelope_digest(
    facade: &BridgeDiagnosticsFacade,
    reference_identity: &str,
) -> Option<BridgeIdentityEvidence> {
    facade
        .writeback_mapper_envelope_for_identity(&BridgeWritebackMapperEnvelopeIdentity::new(
            reference_identity,
        ))
        .map(|envelope| writeback_mapper_envelope_artifact_digest(&envelope))
}

pub(crate) fn writeback_mapper_envelope_artifact_digest(
    envelope: &crate::writeback::BridgeWritebackMapperEnvelope,
) -> BridgeIdentityEvidence {
    compose_retained_causal_mapping_evidence_identity(
        RetainedCausalMappingDigestArtifact::WritebackMapperEnvelope,
        &[
            retained_mapping_identity_digest_part(envelope.envelope_identity().as_str()),
            retained_mapping_identity_digest_part(envelope.contract_digest()),
            retained_mapping_shape_part(writeback_family_kind_label(envelope.family_kind())),
            retained_mapping_shape_part(writeback_effect_class_label(envelope.effect_class())),
            retained_mapping_shape_part(writeback_strategy_class_label(envelope.strategy_class())),
            retained_mapping_identity_digest_part(envelope.strategy_descriptor_digest()),
            retained_mapping_identity_digest_part(envelope.causality_digest()),
            retained_mapping_identity_digest_part(envelope.effect_intent_digest()),
            retained_mapping_identity_digest_part(envelope.digest()),
        ],
    )
}

pub(crate) fn writeback_mapped_family_input_digest(
    facade: &BridgeDiagnosticsFacade,
    reference_identity: &str,
) -> Option<BridgeIdentityEvidence> {
    facade
        .writeback_mapped_family_input_for_identity(&BridgeMappedWritebackFamilyInputIdentity::new(
            reference_identity,
        ))
        .map(|mapped_input| writeback_mapped_family_input_artifact_digest(&mapped_input))
}

pub(crate) fn writeback_mapped_family_input_artifact_digest(
    mapped_input: &crate::writeback::BridgeMappedWritebackFamilyInput,
) -> BridgeIdentityEvidence {
    compose_retained_causal_mapping_evidence_identity(
        RetainedCausalMappingDigestArtifact::WritebackMappedFamilyInput,
        &[
            retained_mapping_identity_digest_part(mapped_input.mapped_input_identity().as_str()),
            retained_mapping_identity_digest_part(mapped_input.mapper_envelope_digest()),
            retained_mapping_identity_digest_part(mapped_input.contract_digest()),
            retained_mapping_shape_part(writeback_family_kind_label(mapped_input.family_kind())),
            retained_mapping_shape_part(writeback_effect_class_label(mapped_input.effect_class())),
            retained_mapping_shape_part(writeback_strategy_class_label(
                mapped_input.strategy_class(),
            )),
            retained_mapping_identity_digest_part(mapped_input.strategy_descriptor_digest()),
            retained_mapping_identity_digest_part(mapped_input.causality_digest()),
            retained_mapping_identity_digest_part(mapped_input.effect_intent_digest()),
            retained_mapping_identity_digest_part(mapped_input.digest()),
        ],
    )
}

pub(crate) fn writeback_mapper_record_digest(
    facade: &BridgeDiagnosticsFacade,
    reference_identity: &str,
) -> Option<BridgeIdentityEvidence> {
    facade
        .writeback_mapper_record_for_identity(&BridgeWritebackMapperRecordIdentity::new(
            reference_identity,
        ))
        .map(|record| writeback_mapper_record_artifact_digest(&record))
}

pub(crate) fn writeback_mapper_record_artifact_digest(
    record: &crate::writeback::BridgeWritebackMapperRecord,
) -> BridgeIdentityEvidence {
    compose_retained_causal_mapping_evidence_identity(
        RetainedCausalMappingDigestArtifact::WritebackMapperRecord,
        &[
            retained_mapping_identity_digest_part(record.record_identity().as_str()),
            retained_mapping_identity_digest_part(record.mapper_envelope_digest()),
            retained_mapping_identity_digest_part(record.mapped_input_digest()),
            retained_mapping_identity_digest_part(record.witness_digest()),
            retained_mapping_identity_digest_part(record.candidate_digest()),
            retained_mapping_shape_part(writeback_family_kind_label(record.family_kind())),
            retained_mapping_shape_part(writeback_effect_class_label(record.effect_class())),
            retained_mapping_shape_part(writeback_strategy_class_label(record.strategy_class())),
            retained_mapping_identity_digest_part(record.strategy_descriptor_digest()),
            retained_mapping_identity_digest_part(record.causality_digest()),
            retained_mapping_identity_digest_part(record.effect_intent_digest()),
            retained_mapping_identity_digest_part(record.digest()),
        ],
    )
}

pub(crate) fn writeback_execution_record_digest(
    facade: &BridgeDiagnosticsFacade,
    reference_identity: &str,
) -> Option<BridgeIdentityEvidence> {
    facade
        .writeback_execution_record_for_identity(&BridgeWritebackExecutionRecordIdentity::new(
            reference_identity,
        ))
        .map(|record| writeback_execution_artifact_digest(&record))
}

pub(crate) fn writeback_execution_artifact_digest(
    record: &crate::writeback::BridgeWritebackExecutionRecord,
) -> BridgeIdentityEvidence {
    let outcome_class = record
        .outcome_class()
        .map(writeback_outcome_class_label)
        .unwrap_or("none");
    let failure_class = record
        .failure_class()
        .map(writeback_failure_class_label)
        .unwrap_or("none");
    compose_retained_causal_mapping_evidence_identity(
        RetainedCausalMappingDigestArtifact::WritebackExecutionRecord,
        &[
            retained_mapping_identity_digest_part(record.record_identity().as_str()),
            retained_mapping_identity_digest_part(record.contract_digest()),
            retained_mapping_identity_digest_part(record.writeback_effect_artifact_digest()),
            retained_mapping_identity_digest_part(record.effect_intent_digest()),
            retained_mapping_shape_part(writeback_family_kind_label(record.family_kind())),
            retained_mapping_shape_part(writeback_strategy_class_label(record.strategy_class())),
            retained_mapping_identity_digest_part(record.causality_digest()),
            retained_mapping_identity_digest_part(record.idempotence_digest()),
            retained_mapping_identity_digest_part(record.loop_prevention_digest()),
            retained_mapping_identity_digest_part(record.strategy_coherence_digest()),
            retained_mapping_identity_digest_part(record.mapper_record_digest().unwrap_or("none")),
            retained_mapping_identity_digest_part(record.candidate_digest().unwrap_or("none")),
            retained_mapping_identity_digest_part(record.outcome_digest().unwrap_or("none")),
            retained_mapping_shape_part(outcome_class),
            retained_mapping_identity_digest_part(record.replay_bundle_digest().unwrap_or("none")),
            retained_mapping_identity_digest_part(record.request_digest().unwrap_or("none")),
            retained_mapping_identity_digest_part(record.receipt_digest().unwrap_or("none")),
            retained_mapping_shape_part(failure_class),
            retained_mapping_identity_digest_part(record.failure_digest().unwrap_or("none")),
            retained_mapping_identity_digest_part(record.counters().digest()),
            retained_mapping_identity_digest_part(record.digest()),
        ],
    )
}

pub(crate) fn writeback_replay_record_digest(
    facade: &BridgeDiagnosticsFacade,
    reference_identity: &str,
) -> Option<BridgeIdentityEvidence> {
    facade
        .writeback_replay_record_for_identity(&BridgeWritebackReplayRecordIdentity::new(
            reference_identity,
        ))
        .map(|record| writeback_replay_artifact_digest(&record))
}

pub(crate) fn writeback_replay_artifact_digest(
    record: &crate::writeback::BridgeWritebackReplayRecord,
) -> BridgeIdentityEvidence {
    let failure_class = record
        .failure_class()
        .map(writeback_failure_class_label)
        .unwrap_or("none");
    compose_retained_causal_mapping_evidence_identity(
        RetainedCausalMappingDigestArtifact::WritebackReplayRecord,
        &[
            retained_mapping_identity_digest_part(record.record_identity().as_str()),
            retained_mapping_shape_part(writeback_family_kind_label(record.family_kind())),
            retained_mapping_identity_digest_part(record.expected_replay_digest()),
            retained_mapping_identity_digest_part(record.replayed_replay_digest()),
            retained_mapping_identity_digest_part(record.expected_semantic_digest()),
            retained_mapping_identity_digest_part(record.replayed_semantic_digest()),
            retained_mapping_identity_digest_part(record.expected_effect_intent_digest()),
            retained_mapping_identity_digest_part(record.replayed_effect_intent_digest()),
            retained_mapping_identity_digest_part(
                record.expected_effect_intent_patch_canonical_basis(),
            ),
            retained_mapping_identity_digest_part(
                record.replayed_effect_intent_patch_canonical_basis(),
            ),
            retained_mapping_identity_digest_part(record.expected_causality_digest()),
            retained_mapping_identity_digest_part(record.replayed_causality_digest()),
            retained_mapping_shape_part(failure_class),
            retained_mapping_identity_digest_part(record.counters().digest()),
            retained_mapping_identity_digest_part(record.digest()),
        ],
    )
}

fn writeback_family_kind_label(value: BridgeWritebackFamilyKind) -> &'static str {
    match value {
        BridgeWritebackFamilyKind::ProjectedStateDiff => "projected-state-diff",
        BridgeWritebackFamilyKind::AspectReconciliation => "aspect-reconciliation",
    }
}

fn writeback_effect_class_label(value: BridgeWritebackEffectClass) -> &'static str {
    match value {
        BridgeWritebackEffectClass::ProjectedStateDiff => "projected-state-diff",
        BridgeWritebackEffectClass::AspectReconciliation => "aspect-reconciliation",
    }
}

fn writeback_strategy_class_label(value: BridgeWritebackStrategyClass) -> &'static str {
    match value {
        BridgeWritebackStrategyClass::ProjectedStateDiffReconciliation => {
            "projected-state-diff-reconciliation"
        }
        BridgeWritebackStrategyClass::AspectReconciliationCommit => "aspect-reconciliation-commit",
    }
}

fn writeback_outcome_class_label(value: BridgeWritebackOutcomeClass) -> &'static str {
    match value {
        BridgeWritebackOutcomeClass::CanonicalNoop => "canonical-noop",
        BridgeWritebackOutcomeClass::AuthoritativeCommit => "authoritative-commit",
        BridgeWritebackOutcomeClass::Rejected => "rejected",
    }
}

fn writeback_failure_class_label(value: BridgeWritebackFailureClass) -> &'static str {
    match value {
        BridgeWritebackFailureClass::WritebackNotRequested => "writeback-not-requested",
        BridgeWritebackFailureClass::PolicyRejected => "policy-rejected",
        BridgeWritebackFailureClass::StrategyUnavailable => "strategy-unavailable",
        BridgeWritebackFailureClass::FamilyBindingMismatch => "family-binding-mismatch",
        BridgeWritebackFailureClass::StrategyDescriptorMismatch => "strategy-descriptor-mismatch",
        BridgeWritebackFailureClass::IdempotenceBasisMismatch => "idempotence-basis-mismatch",
        BridgeWritebackFailureClass::StaleTruthBasis => "stale-truth-basis",
        BridgeWritebackFailureClass::InvariantRejected => "invariant-rejected",
        BridgeWritebackFailureClass::MergeAuthorityRejected => "merge-authority-rejected",
        BridgeWritebackFailureClass::StrategyFailed => "strategy-failed",
        BridgeWritebackFailureClass::StrategyPanicked => "strategy-panicked",
        BridgeWritebackFailureClass::ReplayMismatch => "replay-mismatch",
        BridgeWritebackFailureClass::AuthorityDenied => "authority-denied",
        BridgeWritebackFailureClass::PreviewWritebackRejected => "preview-writeback-rejected",
    }
}

fn diagnostics_tier_label(value: BridgeDiagnosticsTier) -> &'static str {
    match value {
        BridgeDiagnosticsTier::Minimal => "minimal",
        BridgeDiagnosticsTier::Standard => "standard",
        BridgeDiagnosticsTier::Exhaustive => "exhaustive",
    }
}
