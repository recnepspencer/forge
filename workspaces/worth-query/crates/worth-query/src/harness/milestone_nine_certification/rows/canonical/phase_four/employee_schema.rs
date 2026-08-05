use crate::authorized_projection::PolicyAspectMask;
use crate::harness::certification::digest_parts;
use crate::harness::certification::HostileExpectation;
use crate::harness::certification::ParityAnchor;
use crate::harness::milestone_nine_certification::bundles::MilestoneNineCertificationRow;
use crate::harness::milestone_nine_certification::classifications::MilestoneNinePerturbationClass;
use crate::harness::milestone_nine_certification::fixtures::canonical_query_with_secret_projection;
use crate::harness::milestone_nine_certification::fixtures::native_authorized_projection_fields;
use crate::harness::milestone_nine_certification::fixtures::phase_four_bundle;
use crate::harness::milestone_nine_certification::fixtures::phase_four_bundle_from_narrowed;
use crate::harness::milestone_nine_certification::fixtures::phase_three_test_narrowed_artifact;
use crate::harness::milestone_nine_certification::fixtures::phase_three_test_unmasked_artifact;
use crate::harness::milestone_nine_certification::fixtures::phase_two_bundle;
use crate::harness::milestone_nine_certification::fixtures::secret_salary_key;
use crate::policy_certification::{
    employee_record_policy_fixture, employee_record_policy_scale_report,
    policy_composition_parity_report, policy_mask_parity_report, policy_view_shape_parity_report,
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

fn mask_digest(employee_alpha: &EmployeeRecordCertificationBundle) -> String {
    let canonical = canonical_query_with_secret_projection();
    let no_proof = phase_two_bundle(
        canonical.clone(),
        PolicyAspectMask::allow_all().with_masked(secret_salary_key()),
        RelationshipProofDescriptorSet::none(),
    );
    let unmasked = phase_two_bundle(
        canonical,
        PolicyAspectMask::allow_all(),
        RelationshipProofDescriptorSet::none(),
    );
    policy_mask_parity_report(
        unmasked.authorized_projection_digest.clone(),
        no_proof.authorized_projection_digest.clone(),
        no_proof.narrowed_result_shape_digest.clone(),
        employee_alpha.masked_field_digest(),
    )
    .parity_digest()
    .to_string()
}

pub(super) fn employee_schema_rows() -> Vec<MilestoneNineCertificationRow> {
    let artifact = phase_three_test_narrowed_artifact();
    let employee_alpha = employee_bundle(EmployeeRecordTenantVariant::TenantAlpha);
    let employee_beta = employee_bundle(EmployeeRecordTenantVariant::TenantBeta);
    let scale = scale_digest();
    let live_density = live_density_evidence(&artifact);
    let delivery = delivery_width_digest(&artifact);
    let composition = composition_digest(&artifact);
    let view_shape = view_shape_digest(&artifact);
    let mask = mask_digest(&employee_alpha);
    let phase_four_employee = phase_four_bundle(
        "employee-record-fixture",
        employee_alpha.employee_fixture_digest(),
        scale.as_str(),
        live_density.digest(),
        delivery.clone(),
        composition.as_str(),
        view_shape.as_str(),
        &[],
    );
    let phase_four_tenant_beta = phase_four_bundle(
        "employee-record-tenant-beta",
        employee_beta.employee_fixture_digest(),
        scale.as_str(),
        live_density.digest(),
        delivery.clone(),
        composition.as_str(),
        view_shape.as_str(),
        &[],
    );
    let phase_four_delivery_width = phase_four_bundle(
        "delivery-width-class-honesty",
        employee_alpha.employee_fixture_digest(),
        scale.as_str(),
        live_density.digest(),
        delivery.clone(),
        composition.as_str(),
        view_shape.as_str(),
        &[],
    );
    let phase_four_mask_parity = phase_four_bundle(
        "masked-versus-unmasked-policy-parity",
        employee_alpha.employee_fixture_digest(),
        scale.as_str(),
        live_density.digest(),
        delivery.clone(),
        mask.as_str(),
        view_shape.as_str(),
        &[],
    );
    let phase_four_unmasked_policy = phase_four_bundle_from_narrowed(
        "masked-versus-unmasked-policy-control",
        phase_three_test_unmasked_artifact(),
        employee_alpha.employee_fixture_digest(),
        scale.as_str(),
        live_density.digest(),
        delivery,
        mask.as_str(),
        view_shape.as_str(),
        &[],
    );
    vec![
        MilestoneNineCertificationRow {
            row_name: "employee-record-fixture-policy-basis",
            perturbation_class: MilestoneNinePerturbationClass::EmployeeRecordFixturePolicyBasis,
            hostile_expectation: HostileExpectation::EquivalentToControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: phase_four_employee.clone(),
            hostile_lane: phase_four_employee.clone(),
            parity_lane: phase_four_employee.clone(),
        },
        MilestoneNineCertificationRow {
            row_name: "tenant-alpha-versus-tenant-beta-schema",
            perturbation_class: MilestoneNinePerturbationClass::TenantAlphaVersusTenantBetaSchema,
            hostile_expectation: HostileExpectation::DistinctFromControl,
            parity_anchor: ParityAnchor::Hostile,
            control_lane: phase_four_employee.clone(),
            hostile_lane: phase_four_tenant_beta.clone(),
            parity_lane: phase_four_tenant_beta,
        },
        MilestoneNineCertificationRow {
            row_name: "masked-versus-unmasked-policy-parity",
            perturbation_class: MilestoneNinePerturbationClass::MaskedVersusUnmaskedPolicyParity,
            hostile_expectation: HostileExpectation::DistinctFromControl,
            parity_anchor: ParityAnchor::Hostile,
            control_lane: phase_four_unmasked_policy,
            hostile_lane: phase_four_mask_parity.clone(),
            parity_lane: phase_four_mask_parity,
        },
        MilestoneNineCertificationRow {
            row_name: "delivery-width-class-honesty",
            perturbation_class: MilestoneNinePerturbationClass::DeliveryWidthClassHonesty,
            hostile_expectation: HostileExpectation::EquivalentToControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: phase_four_delivery_width.clone(),
            hostile_lane: phase_four_delivery_width.clone(),
            parity_lane: phase_four_delivery_width,
        },
    ]
}
