use std::path::PathBuf;
use std::process::Command;

fn fixture_root(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}

fn run_fixture(name: &str) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_boundary-check"))
        .arg("--root")
        .arg(fixture_root(name))
        .arg("--config")
        .arg("tools/boundary-check/config/road1.toml")
        .output()
        .expect("run boundary-check fixture");
    assert!(
        !output.status.success(),
        "fixture {name} unexpectedly passed"
    );
    String::from_utf8_lossy(&output.stderr).into_owned()
}

#[test]
fn illegal_crate_name_is_rejected() {
    let output = run_fixture("illegal_crate_name");
    assert!(output.contains("BC1002_UNRESERVED_DOMAIN"));
}

#[test]
fn schema_query_import_is_rejected() {
    let output = run_fixture("schema_query_import");
    assert!(output.contains("BC3001_QUERY_IMPORT_OUTSIDE_ENTRY"));
}

#[test]
fn query_bridge_module_in_schema_is_rejected() {
    let output = run_fixture("schema_query_import");
    assert!(output.contains("BC3001_QUERY_IMPORT_OUTSIDE_ENTRY"));
}

#[test]
fn ordinary_replay_import_is_rejected() {
    let output = run_fixture("ordinary_replay_import");
    assert!(output.contains("BC4001_ORDINARY_REPLAY_IMPORT"));
}

#[test]
fn ordinary_reconstruction_import_is_rejected() {
    let output = run_fixture("ordinary_reconstruction_import");
    assert!(output.contains("BC4001_ORDINARY_REPLAY_IMPORT"));
}

#[test]
fn worth_to_worthy_inversion_is_rejected() {
    let output = run_fixture("worth_to_worthy_inversion");
    assert!(output.contains("BC2002_WORTH_TO_WORTHY_INVERSION"));
}

#[test]
fn root_owned_road1_package_is_rejected() {
    let output = run_fixture("root_owned_road1_package");
    assert!(output.contains("BC5001_ROOT_OWNS_ROAD1_PACKAGE"));
}

#[test]
fn schema_pack_import_is_rejected() {
    let output = run_fixture("schema_pack_import");
    assert!(output.contains("BC2001_BAND_DEPENDENCY_VIOLATION"));
}

#[test]
fn runtime_adapter_in_pack_registry_is_rejected() {
    let output = run_fixture("runtime_adapter_in_pack_registry");
    assert!(output.contains("BC2001_BAND_DEPENDENCY_VIOLATION"));
}

#[test]
fn placeholder_entry_birth_is_rejected() {
    let output = run_fixture("placeholder_entry_birth");
    assert!(output.contains("BC5003_SEED_CONTRACT_VIOLATION"));
    assert!(output.contains("born crate set mismatch"));
}

#[test]
fn facade_behavior_is_rejected() {
    let output = run_fixture("facade_behavior_seed");
    assert!(output.contains("BC5003_SEED_CONTRACT_VIOLATION"));
    assert!(output.contains("facade.rs must aggregate public exports only"));
}

#[test]
fn mixed_class_seed_module_is_rejected() {
    let output = run_fixture("mixed_class_seed_module");
    assert!(output.contains("BC5003_SEED_CONTRACT_VIOLATION"));
    assert!(output.contains("seed crate skeleton mismatch"));
}
