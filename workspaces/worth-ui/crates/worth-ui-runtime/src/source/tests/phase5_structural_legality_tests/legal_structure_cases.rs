use super::structural_legality_body_fixture_support::{
    resolved_artifact_input_from_modules, standard_component_module,
};
use super::structural_legality_capability_fixture_support::{
    component_with_schema, standard_app, standard_app_with_dashboard_component,
};
use crate::source::WorthUiStructuralLegalityLowerer;

#[test]
fn equivalent_legal_mosaic_structures_produce_equivalent_legality_artifacts() {
    let app = standard_app();
    let snapshot = app.capabilities();
    let resolved = resolved_artifact_input_from_modules([standard_component_module()], snapshot);

    let left = WorthUiStructuralLegalityLowerer::lower(&resolved, snapshot).unwrap();
    let right = WorthUiStructuralLegalityLowerer::lower(&resolved, snapshot).unwrap();

    assert!(left.equivalent_shape(&right));
    let component = left
        .module(left.module_ids().first().unwrap())
        .unwrap()
        .nodes()
        .iter()
        .find_map(|node| match node {
            crate::source::WorthUiLegallyStructuredArtifactInputNode::Component(node) => Some(node),
            _ => None,
        })
        .unwrap();
    assert_eq!(component.structure().root_regions().len(), 2);
    assert_eq!(
        component.structure().root_regions()[0].mounts()[0]
            .placement_policy()
            .unwrap()
            .0
            .id()
            .as_str(),
        "workspace.placement.primary"
    );
}

#[test]
fn identical_structure_with_reordered_rust_modules_stays_equivalent() {
    let app = standard_app();
    let snapshot = app.capabilities();
    let left = resolved_artifact_input_from_modules(
        [
            standard_component_module(),
            worth_ui_dsl::WorthUiRustAuthoredArtifactInputModule::new("app/extra.wui"),
        ],
        snapshot,
    );
    let right = resolved_artifact_input_from_modules(
        [
            worth_ui_dsl::WorthUiRustAuthoredArtifactInputModule::new("app/extra.wui"),
            standard_component_module(),
        ],
        snapshot,
    );

    let left = WorthUiStructuralLegalityLowerer::lower(&left, snapshot).unwrap();
    let right = WorthUiStructuralLegalityLowerer::lower(&right, snapshot).unwrap();
    assert!(left.equivalent_shape(&right));
}

#[test]
fn structural_legality_does_not_depend_on_component_runtime_execution() {
    let baseline_app = standard_app();
    let varied_app = standard_app_with_dashboard_component(component_with_schema(
        "workspace.component.dashboard",
        "workspace.component.dashboard.runtime_variant",
    ));
    let baseline_snapshot = baseline_app.capabilities();
    let varied_snapshot = varied_app.capabilities();

    let baseline_resolved =
        resolved_artifact_input_from_modules([standard_component_module()], baseline_snapshot);
    let varied_resolved =
        resolved_artifact_input_from_modules([standard_component_module()], varied_snapshot);

    let baseline =
        WorthUiStructuralLegalityLowerer::lower(&baseline_resolved, baseline_snapshot).unwrap();
    let varied =
        WorthUiStructuralLegalityLowerer::lower(&varied_resolved, varied_snapshot).unwrap();

    assert!(baseline.equivalent_shape(&varied));
}
