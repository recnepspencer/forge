use std::collections::BTreeMap;
use std::process::Command;

use serde::Deserialize;

use crate::workspace_root;

const FORBIDDEN_ORDINARY_FEATURES: &[&str] = &[
    "certification-authority",
    "certification-test-authority",
    "certification-test-support",
    "certification-world",
    "replay",
    "replay-authority",
];

#[test]
fn workspace_has_no_phase_numbered_feature_authority() {
    let metadata = cargo_metadata().expect("read WORTH Store Cargo metadata");
    inspect_phase_numbered_features(&metadata).unwrap_or_else(|denial| panic!("{denial}"));
}

#[test]
fn ordinary_store_graph_excludes_certification_and_replay_authority() {
    let tree = ordinary_store_feature_tree().expect("read ordinary Store feature graph");
    inspect_ordinary_feature_tree(&tree).unwrap_or_else(|denial| panic!("{denial}\n{tree}"));
}

#[test]
fn feature_cutover_gate_rejects_phase_and_authority_mutants() {
    let phase_mutant = CargoMetadata {
        packages: vec![CargoPackage {
            name: "worth-store-mutant".to_owned(),
            features: BTreeMap::from([("phase99-shortcut".to_owned(), Vec::new())]),
            dependencies: Vec::new(),
        }],
    };
    inspect_phase_numbered_features(&phase_mutant)
        .expect_err("phase-numbered feature declaration must fail the cutover gate");

    let dependency_mutant = CargoMetadata {
        packages: vec![CargoPackage {
            name: "worth-store-mutant".to_owned(),
            features: BTreeMap::new(),
            dependencies: vec![CargoDependency {
                name: "worth-store-wal".to_owned(),
                features: vec!["phase98-direct-execution".to_owned()],
            }],
        }],
    };
    inspect_phase_numbered_features(&dependency_mutant)
        .expect_err("phase-numbered dependency activation must fail the cutover gate");

    for forbidden in FORBIDDEN_ORDINARY_FEATURES {
        let tree = format!("worth-store v0.0.0\nworth-store-wal v0.0.0 [{forbidden}]");
        inspect_ordinary_feature_tree(&tree)
            .expect_err("ordinary certification or replay authority must fail the cutover gate");
    }
}

fn cargo_metadata() -> Result<CargoMetadata, String> {
    let output = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()))
        .args(["metadata", "--format-version", "1", "--no-deps"])
        .current_dir(workspace_root())
        .output()
        .map_err(|error| format!("cannot start cargo metadata: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "cargo metadata failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    serde_json::from_slice(&output.stdout)
        .map_err(|error| format!("invalid Cargo metadata: {error}"))
}

fn ordinary_store_feature_tree() -> Result<String, String> {
    let output = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()))
        .args([
            "tree",
            "--manifest-path",
            "Cargo.toml",
            "-p",
            "worth-store",
            "-e",
            "normal,build",
            "-f",
            "{p} [{f}]",
        ])
        .current_dir(workspace_root())
        .output()
        .map_err(|error| format!("cannot start cargo tree: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "ordinary Store cargo tree failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    String::from_utf8(output.stdout)
        .map_err(|error| format!("ordinary Store cargo tree was not UTF-8: {error}"))
}

fn inspect_phase_numbered_features(metadata: &CargoMetadata) -> Result<(), String> {
    for package in &metadata.packages {
        for feature in package.features.keys() {
            if is_phase_numbered_feature(feature) {
                return Err(format!(
                    "package `{}` declares phase-numbered feature `{feature}`",
                    package.name
                ));
            }
        }
        for dependency in &package.dependencies {
            for feature in &dependency.features {
                if is_phase_numbered_feature(feature) {
                    return Err(format!(
                        "package `{}` activates phase-numbered feature `{feature}` on `{}`",
                        package.name, dependency.name
                    ));
                }
            }
        }
    }
    Ok(())
}

fn is_phase_numbered_feature(feature: &str) -> bool {
    feature
        .strip_prefix("phase")
        .and_then(|rest| rest.split_once('-'))
        .is_some_and(|(number, _)| {
            !number.is_empty() && number.bytes().all(|byte| byte.is_ascii_digit())
        })
}

fn inspect_ordinary_feature_tree(tree: &str) -> Result<(), String> {
    for line in tree.lines() {
        let Some((_, features)) = line.rsplit_once('[') else {
            continue;
        };
        let Some(features) = features.strip_suffix(']') else {
            continue;
        };
        for feature in features.split(',').map(str::trim) {
            if FORBIDDEN_ORDINARY_FEATURES.contains(&feature) {
                return Err(format!(
                    "ordinary Store graph activates forbidden feature `{feature}`"
                ));
            }
        }
    }
    Ok(())
}

#[derive(Debug, Deserialize)]
struct CargoMetadata {
    packages: Vec<CargoPackage>,
}

#[derive(Debug, Deserialize)]
struct CargoPackage {
    name: String,
    features: BTreeMap<String, Vec<String>>,
    dependencies: Vec<CargoDependency>,
}

#[derive(Debug, Deserialize)]
struct CargoDependency {
    name: String,
    features: Vec<String>,
}
