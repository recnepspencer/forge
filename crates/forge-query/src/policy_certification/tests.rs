use super::{
    employee_record_policy_fixture, employee_record_policy_scale_report,
    policy_composition_parity_report, policy_identity_aware_inspector_parity_report,
    policy_mask_parity_report, policy_view_shape_parity_report, EmployeeRecordPolicyScenario,
    EmployeeRecordQueryFamily, EmployeeRecordTenantVariant, PolicyScaleCounterSnapshot,
    PolicyScaleFixtureSize, PolicyScaleSlopeReport,
};

#[test]
fn employee_record_fixture_is_concrete_and_deterministic() {
    let fixture = employee_record_policy_fixture();
    let repeated = employee_record_policy_fixture();
    let bundle = fixture.certify(EmployeeRecordPolicyScenario::new(
        EmployeeRecordTenantVariant::TenantAlpha,
        EmployeeRecordQueryFamily::DirectDetail,
    ));

    assert_eq!(fixture.digest(), repeated.digest());
    assert!(fixture
        .public_fields()
        .contains(&"employee.employee_id".to_string()));
    assert!(fixture
        .public_fields()
        .contains(&"profile.display_name".to_string()));
    assert_eq!(fixture.masked_field(), "compensation.salary_band");
    assert_eq!(bundle.employee_fixture_digest(), fixture.digest());
}

#[test]
fn employee_record_tenant_variants_have_distinct_basis_digests() {
    let fixture = employee_record_policy_fixture();
    let alpha = fixture.certify(EmployeeRecordPolicyScenario::new(
        EmployeeRecordTenantVariant::TenantAlpha,
        EmployeeRecordQueryFamily::DirectDetail,
    ));
    let beta = fixture.certify(EmployeeRecordPolicyScenario::new(
        EmployeeRecordTenantVariant::TenantBeta,
        EmployeeRecordQueryFamily::DirectDetail,
    ));

    assert_ne!(
        alpha.tenant_truth_basis_digest(),
        beta.tenant_truth_basis_digest()
    );
    assert_ne!(
        alpha.tenant_schema_basis_digest(),
        beta.tenant_schema_basis_digest()
    );
}

#[test]
fn employee_record_query_families_are_distinct_scenarios_not_abstract_labels() {
    let fixture = employee_record_policy_fixture();
    let families = [
        EmployeeRecordQueryFamily::DirectDetail,
        EmployeeRecordQueryFamily::CollectionOrderedByDisplayName,
        EmployeeRecordQueryFamily::FilterBySalaryBand,
        EmployeeRecordQueryFamily::OrderBySalaryBand,
        EmployeeRecordQueryFamily::GroupBySalaryBand,
        EmployeeRecordQueryFamily::AggregateSalaryBand,
        EmployeeRecordQueryFamily::CursorBySalaryBand,
        EmployeeRecordQueryFamily::ViewMembershipBySalaryBand,
        EmployeeRecordQueryFamily::LiveRelevanceBySalaryBand,
        EmployeeRecordQueryFamily::SavedQueryReuse,
        EmployeeRecordQueryFamily::RuntimeHistoricalRead,
    ];
    let mut scenario_digests = families
        .iter()
        .map(|family| {
            fixture
                .certify(EmployeeRecordPolicyScenario::new(
                    EmployeeRecordTenantVariant::TenantAlpha,
                    *family,
                ))
                .scenario_digest()
                .to_string()
        })
        .collect::<Vec<_>>();
    scenario_digests.sort();
    scenario_digests.dedup();

    assert_eq!(
        scenario_digests.len(),
        families.len(),
        "each EmployeeRecord query family must produce distinct scenario evidence"
    );
}

#[test]
fn policy_scale_slope_is_structural_and_zero_rediscovery() {
    let report = employee_record_policy_scale_report();

    assert!(report.executor_rediscovery_is_zero());
    assert!(report.structural_widths_are_constant());
    assert_eq!(report.small().fixture_size().row_count(), 3);
    assert_eq!(report.medium().fixture_size().row_count(), 30);
    assert_eq!(report.larger().fixture_size().row_count(), 300);
    assert!(!report.digest().as_str().is_empty());
}

#[test]
fn policy_scale_slope_digest_changes_when_counter_slope_drifts() {
    let report = employee_record_policy_scale_report();
    let drifted = PolicyScaleSlopeReport::new(
        PolicyScaleCounterSnapshot::new(PolicyScaleFixtureSize::Small, 4, 2, 2, 4, 4, 1, 12, 0),
        PolicyScaleCounterSnapshot::new(PolicyScaleFixtureSize::Medium, 5, 2, 2, 4, 4, 1, 12, 0),
        PolicyScaleCounterSnapshot::new(PolicyScaleFixtureSize::Larger, 4, 2, 2, 4, 4, 1, 12, 0),
    );

    assert!(!drifted.structural_widths_are_constant());
    assert_ne!(report.digest().as_str(), drifted.digest().as_str());
}

#[test]
fn policy_composition_and_view_shape_parity_reports_are_digest_bound() {
    let composition = policy_composition_parity_report("narrowed-a");
    let view = policy_view_shape_parity_report("table-a", "grouped-a", "inspector-a");

    assert!(composition.all_lanes_equal());
    assert_eq!(composition.direct_narrowed_artifact_digest(), "narrowed-a");
    assert_ne!(view.table_delivery_digest(), view.grouped_delivery_digest());
    assert!(!composition.parity_digest().is_empty());
    assert!(!view.parity_digest().is_empty());
}

#[test]
fn policy_mask_and_identity_inspector_parity_reports_are_digest_bound() {
    let mask = policy_mask_parity_report(
        "unmasked-projection",
        "masked-projection",
        "shape",
        "salary",
    );
    let inspector = policy_identity_aware_inspector_parity_report(
        "identity-class",
        "inspector-delivery",
        "masked-shape",
    );

    assert!(mask.projections_are_distinct());
    assert_eq!(mask.masked_field_digest(), "salary");
    assert_eq!(inspector.identity_classification_digest(), "identity-class");
    assert_eq!(inspector.masked_shape_digest(), "masked-shape");
    assert!(!mask.parity_digest().is_empty());
    assert!(!inspector.parity_digest().is_empty());
}
