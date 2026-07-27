use std::path::Path;

use crate::topology::WorkspaceSourceInventory;

use super::{
    courtroom_contract, destination_topology, evidence_document, opening_cost_budget,
    source_to_pixel_contract,
};

fn repository_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .and_then(Path::parent)
        .and_then(Path::parent)
        .expect("repository root")
}

fn contract() -> toml::Value {
    evidence_document::load_toml(
        &repository_root().join("_docs/worth-ui/milestone-3.10.2-phase-1-source-to-pixel.toml"),
    )
    .expect("Phase 1 contract")
}

fn workspace_inventory() -> WorkspaceSourceInventory {
    WorkspaceSourceInventory::capture(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("crate parent")
            .parent()
            .expect("workspace root"),
    )
}

#[test]
fn each_required_courtroom_mutation_is_independently_required() {
    for mutation_id in [
        "M01_DIRECT_EGUI_DRAWING",
        "M02_INJECTED_SOURCE",
        "M03_COUNT_ONLY_PAINT",
        "M04_DETACHED_SCREENSHOT",
    ] {
        let mut document = contract();
        let rows = document
            .get_mut("mutation")
            .and_then(toml::Value::as_array_mut)
            .expect("mutation rows");
        rows.retain(|row| row.get("id").and_then(toml::Value::as_str) != Some(mutation_id));
        let error = courtroom_contract::audit(&document)
            .expect_err("removing a required mutation should open the courtroom");
        assert!(error.contains(mutation_id), "{error}");
    }
}

#[test]
fn each_courtroom_mutation_requires_its_full_causal_evidence_set() {
    for mutation_id in [
        "M01_DIRECT_EGUI_DRAWING",
        "M02_INJECTED_SOURCE",
        "M03_COUNT_ONLY_PAINT",
        "M04_DETACHED_SCREENSHOT",
    ] {
        let mut document = contract();
        let row = document
            .get_mut("mutation")
            .and_then(toml::Value::as_array_mut)
            .expect("mutation rows")
            .iter_mut()
            .find(|row| row.get("id").and_then(toml::Value::as_str) == Some(mutation_id))
            .expect("required mutation");
        row["detection"] = toml::Value::String("the screenshot looks right".to_owned());
        row["invalidated_evidence"]
            .as_array_mut()
            .expect("invalidated evidence")
            .pop();
        let error = courtroom_contract::audit(&document)
            .expect_err("generic prose and incomplete causal evidence should fail");
        assert!(error.contains(mutation_id), "{error}");
        assert!(error.contains("invalidate exactly"), "{error}");
    }
}

#[test]
fn assigning_semantic_paint_authority_to_egui_is_rejected() {
    let mut document = contract();
    let row = document
        .get_mut("edge")
        .and_then(toml::Value::as_array_mut)
        .expect("edge rows")
        .iter_mut()
        .find(|row| {
            row.get("id").and_then(toml::Value::as_str) == Some("E11_HOST_ADMISSION_TO_EGUI_SHAPE")
        })
        .expect("egui edge");
    row.as_table_mut().expect("edge table").insert(
        "authority_owner".to_owned(),
        toml::Value::String("worth-ui-host-egui".to_owned()),
    );
    let error = source_to_pixel_contract::audit(&document)
        .expect_err("egui semantic authority should be rejected");
    assert!(error.contains("cannot own semantic authority"), "{error}");
}

#[test]
fn widening_the_pulse_dependency_allowlist_is_rejected() {
    let config_text =
        std::fs::read_to_string(repository_root().join("tools/boundary-check/config/road1.toml"))
            .expect("boundary config");
    let mut config = config_text.parse::<toml::Value>().expect("valid config");
    let rows = config
        .get_mut("source_dependency_allowlists")
        .and_then(toml::Value::as_array_mut)
        .expect("source allowlist rows");
    let pulse = rows
        .iter_mut()
        .find(|row| {
            row.get("sources")
                .and_then(toml::Value::as_array)
                .is_some_and(|sources| {
                    sources
                        .iter()
                        .any(|source| source.as_str() == Some("worth-ui-platform-pulse"))
                })
        })
        .expect("pulse rule");
    pulse
        .get_mut("allowed_targets")
        .and_then(toml::Value::as_array_mut)
        .expect("allowed targets")
        .push(toml::Value::String("worth-ui-runtime".to_owned()));
    let error = destination_topology::audit_boundary_config(&config)
        .expect_err("runtime should not enter the pulse allowlist");
    assert!(error.contains("dependencies should be exactly"), "{error}");
}

