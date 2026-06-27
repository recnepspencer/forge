use worth_spatial::facade::evidence_lookup_stage_cutover::{
    EvidenceLookupCoveredStageCutoverExplanation, EvidenceLookupCoveredStageCutoverProof,
    EvidenceLookupStageCutoverCounters, EvidenceLookupStageCutoverError,
    EvidenceLookupStageCutoverErrorKind, EvidenceLookupTopologyDerivedReceiptRef,
    EvidenceLookupTopologyDerivedReceiptState,
};
use worth_spatial::facade::workload_vocabulary::WorkloadEvidenceStage;

#[test]
fn spatial_public_api_exports_lookup_stage_cutover_proof_contract() {
    let _: fn(&EvidenceLookupCoveredStageCutoverProof) -> &str =
        EvidenceLookupCoveredStageCutoverProof::family_identity;
    let _: fn(&EvidenceLookupCoveredStageCutoverProof) -> WorkloadEvidenceStage =
        EvidenceLookupCoveredStageCutoverProof::stage;
    let _: fn(&EvidenceLookupCoveredStageCutoverProof) -> &str =
        EvidenceLookupCoveredStageCutoverProof::stage_receipt_identity;
    let _: fn(&EvidenceLookupCoveredStageCutoverProof) -> &str =
        EvidenceLookupCoveredStageCutoverProof::selected_lookup_plan_digest;
    let _: fn(&EvidenceLookupCoveredStageCutoverProof) -> &str =
        EvidenceLookupCoveredStageCutoverProof::lookup_execution_receipt_digest;
    let _: fn(&EvidenceLookupCoveredStageCutoverProof) -> &str =
        EvidenceLookupCoveredStageCutoverProof::lookup_product_output_digest;
    let _: fn(&EvidenceLookupCoveredStageCutoverProof) -> &[String] =
        EvidenceLookupCoveredStageCutoverProof::covered_family_identities;
    let _: fn(&EvidenceLookupCoveredStageCutoverProof) -> &EvidenceLookupStageCutoverCounters =
        EvidenceLookupCoveredStageCutoverProof::counters;
    let _: fn(
        &EvidenceLookupCoveredStageCutoverProof,
    ) -> EvidenceLookupCoveredStageCutoverExplanation =
        EvidenceLookupCoveredStageCutoverProof::explain;
}

#[test]
fn spatial_public_api_exports_lookup_stage_cutover_support_types() {
    let _: fn(&EvidenceLookupCoveredStageCutoverExplanation) -> &str =
        EvidenceLookupCoveredStageCutoverExplanation::family_identity;
    let _: fn(&EvidenceLookupCoveredStageCutoverExplanation) -> WorkloadEvidenceStage =
        EvidenceLookupCoveredStageCutoverExplanation::stage;
    let _: fn(&EvidenceLookupStageCutoverCounters) -> usize =
        EvidenceLookupStageCutoverCounters::indexed_lookup_count;
    let _: fn(&EvidenceLookupStageCutoverCounters) -> usize =
        EvidenceLookupStageCutoverCounters::raw_row_scan_count;
    let _: fn(&EvidenceLookupStageCutoverCounters) -> usize =
        EvidenceLookupStageCutoverCounters::broad_receipt_scan_count;
    let _: fn(&EvidenceLookupStageCutoverCounters) -> usize =
        EvidenceLookupStageCutoverCounters::caller_owned_scan_count;
    let _: fn(&EvidenceLookupStageCutoverError) -> EvidenceLookupStageCutoverErrorKind =
        EvidenceLookupStageCutoverError::kind;
    let _: fn(&EvidenceLookupStageCutoverError) -> &str = EvidenceLookupStageCutoverError::detail;
    let _: fn(&EvidenceLookupTopologyDerivedReceiptRef) -> &str =
        EvidenceLookupTopologyDerivedReceiptRef::seed_digest;
    let _: fn(&EvidenceLookupTopologyDerivedReceiptRef) -> &str =
        EvidenceLookupTopologyDerivedReceiptRef::receipt_ref_digest;

    let _ = EvidenceLookupTopologyDerivedReceiptState::NotRequired;
}
