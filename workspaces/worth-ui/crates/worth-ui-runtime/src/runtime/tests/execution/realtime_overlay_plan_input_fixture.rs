use crate::runtime::{WorthUiExecutionPlanInput, WorthUiPlanNodeInputFamily};

pub(super) fn plan_input_with_duplicate_render_ref(
    plan_input: &WorthUiExecutionPlanInput,
) -> WorthUiExecutionPlanInput {
    let duplicated = plan_input
        .node_inputs()
        .iter()
        .find(|input| input.family() == WorthUiPlanNodeInputFamily::RenderResourceRef)
        .expect("fixture has render resource ref")
        .clone()
        .with_identity_basis_for_test("realtime.fixture.render_ref.drift");
    let mut node_inputs = plan_input.node_inputs().to_vec();
    node_inputs.push(duplicated);
    WorthUiExecutionPlanInput::new(
        plan_input.basis().clone(),
        plan_input.context().clone(),
        node_inputs,
        plan_input.counters(),
    )
}

pub(super) fn plan_input_with_drifted_diagnostics_ref(
    plan_input: &WorthUiExecutionPlanInput,
) -> WorthUiExecutionPlanInput {
    let mut node_inputs = plan_input.node_inputs().to_vec();
    let diagnostics_index = node_inputs
        .iter()
        .position(|input| input.family() == WorthUiPlanNodeInputFamily::DiagnosticsRef)
        .expect("fixture has diagnostics ref");
    node_inputs[diagnostics_index] = node_inputs[diagnostics_index]
        .clone()
        .with_identity_basis_for_test("realtime.fixture.diagnostics.allocation_drift");
    WorthUiExecutionPlanInput::new(
        plan_input.basis().clone(),
        plan_input.context().clone(),
        node_inputs,
        plan_input.counters(),
    )
}

pub(super) fn plan_input_without_family(
    plan_input: &WorthUiExecutionPlanInput,
    removed_family: WorthUiPlanNodeInputFamily,
) -> WorthUiExecutionPlanInput {
    let node_inputs = plan_input
        .node_inputs()
        .iter()
        .filter(|input| input.family() != removed_family)
        .cloned()
        .collect();
    WorthUiExecutionPlanInput::new(
        plan_input.basis().clone(),
        plan_input.context().clone(),
        node_inputs,
        plan_input.counters(),
    )
}
