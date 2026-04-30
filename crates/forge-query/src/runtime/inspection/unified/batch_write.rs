use crate::identity::hash_parts;

use super::ForgeQueryBatchWriteComponentInspection;
use crate::runtime::{
    ForgeQueryAuthorityLane, ForgeQueryBatchMutationEvidence, ForgeQueryBatchWriteReceipt,
    ForgeQueryMutationCausalityEvidence, ForgeQueryMutationProvenanceEvidence,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBatchWriteReceiptInspection {
    authority_lane: ForgeQueryAuthorityLane,
    basis_lane: ForgeQueryAuthorityLane,
    batch_digest: String,
    batch_mutation_evidence: ForgeQueryBatchMutationEvidence,
    write_receipt_count: usize,
    commit_identities: Vec<String>,
    component_operations: Vec<ForgeQueryBatchWriteComponentInspection>,
    touched_aspect_paths: Vec<String>,
    affected_live_view_ids: Vec<String>,
    affected_derived_view_ids: Vec<String>,
    inspection_digest: String,
}

impl ForgeQueryBatchWriteReceiptInspection {
    pub(in crate::runtime) fn new(receipt: &ForgeQueryBatchWriteReceipt) -> Self {
        let commit_identities = receipt
            .write_receipts()
            .iter()
            .map(|entry| entry.commit_identity().to_string())
            .collect::<Vec<_>>();
        let component_operations = receipt
            .write_receipts()
            .iter()
            .map(ForgeQueryBatchWriteComponentInspection::from_write_receipt)
            .collect::<Vec<_>>();
        let batch_mutation_evidence = receipt.batch_mutation_evidence().clone();
        let touched_aspect_paths = receipt.touched_aspect_paths().to_vec();
        let affected_live_view_ids = receipt.affected_live_view_ids().to_vec();
        let affected_derived_view_ids = receipt.affected_derived_view_ids().to_vec();
        let inspection_digest = hash_parts(
            &std::iter::once("forge_query_batch_write_receipt_inspection_v1".to_string())
                .chain(std::iter::once(format!(
                    "authority:{}",
                    receipt.authority_lane()
                )))
                .chain(std::iter::once(format!("basis:{}", receipt.basis_lane())))
                .chain(std::iter::once(format!("batch:{}", receipt.batch_digest())))
                .chain(std::iter::once(format!(
                    "batch-evidence:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
                    batch_mutation_evidence.component_count(),
                    batch_mutation_evidence.target_evidence_count(),
                    batch_mutation_evidence.existing_truth_assertion_count(),
                    batch_mutation_evidence.retained_authoritative_assertion_count(),
                    batch_mutation_evidence.backend_verified_assertion_count(),
                    batch_mutation_evidence.backend_verified_update_count(),
                    batch_mutation_evidence.backend_verified_delete_count(),
                    batch_mutation_evidence.existing_truth_binding_count(),
                    batch_mutation_evidence.symbolic_target_reference_count(),
                    batch_mutation_evidence.naming_mutation_count(),
                    batch_mutation_evidence.continuity_mutation_count(),
                    batch_mutation_evidence.resolved_target_count(),
                    batch_mutation_evidence.target_collection_count(),
                    batch_mutation_evidence.target_entity_count(),
                    batch_mutation_evidence.causality_bundle_count(),
                    batch_mutation_evidence.provenance_bundle_count(),
                    batch_mutation_evidence.outcome_class_count(),
                    batch_mutation_evidence.request_digest_count(),
                    batch_mutation_evidence.receipt_digest_count(),
                    batch_mutation_evidence.aggregate_target_digest()
                )))
                .chain(std::iter::once(format!(
                    "batch-assertion:{}",
                    batch_mutation_evidence
                        .aggregate_existing_truth_assertion_digest()
                        .unwrap_or("none")
                )))
                .chain(std::iter::once(format!(
                    "batch-existing-truth-mode:{}",
                    batch_mutation_evidence
                        .aggregate_existing_truth_mode_digest()
                        .unwrap_or("none")
                )))
                .chain(std::iter::once(format!(
                    "batch-continuity:{}",
                    batch_mutation_evidence
                        .aggregate_continuity_mutation_digest()
                        .unwrap_or("none")
                )))
                .chain(std::iter::once(format!(
                    "batch-existing:{}",
                    batch_mutation_evidence
                        .aggregate_existing_truth_binding_digest()
                        .unwrap_or("none")
                )))
                .chain(std::iter::once(format!(
                    "batch-symbolic:{}",
                    batch_mutation_evidence
                        .aggregate_symbolic_target_reference_digest()
                        .unwrap_or("none")
                )))
                .chain(std::iter::once(format!(
                    "batch-naming:{}",
                    batch_mutation_evidence
                        .aggregate_naming_mutation_digest()
                        .unwrap_or("none")
                )))
                .chain(std::iter::once(format!(
                    "batch-causality:{}",
                    batch_mutation_evidence
                        .aggregate_causality_digest()
                        .unwrap_or("none")
                )))
                .chain(std::iter::once(format!(
                    "batch-provenance:{}",
                    batch_mutation_evidence
                        .aggregate_provenance_digest()
                        .unwrap_or("none")
                )))
                .chain(
                    commit_identities
                        .iter()
                        .map(|commit| format!("commit:{commit}")),
                )
                .chain(component_operations.iter().flat_map(|component| {
                    std::iter::once(format!("family:{}", component.family()))
                        .chain(std::iter::once(format!(
                            "component-target:{}:{}:{}:{}:{}:{}",
                            component.target_evidence().declared().target_class(),
                            component
                                .target_evidence()
                                .declared()
                                .collection()
                                .unwrap_or(""),
                            component
                                .target_evidence()
                                .declared()
                                .entity_identity()
                                .unwrap_or(""),
                            component.target_evidence().resolved().target_class(),
                            component
                                .target_evidence()
                                .resolved()
                                .collection()
                                .unwrap_or(""),
                            component
                                .target_evidence()
                                .resolved()
                                .entity_identity()
                                .unwrap_or("")
                        )))
                        .chain(std::iter::once(format!(
                            "component-assertion:{}",
                            component
                                .existing_truth_assertion_evidence()
                                .map_or("none", |evidence| evidence.verification_digest())
                        )))
                        .chain(std::iter::once(format!(
                            "component-existing-truth:{}:{}:{}:{}:{}",
                            component
                                .existing_truth_binding_evidence()
                                .map_or("none", |evidence| evidence.family().as_str()),
                            component
                                .existing_truth_binding_evidence()
                                .map_or("none", |evidence| evidence.authoritative_identity()),
                            component
                                .existing_truth_binding_evidence()
                                .map_or("none", |evidence| evidence.resolved_target_identity()),
                            component
                                .existing_truth_binding_evidence()
                                .and_then(|evidence| evidence.target_collection())
                                .unwrap_or("none"),
                            component
                                .existing_truth_binding_evidence()
                                .map_or("none", |evidence| evidence.binding_digest())
                        )))
                        .chain(std::iter::once(format!(
                            "component-symbolic:{}",
                            component
                                .symbolic_target_reference_evidence()
                                .map_or("none", |evidence| evidence.symbol())
                        )))
                        .chain(std::iter::once(format!(
                            "component-causality:{}",
                            component.causality_evidence().map_or(
                                "none",
                                ForgeQueryMutationCausalityEvidence::causality_digest
                            )
                        )))
                        .chain(std::iter::once(format!(
                            "component-continuity:{}:{}",
                            component
                                .continuity_mutation_evidence()
                                .map_or("none", |evidence| evidence.lineage_digest()),
                            component
                                .continuity_mutation_evidence()
                                .map_or("none", |evidence| {
                                    evidence.continuity_resolution_digest()
                                })
                        )))
                        .chain(std::iter::once(format!(
                            "component-provenance:{}",
                            component.provenance_evidence().map_or(
                                "none",
                                ForgeQueryMutationProvenanceEvidence::execution_record_digest
                            )
                        )))
                        .chain(
                            component
                                .collections()
                                .iter()
                                .map(|collection| format!("collection:{collection}")),
                        )
                        .chain(
                            component
                                .entity_identities()
                                .iter()
                                .map(|entity| format!("entity:{entity}")),
                        )
                        .chain(
                            component
                                .declared_aspect_operations()
                                .iter()
                                .map(|operation| {
                                    format!(
                                        "component-operation:{}:{}",
                                        operation.kind(),
                                        operation.aspect_path()
                                    )
                                }),
                        )
                        .chain(
                            component
                                .touched_aspect_paths()
                                .iter()
                                .map(|path| format!("component-aspect:{path}")),
                        )
                }))
                .chain(
                    touched_aspect_paths
                        .iter()
                        .map(|path| format!("aspect:{path}")),
                )
                .chain(
                    affected_live_view_ids
                        .iter()
                        .map(|view| format!("live:{view}")),
                )
                .chain(
                    affected_derived_view_ids
                        .iter()
                        .map(|view| format!("derived:{view}")),
                )
                .collect::<Vec<_>>(),
        );
        Self {
            authority_lane: receipt.authority_lane(),
            basis_lane: receipt.basis_lane(),
            batch_digest: receipt.batch_digest().to_string(),
            batch_mutation_evidence,
            write_receipt_count: receipt.write_count(),
            commit_identities,
            component_operations,
            touched_aspect_paths,
            affected_live_view_ids,
            affected_derived_view_ids,
            inspection_digest,
        }
    }

    pub fn authority_lane(&self) -> ForgeQueryAuthorityLane {
        self.authority_lane
    }

    pub fn basis_lane(&self) -> ForgeQueryAuthorityLane {
        self.basis_lane
    }

    pub fn batch_digest(&self) -> &str {
        &self.batch_digest
    }

    pub fn batch_mutation_evidence(&self) -> &ForgeQueryBatchMutationEvidence {
        &self.batch_mutation_evidence
    }

    pub fn write_receipt_count(&self) -> usize {
        self.write_receipt_count
    }

    pub fn commit_identities(&self) -> &[String] {
        &self.commit_identities
    }

    pub fn component_operations(&self) -> &[ForgeQueryBatchWriteComponentInspection] {
        &self.component_operations
    }

    pub fn touched_aspect_paths(&self) -> &[String] {
        &self.touched_aspect_paths
    }

    pub fn affected_live_view_ids(&self) -> &[String] {
        &self.affected_live_view_ids
    }

    pub fn affected_derived_view_ids(&self) -> &[String] {
        &self.affected_derived_view_ids
    }

    pub fn inspection_digest(&self) -> &str {
        &self.inspection_digest
    }
}
