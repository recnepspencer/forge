use crate::identity::hash_parts;
use crate::runtime::{
    ForgeQueryAspectMutationOperation, ForgeQueryContinuityMutationEvidence,
    ForgeQueryContinuityOutcomeClass, ForgeQueryExistingTruthAssertionEvidence,
    ForgeQueryExistingTruthBindingEvidence, ForgeQueryMutationCausalityEvidence,
    ForgeQueryMutationMetadata, ForgeQueryMutationProvenanceEvidence,
    ForgeQueryMutationTargetEvidence, ForgeQueryNamingMutationEvidence,
    ForgeQueryRuntimeInspectionEvidence, ForgeQuerySymbolicTargetReferenceEvidence,
    ForgeQueryWriteReceipt,
};

#[allow(clippy::too_many_arguments)]
pub(super) fn build_write_receipt_inspection_digest(
    receipt: &ForgeQueryWriteReceipt,
    target_evidence: &ForgeQueryMutationTargetEvidence,
    existing_truth_assertion_evidence: Option<&ForgeQueryExistingTruthAssertionEvidence>,
    existing_truth_binding_evidence: Option<&ForgeQueryExistingTruthBindingEvidence>,
    symbolic_target_reference_evidence: Option<&ForgeQuerySymbolicTargetReferenceEvidence>,
    naming_mutation_evidence: Option<&ForgeQueryNamingMutationEvidence>,
    continuity_mutation_evidence: Option<&ForgeQueryContinuityMutationEvidence>,
    causality_evidence: Option<&ForgeQueryMutationCausalityEvidence>,
    provenance_evidence: Option<&ForgeQueryMutationProvenanceEvidence>,
    runtime_evidence: &ForgeQueryRuntimeInspectionEvidence,
    declared_aspect_operations: &[ForgeQueryAspectMutationOperation],
    declared_aspect_value_digest: Option<&str>,
    mutation_metadata: &ForgeQueryMutationMetadata,
    live_patch_artifacts: &[String],
) -> String {
    hash_parts(&[
        "forge_query_write_receipt_inspection_v1".to_string(),
        format!("family:{}", receipt.mutation_family()),
        format!("authority:{}", receipt.authority_lane()),
        format!("basis:{}", receipt.basis_lane()),
        format!("commit:{}", receipt.commit_identity()),
        format!("snapshot:{}", receipt.snapshot_token()),
        format!(
            "declared-collection:{}",
            receipt.declared_collection().unwrap_or("")
        ),
        format!(
            "declared-entity:{}",
            receipt.declared_entity_identity().unwrap_or("")
        ),
        format!(
            "target-collection:{}",
            receipt.target_collection().unwrap_or("")
        ),
        format!(
            "target-entity:{}",
            receipt.target_entity_identity().unwrap_or("")
        ),
        format!(
            "target-evidence:{}:{}:{}:{}:{}:{}",
            target_evidence.declared().target_class(),
            target_evidence.declared().collection().unwrap_or(""),
            target_evidence.declared().entity_identity().unwrap_or(""),
            target_evidence.resolved().target_class(),
            target_evidence.resolved().collection().unwrap_or(""),
            target_evidence.resolved().entity_identity().unwrap_or("")
        ),
        format!(
            "existing-assertion:{}:{}:{}:{}:{}:{}",
            existing_truth_assertion_evidence
                .map_or("none", |evidence| { evidence.mode().as_str() }),
            existing_truth_assertion_evidence
                .map_or(0, |evidence| { evidence.asserted_aspect_count() }),
            existing_truth_assertion_evidence
                .map_or("none", |evidence| { evidence.verification_digest() }),
            existing_truth_assertion_evidence
                .and_then(ForgeQueryExistingTruthAssertionEvidence::assumption_snapshot_digest)
                .unwrap_or("none"),
            existing_truth_assertion_evidence
                .and_then(ForgeQueryExistingTruthAssertionEvidence::verified_precondition_digest)
                .unwrap_or("none"),
            existing_truth_assertion_evidence
                .and_then(ForgeQueryExistingTruthAssertionEvidence::verification_read_set_breadth)
                .map_or("none", |breadth| breadth.counter_snapshot()),
        ),
        format!(
            "existing-truth:{}:{}:{}:{}:{}",
            existing_truth_binding_evidence.map_or("none", |evidence| evidence.family().as_str()),
            existing_truth_binding_evidence.map_or(
                "none",
                ForgeQueryExistingTruthBindingEvidence::authoritative_identity
            ),
            existing_truth_binding_evidence
                .map_or("none", |evidence| { evidence.resolved_entity_identity() }),
            existing_truth_binding_evidence
                .and_then(ForgeQueryExistingTruthBindingEvidence::target_collection)
                .unwrap_or("none"),
            existing_truth_binding_evidence.map_or(
                "none",
                ForgeQueryExistingTruthBindingEvidence::binding_digest
            )
        ),
        format!(
            "symbolic-target:{}:{}:{}",
            symbolic_target_reference_evidence
                .map_or("none", ForgeQuerySymbolicTargetReferenceEvidence::symbol),
            symbolic_target_reference_evidence
                .map_or("none", |evidence| { evidence.resolved_entity_identity() }),
            symbolic_target_reference_evidence
                .and_then(ForgeQuerySymbolicTargetReferenceEvidence::target_collection)
                .unwrap_or("none")
        ),
        format!(
            "naming:{}:{}:{}:{}:{}:{}:{}",
            naming_mutation_evidence.map_or("none", |evidence| evidence.family().as_str()),
            naming_mutation_evidence.map_or("none", |evidence| evidence.attachment_identity()),
            naming_mutation_evidence
                .and_then(ForgeQueryNamingMutationEvidence::prior_authoritative_identity)
                .unwrap_or("none"),
            naming_mutation_evidence
                .and_then(ForgeQueryNamingMutationEvidence::target_authoritative_identity)
                .unwrap_or("none"),
            naming_mutation_evidence
                .and_then(ForgeQueryNamingMutationEvidence::resolved_target_entity_identity)
                .unwrap_or("none"),
            naming_mutation_evidence
                .and_then(ForgeQueryNamingMutationEvidence::target_collection)
                .unwrap_or("none"),
            naming_mutation_evidence
                .map(|evidence| format!("{:?}", evidence.outcome()))
                .unwrap_or_else(|| "none".to_string())
        ),
        format!(
            "continuity:{}:{}:{}:{}:{}:{}:{}:{}:{}",
            continuity_mutation_evidence.map_or("none".to_string(), |evidence| {
                evidence.family().as_str().to_string()
            }),
            continuity_mutation_evidence.map_or("none", |evidence| {
                evidence.prior_authoritative_identity()
            }),
            continuity_mutation_evidence.map_or("none".to_string(), |evidence| {
                if evidence.successor_authoritative_identities().is_empty() {
                    "none".to_string()
                } else {
                    evidence.successor_authoritative_identities().join("|")
                }
            }),
            continuity_mutation_evidence
                .map(continuity_outcome_label)
                .unwrap_or("none"),
            continuity_mutation_evidence
                .and_then(ForgeQueryContinuityMutationEvidence::basis_binding_digest)
                .unwrap_or("none"),
            continuity_mutation_evidence
                .and_then(ForgeQueryContinuityMutationEvidence::resolved_target_entity_identity)
                .unwrap_or("none"),
            continuity_mutation_evidence
                .and_then(ForgeQueryContinuityMutationEvidence::target_collection)
                .unwrap_or("none"),
            continuity_mutation_evidence
                .map_or("none", ForgeQueryContinuityMutationEvidence::lineage_digest),
            continuity_mutation_evidence.map_or("none", |evidence| {
                evidence.continuity_resolution_digest()
            }),
        ),
        format!(
            "causality-evidence:{}:{}:{}:{}:{}",
            causality_evidence.map_or(
                "none",
                ForgeQueryMutationCausalityEvidence::causality_digest
            ),
            causality_evidence.map_or(
                "none",
                ForgeQueryMutationCausalityEvidence::truth_trigger_digest
            ),
            causality_evidence.map_or("none", ForgeQueryMutationCausalityEvidence::route_digest),
            causality_evidence.map_or(
                "none",
                ForgeQueryMutationCausalityEvidence::evaluation_surface_digest
            ),
            causality_evidence.map_or(
                "none",
                ForgeQueryMutationCausalityEvidence::truth_view_digest
            )
        ),
        format!(
            "provenance-evidence:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
            provenance_evidence.map_or(
                "none",
                ForgeQueryMutationProvenanceEvidence::contract_digest
            ),
            provenance_evidence.map_or(
                "none",
                ForgeQueryMutationProvenanceEvidence::writeback_effect_artifact_digest
            ),
            provenance_evidence.map_or(
                "none",
                ForgeQueryMutationProvenanceEvidence::effect_intent_digest
            ),
            provenance_evidence.map_or(
                "none",
                ForgeQueryMutationProvenanceEvidence::effect_intent_patch_canonical_basis
            ),
            provenance_evidence.map_or(
                "none",
                ForgeQueryMutationProvenanceEvidence::feedback_provenance_digest
            ),
            provenance_evidence.map_or(
                "none",
                ForgeQueryMutationProvenanceEvidence::causality_digest
            ),
            provenance_evidence.map_or(
                "none",
                ForgeQueryMutationProvenanceEvidence::strategy_descriptor_digest
            ),
            provenance_evidence.map_or(
                "none",
                ForgeQueryMutationProvenanceEvidence::execution_record_digest
            ),
            provenance_evidence
                .and_then(ForgeQueryMutationProvenanceEvidence::authoritative_artifact_digest)
                .unwrap_or("none"),
            provenance_evidence
                .and_then(ForgeQueryMutationProvenanceEvidence::request_digest)
                .unwrap_or("none"),
            provenance_evidence
                .and_then(ForgeQueryMutationProvenanceEvidence::receipt_digest)
                .unwrap_or("none"),
            provenance_evidence
                .and_then(|e| e.outcome_class().map(|v| format!("{v:?}")))
                .unwrap_or_else(|| "none".to_string()),
            provenance_evidence
                .and_then(|e| e.failure_class().map(|v| format!("{v:?}")))
                .unwrap_or_else(|| "none".to_string())
        ),
        format!(
            "runtime:{}:{}:{}",
            runtime_evidence.artifact_family(),
            runtime_evidence.authority_lane(),
            runtime_evidence.evidence().join("|")
        ),
        format!(
            "declared-aspect-operations:{}",
            declared_aspect_operations
                .iter()
                .map(|operation| format!("{}:{}", operation.kind(), operation.aspect_path()))
                .collect::<Vec<_>>()
                .join("|")
        ),
        format!(
            "declared-aspect-value-digest:{}",
            declared_aspect_value_digest.unwrap_or("none")
        ),
        format!(
            "mutation-metadata:{}",
            mutation_metadata
                .entries()
                .iter()
                .map(|(key, value)| format!(
                    "{key}:{}",
                    serde_json::to_string(value).unwrap_or_else(|_| value.to_string())
                ))
                .collect::<Vec<_>>()
                .join("|")
        ),
        format!("patches:{}", live_patch_artifacts.join("|")),
    ])
}

fn continuity_outcome_label(evidence: &ForgeQueryContinuityMutationEvidence) -> &'static str {
    match evidence.outcome_class() {
        ForgeQueryContinuityOutcomeClass::ContinuesAsSingleSuccessor => {
            "continues_as_single_successor"
        }
        ForgeQueryContinuityOutcomeClass::ContinuesAsSplitSuccessors => {
            "continues_as_split_successors"
        }
        ForgeQueryContinuityOutcomeClass::ContinuesViaTruthLoweredCanonicalMergeSuccessor => {
            "continues_via_truth_lowered_canonical_merge_successor"
        }
        ForgeQueryContinuityOutcomeClass::RejectedNoAuthoritativeSuccessor => {
            "rejected_no_authoritative_successor"
        }
        ForgeQueryContinuityOutcomeClass::RejectedAmbiguousSuccessor => {
            "rejected_ambiguous_successor"
        }
        ForgeQueryContinuityOutcomeClass::RejectedUnsupportedContinuityClass => {
            "rejected_unsupported_continuity_class"
        }
        ForgeQueryContinuityOutcomeClass::RejectedHistoricalResolutionFailure => {
            "rejected_historical_resolution_failure"
        }
    }
}
