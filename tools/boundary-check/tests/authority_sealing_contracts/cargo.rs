//! Production-binary matrix: complete fail-closed Cargo dependency authority.

use super::authority_sealing_fixture::{
    dep_describe_only, dep_item_macro_export_generation, dep_opaque_attr_private_item,
    dep_opaque_attr_private_module, dep_opaque_attr_public_item, dep_private_custom_derive,
    dep_renames_marker_to_gate, dep_unresolved_module_root, entry_cargo_qualified_dep_gate,
    entry_cargo_use_dep_gate, entry_non_path_registry_host, entry_use_describe,
    AuthoritySealingTestRepository,
};

const DEP: &str = "worth-schema-authgate";
/// Real crates.io package so cargo metadata succeeds; sealing must still fail closed.
const REGISTRY_DEP: &str = "cfg-if";
const REGISTRY_VERSION: &str = "1";

fn assert_denial(label: &str, ok: bool, output: &str) {
    assert!(!ok, "{label} must fail:\n{output}");
    assert!(
        output.contains("BC7001_AUTHORITY_SEALING"),
        "{label}: expected BC7001, got:\n{output}"
    );
    assert!(
        output.contains("Authority sealing law"),
        "{label}: expected law quote, got:\n{output}"
    );
}

fn assert_pass(label: &str, ok: bool, output: &str) {
    assert!(ok, "{label} must pass:\n{output}");
    assert!(
        !output.contains("BC7001_AUTHORITY_SEALING"),
        "{label}: unexpected sealing diagnostic:\n{output}"
    );
}

#[test]
fn workspace_inherited_dep_renamed_authority_is_denied() {
    let repo = AuthoritySealingTestRepository::create("hostile-ws-inherited");
    repo.assemble_with_workspace_inherited_dep(
        entry_cargo_use_dep_gate(),
        DEP,
        dep_renames_marker_to_gate(),
    );
    let (ok, output) = repo.run_boundary_check();
    repo.cleanup();
    assert_denial("hostile-ws-inherited", ok, &output);
}

#[test]
fn workspace_inherited_qualified_dep_gate_is_denied() {
    let repo = AuthoritySealingTestRepository::create("hostile-ws-qualified");
    repo.assemble_with_workspace_inherited_dep(
        entry_cargo_qualified_dep_gate(),
        DEP,
        dep_renames_marker_to_gate(),
    );
    let (ok, output) = repo.run_boundary_check();
    repo.cleanup();
    assert_denial("hostile-ws-qualified", ok, &output);
}

#[test]
fn target_specific_dep_renamed_authority_is_denied() {
    let repo = AuthoritySealingTestRepository::create("hostile-target-dep");
    repo.assemble_with_target_specific_dep(
        entry_cargo_use_dep_gate(),
        DEP,
        dep_renames_marker_to_gate(),
    );
    let (ok, output) = repo.run_boundary_check();
    repo.cleanup();
    assert_denial("hostile-target-dep", ok, &output);
}

#[test]
fn dep_item_macro_export_generation_is_denied() {
    let repo = AuthoritySealingTestRepository::create("hostile-dep-item-macro");
    repo.assemble_with_external_path_dependency(
        entry_cargo_use_dep_gate(),
        DEP,
        dep_item_macro_export_generation(),
    );
    let (ok, output) = repo.run_boundary_check();
    repo.cleanup();
    assert_denial("hostile-dep-item-macro", ok, &output);
}

#[test]
fn dep_opaque_attr_export_generation_is_denied() {
    let repo = AuthoritySealingTestRepository::create("hostile-dep-opaque-attr");
    repo.assemble_with_external_path_dependency(
        entry_cargo_use_dep_gate(),
        DEP,
        dep_opaque_attr_public_item(),
    );
    let (ok, output) = repo.run_boundary_check();
    repo.cleanup();
    assert_denial("hostile-dep-opaque-attr", ok, &output);
}

