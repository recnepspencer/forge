//! Entry-band consumer equivalence: real decl+host facades, engine type identity.
//!
//! This specimen depends on the workspace `worth-query-decl` and
//! `worth-query-host` packages (not vendor stubs) and proves the exported types
//! are nominally identical to their `worth_query::facade` items. Direct engine use is
//! confined to this equivalence specimen; production denial of direct engine
//! edges remains in `query_audience_contract`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("tools")
        .parent()
        .expect("workspace root")
        .to_path_buf()
}

fn unique_temp_root() -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "worth-entry-audience-equivalence-{}-{nanos}",
        std::process::id()
    ))
}

fn write_file(root: &Path, relative: &str, contents: &str) {
    let path = root.join(relative);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parents");
    }
    fs::write(path, contents).expect("write");
}

#[test]
fn entry_band_consumer_sees_identical_decl_and_host_engine_types() {
    let root = unique_temp_root();
    let _ = fs::remove_dir_all(&root);
    let workspace = workspace_root();
    let decl = workspace.join("crates/worth-query-decl");
    let host = workspace.join("crates/worth-query-host");
    let engine = workspace.join("crates/worth-query");
    assert!(decl.is_dir(), "real worth-query-decl must exist");
    assert!(host.is_dir(), "real worth-query-host must exist");
    assert!(engine.is_dir(), "real worth-query must exist");

    // Entry-band package name: production consumers are worth-entry-*.
    write_file(
        &root,
        "Cargo.toml",
        &format!(
            r#"[package]
name = "worth-entry-audience-equivalence"
version = "0.1.0"
edition = "2021"
publish = false

[dependencies]
worth-query-decl = {{ path = {} }}
worth-query-host = {{ path = {} }}
# Equivalence specimen only: engine is referenced solely to prove nominal
# identity with the audience re-exports. Production governed crates must not
# take this edge (see query_audience_contract BC3001 proofs).
worth-query = {{ path = {} }}

[workspace]
"#,
            toml_path(&decl),
            toml_path(&host),
            toml_path(&engine),
        ),
    );
    write_file(
        &root,
        "src/lib.rs",
        r#"//! Entry-band ordinary caller: declaration + host audience imports.

use worth_query_decl::facade::CanonicalQueryArtifact;
use worth_query_host::facade::runtime::WorthQueryRuntime;

/// Nominal identity: decl re-export is the engine type (no wrapper).
pub fn retain_declaration_artifact(
    engine: worth_query::facade::foundation::CanonicalQueryArtifact,
) -> CanonicalQueryArtifact {
    engine
}

/// Nominal identity: host re-export is the engine type (no wrapper).
pub fn retain_host_runtime(
    engine: worth_query::facade::runtime::WorthQueryRuntime,
) -> WorthQueryRuntime {
    engine
}
"#,
    );

    let status = Command::new("cargo")
        .arg("check")
        .arg("--manifest-path")
        .arg(root.join("Cargo.toml"))
        .arg("--offline")
        .status()
        .expect("spawn cargo check");
    let _ = fs::remove_dir_all(&root);
    assert!(
        status.success(),
        "entry-band consumer against real decl+host must type-check"
    );
}

fn toml_path(path: &Path) -> String {
    let text = path
        .canonicalize()
        .unwrap_or_else(|_| path.to_path_buf())
        .to_string_lossy()
        .replace('\\', "/");
    // Strip Windows verbatim prefix for Cargo path deps when present.
    let trimmed = text.strip_prefix("//?/").unwrap_or(&text);
    format!("\"{trimmed}\"")
}
