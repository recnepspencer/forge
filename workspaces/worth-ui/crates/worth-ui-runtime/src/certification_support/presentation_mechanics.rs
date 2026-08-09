pub fn initial_presentation_mechanics_for_certification(
    projection: &worth_ui_host_contract::UiMountedProjectionView,
    requirement: worth_ui_host_contract::UiMountedSurfaceBindingRequirement,
) -> worth_ui_host_contract::UiMountedPresentationInitial {
    let lease = crate::mounting::presentation::UiMountedPresentationLeaseGate::default()
        .claim()
        .expect("an isolated certification presentation lease is available");
    crate::mounting::presentation::work_producer::UiMountedPresentationState::from_projection(
        projection,
        requirement,
    )
    .issue_initial(&lease, projection)
    .into_initial_mechanics()
    .expect("the initial producer emits initial mechanics")
}