#[test]
fn dep_unresolved_module_resolution_is_denied() {
    let repo = AuthoritySealingTestRepository::create("hostile-dep-unresolved");
    repo.assemble_with_external_path_dependency(
        entry_cargo_use_dep_gate(),
        DEP,
        dep_unresolved_module_root(),
    );
    let (ok, output) = repo.run_boundary_check();
    repo.cleanup();
    assert_denial("hostile-dep-unresolved", ok, &output);
}

#[test]
fn version_only_non_path_dep_is_denied() {
    let repo = AuthoritySealingTestRepository::create("hostile-version-dep");
    repo.assemble_with_version_only_dep(
        entry_non_path_registry_host(),
        REGISTRY_DEP,
        REGISTRY_VERSION,
    );
    let (ok, output) = repo.run_boundary_check();
    repo.cleanup();
    assert_denial("hostile-version-dep", ok, &output);
    assert!(
        output.contains("non-path source") || output.contains("version"),
        "hostile-version-dep: expected non-path diagnostic, got:\n{output}"
    );
}

#[test]
fn registry_table_non_path_dep_is_denied() {
    let repo = AuthoritySealingTestRepository::create("hostile-registry-table");
    repo.assemble_with_registry_table_dep(
        entry_non_path_registry_host(),
        REGISTRY_DEP,
        REGISTRY_VERSION,
    );
    let (ok, output) = repo.run_boundary_check();
    repo.cleanup();
    assert_denial("hostile-registry-table", ok, &output);
    assert!(
        output.contains("non-path source") || output.contains("version"),
        "hostile-registry-table: expected non-path diagnostic, got:\n{output}"
    );
}

#[test]
fn dep_opaque_attr_on_private_item_is_denied() {
    let repo = AuthoritySealingTestRepository::create("hostile-dep-private-attr");
    repo.assemble_with_external_path_dependency(
        entry_cargo_use_dep_gate(),
        DEP,
        dep_opaque_attr_private_item(),
    );
    let (ok, output) = repo.run_boundary_check();
    repo.cleanup();
    assert_denial("hostile-dep-private-attr", ok, &output);
    assert!(
        output.contains("private") || output.contains("opaque"),
        "hostile-dep-private-attr: expected private opaque fence, got:\n{output}"
    );
}

#[test]
fn dep_opaque_attr_on_private_module_is_denied() {
    let repo = AuthoritySealingTestRepository::create("hostile-dep-private-mod");
    repo.assemble_with_external_path_dependency(
        entry_cargo_use_dep_gate(),
        DEP,
        dep_opaque_attr_private_module(),
    );
    let (ok, output) = repo.run_boundary_check();
    repo.cleanup();
    assert_denial("hostile-dep-private-mod", ok, &output);
}

#[test]
fn dep_private_custom_derive_is_denied() {
    let repo = AuthoritySealingTestRepository::create("hostile-dep-private-derive");
    repo.assemble_with_external_path_dependency(
        entry_cargo_use_dep_gate(),
        DEP,
        dep_private_custom_derive(),
    );
    let (ok, output) = repo.run_boundary_check();
    repo.cleanup();
    assert_denial("hostile-dep-private-derive", ok, &output);
}

#[test]
fn workspace_inherited_non_authority_dep_passes() {
    let repo = AuthoritySealingTestRepository::create("legal-ws-describe");
    repo.assemble_with_workspace_inherited_dep(entry_use_describe(), DEP, dep_describe_only());
    let (ok, output) = repo.run_boundary_check();
    repo.cleanup();
    assert_pass("legal-ws-describe", ok, &output);
}

#[test]
fn target_specific_non_authority_dep_passes() {
    let repo = AuthoritySealingTestRepository::create("legal-target-describe");
    repo.assemble_with_target_specific_dep(entry_use_describe(), DEP, dep_describe_only());
    let (ok, output) = repo.run_boundary_check();
    repo.cleanup();
    assert_pass("legal-target-describe", ok, &output);
}
