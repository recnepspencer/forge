use forge_runtime_bridge::facade::TruthCommitIdentity;

use super::super::super::super::*;
use super::artifact_support::{
    admitted_artifact, advisory_artifacts, denied_artifact_and_missing_evidence,
};
use super::matrix_support::representative_matrix;

#[test]
fn causal_inspection_proof_shape_binds_runtime_path_inputs() {
    let changed = admitted_artifact(TruthCommitIdentity::new("commit-query-proof-shape-changed"));
    let (_, redacted) = advisory_artifacts(TruthCommitIdentity::new(
        "commit-query-proof-shape-redacted",
    ));
    let (denied, _) = denied_artifact_and_missing_evidence();
    let representatives = representative_matrix(&changed, &redacted, &denied);
    let boundary_audit =
        CausalInspectionBoundaryAudit::from_query_artifact_public_surface(&changed);

    let proof_shape = CausalInspectionProofShapeCertification::from_runtime_path(
        &changed,
        &representatives,
        &boundary_audit,
    );

    assert!(proof_shape.phase_skipping_rejected());
    assert!(proof_shape.raw_collection_substitution_rejected());
    assert!(proof_shape.stale_proof_reuse_rejected());
    assert!(proof_shape.forged_authority_witness_rejected());
    assert_eq!(
        proof_shape.inspected_artifact_digest(),
        changed.artifact_digest()
    );
    assert_eq!(
        proof_shape.representative_matrix_digest(),
        representatives.matrix_digest()
    );
    assert_eq!(
        proof_shape.boundary_audit_digest(),
        boundary_audit.audit_digest()
    );

    let (_, alternate_redacted) = advisory_artifacts(TruthCommitIdentity::new(
        "commit-query-proof-shape-alternate-redacted",
    ));
    let alternate_representatives = representative_matrix(&changed, &alternate_redacted, &denied);
    let alternate_matrix_proof = CausalInspectionProofShapeCertification::from_runtime_path(
        &changed,
        &alternate_representatives,
        &boundary_audit,
    );
    assert_ne!(
        proof_shape.representative_matrix_digest(),
        alternate_matrix_proof.representative_matrix_digest()
    );
    assert_ne!(
        proof_shape.phase_progression_digest(),
        alternate_matrix_proof.phase_progression_digest()
    );
    assert_ne!(
        proof_shape.witness_authority_digest(),
        alternate_matrix_proof.witness_authority_digest()
    );
    assert_ne!(
        proof_shape.proof_shape_digest(),
        alternate_matrix_proof.proof_shape_digest()
    );

    let alternate_changed = admitted_artifact(TruthCommitIdentity::new(
        "commit-query-proof-shape-alternate-changed",
    ));
    let alternate_boundary =
        CausalInspectionBoundaryAudit::from_query_artifact_public_surface(&alternate_changed);
    let alternate_artifact_proof = CausalInspectionProofShapeCertification::from_runtime_path(
        &alternate_changed,
        &representatives,
        &alternate_boundary,
    );
    assert_ne!(
        proof_shape.inspected_artifact_digest(),
        alternate_artifact_proof.inspected_artifact_digest()
    );
    assert_ne!(
        proof_shape.boundary_audit_digest(),
        alternate_artifact_proof.boundary_audit_digest()
    );
    assert_ne!(
        proof_shape.proof_shape_digest(),
        alternate_artifact_proof.proof_shape_digest()
    );
}

#[test]
fn causal_inspection_certification_rejects_forged_proof_shape() {
    let changed = admitted_artifact(TruthCommitIdentity::new(
        "commit-query-proof-shape-forged-changed",
    ));
    let (full, redacted) = advisory_artifacts(TruthCommitIdentity::new(
        "commit-query-proof-shape-forged-redacted",
    ));
    let (denied, missing_evidence_digest) = denied_artifact_and_missing_evidence();
    let representatives = representative_matrix(&changed, &redacted, &denied);
    let boundary_audit =
        CausalInspectionBoundaryAudit::from_query_artifact_public_surface(&changed);
    let proof_shape = CausalInspectionProofShapeCertification::forged_for_tests(
        &changed,
        &representatives,
        &boundary_audit,
    );
    let small = CausalInspectionScaleCounterSnapshot::from_artifact(
        CausalInspectionScaleFixtureSize::Small,
        &changed,
    );
    let medium = CausalInspectionScaleCounterSnapshot::from_artifact(
        CausalInspectionScaleFixtureSize::Medium,
        &changed,
    );
    let large = CausalInspectionScaleCounterSnapshot::from_artifact(
        CausalInspectionScaleFixtureSize::Large,
        &changed,
    );

    let error = build_causal_inspection_certification_scope(
        &changed,
        &full,
        &redacted,
        &denied,
        &missing_evidence_digest,
        boundary_audit,
        representatives,
        proof_shape,
        small,
        medium,
        large,
    )
    .unwrap_err();

    assert_eq!(
        error.kind(),
        CausalInspectionCertificationErrorKind::ProofShapeBypass
    );
}

#[test]
fn causal_inspection_certification_rejects_stale_proof_shape_digest() {
    let changed = admitted_artifact(TruthCommitIdentity::new(
        "commit-query-proof-shape-stale-changed",
    ));
    let (full, redacted) = advisory_artifacts(TruthCommitIdentity::new(
        "commit-query-proof-shape-stale-redacted",
    ));
    let (denied, missing_evidence_digest) = denied_artifact_and_missing_evidence();
    let representatives = representative_matrix(&changed, &redacted, &denied);
    let boundary_audit =
        CausalInspectionBoundaryAudit::from_query_artifact_public_surface(&changed);
    let proof_shape = CausalInspectionProofShapeCertification::stale_digest_for_tests(
        &changed,
        &representatives,
        &boundary_audit,
    );
    let small = CausalInspectionScaleCounterSnapshot::from_artifact(
        CausalInspectionScaleFixtureSize::Small,
        &changed,
    );
    let medium = CausalInspectionScaleCounterSnapshot::from_artifact(
        CausalInspectionScaleFixtureSize::Medium,
        &changed,
    );
    let large = CausalInspectionScaleCounterSnapshot::from_artifact(
        CausalInspectionScaleFixtureSize::Large,
        &changed,
    );

    let error = build_causal_inspection_certification_scope(
        &changed,
        &full,
        &redacted,
        &denied,
        &missing_evidence_digest,
        boundary_audit,
        representatives,
        proof_shape,
        small,
        medium,
        large,
    )
    .unwrap_err();

    assert_eq!(
        error.kind(),
        CausalInspectionCertificationErrorKind::ProofShapeBypass
    );
}
