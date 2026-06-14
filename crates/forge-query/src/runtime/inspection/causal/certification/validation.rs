use crate::identity::hash_parts;

use super::super::materialization::QueryCausalInspectionArtifact;
use super::artifacts::{
    CausalInspectionBoundaryAudit, CausalInspectionCertificationBundle,
    CausalInspectionCertificationScope, CausalInspectionScaleCounterSnapshot,
};
use super::error::{CausalInspectionCertificationError, CausalInspectionCertificationErrorKind};

pub fn certify_causal_inspection_runtime_path(
    scope: CausalInspectionCertificationScope,
) -> CausalInspectionCertificationBundle {
    let parts = scope.into_bundle_parts();
    let certification_bundle_digest = hash_parts(&[
        "causal_inspection_certification_bundle_v1".to_string(),
        format!("scope:{}", parts.certification_scope_digest),
        format!("performance:{}", parts.performance_certification_digest),
        format!("readmission:{}", parts.bridge_readmission_proof_digest),
        format!("scale-slope:{}", parts.scale_slope_digest),
        format!("anchor-slope:{}", parts.anchor_derivation_slope_digest),
        format!(
            "reference-slope:{}",
            parts.reference_resolution_slope_digest
        ),
        format!("admission-slope:{}", parts.admission_slope_digest),
        format!("bridge-slope:{}", parts.bridge_envelope_slope_digest),
        format!(
            "materialization-slope:{}",
            parts.materialization_slope_digest
        ),
        format!(
            "serialization:{}",
            parts.artifact_serialization_slope_digest
        ),
        format!("boundary:{}", parts.boundary_audit_digest),
        format!("representatives:{}", parts.representative_matrix_digest),
        format!("proof-shape:{}", parts.proof_shape_digest),
        format!("phase-progression:{}", parts.phase_progression_digest),
        format!("witness-authority:{}", parts.witness_authority_digest),
        format!("row-count:{}", parts.certification_row_count),
        format!("hostile-count:{}", parts.hostile_row_count),
        format!("representative-count:{}", parts.representative_row_count),
        format!("scale-count:{}", parts.scale_fixture_row_count),
    ]);
    CausalInspectionCertificationBundle::from_parts(
        certification_bundle_digest,
        parts.certification_scope_digest,
        parts.performance_certification_digest,
        parts.bridge_readmission_proof_digest,
        parts.scale_slope_digest,
        parts.anchor_derivation_slope_digest,
        parts.reference_resolution_slope_digest,
        parts.admission_slope_digest,
        parts.bridge_envelope_slope_digest,
        parts.materialization_slope_digest,
        parts.artifact_serialization_slope_digest,
        parts.boundary_audit_digest,
        parts.representative_matrix_digest,
        parts.proof_shape_digest,
        parts.phase_progression_digest,
        parts.witness_authority_digest,
        parts.certification_row_count,
        parts.hostile_row_count,
        parts.representative_row_count,
        parts.scale_fixture_row_count,
    )
}

pub(super) fn validate_required_artifact_lanes(
    changed_artifact: &QueryCausalInspectionArtifact,
    denied_artifact: &QueryCausalInspectionArtifact,
) -> Result<(), CausalInspectionCertificationError> {
    if !changed_artifact.is_admitted() || !denied_artifact.is_denied() {
        return Err(CausalInspectionCertificationError::new(
            CausalInspectionCertificationErrorKind::MissingRequiredHostileLane,
            "causal certification requires admitted changed and denied hostile rows",
            &[
                format!("changed-kind:{}", changed_artifact.kind().as_str()),
                format!("denied-kind:{}", denied_artifact.kind().as_str()),
            ],
        ));
    }
    Ok(())
}

pub(super) fn validate_redaction_identity(
    full_artifact: &QueryCausalInspectionArtifact,
    redacted_artifact: &QueryCausalInspectionArtifact,
) -> Result<(), CausalInspectionCertificationError> {
    let same_identity = full_artifact.causal_identity_for_reporting()
        == redacted_artifact.causal_identity_for_reporting();
    let changed_detail =
        full_artifact.artifact_for_reporting() != redacted_artifact.artifact_for_reporting();
    if !same_identity || !changed_detail {
        return Err(CausalInspectionCertificationError::new(
            CausalInspectionCertificationErrorKind::RedactionIdentityDrift,
            "redaction rows must change materialized detail without changing causal identity",
            &[
                format!("same-identity:{same_identity}"),
                format!("changed-detail:{changed_detail}"),
            ],
        ));
    }
    Ok(())
}

pub(super) fn validate_boundary_audit(
    boundary_audit: &CausalInspectionBoundaryAudit,
) -> Result<(), CausalInspectionCertificationError> {
    if !boundary_audit.ordinary_path_uses_query_artifact()
        || !boundary_audit.direct_lower_runtime_stitching_absent()
    {
        return Err(CausalInspectionCertificationError::new(
            CausalInspectionCertificationErrorKind::PublicBoundaryBypass,
            "ordinary explanation path must certify Query artifact consumption",
            &[format!("audit:{}", boundary_audit.audit_digest())],
        ));
    }
    Ok(())
}

pub(super) fn validate_missing_evidence_digest(
    missing_evidence_failure_digest: &str,
) -> Result<(), CausalInspectionCertificationError> {
    if missing_evidence_failure_digest.is_empty() {
        return Err(CausalInspectionCertificationError::new(
            CausalInspectionCertificationErrorKind::MissingRequiredHostileLane,
            "missing-evidence row requires a typed denial failure digest",
            &["missing-evidence:none".to_string()],
        ));
    }
    Ok(())
}

pub(super) fn validate_scale_slope(
    small: &CausalInspectionScaleCounterSnapshot,
    medium: &CausalInspectionScaleCounterSnapshot,
    large: &CausalInspectionScaleCounterSnapshot,
) -> Result<(), CausalInspectionCertificationError> {
    let snapshots = [small, medium, large];
    let stable_slope = snapshots.iter().all(|snapshot| {
        snapshot.anchor_derivation_slope_counter() == 1
            && snapshot.reference_resolution_slope_counter() == 1
            && snapshot.admission_slope_counter() == 1
            && snapshot.bridge_envelope_slope_counter() == 1
            && snapshot.materialization_slope_counter() == 1
            && snapshot.artifact_serialization_slope_counter() == 1
            && snapshot.bridge_unindexed_scan_count() == 0
            && snapshot.bridge_readmission_proof_digest().is_some()
    });
    if !stable_slope {
        return Err(CausalInspectionCertificationError::new(
            CausalInspectionCertificationErrorKind::ScaleSlopeDrift,
            "scale rows must prove fixed inspection materialization slope counters",
            &snapshots
                .iter()
                .map(|snapshot| {
                    format!(
                        "{}:{}",
                        snapshot.fixture_size().as_str(),
                        snapshot.snapshot_digest()
                    )
                })
                .collect::<Vec<_>>(),
        ));
    }
    Ok(())
}
