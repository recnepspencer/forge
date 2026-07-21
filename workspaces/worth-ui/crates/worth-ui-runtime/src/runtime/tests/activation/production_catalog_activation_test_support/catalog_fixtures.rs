use super::{activate_catalog, activation_staging_inputs};

pub(crate) fn runtime_with_scroll_catalog() -> (
    crate::runtime::WorthUiRuntimeFrameworkLoop,
    Box<[crate::graph::UiGraphNodeIdentity]>,
    crate::evidence::UiMeasurementResult,
    crate::evidence::UiSettledQueryFactReceipt,
    crate::runtime::UiAllocationReceipt,
    crate::runtime::UiAllocationReceipt,
    crate::runtime::UiCommittedAllocationEvidenceSet,
    worth_ui_query_binding::certification::WorthUiInstalledQueryTestFixture,
) {
    let mut installed_query =
        worth_ui_query_binding::certification::WorthUiInstalledQueryTestFixture::new(
            "production-scroll-catalog-activation-scroll-1",
        );
    let inputs = super::super::activation_staging_test_support::activation_staging_inputs_with_installed_query_view(
        installed_query.installed_view(),
    );
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let predecessor_link = runtime.query_fact_link_for_test("inspector.measurements");
    let mut binding = installed_query.binding_plan().prepare_downstream_state();
    let fact = binding
        .admit_settled_snapshot(installed_query.settle_snapshot())
        .expect("installed Query settlement should admit before catalog planning");
    let admitted_inputs = crate::runtime::tests::allocation_catalog_test_support::admitted_scroll_planning_admissions_from_settled_fact(
        "production-scroll-catalog-activation",
        2,
        "inspector.measurements",
        &fact,
    );
    let activated = activate_catalog(runtime, pending, 2, true, false, Some(admitted_inputs));
    let mut runtime = activated.runtime;
    assert!(
        runtime.query_fact_link_is_current_for_test("inspector.measurements"),
        "activated Query fact link must belong to the published application generation"
    );
    runtime.install_query_binding_state_for_test(binding);
    let mut stale_denial = None;
    let completion = runtime.execute_framework_turn(|turn| {
        turn.query_projection(|source| {
            stale_denial = source.submit_settled(&predecessor_link).err();
        });
    });
    drop(completion);
    assert_eq!(
        stale_denial,
        Some(crate::runtime::WorthUiQueryFrameIngressDenial::StaleApplicationGeneration),
        "a lane link issued by the predecessor generation must not survive publication"
    );
    let basis = activated.planning.measurement_basis();
    let result = basis
        .host_allocation_requests()
        .find_map(|request| basis.host_measurement_result(request))
        .expect("scroll basis carries host evidence")
        .clone();
    (
        runtime,
        activated.roots,
        result,
        activated
            .query
            .expect("scroll catalog carries Query content extent"),
        activated.receipt,
        activated.unrelated_receipt,
        activated.committed_evidence,
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
    let activated = activate_catalog(runtime, pending, 2, false, true, None);
    (
        activated.runtime,
        activated.roots,
        activated.receipt,
        activated.committed_evidence,
    )
}
