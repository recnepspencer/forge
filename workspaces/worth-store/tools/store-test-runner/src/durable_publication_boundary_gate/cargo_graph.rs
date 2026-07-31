use std::collections::BTreeSet;
use std::process::Command;

use serde::Deserialize;

use super::read_repository_document;
use crate::workspace_root;

const GRAPH_DOCUMENT: &str = "_docs/worth-store/physical-reconstruction-c7-cargo-graph.csv";
const HEADER: &str = "package,dependency,kind,optional";
const TRACKED_PACKAGES: &[&str] = &[
    "store-test-runner",
    "worth-store",
    "worth-store-physical-backend",
    "worth-store-physical-isolation",
    "worth-store-recovery-physics",
    "worth-store-wal",
];

#[test]
fn checked_in_cargo_inventory_matches_current_metadata_exactly() {
    let expected =
        parse_graph(&read_repository_document(GRAPH_DOCUMENT).expect("read C.7 Cargo inventory"))
            .expect("parse C.7 Cargo inventory");
    let actual = metadata_edges().expect("discover current Cargo edges");
    assert_eq!(
        actual, expected,
        "C.7 Cargo graph changed without an authority disposition"
    );
}

#[test]
fn current_graph_exposes_islands_and_one_way_wal_meaning() {
    let edges = metadata_edges().expect("discover current Cargo edges");
    assert!(!edges.iter().any(|edge| {
        edge.package == "worth-store"
            && matches!(
                edge.dependency.as_str(),
                "worth-store-recovery-physics" | "worth-store-physical-isolation"
            )
            && edge.kind == "normal"
    }));
    assert!(edges.iter().any(|edge| {
        edge.package == "worth-store"
            && edge.dependency == "worth-store-wal"
            && edge.kind == "normal"
    }));
    assert!(!edges.iter().any(|edge| {
        edge.package == "worth-store-wal"
            && edge.dependency == "worth-store"
            && edge.kind == "normal"
    }));
    assert!(edges.iter().any(|edge| {
        edge.package == "worth-store-recovery-physics"
            && edge.dependency == "worth-store-wal"
            && edge.kind == "normal"
    }));
    assert!(edges.iter().any(|edge| {
        edge.package == "worth-store-physical-isolation"
            && edge.dependency == "worth-store"
            && edge.kind == "normal"
    }));
}

fn metadata_edges() -> Result<BTreeSet<GraphEdge>, String> {
    let output = Command::new("cargo")
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
    let metadata: CargoMetadata = serde_json::from_slice(&output.stdout)
        .map_err(|error| format!("invalid cargo metadata: {error}"))?;
    Ok(metadata
        .packages
        .into_iter()
        .filter(|package| TRACKED_PACKAGES.contains(&package.name.as_str()))
        .flat_map(|package| {
            package
                .dependencies
                .into_iter()
                .map(move |dependency| GraphEdge {
                    package: package.name.clone(),
                    dependency: dependency.name,
                    kind: dependency.kind.unwrap_or_else(|| "normal".to_owned()),
                    optional: dependency.optional,
                })
        })
        .collect())
}

fn parse_graph(document: &str) -> Result<BTreeSet<GraphEdge>, String> {
    let mut lines = document.lines();
    if lines.next() != Some(HEADER) {
        return Err("C.7 Cargo inventory has an invalid schema header".to_owned());
    }
    lines
        .filter(|line| !line.trim().is_empty())
        .enumerate()
        .map(|(index, line)| {
            let columns = line.split(',').map(str::trim).collect::<Vec<_>>();
            if columns.len() != 4 || columns.iter().any(|column| column.is_empty()) {
                return Err(format!("invalid C.7 Cargo row {}", index + 2));
            }
            let optional = columns[3]
                .parse()
                .map_err(|_| format!("invalid optional flag at C.7 Cargo row {}", index + 2))?;
            Ok(GraphEdge {
                package: columns[0].to_owned(),
                dependency: columns[1].to_owned(),
                kind: columns[2].to_owned(),
                optional,
            })
        })
        .collect()
}

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
struct GraphEdge {
    package: String,
    dependency: String,
    kind: String,
    optional: bool,
}

#[derive(Deserialize)]
struct CargoMetadata {
    packages: Vec<CargoPackage>,
}

#[derive(Deserialize)]
struct CargoPackage {
    name: String,
    dependencies: Vec<CargoDependency>,
}

#[derive(Deserialize)]
struct CargoDependency {
    name: String,
    kind: Option<String>,
    optional: bool,
}
