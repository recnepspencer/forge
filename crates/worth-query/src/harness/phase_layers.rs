#[test]
fn phase_aligned_fixtures_compose_pipeline_artifacts() {
    let canonical = crate::harness::fixtures::canonical_bundles::runtime_detail_bundle();
    let validated = crate::harness::fixtures::validated_bundles::runtime_detail_bundle();
    let request = crate::harness::fixtures::planning_requests::direct_runtime_request(&validated);
    let basis = crate::harness::fixtures::resolved_bases::runtime_basis(
        &validated,
        &crate::harness::fixtures::resolved_bases::primary_snapshot_identity(),
    );
    let plan = crate::facade::policy::plan_validated_bundle(&validated, request).unwrap();
    let preflight = crate::facade::foundation::preflight_execution_basis(plan, basis).unwrap();

    assert_eq!(
        canonical.query().digest().as_str(),
        validated.query().canonical_query_digest().as_str()
    );
    assert_eq!(preflight.report().snapshot_basis_resolution_count(), 1);
}

#[test]
fn bound_fixture_paths_support_bound_and_pre_resolved_planning() {
    let bound_bundle = crate::harness::fixtures::validated_bundles::runtime_bound_detail_bundle();
    let bound_request =
        crate::harness::fixtures::planning_requests::bound_runtime_request(&bound_bundle, "user-1");
    let pre_resolved_request =
        crate::harness::fixtures::planning_requests::pre_resolved_bound_runtime_request(
            &bound_bundle,
            "user-1",
        );

    let bound_plan =
        crate::facade::policy::plan_validated_bundle(&bound_bundle, bound_request).unwrap();
    let pre_resolved_plan =
        crate::facade::policy::plan_validated_bundle(&bound_bundle, pre_resolved_request).unwrap();

    assert_eq!(
        bound_plan.query().plan_digest(),
        pre_resolved_plan.query().plan_digest()
    );
}
