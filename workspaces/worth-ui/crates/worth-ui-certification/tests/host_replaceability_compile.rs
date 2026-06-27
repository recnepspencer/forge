#[path = "trybuild_support.rs"]
mod trybuild_support;
use std::path::Path;
use std::process::Command;

#[test]
fn second_adapter_can_satisfy_the_host_contract_without_egui_internals() {
    trybuild_support::new_test_cases().pass("tests/ui/host/second_adapter_implements_contract.rs");
}

#[test]
fn alternate_adapter_crate_compiles_with_host_contract_only_dependency() {
    let fixture_manifest = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/host_contract_only_adapter/Cargo.toml");
    let output = Command::new("cargo")
        .arg("check")
        .arg("--manifest-path")
        .arg(&fixture_manifest)
        .output()
        .expect("host-contract-only adapter fixture should run cargo check");
    assert!(
        output.status.success(),
        "cargo check failed for {}:\nstdout:\n{}\nstderr:\n{}",
        fixture_manifest.display(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