#[test]
fn drifting_the_boundary_native_shell_contract_is_rejected() {
    let config_text =
        std::fs::read_to_string(repository_root().join("tools/boundary-check/config/road1.toml"))
            .expect("boundary config");
    let mut config = config_text.parse::<toml::Value>().expect("valid config");
    let pulse = config
        .get_mut("source_dependency_allowlists")
        .and_then(toml::Value::as_array_mut)
        .expect("source allowlist rows")
        .iter_mut()
        .find(|row| {
            row.get("sources")
                .and_then(toml::Value::as_array)
                .is_some_and(|sources| {
                    sources
                        .iter()
                        .any(|source| source.as_str() == Some("worth-ui-platform-pulse"))
                })
        })
        .expect("pulse rule");
    pulse["dependency_contracts"][0]["version_requirement"] =
        toml::Value::String("^0.31".to_owned());
    let error = destination_topology::audit_boundary_config(&config)
        .expect_err("native-shell version drift should fail");
    assert!(error.contains("dependency contract drifted"), "{error}");
}

#[test]
fn increasing_the_integration_target_budget_is_rejected() {
    let path =
        repository_root().join("_docs/worth-ui/milestone-3.10.2-phase-1-opening-baseline.json");
    let mut baseline = evidence_document::load_json(&path).expect("opening baseline");
    baseline["topology"]["maximum_integration_test_targets"] = serde_json::Value::from(10);
    let error = opening_cost_budget::audit(&workspace_inventory(), &baseline)
        .expect_err("integration target budget should remain frozen");
    assert!(
        error.contains("maximum_integration_test_targets"),
        "{error}"
    );
}

#[test]
fn a_misnamed_successor_library_cannot_consume_target_twenty_two() {
    let path = repository_root().join("workspaces/worth-ui/apps/platform-pulse/Cargo.toml");
    let text = std::fs::read_to_string(path).expect("pulse manifest");
    let mut manifest = text.parse::<toml::Value>().expect("pulse manifest TOML");
    manifest["lib"]["path"] = toml::Value::String("src/application.rs".to_owned());

    let error = opening_cost_budget::audit_successor_observation_library(&manifest)
        .expect_err("target 22 cannot be a disguised application library");

    assert!(error.contains("observation-only library"), "{error}");
}

#[test]
fn a_misnamed_executable_world_cannot_consume_target_twenty_three() {
    let path = repository_root().join("workspaces/worth-ui/apps/platform-pulse/Cargo.toml");
    let text = std::fs::read_to_string(path).expect("pulse manifest");
    let mut manifest = text.parse::<toml::Value>().expect("pulse manifest TOML");
    manifest["test"][0]["path"] =
        toml::Value::String("tests/disguised_executable_world.rs".to_owned());

    let error = opening_cost_budget::audit_successor_executable_world(&manifest)
        .expect_err("target 23 cannot be a disguised executable-world test");

    assert!(
        error.contains("exactly the pulse executable-world"),
        "{error}"
    );
}

#[test]
fn native_shell_version_and_feature_drift_is_rejected() {
    let mut document = contract();
    document["native_shell"]["version_requirement"] = toml::Value::String("^0.31".to_owned());
    document["native_shell"]["features"]
        .as_array_mut()
        .expect("native features")
        .push(toml::Value::String("wgpu".to_owned()));
    let error = courtroom_contract::audit(&document)
        .expect_err("native shell drift should reopen the courtroom");
    assert!(error.contains("native shell"), "{error}");
}

#[test]
fn born_pulse_manifest_must_disable_implicit_test_discovery() {
    let manifest = r#"
        [package]
        name = "worth-ui-platform-pulse"

        [dependencies]
        eframe = { workspace = true }
        worth-ui = { path = "../../crates/worth-ui" }
        worth-ui-host-egui = { path = "../../crates/worth-ui-host-egui" }
    "#
    .parse::<toml::Value>()
    .expect("synthetic pulse manifest");
    let error = destination_topology::audit_pulse_manifest(&manifest)
        .expect_err("pulse must disable implicit test targets");
    assert!(error.contains("autotests = false"), "{error}");
}

#[test]
fn implicit_test_counter_cannot_hide_a_discovered_test_target() {
    let enabled = r#"
        [package]
        name = "worth-ui-platform-pulse"
    "#
    .parse::<toml::Value>()
    .expect("enabled manifest");
    let disabled = r#"
        [package]
        name = "worth-ui-platform-pulse"
        autotests = false
    "#
    .parse::<toml::Value>()
    .expect("disabled manifest");
    let entries = vec![std::path::PathBuf::from(
        "apps/platform-pulse/tests/hidden.rs",
    )];
    assert_eq!(
        opening_cost_budget::implicit_test_target_count(&enabled, &entries),
        1
    );
    assert_eq!(
        opening_cost_budget::implicit_test_target_count(&disabled, &entries),
        0
    );
}
