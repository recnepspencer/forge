use crate::harness::certification::HostileExpectation;
use crate::harness::certification::ParityAnchor;
use crate::harness::milestone_nine_certification::bundles::MilestoneNineCertificationRow;
use crate::harness::milestone_nine_certification::classifications::MilestoneNinePerturbationClass;
use crate::harness::milestone_nine_certification::fixtures::native_authorized_projection_fields;
use crate::harness::milestone_nine_certification::fixtures::phase_four_bundle;
use crate::harness::milestone_nine_certification::fixtures::phase_three_test_narrowed_artifact;
use crate::policy_certification::{
    employee_record_policy_fixture, employee_record_policy_scale_report,
    policy_composition_parity_report, policy_view_shape_parity_report,
    EmployeeRecordCertificationBundle, EmployeeRecordPolicyScenario, EmployeeRecordQueryFamily,
    EmployeeRecordTenantVariant,
};
use crate::policy_delivery::{lower_policy_aware_delivery_shape, DeliveryWidthClass};
use crate::policy_live::{
    admit_policy_aware_live_plan, certify_policy_live_drift_evidence, PolicyDriftDisposition,
    PolicyLiveDensityEvidence, PolicyLiveDensityPosture, PolicyLiveDriftEvidenceReport,
    PolicyLiveEpochEvidence,
};
use crate::policy_narrowing::NarrowedPolicyQueryArtifact;

fn employee_bundle(tenant: EmployeeRecordTenantVariant) -> EmployeeRecordCertificationBundle {
    employee_record_policy_fixture().certify(EmployeeRecordPolicyScenario::new(
        tenant,
        EmployeeRecordQueryFamily::DirectDetail,
    ))
}

fn scale_digest() -> String {
    employee_record_policy_scale_report()
        .digest()
        .as_str()
        .to_string()
}

fn delivery_width_digest(artifact: &NarrowedPolicyQueryArtifact) -> String {
    let scalar =
        lower_policy_aware_delivery_shape(artifact, DeliveryWidthClass::ScalarDetail).unwrap();
    let narrow =
        lower_policy_aware_delivery_shape(artifact, DeliveryWidthClass::NarrowCollection).unwrap();
    let grouped =
        lower_policy_aware_delivery_shape(artifact, DeliveryWidthClass::GroupedDelta).unwrap();
    let diff = lower_policy_aware_delivery_shape(artifact, DeliveryWidthClass::DiffDelta).unwrap();
    crate::harness::certification::digest_parts(&[
        format!("scalar:{}", scalar.report().digest()),
        format!("narrow:{}", narrow.report().digest()),
        format!("grouped:{}", grouped.report().digest()),
        format!("diff:{}", diff.report().digest()),
    ])
}

fn live_density_evidence(artifact: &NarrowedPolicyQueryArtifact) -> PolicyLiveDriftEvidenceReport {
    let plan = admit_policy_aware_live_plan(
        artifact,
        &native_authorized_projection_fields(artifact),
        PolicyDriftDisposition::NoChange,
        PolicyLiveDensityPosture::BurstReadmission,
    )
    .unwrap();
    certify_policy_live_drift_evidence(
        &plan,
        PolicyLiveEpochEvidence::new(
            artifact.policy_digest(),
            artifact.tenant_truth_basis_digest(),
            artifact.policy_digest(),
            artifact.tenant_truth_basis_digest(),
        ),
        PolicyLiveDensityEvidence::new(
            artifact.authorized_projection().visible_field_paths().len(),
            artifact.authorized_projection().visible_field_paths().len(),
            1,
        ),
    )
    .unwrap()
}

fn live_drift_evidence(artifact: &NarrowedPolicyQueryArtifact) -> PolicyLiveDriftEvidenceReport {
    let plan = admit_policy_aware_live_plan(
        artifact,
        &native_authorized_projection_fields(artifact),
        PolicyDriftDisposition::FreshAdmissionFromCheckpoint,
        PolicyLiveDensityPosture::SparseDelta,
    )
    .unwrap();
    certify_policy_live_drift_evidence(
        &plan,
        PolicyLiveEpochEvidence::new(
            "previous-policy-digest",
            artifact.tenant_truth_basis_digest(),
            artifact.policy_digest(),
            artifact.tenant_truth_basis_digest(),
        ),
        PolicyLiveDensityEvidence::new(
            artifact.authorized_projection().visible_field_paths().len(),
            1,
            1,
        ),
    )
    .unwrap()
}

