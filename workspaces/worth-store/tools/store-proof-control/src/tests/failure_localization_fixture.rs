use std::process::Command;

use super::scratch_workspace::ScratchCargoWorkspace;

#[test]
fn consolidated_failure_names_the_scenario_and_predicate() {
    let workspace = ScratchCargoWorkspace::new("failure-localization");
    workspace.write(
        "Cargo.toml",
        "[package]\nname = \"fixture-suite\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    );
    workspace.write("src/lib.rs", "pub fn production() {}\n");
    workspace.write(
        "tests/consolidated.rs",
        "mod recovery {\n    #[test]\n    fn stale_root__predicate_generation_matches() {\n        assert_eq!(1, 2, \"predicate generation_matches\");\n    }\n}\n",
    );
    let output = Command::new("cargo")
        .args([
            "test",
            "--test",
            "consolidated",
            "--",
            "--exact",
            "recovery::stale_root__predicate_generation_matches",
            "--nocapture",
        ])
        .current_dir(workspace.root())
        .output()
        .unwrap();
    assert!(!output.status.success());
    let transcript = format!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(transcript.contains("recovery::stale_root__predicate_generation_matches"));
    assert!(transcript.contains("predicate generation_matches"));
}
