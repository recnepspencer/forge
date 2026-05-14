mod retained_digests;

use super::{
    BridgeCausalEnvelopeCounters, BridgeCausalEnvelopeDenial, BridgeCausalEnvelopeDenialKind,
    BridgeCausalEvidenceFamily, BridgeCausalEvidenceReference,
};
use crate::diagnostics::BridgeDiagnosticsFacade;
use retained_digests::{
    bulk_planning_digest, historical_evaluation_failure_digest, stream_checkpoint_digest,
};

pub(crate) fn retained_record_digest(
    facade: &BridgeDiagnosticsFacade,
    reference: &BridgeCausalEvidenceReference,
) -> Result<Option<String>, BridgeCausalEnvelopeDenial> {
    match reference.family() {
        BridgeCausalEvidenceFamily::BridgeBulkPlanning => Ok(facade
            .bulk_record_for_workload_identity(reference.reference_identity())
            .map(|record| bulk_planning_digest(&record))),
        BridgeCausalEvidenceFamily::BridgeRoute => Ok(facade
            .route_record_for_route_identity(reference.reference_identity())
            .map(|record| {
                digest(
                    "bridge-causal-retained-route-record",
                    &[
                        record.route_identity().as_str(),
                        record.invalidation_identity().as_str(),
                        record.source_commit().as_str(),
                        record.planning_summary_digest(),
                        record.lowering_summary_digest(),
                    ],
                )
            })),
        BridgeCausalEvidenceFamily::BridgeHistoricalEvaluation => Ok(facade
            .historical_record_for_record_identity(reference.reference_identity())
            .map(|record| {
                digest(
                    "bridge-causal-retained-historical-record",
                    &[
                        record.record_identity().as_str(),
                        record.decision_log().decision_log_identity().as_str(),
                        record.decision_log().snapshot_identity().as_str(),
                    ],
                )
            })),
        BridgeCausalEvidenceFamily::BridgeHistoricalEvaluationFailure => Ok(facade
            .historical_failure_for_identity(reference.reference_identity())
            .map(|record| historical_evaluation_failure_digest(&record))),
        BridgeCausalEvidenceFamily::BridgePreviewExecution => Ok(facade
            .preview_execution_record_for_identity(reference.reference_identity())
            .map(|record| {
                digest(
                    "bridge-causal-retained-preview-execution-record",
                    &[
                        record.record_identity().as_str(),
                        record.preview_session_identity(),
                        record.preview_declaration_digest(),
                        record.branch_binding_digest(),
                        record.digest(),
                    ],
                )
            })),
        BridgeCausalEvidenceFamily::BridgePreviewDiscard => Ok(facade
            .preview_discard_record_for_identity(reference.reference_identity())
            .map(|record| {
                digest(
                    "bridge-causal-retained-preview-discard-record",
                    &[
                        record.record_identity().as_str(),
                        record.preview_session_identity(),
                        record.preview_execution_record_identity().as_str(),
                        record.residue_report().digest(),
                        record.digest(),
                    ],
                )
            })),
        BridgeCausalEvidenceFamily::BridgePreviewPromotion => Ok(facade
            .preview_promotion_record_for_identity(reference.reference_identity())
            .map(|record| {
                digest(
                    "bridge-causal-retained-preview-promotion-record",
                    &[
                        record.record_identity().as_str(),
                        record.preview_session_identity(),
                        record.preview_execution_record_identity().as_str(),
                        record.promotion_proof_digest(),
                        record.authoritative_commit_boundary_digest(),
                        record.authoritative_artifact_digest(),
                        record.digest(),
                    ],
                )
            })),
        BridgeCausalEvidenceFamily::BridgeSourceMaterialization => Ok(facade
            .source_materialization_record_for_identity(reference.reference_identity())
            .map(|record| {
                digest(
                    "bridge-causal-retained-source-materialization-record",
                    &[
                        record.record_identity().as_str(),
                        record.source_contract_identity(),
                        record.source_declaration_identity(),
                        record.source_capability_digest(),
                        record.adapter_capability_digest(),
                        record.planned_packet_set_digest(),
                        record.materialized_packet_set_digest(),
                        record.digest(),
                    ],
                )
            })),
        BridgeCausalEvidenceFamily::BridgeSourceFailure => Ok(facade
            .source_failure_record_for_identity(reference.reference_identity())
            .map(|record| {
                let failure_class = format!("{:?}", record.failure_class());
                let delivery_error_kind = format!("{:?}", record.delivery_error_kind());
                digest(
                    "bridge-causal-retained-source-failure-record",
                    &[
                        record.failure_identity().as_str(),
                        record.declaration_identity().as_str(),
                        record.selector_identity(),
                        record.source_capability_digest(),
                        failure_class.as_str(),
                        delivery_error_kind.as_str(),
                        record.digest(),
                    ],
                )
            })),
        BridgeCausalEvidenceFamily::BridgeStructuralRemap => Ok(facade
            .structural_remap_record_for_identity(reference.reference_identity())
            .map(|record| {
                digest(
                    "bridge-causal-retained-structural-remap-record",
                    &[
                        record.record_identity().as_str(),
                        record.schema_version(),
                        record.contract().digest(),
                        record.planned_packet_set().digest(),
                        record.reduced_match_set().digest(),
                        record.artifact().digest(),
                    ],
                )
            })),
        BridgeCausalEvidenceFamily::BridgeStructuralBranchComparison => Ok(facade
            .structural_branch_comparison_record_for_identity(reference.reference_identity())
            .map(|record| {
                digest(
                    "bridge-causal-retained-structural-branch-comparison-record",
                    &[
                        record.record_identity().as_str(),
                        record.schema_version(),
                        record.contract().digest(),
                        record.planned_packet_set().digest(),
                        record.reduced_match_set().digest(),
                        record.artifact().digest(),
                    ],
                )
            })),
        BridgeCausalEvidenceFamily::BridgeStreamReplay => Ok(facade
            .stream_replay_record_for_identity(reference.reference_identity())
            .map(|record| {
                digest(
                    "bridge-causal-retained-stream-replay-record",
                    &[
                        record.replay_record_identity().as_str(),
                        record.consumer_contract_identity().as_str(),
                        record.stream_window_identity().as_str(),
                        record.checkpoint_token_identity(),
                        record.replay_basis_digest(),
                        record.protocol_semantics_version(),
                        record.digest(),
                    ],
                )
            })),
        BridgeCausalEvidenceFamily::BridgeStreamCheckpoint => Ok(facade
            .stream_checkpoint_for_identity(reference.reference_identity())
            .map(|record| stream_checkpoint_digest(&record))),
        BridgeCausalEvidenceFamily::BridgeContinuity => Ok(facade
            .continuity_record_for_route_identity(reference.reference_identity())
            .map(|record| {
                digest(
                    "bridge-causal-retained-continuity-record",
                    &[
                        record.route_identity().as_str(),
                        record.schema_version(),
                        record.continuity_request_digest(),
                        record.continuity_resolution_digest(),
                        record.continuity_artifact_identity().as_str(),
                        record.remapped_subscription_slice_identity().as_str(),
                    ],
                )
            })),
        BridgeCausalEvidenceFamily::BridgeMerge => Ok(facade
            .merge_record_for_identity(reference.reference_identity())
            .map(|record| {
                digest(
                    "bridge-causal-retained-merge-record",
                    &[
                        record.record_identity().as_str(),
                        record.schema_version(),
                        record.contract().digest(),
                        record.bundle().digest(),
                    ],
                )
            })),
        BridgeCausalEvidenceFamily::BridgeWritebackAdmission => Ok(facade
            .writeback_admission_record_for_identity(reference.reference_identity())
            .map(|record| {
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
            })),
        BridgeCausalEvidenceFamily::BridgeWritebackMapperEnvelope => Ok(facade
            .writeback_mapper_envelope_for_identity(reference.reference_identity())
            .map(|envelope| {
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
            })),
        BridgeCausalEvidenceFamily::BridgeWritebackMappedFamilyInput => Ok(facade
            .writeback_mapped_family_input_for_identity(reference.reference_identity())
            .map(|mapped_input| {
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
            })),
        BridgeCausalEvidenceFamily::BridgeWritebackMapper => Ok(facade
            .writeback_mapper_record_for_identity(reference.reference_identity())
            .map(|record| {
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
            })),
        BridgeCausalEvidenceFamily::BridgeWritebackExecution => Ok(facade
            .writeback_execution_record_for_identity(reference.reference_identity())
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
            })),
        BridgeCausalEvidenceFamily::BridgeWritebackReplay => Ok(facade
            .writeback_replay_record_for_identity(reference.reference_identity())
            .map(|record| {
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
            })),
        BridgeCausalEvidenceFamily::QueryObservation
        | BridgeCausalEvidenceFamily::RelationalAuthority
        | BridgeCausalEvidenceFamily::SignalInvalidation
        | BridgeCausalEvidenceFamily::SignalEvaluation
        | BridgeCausalEvidenceFamily::SignalForensicAvailability
        | BridgeCausalEvidenceFamily::SignalReplayCursor
        | BridgeCausalEvidenceFamily::SignalLineage
        | BridgeCausalEvidenceFamily::SignalProvenance => Err(BridgeCausalEnvelopeDenial::new(
            BridgeCausalEnvelopeDenialKind::EvidenceOwnerMismatch,
            reference.family(),
            reference.owner(),
            reference.family().expected_owner(),
            reference.reference_identity().into(),
            BridgeCausalEnvelopeCounters::new(1, 1, 1, 0, 0, 0, 1),
        )),
    }
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
