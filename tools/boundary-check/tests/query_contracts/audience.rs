//! Production-binary proofs for the Query audience dependency matrix.

use super::query_audience_fixture::{
    base_config, run_boundary_check, unique_temp_root, write_file, write_query_stubs,
    write_root_shell, write_subworkspace_crate,
};
use std::fs;

#[test]
fn entry_direct_engine_is_denied_naming_decl_and_host() {
    let root = unique_temp_root("entry-engine");
    let _ = fs::remove_dir_all(&root);
    write_query_stubs(&root);
    write_subworkspace_crate(
        &root,
        "cad/workspaces/worth-entry",
        "worth-entry",
        "worth-entry-",
        "worth-entry-adoption",
        "worth-entry-adoption",
        r#"worth-query = { path = "../../../../../vendor/worth-query" }"#,
    );
    write_root_shell(
        &root,
        &base_config(
            r#"[[subworkspaces]]
path = "cad/workspaces/worth-entry"
allowed_crate_prefixes = ["worth-entry-"]
member_lane = "crates/*"
"#,
            r#"[[born_crates]]
path = "cad/workspaces/worth-entry/crates/worth-entry-adoption"
package = "worth-entry-adoption"
"#,
            r#"[[naming.reserved_domains]]
tier = "worth"
band = "entry"
domains = ["adoption"]
"#,
            r#"[[rule_contracts.band_rules]]
source_band = "entry"
allowed_target_bands = ["schema", "resolver", "derived"]
"#,
        ),
    );

    let (ok, output) = run_boundary_check(&root);
    let _ = fs::remove_dir_all(&root);
    assert!(!ok, "entry direct engine must fail:\n{output}");
    assert!(
        output.contains("BC3001_DIRECT_QUERY_ENGINE"),
        "expected BC3001, got:\n{output}"
    );
    assert!(
        output.contains("worth-entry-adoption"),
        "expected subject package, got:\n{output}"
    );
    assert!(
        output.contains("worth-query-decl") && output.contains("worth-query-host"),
        "expected decl and host named, got:\n{output}"
    );
}

