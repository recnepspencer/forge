use crate::harness::milestone_nine_certification::phase_three_test_narrowed_artifact;
use forge_foundational::facade::{AspectKey, FieldKey};

use super::{
    admit_policy_aware_live_plan, certify_policy_live_drift_evidence, PolicyDriftDisposition,
    PolicyLiveDensityEvidence, PolicyLiveDensityPosture, PolicyLiveEpochEvidence,
};

#[test]
fn live_relevance_uses_authorized_fields_only() {
    let artifact = phase_three_test_narrowed_artifact();
    let fields = authorized_fields([("identity", "id"), ("profile", "display_name")]);
    let live = admit_policy_aware_live_plan(
        &artifact,
        &fields,
        PolicyDriftDisposition::NoChange,
        PolicyLiveDensityPosture::SparseDelta,
    )
    .expect("authorized live relevance should admit");

    assert_eq!(
        native_relevance_fields(live.relevance().authorized_field_paths()),
        vec![
            native_field_pair("identity", "id"),
            native_field_pair("profile", "display_name")
        ]
    );
    assert_eq!(
        live.core().seam().counters().live_relevance_field_width(),
        2
    );
    assert_eq!(
        live.report().drift_disposition(),
        PolicyDriftDisposition::NoChange
    );
}

fn native_relevance_fields(
    fields: &[crate::authorized_projection::AuthorizedProjectionFieldPath],
) -> Vec<(AspectKey, FieldKey)> {
    fields
        .iter()
        .map(|field| {
            (
                field.native_aspect_key().clone(),
                field.native_field_key().clone(),
            )
        })
        .collect()
}

fn native_field_pair(aspect: &str, field: &str) -> (AspectKey, FieldKey) {
    (
        AspectKey::new(aspect).expect("test aspect key should admit"),
        FieldKey::new(field).expect("test field key should admit"),
    )
}

fn authorized_fields(
    fields: impl IntoIterator<Item = (&'static str, &'static str)>,
) -> Vec<crate::authorized_projection::AuthorizedProjectionFieldPath> {
    fields
        .into_iter()
        .map(|(aspect, field)| {
            crate::authorized_projection::AuthorizedProjectionFieldPath::from_native_keys(
                AspectKey::new(aspect).expect("test aspect key"),
                FieldKey::new(field).expect("test field key"),
            )
        })
        .collect()
}

#[test]
fn masked_live_relevance_denies_before_admission() {
    let artifact = phase_three_test_narrowed_artifact();
    let fields = authorized_fields([("secret", "salary")]);
    let error = admit_policy_aware_live_plan(
        &artifact,
        &fields,
        PolicyDriftDisposition::NoChange,
        PolicyLiveDensityPosture::SparseDelta,
    )
    .expect_err("masked live relevance must deny");

    assert_eq!(error.counters().raw_live_relevance_denial_count(), 1);
}

#[test]
fn live_drift_evidence_must_match_admitted_plan() {
    let artifact = phase_three_test_narrowed_artifact();
    let fields = authorized_fields([("identity", "id"), ("profile", "display_name")]);
    let live = admit_policy_aware_live_plan(
        &artifact,
        &fields,
        PolicyDriftDisposition::NoChange,
        PolicyLiveDensityPosture::SparseDelta,
    )
    .expect("authorized live relevance should admit");
    let epoch = PolicyLiveEpochEvidence::new(
        artifact.policy_digest(),
        artifact.tenant_truth_basis_digest(),
        artifact.policy_digest(),
        artifact.tenant_truth_basis_digest(),
    );
    let density = PolicyLiveDensityEvidence::new(fields.len(), 1, 1);
    let report = certify_policy_live_drift_evidence(&live, epoch, density)
        .expect("matching drift and density evidence should certify");

    assert_eq!(
        report.epoch_evidence().disposition(),
        PolicyDriftDisposition::NoChange
    );
    assert_eq!(
        report.density_evidence().posture(),
        PolicyLiveDensityPosture::SparseDelta
    );
    assert!(!report.digest().is_empty());
}

#[test]
fn live_epoch_readmission_counter_is_exact_in_certified_evidence() {
    let artifact = phase_three_test_narrowed_artifact();
    let fields = authorized_fields([("identity", "id"), ("profile", "display_name")]);
    let live = admit_policy_aware_live_plan(
        &artifact,
        &fields,
        PolicyDriftDisposition::FreshAdmissionFromCheckpoint,
        PolicyLiveDensityPosture::SparseDelta,
    )
    .expect("fresh admission should keep readmission visible in counters");
    let epoch = PolicyLiveEpochEvidence::new(
        "previous-policy-digest",
        artifact.tenant_truth_basis_digest(),
        artifact.policy_digest(),
        artifact.tenant_truth_basis_digest(),
    );
    let density = PolicyLiveDensityEvidence::new(fields.len(), 1, 1);
    let report = certify_policy_live_drift_evidence(&live, epoch, density)
        .expect("policy-only drift should certify as fresh admission");

    assert_eq!(report.counters().policy_epoch_drift_readmission_count(), 1);
    assert_eq!(report.counters().tenant_basis_drift_readmission_count(), 0);
    assert_eq!(
        report.counters().policy_sparse_to_burst_readmission_count(),
        0
    );
}

