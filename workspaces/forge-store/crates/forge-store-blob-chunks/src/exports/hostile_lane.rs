// Hostile-lane constructors: typed rejections for forgeable authority paths.
// Grouped by lifecycle stage; not mixed with capability admission exports.

// --- Identity ---
pub use crate::chunk_identity::{
    reject_application_org_claim_as_blob_chunk_security_scope,
    reject_deserialized_metadata_as_blob_chunk_security_scope,
    reject_iam_role_as_blob_chunk_security_scope, reject_jwt_claim_as_blob_chunk_security_scope,
    reject_kms_key_id_as_blob_chunk_security_scope,
    reject_operator_identity_as_blob_chunk_security_scope,
};
// --- Integrity ---
pub use crate::chunk_integrity::{
    reject_checksum_only_evidence_as_blob_chunk_integrity,
    reject_checksum_only_evidence_as_chunk_root_publication,
    reject_digest_only_evidence_as_blob_chunk_integrity,
    reject_digest_only_evidence_as_chunk_root_publication,
};
// --- Import readmission ---
pub fn reject_copied_export_row_as_blob_import(raw: &str) -> crate::BlobImportReadmissionDenial {
    crate::import_readmission::reject_copied_export_row_as_blob_import(raw)
}

pub fn reject_placement_only_evidence_as_imported_blob_witness(
    placement: &crate::AdmittedBlobPlacement,
) -> crate::BlobImportReadmissionDenial {
    crate::import_readmission::reject_placement_only_evidence_as_imported_blob_witness(placement)
}
// --- Capsule readiness ---
pub fn reject_copied_capsule_row_as_capsule_readiness(
    raw: &str,
) -> crate::BlobCapsuleReadinessDenial {
    crate::capsule_readiness::reject_copied_capsule_row_as_capsule_readiness(raw)
}

pub fn reject_digest_only_chunk_reference_as_capsule_readiness(
    raw: &str,
) -> crate::BlobCapsuleReadinessDenial {
    crate::capsule_readiness::reject_digest_only_chunk_reference_as_capsule_readiness(raw)
}
// --- Lifecycle ---
pub use crate::lifecycle::{
    reject_chunk_tree_equality_as_blob_identity, reject_copied_counters_as_lifecycle_receipt,
    reject_copied_digest_string_as_lifecycle_receipt,
    reject_copied_lifecycle_receipt_as_blob_identity, reject_digest_equality_as_blob_identity,
    reject_imported_manifest_text_as_lifecycle_receipt,
    reject_physical_generation_as_blob_generation, reject_raw_generation_number_as_blob_identity,
    reject_physical_integrity_report_as_lifecycle_receipt, reject_io_qos_placement_seed_as_lifecycle_receipt,
    reject_semantic_reference_id_as_blob_identity, reject_terminal_projection_row_as_blob_identity,
    reject_terminal_projection_row_as_lifecycle_receipt,
};
// --- Publication ---
pub use crate::publication::{
    reject_copied_publication_record_as_blob_visibility, reject_root_candidate_as_blob_visibility,
    reject_semantic_reference_as_blob_visibility, reject_staged_reachability_as_blob_visibility,
};
// --- Reachability ---
pub use crate::reachability::{
    reject_backend_residue_as_blob_reachability, reject_copied_refcount_row_as_reachability,
    reject_empty_reference_proof_as_reachability, reject_terminal_projection_as_blob_reachability,
};
// --- Corruption ---
pub use crate::corruption::{
    classify_and_reject_physical_handoff, observe_physical_pre_decode_denial,
    reject_chunk_integrity_report_as_blob_corruption_authority,
    reject_copied_counters_as_blob_corruption_authority,
    reject_offline_observation_as_blob_corruption_authority,
    reject_physical_handoff_as_blob_authority,
    reject_physical_quarantine_record_as_blob_corruption_authority,
    reject_raw_digest_as_blob_corruption_authority,
};
pub use crate::handoffs::reject_physical_handoff_from_pre_decode_denial;
// --- Retention / reclaim ---
pub use crate::retention_reclaim::{
    reject_backend_residue_as_retention_reclaim_authority,
    reject_copied_counter_as_retention_reclaim_authority,
    reject_copied_receipt_as_retention_reclaim_authority,
    reject_reclaim_policy_evidence_as_retention_reclaim_authority,
    reject_terminal_projection_as_retention_reclaim_authority,
};
// --- Streaming ---
pub use crate::streaming::{
    reject_allocation_denial_as_streaming_ingest, reject_full_blob_vec_as_streaming_ingest,
    reject_full_blob_vec_as_streaming_read, reject_scalar_backend_api_as_streaming_ingest,
};
pub use crate::{
    reject_chunk_tree_root_as_blob_object_layout_authority,
    reject_full_blob_buffer_as_streaming_layout_authority,
    reject_streaming_frontier_as_chunk_tree_layout_authority,
};
