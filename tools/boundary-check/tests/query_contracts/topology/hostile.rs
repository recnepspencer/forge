use super::super::query_audience_fixture::{
    base_config, run_boundary_check, unique_temp_root, write_file, write_query_stubs,
    write_root_shell,
};
use std::fs;

fn write_hostile_root(root: &std::path::Path) {
    let _ = fs::remove_dir_all(root);
    write_query_stubs(root);
    write_root_shell(
        root,
        &base_config(
            "",
            "",
            r#"[[naming.reserved_domains]]
tier = "worth"
band = "schema"
domains = ["core"]
"#,
            r#"[[rule_contracts.band_rules]]
source_band = "schema"
allowed_target_bands = []
"#,
        ),
    );
}

#[test]
fn hostile_extra_facade_dependency_is_denied() {
    let root = unique_temp_root("facade-extra-dep");
    write_hostile_root(&root);
    write_file(
        &root,
        "crates/worth-query-decl/Cargo.toml",
        r#"[package]
name = "worth-query-decl"
version = "0.1.0"
edition = "2021"

[dependencies]
worth-query = { path = "../worth-query" }
serde = "1"

[workspace]
"#,
    );

    let (ok, output) = run_boundary_check(&root);
    let _ = fs::remove_dir_all(&root);
    assert!(!ok, "extra facade dependency must fail:\n{output}");
    assert!(
        output.contains("BC3003_QUERY_AUDIENCE_FACADE_CONTRACT"),
        "expected BC3003, got:\n{output}"
    );
    assert!(
        output.contains("worth-query-decl") || output.contains("depend only on engine"),
        "expected engine-only guidance, got:\n{output}"
    );
}

#[test]
fn hostile_facade_behavior_item_is_denied() {
    let root = unique_temp_root("facade-behavior");
    write_hostile_root(&root);
    write_file(
        &root,
        "crates/worth-query-host/src/facade.rs",
        r#"pub use worth_query::facade::runtime;

pub fn wrapper() {}
"#,
    );

    let (ok, output) = run_boundary_check(&root);
    let _ = fs::remove_dir_all(&root);
    assert!(!ok, "behavior in facade.rs must fail:\n{output}");
    assert!(
        output.contains("BC3003_QUERY_AUDIENCE_FACADE_CONTRACT"),
        "expected BC3003, got:\n{output}"
    );
    assert!(
        output.contains("re-export") || output.contains("wrappers") || output.contains("functions"),
        "expected re-export-only message, got:\n{output}"
    );
}

#[test]
fn hostile_cross_audience_reexport_is_denied() {
    let root = unique_temp_root("facade-cross");
    write_hostile_root(&root);
    write_file(
        &root,
        "crates/worth-query-decl/src/facade.rs",
        r#"pub use worth_query::facade::foundation::CanonicalQueryArtifact;
pub use worth_query_host::facade::runtime;
"#,
    );

    let (ok, output) = run_boundary_check(&root);
    let _ = fs::remove_dir_all(&root);
    assert!(!ok, "cross-audience re-export must fail:\n{output}");
    assert!(
        output.contains("BC3003_QUERY_AUDIENCE_FACADE_CONTRACT"),
        "expected BC3003, got:\n{output}"
    );
}