#[test]
fn live_drift_evidence_current_basis_must_match_admitted_plan() {
    let artifact = phase_three_test_narrowed_artifact();
    let fields = authorized_fields([("identity", "id"), ("profile", "display_name")]);
    let live = admit_policy_aware_live_plan(
        &artifact,
        &fields,
        PolicyDriftDisposition::FreshAdmissionFromCheckpoint,
        PolicyLiveDensityPosture::SparseDelta,
    )
    .expect("fresh admission should admit with explicit evidence");
    let forged_current_policy = PolicyLiveEpochEvidence::new(
        "previous-policy-digest",
        artifact.tenant_truth_basis_digest(),
        "different-current-policy",
        artifact.tenant_truth_basis_digest(),
    );
    let density = PolicyLiveDensityEvidence::new(fields.len(), 1, 1);
    let error = certify_policy_live_drift_evidence(&live, forged_current_policy, density)
        .expect_err("current drift evidence cannot point at a different policy basis");

    assert_eq!(error.counters().raw_live_relevance_denial_count(), 1);
}

#[test]
fn live_tenant_readmission_counter_is_exact_in_certified_evidence() {
    let artifact = phase_three_test_narrowed_artifact();
    let fields = authorized_fields([("identity", "id"), ("profile", "display_name")]);
    let live = admit_policy_aware_live_plan(
        &artifact,
        &fields,
        PolicyDriftDisposition::FreshAdmissionFromCheckpoint,
        PolicyLiveDensityPosture::SparseDelta,
    )
    .expect("fresh admission should admit with explicit evidence");
    let epoch = PolicyLiveEpochEvidence::new(
        artifact.policy_digest(),
        "previous-tenant-truth-digest",
        artifact.policy_digest(),
        artifact.tenant_truth_basis_digest(),
    );
    let density = PolicyLiveDensityEvidence::new(fields.len(), 1, 1);
    let report = certify_policy_live_drift_evidence(&live, epoch, density)
        .expect("tenant-only drift should certify as fresh admission");

    assert_eq!(report.counters().policy_epoch_drift_readmission_count(), 0);
    assert_eq!(report.counters().tenant_basis_drift_readmission_count(), 1);
}

#[test]
fn live_burst_readmission_is_an_exact_seam_counter() {
    let artifact = phase_three_test_narrowed_artifact();
    let fields = authorized_fields([("identity", "id"), ("profile", "display_name")]);
    let live = admit_policy_aware_live_plan(
        &artifact,
        &fields,
        PolicyDriftDisposition::NoChange,
        PolicyLiveDensityPosture::BurstReadmission,
    )
    .expect("burst readmission should admit but remain visible in counters");

    assert_eq!(
        live.core()
            .seam()
            .counters()
            .policy_sparse_to_burst_readmission_count(),
        1
    );
    assert_eq!(
        live.core()
            .seam()
            .counters()
            .policy_dense_restart_debt_count(),
        0
    );
}

#[test]
fn live_dense_restart_debt_is_an_exact_denial_counter() {
    let artifact = phase_three_test_narrowed_artifact();
    let fields = authorized_fields([("identity", "id"), ("profile", "display_name")]);
    let error = admit_policy_aware_live_plan(
        &artifact,
        &fields,
        PolicyDriftDisposition::NoChange,
        PolicyLiveDensityPosture::DenseRestartDebt,
    )
    .expect_err("dense restart debt remains an explicit denial");

    assert_eq!(error.counters().policy_dense_restart_debt_count(), 1);
    assert_eq!(error.counters().raw_live_relevance_denial_count(), 1);
}

#[test]
fn live_density_evidence_rejects_unadmitted_dense_restart() {
    let artifact = phase_three_test_narrowed_artifact();
    let fields = authorized_fields([("identity", "id"), ("profile", "display_name")]);
    let live = admit_policy_aware_live_plan(
        &artifact,
        &fields,
        PolicyDriftDisposition::NoChange,
        PolicyLiveDensityPosture::SparseDelta,
    )
    .expect("authorized live relevance should admit");
    let epoch = PolicyLiveEpochEvidence::new(
        artifact.policy_digest(),
        artifact.tenant_truth_basis_digest(),
        artifact.policy_digest(),
        artifact.tenant_truth_basis_digest(),
    );
    let dense = PolicyLiveDensityEvidence::new(fields.len(), fields.len() + 1, 1);
    let error = certify_policy_live_drift_evidence(&live, epoch, dense)
        .expect_err("dense evidence cannot be attached to a sparse admitted plan");

    assert_eq!(error.counters().raw_live_relevance_denial_count(), 1);
}

#[test]
fn live_density_evidence_width_must_match_admitted_relevance_width() {
    let artifact = phase_three_test_narrowed_artifact();
    let fields = authorized_fields([("identity", "id"), ("profile", "display_name")]);
    let live = admit_policy_aware_live_plan(
        &artifact,
        &fields,
        PolicyDriftDisposition::NoChange,
        PolicyLiveDensityPosture::SparseDelta,
    )
    .expect("authorized live relevance should admit");
    let epoch = PolicyLiveEpochEvidence::new(
        artifact.policy_digest(),
        artifact.tenant_truth_basis_digest(),
        artifact.policy_digest(),
        artifact.tenant_truth_basis_digest(),
    );
    let mismatched_width = PolicyLiveDensityEvidence::new(fields.len() + 1, 1, 1);
    let error = certify_policy_live_drift_evidence(&live, epoch, mismatched_width)
        .expect_err("density evidence must bind to admitted relevance width");

    assert_eq!(error.counters().raw_live_relevance_denial_count(), 1);
}
