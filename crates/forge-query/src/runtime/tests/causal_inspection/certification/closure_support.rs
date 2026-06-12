use forge_runtime_bridge::facade::TruthCommitIdentity;

use super::artifact_support::{
    admitted_artifact, advisory_artifacts, denied_artifact_and_missing_evidence,
};
use super::matrix_support::representative_matrix;
use super::*;

pub(in crate::runtime::tests) fn runtime_backed_causal_certification_bundle(
) -> CausalInspectionCertificationBundle {
    let changed = admitted_artifact(TruthCommitIdentity::from_bridge_harness_label(
        "commit-query-cert-changed",
    ));
    let (full, redacted) = advisory_artifacts(TruthCommitIdentity::from_bridge_harness_label(
        "commit-query-cert-redacted",
    ));
    let (denied, missing_evidence_digest) = denied_artifact_and_missing_evidence();
    let representatives = representative_matrix(&changed, &redacted, &denied);
    let boundary_audit =
        CausalInspectionBoundaryAudit::from_query_artifact_public_surface(&changed);
    let proof_shape = CausalInspectionProofShapeCertification::from_runtime_path(
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
    let scope = build_causal_inspection_certification_scope(
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
    .expect("complete runtime-backed rows should build certification scope");

    certify_causal_inspection_runtime_path(scope)
}
