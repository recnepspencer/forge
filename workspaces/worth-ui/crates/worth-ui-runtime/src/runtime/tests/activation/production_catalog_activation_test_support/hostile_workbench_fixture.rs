pub(crate) fn runtime_with_hostile_workbench_catalog() -> (
    crate::runtime::WorthUiRuntimeFrameworkLoop,
    Box<[crate::graph::UiGraphNodeIdentity]>,
    crate::runtime::WorthUiAdmittedDurableResizeInput,
    worth_ui_query_binding::certification::WorthUiInstalledQueryTestFixture,
) {
    let mut installed_query =
        worth_ui_query_binding::certification::WorthUiInstalledQueryTestFixture::new(
            "milestone-3.8-hostile-workbench",
        );
    let mut binding = installed_query.binding_plan().prepare_downstream_state();
    let fact = binding
        .admit_settled_snapshot(installed_query.settle_snapshot())
        .expect("hostile workbench Query settlement admits");
    let (snapshot, admissions) = crate::runtime::tests::allocation_catalog_test_support::admitted_hostile_workbench_planning_admissions(
        "milestone-3.8-hostile-workbench",
        "inspector.measurements",
        &fact,
    );
    let (split_basis, split_selected) = admissions
        .last()
        .expect("mixed workbench includes a split region");
    let split_provenance = split_basis
        .admit_allocation_neighborhood(&snapshot, split_selected)
        .expect("split workbench neighborhood admits")
        .members()
        .iter()
        .find(|member| {
            matches!(
                member.role(),
                crate::evidence::UiAllocationNeighborhoodMemberRole::Root
            )
        })
        .expect("split region has a structural root")
        .authored_provenance_digest();
    let (runtime, pending, _) = crate::runtime::tests::durable_resize_input_boundary_tests::splitter_pending_activation_with_query_view_and_provenance(
        installed_query.installed_view(),
        split_provenance,
    );
    let activated = super::activate_catalog(
        runtime,
        pending,
        4,
        true,
        true,
        Some((snapshot, admissions)),
    );
    let mut runtime = activated.runtime;
    runtime.install_query_binding_state_for_test(binding);
    (
        runtime,
        activated.roots,
        activated
            .durable_resize
            .expect("mixed workbench activates durable splitter input"),
        installed_query,
    )
}
