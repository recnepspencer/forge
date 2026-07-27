use std::{path::Path, process::Command};

use serde_json::Value;

use crate::workspace_root;

const FORBIDDEN_ORDINARY_FEATURES: &[&str] = &[
    "legacy-s2-models",
    "legacy-certification-models",
    "certification-test-authority",
    "certification-authority",
    "certification-test-support",
    "certification-world",
    "replay",
    "replay-authority",
];

const CANONICAL_POOL_OWNERS: &[&str] = &[
    "crates/worth-store/Cargo.toml",
    "crates/worth-store-io-scheduler/Cargo.toml",
];

const PHASE_8_LEGACY_POOL_OWNERS: &[&str] = &[
    "crates/worth-store-blob-chunks/Cargo.toml",
    "crates/worth-store-maintenance/Cargo.toml",
    "crates/worth-store-physical-integrity/Cargo.toml",
    "crates/worth-store-physical-isolation/Cargo.toml",
    "crates/worth-store-recovery-physics/Cargo.toml",
    "crates/worth-store-test-support/Cargo.toml",
];

const CERTIFICATION_POOL_OWNERS: &[&str] = &[
    "crates/worth-store-certification/Cargo.toml",
    "crates/worth-store-physical-certification/Cargo.toml",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ConsumerDisposition {
    CanonicalPhysicalOwner,
    Phase8LegacyPhysicalOwner,
    CertificationOwner,
    OrdinaryConsumer,
}

#[test]
fn every_ordinary_workspace_product_graph_excludes_certification_legacy_and_replay() {
    let metadata = cargo_metadata();
    let (ordinary_products, certification_products) =
        workspace_product_names(&metadata).expect("classify workspace products");
    assert_eq!(
        certification_products,
        [
            "worth-store-certification",
            "worth-store-physical-certification"
        ],
        "the certification-root exclusion must remain exact"
    );
    for required in [
        "store-test-runner",
        "worth-store",
        "worth-store-operations",
        "worth-store-reclaim-policy",
        "worth-store-tiering",
        "worth-store-wal",
    ] {
        assert!(
            ordinary_products.iter().any(|product| product == required),
            "ordinary workspace product denominator omitted `{required}`"
        );
    }

    let tree = cargo_tree(&certification_products);
    inspect_feature_tree("ordinary workspace", &tree)
        .unwrap_or_else(|denial| panic!("{denial}\n{tree}"));
}

#[test]
fn ordinary_consumers_cannot_acquire_direct_pool_or_certification_authority() {
    let metadata = cargo_metadata();
    let ledger = std::fs::read_to_string(
        workspace_root()
            .join("../../_docs/worth-store/physical-reconstruction-c6-removal-ledger.csv"),
    )
    .expect("read C.6 removal ledger");
    for package in metadata["packages"]
        .as_array()
        .expect("Cargo metadata packages must be an array")
    {
        inspect_package(package, &ledger).unwrap_or_else(|denial| panic!("{denial}"));
    }
}

#[test]
fn ordinary_graph_gate_rejects_direct_pool_and_feature_mutants() {
    let metadata = cargo_metadata();
    let (ordinary_products, _) =
        workspace_product_names(&metadata).expect("classify workspace products");
    for product in ordinary_products {
        for forbidden in FORBIDDEN_ORDINARY_FEATURES {
            let mutant = format!("{product} v0.0.0 [{forbidden}]");
            inspect_feature_tree(&product, &mutant)
                .expect_err("forbidden activation in an ordinary root must be rejected");
        }
    }

    inspect_direct_pool_edge(
        Path::new("crates/worth-store-authority/Cargo.toml"),
        ConsumerDisposition::OrdinaryConsumer,
        false,
    )
    .expect_err("a Part II consumer must not acquire the pool");
    inspect_direct_pool_edge(
        Path::new("crates/worth-store-physical-integrity/Cargo.toml"),
        ConsumerDisposition::Phase8LegacyPhysicalOwner,
        false,
    )
    .expect_err("a legacy exception without its removal-ledger row must be rejected");
    inspect_direct_pool_edge(
        Path::new("crates/worth-store-physical-integrity/Cargo.toml"),
        ConsumerDisposition::Phase8LegacyPhysicalOwner,
        true,
    )
    .expect("a ledger-bound Phase 8 exception remains explicit until its assigned phase");

    inspect_dependency_features(
        Path::new("crates/worth-store-modes/Cargo.toml"),
        ConsumerDisposition::OrdinaryConsumer,
        &["certification-authority"],
    )
    .expect_err("an ordinary dependency cannot activate certification authority");

    inspect_dependency_declaration(
        Path::new("crates/worth-store-modes/Cargo.toml"),
        ConsumerDisposition::OrdinaryConsumer,
        "worth-store-buffer-pool",
        true,
        &[],
        false,
    )
    .expect_err("an optional direct pool edge must not bypass ordinary classification");
    inspect_dependency_declaration(
        Path::new("crates/worth-store-test-support/Cargo.toml"),
        ConsumerDisposition::Phase8LegacyPhysicalOwner,
        "worth-store-buffer-pool",
        true,
        &[],
        true,
    )
    .expect("a ledger-bound Phase 8 optional edge remains explicit until Phase 8");
}

#[test]
fn certification_direct_pool_edge_requires_an_exact_phase_eight_row() {
    inspect_direct_pool_edge(
        Path::new("crates/worth-store-certification/Cargo.toml"),
        ConsumerDisposition::CertificationOwner,
        false,
    )
    .expect_err("certification cannot receive a broad direct-pool exemption");
    inspect_direct_pool_edge(
        Path::new("crates/worth-store-certification/Cargo.toml"),
        ConsumerDisposition::CertificationOwner,
        true,
    )
    .expect("a certification edge remains visible only through its exact Phase 8 row");
}

fn inspect_package(package: &Value, ledger: &str) -> Result<(), String> {
    let manifest = manifest_relative_path(package)?;
    let disposition = disposition(&manifest);
    let dependencies = package["dependencies"]
        .as_array()
        .ok_or_else(|| format!("{} dependencies are not an array", manifest.display()))?;
    for dependency in dependencies {
        let ordinary = dependency["kind"].is_null() || dependency["kind"] == "build";
        if !ordinary {
            continue;
        }
        let optional = dependency["optional"].as_bool().unwrap_or(false);
        let name = dependency["name"]
            .as_str()
            .ok_or_else(|| format!("{} has an unnamed dependency", manifest.display()))?;
        let features = dependency["features"]
            .as_array()
            .ok_or_else(|| {
                format!(
                    "{} dependency features are not an array",
                    manifest.display()
                )
            })?
            .iter()
            .filter_map(Value::as_str)
            .collect::<Vec<_>>();
        let ledger_has_entry = ledger.lines().any(|line| {
            line.starts_with(&manifest.to_string_lossy().replace('\\', "/"))
                && line.contains(",phase-8,")
        });
        inspect_dependency_declaration(
            &manifest,
            disposition,
            name,
            optional,
            &features,
            ledger_has_entry,
        )?;
    }
    Ok(())
}

fn inspect_dependency_declaration(
    manifest: &Path,
    disposition: ConsumerDisposition,
    name: &str,
    optional: bool,
    features: &[&str],
    ledger_has_entry: bool,
) -> Result<(), String> {
    if name == "worth-store-buffer-pool" {
        inspect_direct_pool_edge(manifest, disposition, ledger_has_entry)?;
    }
    if optional {
        return Ok(());
    }
    inspect_dependency_features(manifest, disposition, features)
}

fn inspect_dependency_features(
    manifest: &Path,
    disposition: ConsumerDisposition,
    features: &[&str],
) -> Result<(), String> {
    if disposition == ConsumerDisposition::CertificationOwner {
        return Ok(());
    }
    for forbidden in FORBIDDEN_ORDINARY_FEATURES {
        if features.iter().any(|feature| feature == forbidden) {
            return Err(format!(
                "Phase 7 ordinary graph: {} activates forbidden feature `{forbidden}`",
                manifest.display()
            ));
        }
    }
    Ok(())
}

fn inspect_direct_pool_edge(
    manifest: &Path,
    disposition: ConsumerDisposition,
    ledger_has_entry: bool,
) -> Result<(), String> {
    match disposition {
        ConsumerDisposition::CanonicalPhysicalOwner => Ok(()),
        ConsumerDisposition::CertificationOwner if ledger_has_entry => Ok(()),
        ConsumerDisposition::CertificationOwner => Err(format!(
            "Phase 8 certification consumer {} imports buffer-pool authority without an exact removal row",
            manifest.display()
        )),
        ConsumerDisposition::Phase8LegacyPhysicalOwner if ledger_has_entry => Ok(()),
        ConsumerDisposition::Phase8LegacyPhysicalOwner => Err(format!(
            "Phase 7 ordinary graph: legacy pool owner {} is not bound to a Phase 8 removal row",
            manifest.display()
        )),
        ConsumerDisposition::OrdinaryConsumer => Err(format!(
            "Phase 7 ordinary graph: unadmitted consumer {} imports buffer-pool authority",
            manifest.display()
        )),
    }
}

fn disposition(manifest: &Path) -> ConsumerDisposition {
    let manifest = manifest.to_string_lossy().replace('\\', "/");
    if CANONICAL_POOL_OWNERS.contains(&manifest.as_str()) {
        ConsumerDisposition::CanonicalPhysicalOwner
    } else if PHASE_8_LEGACY_POOL_OWNERS.contains(&manifest.as_str()) {
        ConsumerDisposition::Phase8LegacyPhysicalOwner
    } else if CERTIFICATION_POOL_OWNERS.contains(&manifest.as_str()) {
        ConsumerDisposition::CertificationOwner
    } else {
        ConsumerDisposition::OrdinaryConsumer
    }
}

fn inspect_feature_tree(label: &str, tree: &str) -> Result<(), String> {
    for forbidden in FORBIDDEN_ORDINARY_FEATURES {
        if tree.lines().any(|line| {
            line.rsplit_once('[')
                .and_then(|(_, features)| features.strip_suffix(']'))
                .is_some_and(|features| features.split(',').any(|item| item.trim() == *forbidden))
        }) {
            return Err(format!("Phase 7 {label} graph activated `{forbidden}`"));
        }
    }
    Ok(())
}

fn workspace_product_names(metadata: &Value) -> Result<(Vec<String>, Vec<String>), String> {
    let packages = metadata["packages"]
        .as_array()
        .ok_or_else(|| "Cargo metadata packages are not an array".to_owned())?;
    let mut ordinary = Vec::new();
    let mut certification = Vec::new();
    for package in packages {
        let manifest = manifest_relative_path(package)?;
        let name = package["name"]
            .as_str()
            .ok_or_else(|| format!("{} has no package name", manifest.display()))?
            .to_owned();
        if disposition(&manifest) == ConsumerDisposition::CertificationOwner {
            certification.push(name);
        } else {
            ordinary.push(name);
        }
    }
    ordinary.sort_unstable();
    certification.sort_unstable();
    ordinary.dedup();
    certification.dedup();
    if ordinary.len() + certification.len() != packages.len() {
        return Err("workspace product classification lost or duplicated a package".to_owned());
    }
    Ok((ordinary, certification))
}

fn cargo_tree(certification_products: &[String]) -> String {
    let mut command = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()));
    command.current_dir(workspace_root()).args([
        "tree",
        "--manifest-path",
        "Cargo.toml",
        "--workspace",
        "-e",
        "normal,build",
        "-f",
        "{p} [{f}]",
    ]);
    for product in certification_products {
        command.args(["--exclude", product]);
    }
    let output = command
        .output()
        .expect("run ordinary workspace feature-tree audit");
    assert!(
        output.status.success(),
        "ordinary workspace feature-tree audit failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).expect("cargo tree output must be UTF-8")
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

fn manifest_relative_path(package: &Value) -> Result<std::path::PathBuf, String> {
    let absolute = package["manifest_path"]
        .as_str()
        .ok_or_else(|| "Cargo package has no manifest path".to_owned())?;
    Path::new(absolute)
        .strip_prefix(workspace_root())
        .map(Path::to_path_buf)
        .map_err(|_| format!("manifest `{absolute}` is outside the Store workspace"))
}
