use worth_ui::facade::{
    WorthUiHostFrameObservationDraft, WorthUiLayoutAllocationRequest,
    WorthUiMeasuredProductViewReceipt, WorthUiMountedProductViewReceipt,
};
use worth_ui_validation_app::reload::ValidationLiveViewSource;
use worth_ui_validation_app::{
    ValidationWorkbenchApp, ValidationWorkbenchAuthoredInputs, ValidationWorkbenchLaunch,
};

#[path = "live_view_product_fixtures.rs"]
#[allow(dead_code)]
mod live_view_product_fixtures;

pub fn prepared_app_with_live_view_source(source_text: String) -> ValidationWorkbenchApp {
    let inputs = ValidationWorkbenchAuthoredInputs::sample()
        .with_live_view_source(ValidationLiveViewSource::new(source_text));
    let launch = ValidationWorkbenchLaunch::new()
        .prepare_from_authored_inputs(inputs)
        .expect("validation app should prepare");
    ValidationWorkbenchApp::new(launch)
}

pub fn mounted_product_view(app: &ValidationWorkbenchApp) -> WorthUiMountedProductViewReceipt {
    app.live_view_projection_proof()
        .expect("projection should admit")
        .mounted_product_view()
        .clone()
}

pub fn allocate_input_stack(
    app: &ValidationWorkbenchApp,
    width: f32,
) -> worth_ui::facade::WorthUiLayoutAllocationReceipt {
    let mounted = mounted_product_view(app);
    allocate_input_stack_for_mounted(app, &mounted, width)
}

pub fn allocate_input_stack_for_mounted(
    app: &ValidationWorkbenchApp,
    mounted: &WorthUiMountedProductViewReceipt,
    width: f32,
) -> worth_ui::facade::WorthUiLayoutAllocationReceipt {
    let measured =
        measured_view_with_observations(app, mounted, "input_stack", width, 180.0, |draft| draft);
    allocate(app, &measured, "input_stack")
}

pub fn measured_view_with_observations<F>(
    app: &ValidationWorkbenchApp,
    mounted: &WorthUiMountedProductViewReceipt,
    node_id: &str,
    width: f32,
    height: f32,
    observe: F,
) -> WorthUiMeasuredProductViewReceipt
where
    F: FnOnce(WorthUiHostFrameObservationDraft) -> WorthUiHostFrameObservationDraft,
{
    let draft =
        WorthUiHostFrameObservationDraft::for_mounted_product_view(mounted.receipt_digest(), 11)
            .observe_available_bounds(node_id, width, height)
            .observe_viewport(node_id, width, height);
    let admitted = app
        .workbench()
        .runtime()
        .admit_host_frame_observations(mounted, observe(draft))
        .expect("host observations admit");
    app.workbench()
        .runtime()
        .measure_mounted_product_view(mounted, admitted)
        .expect("measured product view admits")
}

pub fn allocate(
    app: &ValidationWorkbenchApp,
    measured: &WorthUiMeasuredProductViewReceipt,
    root: &str,
) -> worth_ui::facade::WorthUiLayoutAllocationReceipt {
    app.workbench()
        .runtime()
        .allocate_mounted_product_view(
            measured,
            WorthUiLayoutAllocationRequest::for_root_node(root),
        )
        .expect("layout allocation should admit")
}

pub fn input_stack_policy(
    allocation: &worth_ui::facade::WorthUiLayoutAllocationReceipt,
) -> &worth_ui::facade::WorthUiLayoutAllocationContainerPolicyReceipt {
    allocation
        .container_policies()
        .iter()
        .find(|policy| policy.node_id() == "input_stack")
        .expect("input stack policy receipt")
}

pub fn allocated_child<'a>(
    allocation: &'a worth_ui::facade::WorthUiLayoutAllocationReceipt,
    child_node_id: &str,
) -> &'a worth_ui::facade::WorthUiAllocatedChildReceipt {
    allocation
        .children()
        .iter()
        .find(|child| child.child_node_id() == child_node_id)
        .unwrap_or_else(|| panic!("missing allocated frame for {child_node_id}"))
}

pub fn row_contact_form_source() -> String {
    live_view_product_fixtures::contact_form_source().replace("flow_kind column", "flow_kind row")
}

pub fn weighted_row_with_hug_text_source() -> String {
    row_contact_form_source().replace(
        "child control contact_mode_input sizing fill(1)",
        "child text helper_copy sizing hug\n                child control contact_mode_input sizing fill(2)",
    )
}

pub fn text_icon_baseline_source() -> String {
    row_contact_form_source()
        .replace("flow_align end", "flow_align end\n    flow_cross_align baseline")
        .replace(
            "child control first_name_input sizing fill(1)\n                child control contact_mode_input sizing fill(1)",
            "child text label_text sizing hug\n                child icon status_icon sizing hug",
        )
}

pub fn assert_close(left: f32, right: f32) {
    assert!(
        (left - right).abs() < 0.01,
        "expected {left} to be within 0.01 of {right}"
    );
}
