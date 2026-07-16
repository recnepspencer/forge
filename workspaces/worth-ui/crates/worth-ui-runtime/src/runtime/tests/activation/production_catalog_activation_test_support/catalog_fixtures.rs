use super::{activate_catalog, activation_staging_inputs};

pub(crate) fn runtime_with_scroll_catalog() -> (
    crate::runtime::WorthUiRuntimeFrameworkLoop,
    Box<[crate::graph::UiGraphNodeIdentity]>,
    crate::evidence::UiMeasurementResult,
    crate::evidence::UiProjectionFactReceipt,
    crate::runtime::UiAllocationReceipt,
    crate::runtime::UiAllocationReceipt,
    crate::runtime::UiCommittedAllocationEvidenceSet,
    worth_ui_query_binding::certification::WorthUiInstalledQueryTestFixture,
) {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let mut installed_query =
        worth_ui_query_binding::certification::WorthUiInstalledQueryTestFixture::new(
            "production-scroll-catalog-activation-scroll-1",
        );
    let mut binding = installed_query.binding_plan().activate();
    let settlement = binding
        .admit(installed_query.project())
        .expect("installed Query projection should settle before catalog admission");
    let admitted_inputs = crate::runtime::tests::allocation_catalog_test_support::admitted_scroll_planning_admissions_from_settlement(
        "production-scroll-catalog-activation",
        2,
        &settlement,
    );
    let (runtime, roots, planning, _, _, receipt, unrelated_receipt, query, evidence) =
        activate_catalog(runtime, pending, 2, true, false, Some(admitted_inputs));
    let mut runtime = runtime;
    runtime.install_query_binding_for_test(installed_query.binding_plan());
    let basis = planning.measurement_basis();
    let result = basis
        .host_allocation_requests()
        .find_map(|request| basis.host_measurement_result(request))
        .expect("scroll basis carries host evidence")
        .clone();
    (
        runtime,
        roots,
        result,
        query.expect("scroll catalog carries Query content extent"),
        receipt,
        unrelated_receipt,
        evidence.expect("scroll activation retains committed evidence"),
        installed_query,
    )
}

pub(crate) fn runtime_with_portal_catalog() -> (
    crate::runtime::WorthUiRuntimeFrameworkLoop,
    Box<[crate::graph::UiGraphNodeIdentity]>,
    crate::runtime::UiAllocationReceipt,
    crate::runtime::UiCommittedAllocationEvidenceSet,
) {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let (runtime, roots, _, _, _, receipt, _, _, evidence) =
        activate_catalog(runtime, pending, 2, false, true, None);
    (
        runtime,
        roots,
        receipt,
        evidence.expect("portal activation retains committed evidence"),
    )
}
