use std::process::Command;

use super::scratch_workspace::ScratchCargoWorkspace;

#[test]
fn shared_support_is_compiled_once_for_one_suite_boundary() {
    let workspace = ScratchCargoWorkspace::new("shared-codegen");
    workspace.write(
        "Cargo.toml",
        "[workspace]\nmembers = [\"support\", \"suite\"]\nresolver = \"2\"\n",
    );
    workspace.write(
        "support/Cargo.toml",
        "[package]\nname = \"fixture-support\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    );
    workspace.write("support/src/lib.rs", "pub fn setup() -> u64 { 7 }\n");
    workspace.write(
        "suite/Cargo.toml",
        "[package]\nname = \"fixture-suite\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dev-dependencies]\nfixture-support = { path = \"../support\" }\n",
    );
    workspace.write("suite/src/lib.rs", "pub fn production() {}\n");
    workspace.write(
        "suite/tests/consolidated.rs",
        "#[test]\nfn first_scenario() { assert_eq!(fixture_support::setup(), 7); }\n\n#[test]\nfn second_scenario() { assert_eq!(fixture_support::setup(), 7); }\n",
    );
    let output = Command::new("cargo")
        .args([
            "test",
            "-p",
            "fixture-suite",
            "--test",
            "consolidated",
            "--no-run",
            "--message-format=json",
        ])
        .current_dir(workspace.root())
        .output()
        .unwrap();
    assert!(output.status.success());
    let support_artifacts = String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| serde_json::from_str::<serde_json::Value>(line).ok())
        .filter(|event| event["reason"] == "compiler-artifact")
        .filter(|event| event["target"]["name"] == "fixture_support")
        .count();
    assert_eq!(support_artifacts, 1);
}
