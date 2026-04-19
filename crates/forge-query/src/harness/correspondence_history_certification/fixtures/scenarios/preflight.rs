pub(crate) fn detail_preflight_bundle() -> crate::basis::ExecutionPreflightBundle {
    let validated = crate::harness::fixtures::validated_bundles::runtime_detail_bundle();
    let request = crate::harness::fixtures::planning_requests::direct_runtime_request(&validated);
    let basis = crate::harness::fixtures::resolved_bases::runtime_basis(&validated, "snapshot-1");
    let plan = crate::facade::plan_validated_bundle(&validated, request)
        .expect("detail validated bundle should plan");
    crate::facade::preflight_execution_basis(plan, basis).expect("detail plan should preflight")
}

pub(crate) fn collection_preflight_bundle() -> crate::basis::ExecutionPreflightBundle {
    let validated = crate::harness::fixtures::validated_bundles::ordered_collection_bundle();
    let request = crate::harness::fixtures::planning_requests::direct_runtime_request(&validated);
    let basis = crate::harness::fixtures::resolved_bases::runtime_basis(&validated, "snapshot-1");
    let plan = crate::facade::plan_validated_bundle(&validated, request)
        .expect("collection validated bundle should plan");
    crate::facade::preflight_execution_basis(plan, basis).expect("collection plan should preflight")
}
