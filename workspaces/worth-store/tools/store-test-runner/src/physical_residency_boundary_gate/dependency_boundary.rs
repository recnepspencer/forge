use std::{path::Path, process::Command};

use serde_json::Value;

use super::workspace_source::{read, rust_sources};
use crate::workspace_root;

const BUFFER_POOL_SOURCE: &str = "crates/worth-store-buffer-pool/src";
const PHYSICAL_RESIDENCY_SOURCE: &str = "crates/worth-store-buffer-pool/src/physical_residency";
const FORBIDDEN_POOL_AUTHORITIES: &[&str] = &[
    "worth-signal",
    "worth_signal",
    "worth-proof",
    "worth_proof",
    "worth-foundational",
    "worth_foundational",
    "worth-store-aspect-native",
    "worth_store_aspect_native",
];
const FORBIDDEN_RESIDENCY_BACKEND_AUTHORITY: &[&str] = &[
    "worth_store_physical_backend",
    "CompletedArtifactRangeWrite",
];

const ORDINARY_MANIFESTS: &[&str] = &[
    "crates/worth-store/Cargo.toml",
    "crates/worth-store-blob-chunks/Cargo.toml",
    "crates/worth-store-maintenance/Cargo.toml",
    "crates/worth-store-test-support/Cargo.toml",
    "crates/worth-store-io-scheduler/Cargo.toml",
    "crates/worth-store-buffer-pool/Cargo.toml",
    "crates/worth-store-physical-backend/Cargo.toml",
    "crates/worth-store-physical-integrity/Cargo.toml",
    "crates/worth-store-recovery-physics/Cargo.toml",
];

#[test]
fn buffer_pool_metadata_has_no_direct_foreign_authority() {
    let metadata = cargo_metadata();
    let packages = metadata["packages"]
        .as_array()
        .expect("Cargo metadata packages must be an array");
    let pool = packages
        .iter()
        .find(|package| package["name"] == "worth-store-buffer-pool")
        .expect("buffer-pool package must exist");
    let dependencies = pool["dependencies"]
        .as_array()
        .expect("package dependencies must be an array");

    let forbidden = dependencies
        .iter()
        .filter_map(|dependency| dependency["name"].as_str())
        .filter(|name| {
            FORBIDDEN_POOL_AUTHORITIES
                .iter()
                .any(|forbidden| name == forbidden)
        })
        .collect::<Vec<_>>();
    assert!(
        forbidden.is_empty(),
        "buffer pool acquired direct foreign authority dependencies: {forbidden:?}"
    );
}

#[test]
fn buffer_pool_source_and_api_have_no_foreign_authority_types() {
    for source in rust_sources(&workspace_root().join(BUFFER_POOL_SOURCE))
        .expect("discover buffer-pool sources")
    {
        let text = read(&source).expect("read buffer-pool source");
        inspect_pool_source(&source, &text).unwrap_or_else(|denial| panic!("{denial}"));
    }
}

#[test]
fn physical_residency_source_and_api_have_no_backend_receipt_authority() {
    for source in rust_sources(&workspace_root().join(PHYSICAL_RESIDENCY_SOURCE))
        .expect("discover physical-residency sources")
    {
        let text = read(&source).expect("read physical-residency source");
        inspect_residency_source(&source, &text).unwrap_or_else(|denial| panic!("{denial}"));
    }
}

#[test]
fn ordinary_manifests_have_no_active_legacy_feature_edges() {
    for relative in ORDINARY_MANIFESTS {
        let path = workspace_root().join(relative);
        let manifest = read(&path).expect("read ordinary manifest");
        inspect_ordinary_manifest(&path, &manifest).unwrap_or_else(|denial| panic!("{denial}"));
    }
}

#[test]
fn dependency_gate_rejects_direct_authority_and_legacy_edge_mutants() {
    for authority in [
        "worth-signal",
        "worth-proof",
        "worth-foundational",
        "worth-store-aspect-native",
    ] {
        let manifest = format!("[dependencies]\n{authority}.workspace = true\n");
        let denial = inspect_pool_manifest(Path::new("Cargo.toml"), &manifest)
            .expect_err("direct foreign authority must be denied");
        assert!(denial.contains(authority), "wrong denial: {denial}");
    }

    let mutant = r#"
[dependencies]
legacy = { package = "worth-store-buffer-pool", workspace = true, features = ["legacy-s2-models"] }
"#;
    let denial = inspect_ordinary_manifest(Path::new("Cargo.toml"), mutant)
        .expect_err("active legacy feature edge must be denied");
    assert!(denial.contains("legacy-s2-models"));

    for mutant in [
        "use worth_store_physical_backend::ArtifactRangeWriteDurabilityRequirement;",
        "pub fn clean(receipt: CompletedArtifactRangeWrite) { drop(receipt); }",
    ] {
        let denial = inspect_residency_source(Path::new("lease/writeback.rs"), mutant)
            .expect_err("backend receipt authority must be denied from physical residency");
        assert!(denial.contains("backend receipt authority"));
    }
}

fn cargo_metadata() -> Value {
    let output = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()))
        .current_dir(workspace_root())
        .args(["metadata", "--format-version", "1", "--no-deps"])
        .output()
        .expect("run Cargo metadata");
    assert!(
        output.status.success(),
        "Cargo metadata failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).expect("Cargo metadata must be valid JSON")
}

fn inspect_pool_source(path: &Path, source: &str) -> Result<(), String> {
    for authority in FORBIDDEN_POOL_AUTHORITIES {
        if source.contains(authority) {
            return Err(format!(
                "physical residency boundary: buffer-pool source imports or exposes `{authority}` at {}",
                path.display()
            ));
        }
    }
    Ok(())
}

fn inspect_residency_source(path: &Path, source: &str) -> Result<(), String> {
    for authority in FORBIDDEN_RESIDENCY_BACKEND_AUTHORITY {
        if source.contains(authority) {
            return Err(format!(
                "physical residency boundary: backend receipt authority `{authority}` appears at {}",
                path.display()
            ));
        }
    }
    Ok(())
}

fn inspect_pool_manifest(path: &Path, manifest: &str) -> Result<(), String> {
    let dependencies = dependency_tables(manifest);
    for authority in FORBIDDEN_POOL_AUTHORITIES {
        if dependencies.contains(authority) {
            return Err(format!(
                "physical residency boundary: buffer-pool manifest has direct `{authority}` edge at {}",
                path.display()
            ));
        }
    }
    Ok(())
}

fn inspect_ordinary_manifest(path: &Path, manifest: &str) -> Result<(), String> {
    for line in dependency_tables(manifest).lines() {
        for feature in ["legacy-s2-models", "legacy-certification-models"] {
            if line.contains(feature) && !line.contains("optional = true") {
                return Err(format!(
                    "physical residency boundary: active `{feature}` edge at {}",
                    path.display()
                ));
            }
        }
    }
    Ok(())
}

fn dependency_tables(manifest: &str) -> String {
    let mut dependencies = String::new();
    let mut ordinary = false;
    for line in manifest.lines() {
        let trimmed = line.trim();
        if let Some(table) = trimmed
            .strip_prefix('[')
            .and_then(|table| table.strip_suffix(']'))
        {
            ordinary = table == "dependencies"
                || (table.starts_with("target.") && table.ends_with(".dependencies"));
        } else if ordinary {
            dependencies.push_str(line);
            dependencies.push('\n');
        }
    }
    dependencies
}
