use crate::authorized_projection::PolicyAspectMask;
use crate::harness::certification::digest_parts;
use crate::harness::certification::HostileExpectation;
use crate::harness::certification::ParityAnchor;
use crate::harness::milestone_nine_certification::bundles::MilestoneNineCertificationRow;
use crate::harness::milestone_nine_certification::classifications::MilestoneNinePerturbationClass;
use crate::harness::milestone_nine_certification::fixtures::canonical_query_with_secret_projection;
use crate::harness::milestone_nine_certification::fixtures::native_authorized_projection_fields;
use crate::harness::milestone_nine_certification::fixtures::phase_four_bundle;
use crate::harness::milestone_nine_certification::fixtures::phase_three_test_narrowed_artifact;
use crate::harness::milestone_nine_certification::fixtures::phase_two_bundle;
use crate::harness::milestone_nine_certification::fixtures::secret_salary_key;
use crate::policy_certification::{
    employee_record_policy_fixture, employee_record_policy_scale_report,
    policy_composition_parity_report, policy_identity_aware_inspector_parity_report,
    policy_view_shape_parity_report, EmployeeRecordCertificationBundle,
    EmployeeRecordPolicyScenario, EmployeeRecordQueryFamily, EmployeeRecordTenantVariant,
};
use crate::policy_delivery::{lower_policy_aware_delivery_shape, DeliveryWidthClass};
use crate::policy_live::{
    admit_policy_aware_live_plan, certify_policy_live_drift_evidence, PolicyDriftDisposition,
    PolicyLiveDensityEvidence, PolicyLiveDensityPosture, PolicyLiveDriftEvidenceReport,
    PolicyLiveEpochEvidence,
};
use crate::policy_narrowing::NarrowedPolicyQueryArtifact;
use crate::relationship_proof::RelationshipProofDescriptorSet;

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
    digest_parts(&[
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

fn identity_digest() -> String {
    let canonical = canonical_query_with_secret_projection();
    let no_proof = phase_two_bundle(
        canonical,
        PolicyAspectMask::allow_all().with_masked(secret_salary_key()),
        RelationshipProofDescriptorSet::none(),
    );
    policy_identity_aware_inspector_parity_report(
        "milestone-seven-identity-classification-preserved",
        "identity-aware-inspector-delivery-preserves-classification",
        no_proof.narrowed_result_shape_digest,
    )
    .parity_digest()
    .to_string()
}

pub(super) fn composition_rows() -> Vec<MilestoneNineCertificationRow> {
    let artifact = phase_three_test_narrowed_artifact();
    let employee_alpha = employee_bundle(EmployeeRecordTenantVariant::TenantAlpha);
    let scale_digest = scale_digest();
    let density = live_density_evidence(&artifact);
    let delivery_digest = delivery_width_digest(&artifact);
    let composition_digest = composition_digest(&artifact);
    let view_shape = view_shape_digest(&artifact);
    let identity_inspector_parity = identity_digest();
    let phase_four_composition = phase_four_bundle(
        "policy-composition-parity",
        employee_alpha.employee_fixture_digest(),
        scale_digest.as_str(),
        density.digest(),
        delivery_digest.clone(),
        composition_digest.as_str(),
        view_shape.as_str(),
        &[],
    );
    let phase_four_view_shape = phase_four_bundle(
        "policy-view-shape-parity",
        employee_alpha.employee_fixture_digest(),
        scale_digest.as_str(),
        density.digest(),
        delivery_digest.clone(),
        composition_digest.as_str(),
        view_shape.as_str(),
        &[],
    );
    let phase_four_identity_inspector = phase_four_bundle(
        "policy-identity-aware-inspector-parity",
        employee_alpha.employee_fixture_digest(),
        scale_digest.as_str(),
        density.digest(),
        delivery_digest,
        composition_digest.as_str(),
        identity_inspector_parity.as_str(),
        &[],
    );
    vec![
        MilestoneNineCertificationRow {
            row_name: "policy-direct-scope-template-saved-parity",
            perturbation_class:
                MilestoneNinePerturbationClass::PolicyDirectScopeTemplateSavedParity,
            hostile_expectation: HostileExpectation::EquivalentToControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: phase_four_composition.clone(),
            hostile_lane: phase_four_composition.clone(),
            parity_lane: phase_four_composition,
        },
        MilestoneNineCertificationRow {
            row_name: "policy-view-shape-delivery-parity",
            perturbation_class: MilestoneNinePerturbationClass::PolicyViewShapeDeliveryParity,
            hostile_expectation: HostileExpectation::EquivalentToControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: phase_four_view_shape.clone(),
            hostile_lane: phase_four_view_shape.clone(),
            parity_lane: phase_four_view_shape,
        },
        MilestoneNineCertificationRow {
            row_name: "policy-identity-aware-inspector-parity",
            perturbation_class: MilestoneNinePerturbationClass::PolicyIdentityAwareInspectorParity,
            hostile_expectation: HostileExpectation::EquivalentToControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: phase_four_identity_inspector.clone(),
            hostile_lane: phase_four_identity_inspector.clone(),
            parity_lane: phase_four_identity_inspector,
        },
    ]
}