#[test]
fn schema_decl_import_is_denied() {
    let root = unique_temp_root("schema-decl");
    let _ = fs::remove_dir_all(&root);
    write_query_stubs(&root);
    write_subworkspace_crate(
        &root,
        "cad/workspaces/worth-contracts",
        "worth-contracts",
        "worth-schema-",
        "worth-schema-core",
        "worth-schema-core",
        r#"worth-query-decl = { path = "../../../../../vendor/worth-query-decl" }"#,
    );
    write_root_shell(
        &root,
        &base_config(
            r#"[[subworkspaces]]
path = "cad/workspaces/worth-contracts"
allowed_crate_prefixes = ["worth-schema-"]
member_lane = "crates/*"
"#,
            r#"[[born_crates]]
path = "cad/workspaces/worth-contracts/crates/worth-schema-core"
package = "worth-schema-core"
"#,
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

    let (ok, output) = run_boundary_check(&root);
    let _ = fs::remove_dir_all(&root);
    assert!(!ok, "schema -> decl must fail:\n{output}");
    assert!(
        output.contains("BC3002_WRONG_QUERY_AUDIENCE"),
        "expected BC3002, got:\n{output}"
    );
    assert!(
        output.contains("worth-query-decl"),
        "expected facade named, got:\n{output}"
    );
    assert!(
        output.contains("entry") || output.contains("allowed bands"),
        "expected matrix bands quoted, got:\n{output}"
    );
}

#[test]
fn derived_host_import_is_denied() {
    let root = unique_temp_root("derived-host");
    let _ = fs::remove_dir_all(&root);
    write_query_stubs(&root);
    write_subworkspace_crate(
        &root,
        "cad/workspaces/worth-derived",
        "worth-derived",
        "worth-derived-",
        "worth-derived-publication",
        "worth-derived-publication",
        r#"worth-query-host = { path = "../../../../../vendor/worth-query-host" }"#,
    );
    write_root_shell(
        &root,
        &base_config(
            r#"[[subworkspaces]]
path = "cad/workspaces/worth-derived"
allowed_crate_prefixes = ["worth-derived-"]
member_lane = "crates/*"
"#,
            r#"[[born_crates]]
path = "cad/workspaces/worth-derived/crates/worth-derived-publication"
package = "worth-derived-publication"
"#,
            r#"[[naming.reserved_domains]]
tier = "worth"
band = "derived"
domains = ["publication"]
"#,
            r#"[[rule_contracts.band_rules]]
source_band = "derived"
allowed_target_bands = ["schema", "solver"]
"#,
        ),
    );

    let (ok, output) = run_boundary_check(&root);
    let _ = fs::remove_dir_all(&root);
    assert!(!ok, "derived -> host must fail:\n{output}");
    assert!(
        output.contains("BC3002_WRONG_QUERY_AUDIENCE"),
        "expected BC3002, got:\n{output}"
    );
    assert!(
        output.contains("worth-query-host"),
        "expected host facade named, got:\n{output}"
    );
}

#[test]
fn entry_replay_import_is_denied_as_cert_only() {
    let root = unique_temp_root("entry-replay");
    let _ = fs::remove_dir_all(&root);
    write_query_stubs(&root);
    write_subworkspace_crate(
        &root,
        "cad/workspaces/worth-entry",
        "worth-entry",
        "worth-entry-",
        "worth-entry-adoption",
        "worth-entry-adoption",
        r#"worth-query-replay = { path = "../../../../../vendor/worth-query-replay" }"#,
    );
    write_root_shell(
        &root,
        &base_config(
            r#"[[subworkspaces]]
path = "cad/workspaces/worth-entry"
allowed_crate_prefixes = ["worth-entry-"]
member_lane = "crates/*"
"#,
            r#"[[born_crates]]
path = "cad/workspaces/worth-entry/crates/worth-entry-adoption"
package = "worth-entry-adoption"
"#,
            r#"[[naming.reserved_domains]]
tier = "worth"
band = "entry"
domains = ["adoption"]
"#,
            r#"[[rule_contracts.band_rules]]
source_band = "entry"
allowed_target_bands = ["schema", "resolver", "derived"]
"#,
        ),
    );

    let (ok, output) = run_boundary_check(&root);
    let _ = fs::remove_dir_all(&root);
    assert!(!ok, "entry -> replay must fail:\n{output}");
    assert!(
        output.contains("BC3002_WRONG_QUERY_AUDIENCE"),
        "expected BC3002, got:\n{output}"
    );
    assert!(
        output.contains("worth-query-replay") && output.contains("cert"),
        "expected cert-only replay matrix, got:\n{output}"
    );
}

#[test]
fn entry_decl_and_host_are_accepted() {
    let root = unique_temp_root("entry-legal");
    let _ = fs::remove_dir_all(&root);
    write_query_stubs(&root);
    write_subworkspace_crate(
        &root,
        "cad/workspaces/worth-entry",
        "worth-entry",
        "worth-entry-",
        "worth-entry-adoption",
        "worth-entry-adoption",
        r#"worth-query-decl = { path = "../../../../../vendor/worth-query-decl" }
worth-query-host = { path = "../../../../../vendor/worth-query-host" }"#,
    );
    write_root_shell(
        &root,
        &base_config(
            r#"[[subworkspaces]]
path = "cad/workspaces/worth-entry"
allowed_crate_prefixes = ["worth-entry-"]
member_lane = "crates/*"
"#,
            r#"[[born_crates]]
path = "cad/workspaces/worth-entry/crates/worth-entry-adoption"
package = "worth-entry-adoption"
"#,
            r#"[[naming.reserved_domains]]
tier = "worth"
band = "entry"
domains = ["adoption"]
"#,
            r#"[[rule_contracts.band_rules]]
source_band = "entry"
allowed_target_bands = ["schema", "resolver", "derived"]
"#,
        ),
    );

    let (ok, output) = run_boundary_check(&root);
    let _ = fs::remove_dir_all(&root);
    assert!(
        ok,
        "entry -> decl+host must pass query audience law, got:\n{output}"
    );
}

#[test]
fn cert_decl_host_and_replay_are_accepted() {
    let root = unique_temp_root("cert-legal");
    let _ = fs::remove_dir_all(&root);
    write_query_stubs(&root);
    write_subworkspace_crate(
        &root,
        "cad/workspaces/worth-certification",
        "worth-certification",
        "worthy-cert-",
        "worthy-cert-replay",
        "worthy-cert-replay",
        r#"worth-query-decl = { path = "../../../../../vendor/worth-query-decl" }
worth-query-host = { path = "../../../../../vendor/worth-query-host" }
worth-query-replay = { path = "../../../../../vendor/worth-query-replay" }"#,
    );
    write_root_shell(
        &root,
        &base_config(
            r#"[[subworkspaces]]
path = "cad/workspaces/worth-certification"
allowed_crate_prefixes = ["worthy-cert-"]
member_lane = "crates/*"
"#,
            r#"[[born_crates]]
path = "cad/workspaces/worth-certification/crates/worthy-cert-replay"
package = "worthy-cert-replay"
"#,
            r#"[[naming.reserved_domains]]
tier = "worthy"
band = "cert"
domains = ["replay"]
"#,
            r#"[[rule_contracts.band_rules]]
source_band = "cert"
allowed_target_bands = []
"#,
        ),
    );

    let (ok, output) = run_boundary_check(&root);
    let _ = fs::remove_dir_all(&root);
    assert!(ok, "cert -> decl+host+replay must pass, got:\n{output}");
}

#[test]
fn renamed_engine_dependency_package_identity_is_denied() {
    let root = unique_temp_root("renamed-engine");
    let _ = fs::remove_dir_all(&root);
    write_query_stubs(&root);
    write_subworkspace_crate(
        &root,
        "cad/workspaces/worth-entry",
        "worth-entry",
        "worth-entry-",
        "worth-entry-adoption",
        "worth-entry-adoption",
        // Dependency key is not the package name; package identity is worth-query.
        r#"query_engine = { package = "worth-query", path = "../../../../../vendor/worth-query" }"#,
    );
    write_file(
        &root,
        "cad/workspaces/worth-entry/crates/worth-entry-adoption/src/lib.rs",
        "fn renamed_key_is_source_visible() { let _ = query_engine::facade::CanonicalQueryArtifact; }\n",
    );
    write_root_shell(
        &root,
        &base_config(
            r#"[[subworkspaces]]
path = "cad/workspaces/worth-entry"
allowed_crate_prefixes = ["worth-entry-"]
member_lane = "crates/*"
"#,
            r#"[[born_crates]]
path = "cad/workspaces/worth-entry/crates/worth-entry-adoption"
package = "worth-entry-adoption"
"#,
            r#"[[naming.reserved_domains]]
tier = "worth"
band = "entry"
domains = ["adoption"]
"#,
            r#"[[rule_contracts.band_rules]]
source_band = "entry"
allowed_target_bands = ["schema", "resolver", "derived"]
"#,
        ),
    );

    let (ok, output) = run_boundary_check(&root);
    let _ = fs::remove_dir_all(&root);
    assert!(!ok, "renamed engine dep must fail:\n{output}");
    assert!(
        output.contains("BC3001_DIRECT_QUERY_ENGINE"),
        "expected BC3001 on package identity, got:\n{output}"
    );
}
