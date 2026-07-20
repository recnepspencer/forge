//! Structural matrix and leaf-facade topology proofs against the real tree
//! and production-binary hostile mutations.

use super::query_audience_fixture::{
    base_config, run_boundary_check, unique_temp_root, write_file, write_query_stubs,
    write_root_shell,
};
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("tools")
        .parent()
        .expect("workspace root")
        .to_path_buf()
}

#[derive(Debug, Deserialize)]
struct Road1ConfigSlice {
    rule_contracts: RuleContractsSlice,
}

#[derive(Debug, Deserialize)]
struct RuleContractsSlice {
    query_audience: QueryAudienceSlice,
}

#[derive(Debug, Deserialize)]
struct QueryAudienceSlice {
    workspace: String,
    engine_package: String,
    certification_package: String,
    audiences: Vec<AudienceSlice>,
}

#[derive(Debug, Deserialize)]
struct AudienceSlice {
    package: String,
    label: String,
    allowed_bands: Vec<String>,
    guidance: String,
}

#[test]
fn canonical_query_audience_matrix_is_exact() {
    let root = workspace_root();
    let text = fs::read_to_string(root.join("tools/boundary-check/config/road1.toml"))
        .expect("read road1.toml");
    let config: Road1ConfigSlice = toml::from_str(&text).expect("parse road1.toml");
    let matrix = config.rule_contracts.query_audience;

    assert_eq!(matrix.workspace, "workspaces/worth-query");
    assert_eq!(matrix.engine_package, "worth-query");
    assert_eq!(matrix.certification_package, "worth-query-certification");
    assert_eq!(matrix.audiences.len(), 3, "exactly three audience rows");
    assert!(
        !text.contains("query_host_bands"),
        "retired query_host_bands must be absent"
    );

    let expected = [
        (
            "worth-query-decl",
            "declaration",
            vec!["entry", "cert"],
            "declaration artifacts and handles",
        ),
        (
            "worth-query-host",
            "host",
            vec!["entry", "cert"],
            "admission, lowering, and execution",
        ),
        (
            "worth-query-replay",
            "replay",
            vec!["cert"],
            "cert-only reconstruction and replay",
        ),
    ];

    for (index, (package, label, bands, guidance)) in expected.into_iter().enumerate() {
        let row = &matrix.audiences[index];
        assert_eq!(row.package, package);
        assert_eq!(row.label, label);
        assert_eq!(row.allowed_bands, bands);
        assert_eq!(row.guidance, guidance);
    }

    let naming = fs::read_to_string(root.join("cad/docs/worthy-foundations/NAMING.md"))
        .expect("read NAMING.md");
    for package in [
        "worth-query",
        "worth-query-decl",
        "worth-query-host",
        "worth-query-replay",
        "worth-query-certification",
    ] {
        assert!(
            naming.contains(&format!("`{package}`")),
            "NAMING.md must name {package}"
        );
    }
}

#[test]
fn real_audience_facades_depend_only_on_their_owned_authority() {
    let root = workspace_root();
    for (package, expected_dependency) in [
        ("worth-query-decl", "worth-query-declaration"),
        ("worth-query-host", "worth-query"),
        ("worth-query-replay", "worth-query"),
    ] {
        let manifest = root
            .join("workspaces/worth-query/crates")
            .join(package)
            .join("Cargo.toml");
        let output = Command::new("cargo")
            .arg("metadata")
            .arg("--format-version")
            .arg("1")
            .arg("--no-deps")
            .arg("--manifest-path")
            .arg(&manifest)
            .output()
            .expect("cargo metadata");
        assert!(
            output.status.success(),
            "metadata failed for {package}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let meta: serde_json::Value =
            serde_json::from_slice(&output.stdout).expect("parse metadata");
        let packages = meta
            .get("packages")
            .and_then(|value| value.as_array())
            .expect("packages array");
        let facade = packages
            .iter()
            .find(|pkg| pkg.get("name").and_then(|n| n.as_str()) == Some(package))
            .expect("facade package in metadata");
        let deps = facade
            .get("dependencies")
            .and_then(|value| value.as_array())
            .expect("dependencies");
        let normal_deps: Vec<&str> = deps
            .iter()
            .filter(|dep| dep.get("kind").and_then(|k| k.as_str()).is_none())
            .filter_map(|dep| dep.get("name").and_then(|n| n.as_str()))
            .collect();
        assert_eq!(
            normal_deps,
            [expected_dependency],
            "{package} must have only {expected_dependency} as a normal dependency, found {normal_deps:?}"
        );
    }
}

#[test]
fn production_boundary_check_accepts_real_leaf_facades() {
    let root = workspace_root();
    let output = Command::new(env!("CARGO_BIN_EXE_boundary-check"))
        .arg("--root")
        .arg(&root)
        .arg("--config")
        .arg("tools/boundary-check/config/road1.toml")
        .output()
        .expect("run boundary-check");
    assert!(
        output.status.success(),
        "real tree must pass facade contract: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn hostile_extra_facade_dependency_is_denied() {
    let root = unique_temp_root("facade-extra-dep");
    let _ = fs::remove_dir_all(&root);
    write_query_stubs(&root);
    write_root_shell(
        &root,
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
    // Second dependency breaks engine-only leaf law.
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
    let _ = fs::remove_dir_all(&root);
    write_query_stubs(&root);
    write_root_shell(
        &root,
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
    let _ = fs::remove_dir_all(&root);
    write_query_stubs(&root);
    write_root_shell(
        &root,
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

#[test]
fn agent_context_names_audience_exports_for_real_facades() {
    let root = workspace_root();
    for package in ["worth-query-decl", "worth-query-host", "worth-query-replay"] {
        let context = fs::read_to_string(
            root.join("workspaces/worth-query/crates")
                .join(package)
                .join("AGENT_CONTEXT.md"),
        )
        .unwrap_or_else(|_| panic!("missing AGENT_CONTEXT for {package}"));
        assert!(
            context.contains("framework/query-audience"),
            "{package} context must be framework audience"
        );
        assert!(
            context.contains("Facade exports:"),
            "{package} must list facade exports"
        );
        assert!(
            context.contains("worth-query"),
            "{package} must fence engine-only dependency"
        );
    }
    let decl = fs::read_to_string(
        root.join("workspaces/worth-query/crates/worth-query-decl/AGENT_CONTEXT.md"),
    )
    .unwrap();
    assert!(decl.contains("CanonicalQueryArtifact"));
    let host = fs::read_to_string(
        root.join("workspaces/worth-query/crates/worth-query-host/AGENT_CONTEXT.md"),
    )
    .unwrap();
    assert!(host.contains("domain"));
    assert!(host.contains("runtime"));
    let replay = fs::read_to_string(
        root.join("workspaces/worth-query/crates/worth-query-replay/AGENT_CONTEXT.md"),
    )
    .unwrap();
    assert!(replay.contains("ScopedReplayBasis"));

    let certification = fs::read_to_string(
        root.join("workspaces/worth-query/crates/worth-query-certification/AGENT_CONTEXT.md"),
    )
    .unwrap();
    assert!(certification.contains("framework/query-certification"));
    assert!(certification.contains("Cold leaf"));
}
