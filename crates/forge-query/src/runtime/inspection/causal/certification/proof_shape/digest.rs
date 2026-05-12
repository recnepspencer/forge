use crate::identity::hash_parts;

#[cfg(test)]
use super::CausalInspectionProofShapeCertification;

pub(super) fn derive_phase_progression_digest(
    inspected_artifact_digest: &str,
    representative_matrix_digest: &str,
    boundary_audit_digest: &str,
) -> String {
    hash_parts(&[
        "causal_inspection_phase_progression_v1".to_string(),
        "phase:artifact-materialized".to_string(),
        "phase:representative-matrix-certified".to_string(),
        "phase:boundary-audit-certified".to_string(),
        format!("artifact:{inspected_artifact_digest}"),
        format!("matrix:{representative_matrix_digest}"),
        format!("boundary:{boundary_audit_digest}"),
    ])
}

pub(super) fn derive_witness_authority_digest(
    inspected_artifact_digest: &str,
    representative_matrix_digest: &str,
    boundary_audit_digest: &str,
) -> String {
    hash_parts(&[
        "causal_inspection_witness_authority_v1".to_string(),
        "artifact-authority:query-causal-inspection-artifact".to_string(),
        "matrix-authority:causal-inspection-representative-matrix".to_string(),
        "boundary-authority:causal-inspection-boundary-audit".to_string(),
        format!("artifact:{inspected_artifact_digest}"),
        format!("matrix:{representative_matrix_digest}"),
        format!("boundary:{boundary_audit_digest}"),
    ])
}

pub(super) fn canonical_proof_shape_digest(
    inspected_artifact_digest: &str,
    representative_matrix_digest: &str,
    boundary_audit_digest: &str,
    phase_progression_digest: &str,
    witness_authority_digest: &str,
) -> String {
    hash_parts(&[
        "causal_inspection_proof_shape_certification_v1".to_string(),
        "phase-skipping-rejected:true".to_string(),
        "raw-collection-substitution-rejected:true".to_string(),
        "stale-proof-reuse-rejected:true".to_string(),
        "forged-authority-witness-rejected:true".to_string(),
        format!("artifact:{inspected_artifact_digest}"),
        format!("matrix:{representative_matrix_digest}"),
        format!("boundary:{boundary_audit_digest}"),
        format!("phase-progression:{phase_progression_digest}"),
        format!("witness-authority:{witness_authority_digest}"),
    ])
}

#[cfg(test)]
pub(super) fn stale_test_proof_shape_digest(inspected_artifact_digest: &str) -> String {
    hash_parts(&[
        "causal_inspection_proof_shape_certification_v1".to_string(),
        "stale-test-digest:true".to_string(),
        format!("artifact:{inspected_artifact_digest}"),
    ])
}

#[cfg(test)]
pub(super) fn forged_test_proof_shape_digest(
    proof: &CausalInspectionProofShapeCertification,
) -> String {
    hash_parts(&[
        "causal_inspection_proof_shape_certification_v1".to_string(),
        "forged-test:false".to_string(),
        format!("artifact:{}", proof.inspected_artifact_digest()),
        format!("matrix:{}", proof.representative_matrix_digest()),
        format!("boundary:{}", proof.boundary_audit_digest()),
        format!("phase:{}", proof.phase_progression_digest()),
        format!("witness:{}", proof.witness_authority_digest()),
    ])
}
