use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::app_reload_evidence_snapshot::manual_flow_matrix_snapshot::observed::observed_for_flow;
use crate::manual_flow::{
    ValidationManualFlowCounterPosture, ValidationManualFlowId, ValidationManualFlowVisibleResult,
};
use crate::reload::ValidationReloadLoopConfig;
use crate::sample_source::{
    VALIDATION_SAMPLE_APPEARANCE_SOURCE, VALIDATION_SAMPLE_COMMAND_PROJECTION_SOURCE,
    VALIDATION_SAMPLE_COMMAND_SOURCE, VALIDATION_SAMPLE_COMPONENT_SOURCE,
    VALIDATION_SAMPLE_DENSITY_SOURCE, VALIDATION_SAMPLE_SOURCE, VALIDATION_SAMPLE_THEME_SOURCE,
};
use crate::{ValidationWorkbenchApp, ValidationWorkbenchLaunch};

#[test]
fn layout_gap_flow_produces_typed_changed_fact_proof() {
    let workspace_root = create_temp_workspace_root();
    let launch = ValidationWorkbenchLaunch::new()
        .prepare_from_workspace_root(&workspace_root)
        .expect("temp workspace launch should prepare");
    let config = workspace_reload_config(&workspace_root, launch.authored_inputs());
    let mut app = ValidationWorkbenchApp::new_with_reload_loop_config(launch, config);

    app.apply_manual_source_text(
        VALIDATION_SAMPLE_SOURCE
            .replace("column gap(0) padding(0) {", "column gap(30) padding(0) {"),
    );

    let proof = app.proof_snapshot();
    let observed = observed_for_flow(ValidationManualFlowId::LayoutGap, &proof);

    assert_eq!(
        observed.visible_result(),
        &ValidationManualFlowVisibleResult::ChangedFact("LayoutGap(HeaderProofPage)".to_owned())
    );
    assert_eq!(
        observed.counter_posture(),
        ValidationManualFlowCounterPosture::HeaderPreservedPageHostRebuilt
    );
    remove_workspace_root(&workspace_root);
}

#[test]
fn observed_workspace_source_edit_emits_layout_gap_changed_fact() {
    let workspace_root = create_temp_workspace_root();
    let launch = ValidationWorkbenchLaunch::new()
        .prepare_from_workspace_root(&workspace_root)
        .expect("temp workspace launch should prepare");
    let config = workspace_reload_config(&workspace_root, launch.authored_inputs());
    let mut app = ValidationWorkbenchApp::new_with_reload_loop_config(launch, config);

    fs::write(
        workspace_root.join("source/header.wui"),
        VALIDATION_SAMPLE_SOURCE
            .replace("column gap(0) padding(0) {", "column gap(30) padding(0) {"),
    )
    .expect("edited source should write");
    app.run_one_reload_observation_cycle();

    let proof = app.proof_snapshot();
    let observed = observed_for_flow(ValidationManualFlowId::LayoutGap, &proof);

    assert_eq!(
        observed.visible_result(),
        &ValidationManualFlowVisibleResult::ChangedFact("LayoutGap(HeaderProofPage)".to_owned())
    );
    remove_workspace_root(&workspace_root);
}

fn workspace_reload_config(
    workspace_root: &Path,
    authored_inputs: &crate::ValidationWorkbenchAuthoredInputs,
) -> ValidationReloadLoopConfig {
    let theme_path = workspace_root.join("theme/header.theme");
    let mut config = ValidationReloadLoopConfig::new(&theme_path)
        .with_source_path(workspace_root.join("source/header.wui"))
        .with_command_path(workspace_root.join("theme/header.commands"))
        .with_command_projection_path(workspace_root.join("theme/header.projections"))
        .with_component_path(workspace_root.join("theme/header.components"))
        .with_appearance_path(workspace_root.join("theme/header.appearance"))
        .with_density_path(workspace_root.join("theme/header.density"))
        .with_initial_source(authored_inputs.source().clone());
    if let Some(theme) = authored_inputs.theme() {
        config = config.with_initial_theme(theme.clone());
    }
    if let Some(command) = authored_inputs.commands() {
        config = config.with_initial_command(command.clone());
    }
    if let Some(projections) = authored_inputs.command_projections() {
        config = config.with_initial_command_projection(projections.clone());
    }
    if let Some(component) = authored_inputs.component() {
        config = config.with_initial_component(component.clone());
    }
    if let Some(appearance) = authored_inputs.appearance() {
        config = config.with_initial_appearance(appearance.clone());
    }
    if let Some(density) = authored_inputs.density() {
        config = config.with_initial_density(density.clone());
    }
    config
}

fn create_temp_workspace_root() -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be valid")
        .as_nanos();
    let root = std::env::temp_dir().join(format!("worth-ui-validation-tests-{nonce}"));
    fs::create_dir_all(root.join("source")).expect("source dir should create");
    fs::create_dir_all(root.join("theme")).expect("theme dir should create");
    fs::write(root.join("source/header.wui"), VALIDATION_SAMPLE_SOURCE)
        .expect("source file should write");
    fs::write(
        root.join("theme/header.theme"),
        VALIDATION_SAMPLE_THEME_SOURCE,
    )
    .expect("theme file should write");
    fs::write(
        root.join("theme/header.commands"),
        VALIDATION_SAMPLE_COMMAND_SOURCE,
    )
    .expect("command file should write");
    fs::write(
        root.join("theme/header.projections"),
        VALIDATION_SAMPLE_COMMAND_PROJECTION_SOURCE,
    )
    .expect("projection file should write");
    fs::write(
        root.join("theme/header.components"),
        VALIDATION_SAMPLE_COMPONENT_SOURCE,
    )
    .expect("component file should write");
    fs::write(
        root.join("theme/header.appearance"),
        VALIDATION_SAMPLE_APPEARANCE_SOURCE,
    )
    .expect("appearance file should write");
    fs::write(
        root.join("theme/header.density"),
        VALIDATION_SAMPLE_DENSITY_SOURCE,
    )
    .expect("density file should write");
    root
}

fn remove_workspace_root(root: &Path) {
    let _ = fs::remove_dir_all(root);
}
