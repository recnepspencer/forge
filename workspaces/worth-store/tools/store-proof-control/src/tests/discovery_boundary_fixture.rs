use crate::discovery::{discover_workspace, CaseKind};

use super::scratch_workspace::ScratchCargoWorkspace;

#[test]
fn actual_unregistered_test_and_ui_paths_are_discovered_as_denials() {
    let workspace = ScratchCargoWorkspace::new("hidden-target");
    workspace.write(
        "Cargo.toml",
        "[workspace]\nmembers = [\"owner\"]\nresolver = \"2\"\n",
    );
    workspace.write(
        "owner/Cargo.toml",
        "[package]\nname = \"fixture-owner\"\nversion = \"0.1.0\"\nedition = \"2021\"\nautotests = false\n\n[[test]]\nname = \"ui_runner\"\npath = \"tests/ui_runner.rs\"\n",
    );
    workspace.write("owner/src/lib.rs", "pub fn production() {}\n");
    workspace.write(
        "owner/tests/hidden.rs",
        "#[test]\nfn hidden_integration_proof() { assert!(true); }\n",
    );
    workspace.write(
        "owner/tests/ui/ignored_boundary.rs",
        "fn main() { let _: MissingAuthority = unreachable!(); }\n",
    );
    workspace.write(
        "owner/tests/ui_runner.rs",
        "#[path = \"ui/parser_support.rs\"]\nmod parser_support;\n\n#[test]\nfn runner_uses_parser_support() { assert_eq!(parser_support::parse(), 7); }\n",
    );
    workspace.write(
        "owner/tests/ui/parser_support.rs",
        "pub fn parse() -> usize { 7 }\n",
    );

    let discovered = discover_workspace(workspace.root(), false).unwrap();
    let hidden = discovered
        .inventory()
        .cases
        .iter()
        .find(|case| case.identity.case_name == "hidden_integration_proof")
        .expect("unregistered integration test remains visible to discovery");
    assert_eq!(hidden.current_invocation, "unregistered");
    assert!(hidden.target_identity.is_none());

    let ignored_ui = discovered
        .inventory()
        .cases
        .iter()
        .find(|case| case.source_path.ends_with("/tests/ui/ignored_boundary.rs"))
        .expect("unregistered UI fixture remains visible to discovery");
    assert_eq!(ignored_ui.kind, CaseKind::UiFixture);
    assert_eq!(ignored_ui.registration_authority, "unregistered");
    assert!(discovered
        .inventory()
        .cases
        .iter()
        .all(|case| !case.source_path.ends_with("/tests/ui/parser_support.rs")));
}

#[test]
fn assertion_operand_changes_alter_the_sealed_behavior_fingerprint() {
    let workspace = ScratchCargoWorkspace::new("assertion-fingerprint");
    workspace.write(
        "Cargo.toml",
        "[workspace]\nmembers = [\"owner\"]\nresolver = \"2\"\n",
    );
    workspace.write(
        "owner/Cargo.toml",
        "[package]\nname = \"fixture-owner\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    );
    workspace.write(
        "owner/src/lib.rs",
        "#[test]\nfn exact_contract() { assert_eq!(observed(), 7); }\nfn observed() -> usize { 7 }\n",
    );
    let before = discover_workspace(workspace.root(), false).unwrap();
    let before = &before.inventory().cases[0].behavior_fingerprint;

    workspace.write(
        "owner/src/lib.rs",
        "#[test]\nfn exact_contract() { assert_eq!(observed(), 8); }\nfn observed() -> usize { 7 }\n",
    );
    let after = discover_workspace(workspace.root(), false).unwrap();
    let after = &after.inventory().cases[0].behavior_fingerprint;

    assert_ne!(before, after);
}
