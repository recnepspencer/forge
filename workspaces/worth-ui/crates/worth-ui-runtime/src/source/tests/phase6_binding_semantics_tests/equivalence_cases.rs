use super::binding_app_fixture::{admitted_app, reordered_artifact_input};
use super::binding_phase_fixture::{bound_artifact_input, bound_artifact_input_for};

#[test]
fn equivalent_binding_inputs_produce_equivalent_bound_semantics() {
    let app = admitted_app();
    let snapshot = app.capabilities();

    let left = bound_artifact_input(snapshot);
    let right = bound_artifact_input(snapshot);

    assert!(left.equivalent_shape(&right));
}

#[test]
fn reordered_module_iteration_produces_equivalent_bound_semantics() {
    let app = admitted_app();
    let snapshot = app.capabilities();

    let left = bound_artifact_input(snapshot);
    let right = bound_artifact_input_for(snapshot, &reordered_artifact_input());

    assert!(left.equivalent_shape(&right));
}