fn composition_digest(artifact: &NarrowedPolicyQueryArtifact) -> String {
    policy_composition_parity_report(artifact.digest())
        .parity_digest()
        .to_string()
}

fn view_shape_digest(artifact: &NarrowedPolicyQueryArtifact) -> String {
    let scalar =
        lower_policy_aware_delivery_shape(artifact, DeliveryWidthClass::ScalarDetail).unwrap();
    let grouped =
        lower_policy_aware_delivery_shape(artifact, DeliveryWidthClass::GroupedDelta).unwrap();
    policy_view_shape_parity_report(
        scalar.digest().as_str(),
        grouped.digest().as_str(),
        "identity-aware-inspector-delivery-preserves-classification",
    )
    .parity_digest()
    .to_string()
}

pub(super) fn live_policy_rows() -> Vec<MilestoneNineCertificationRow> {
    let artifact = phase_three_test_narrowed_artifact();
    let employee_alpha = employee_bundle(EmployeeRecordTenantVariant::TenantAlpha);
    let scale_digest = scale_digest();
    let live_drift = live_drift_evidence(&artifact);
    let live_density = live_density_evidence(&artifact);
    let delivery_digest = delivery_width_digest(&artifact);
    let composition_digest = composition_digest(&artifact);
    let view_shape = view_shape_digest(&artifact);
    let phase_four_live_drift = phase_four_bundle(
        "live-policy-epoch-drift-readmission",
        employee_alpha.employee_fixture_digest(),
        scale_digest.as_str(),
        live_drift.digest(),
        delivery_digest.clone(),
        composition_digest.as_str(),
        view_shape.as_str(),
        &live_drift.counters().digest_parts(),
    );
    let phase_four_live_density = phase_four_bundle(
        "live-policy-density-posture-honesty",
        employee_alpha.employee_fixture_digest(),
        scale_digest.as_str(),
        live_density.digest(),
        delivery_digest.clone(),
        composition_digest.as_str(),
        view_shape.as_str(),
        &live_density.counters().digest_parts(),
    );
    let phase_four_scale = phase_four_bundle(
        "policy-scale-slope",
        employee_alpha.employee_fixture_digest(),
        scale_digest.as_str(),
        live_density.digest(),
        delivery_digest,
        composition_digest.as_str(),
        view_shape.as_str(),
        &[],
    );
    vec![
        MilestoneNineCertificationRow {
            row_name: "live-policy-epoch-drift-readmission",
            perturbation_class: MilestoneNinePerturbationClass::LivePolicyEpochDriftReadmission,
            hostile_expectation: HostileExpectation::DistinctFromControl,
            parity_anchor: ParityAnchor::Hostile,
            control_lane: phase_four_live_density.clone(),
            hostile_lane: phase_four_live_drift.clone(),
            parity_lane: phase_four_live_drift,
        },
        MilestoneNineCertificationRow {
            row_name: "live-policy-density-posture-honesty",
            perturbation_class: MilestoneNinePerturbationClass::LivePolicyDensityPostureHonesty,
            hostile_expectation: HostileExpectation::EquivalentToControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: phase_four_live_density.clone(),
            hostile_lane: phase_four_live_density.clone(),
            parity_lane: phase_four_live_density,
        },
        MilestoneNineCertificationRow {
            row_name: "policy-scale-slope-honesty",
            perturbation_class: MilestoneNinePerturbationClass::PolicyScaleSlopeHonesty,
            hostile_expectation: HostileExpectation::EquivalentToControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: phase_four_scale.clone(),
            hostile_lane: phase_four_scale.clone(),
            parity_lane: phase_four_scale,
        },
    ]
}
