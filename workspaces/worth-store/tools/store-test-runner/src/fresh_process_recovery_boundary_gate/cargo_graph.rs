use std::collections::BTreeSet;
use std::process::Command;

use serde::Deserialize;

use super::documents::{read_repository_document, split_csv, CARGO_GRAPH};
use crate::workspace_root;

const HEADER: &str = "package,dependency,kind,optional,disposition,destination_owner,phase";
const PHYSICS: &str = "worth-store-recovery-physics";

#[test]
fn checked_in_recovery_dependency_cut_matches_cargo_metadata() {
    let document = read_repository_document(CARGO_GRAPH).expect("read C.8 Cargo graph");
    let expected = parse_graph(&document).expect("parse C.8 Cargo graph");
    let actual = metadata_edges().expect("discover C.8 Cargo graph");
    assert_eq!(
        expected.iter().map(GraphRow::edge).collect::<BTreeSet<_>>(),
        actual,
        "C.8 recovery dependency cut changed without a disposition"
    );
}

#[test]
fn phase_one_dependency_direction_is_honest() {
    let edges = metadata_edges().expect("discover C.8 Cargo graph");
    assert!(!edges.iter().any(|edge| {
        edge.package == PHYSICS
            && matches!(
                edge.dependency.as_str(),
                "worth-signal" | "worth-query" | "worth-query-decl" | "worth-query-host"
            )
    }));
    assert!(!edges.iter().any(|edge| {
        edge.package == "worth-store" && edge.dependency == PHYSICS && edge.kind == "normal"
    }));
    assert!(
        edges.iter().any(|edge| {
            edge.package == PHYSICS && edge.dependency == "worth-store" && edge.kind == "normal"
        }),
        "Phase 1 must account for the old reverse edge before Phase 8 removes it"
    );
    let metadata = metadata().expect("read workspace packages");
    assert!(
        metadata
            .packages
            .iter()
            .all(|package| package.name != "worth-store-recovery-runtime"),
        "Phase 1 cannot create the Phase 2 recovery runtime crate"
    );
}

#[test]
fn every_current_edge_has_a_specific_cutover_owner() {
    let document = read_repository_document(CARGO_GRAPH).expect("read C.8 Cargo graph");
    let rows = parse_graph(&document).expect("parse C.8 Cargo graph");
    for row in rows {
        let expected = expected_disposition(&row.package, &row.dependency);
        assert_eq!(
            (
                row.disposition.as_str(),
                row.destination_owner.clone(),
                row.phase.as_str(),
            ),
            expected,
            "wrong C.8 Cargo disposition for {} -> {}",
            row.package,
            row.dependency
        );
        assert!(!matches!(
            row.destination_owner.as_str(),
            "recovery" | "physics" | "support" | "evidence" | "utility" | "none"
        ));
        assert!(matches!(
            row.phase.as_str(),
            "phase-2" | "phase-3" | "phase-4" | "phase-5" | "phase-6" | "phase-8"
        ));
    }
}

fn expected_disposition(package: &str, dependency: &str) -> (&'static str, String, &'static str) {
    if package != PHYSICS {
        return ("narrow", format!("{package}/c8-import-cutover"), "phase-8");
    }
    match dependency {
        "worth-store" => (
            "replace",
            "worth-store-recovery-runtime/composition-root".into(),
            "phase-8",
        ),
        "worth-foundational" => (
            "replace",
            "worth-store-recovery-runtime/observation-protocol".into(),
            "phase-8",
        ),
        "worth-store-aspect-native" => (
            "replace",
            "worth-store-recovery-runtime/work-routing".into(),
            "phase-8",
        ),
        "tempfile" => (
            "replace",
            "worth-store-physical-certification/c8-fixtures".into(),
            "phase-8",
        ),
        "worth-store-test-support" => (
            "narrow",
            "worth-store-test-support/c8-worlds".into(),
            "phase-8",
        ),
        other => (
            "narrow",
            format!(
                "worth-store-recovery-physics/{}",
                other.strip_prefix("worth-store-").unwrap_or(other)
            ),
            "phase-4",
        ),
    }
}

fn metadata_edges() -> Result<BTreeSet<GraphEdge>, String> {
    Ok(metadata()?
        .packages
        .into_iter()
        .flat_map(|package| {
            let package_name = package.name;
            package
                .dependencies
                .into_iter()
                .filter_map(move |dependency| {
                    if package_name != PHYSICS && dependency.name != PHYSICS {
                        return None;
                    }
                    Some(GraphEdge {
                        package: package_name.clone(),
                        dependency: dependency.name,
                        kind: dependency.kind.unwrap_or_else(|| "normal".to_owned()),
                        optional: dependency.optional,
                    })
                })
        })
        .collect())
}

fn metadata() -> Result<CargoMetadata, String> {
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
    serde_json::from_slice(&output.stdout)
        .map_err(|error| format!("invalid Cargo metadata: {error}"))
}

fn parse_graph(document: &str) -> Result<Vec<GraphRow>, String> {
    let mut lines = document.lines();
    if lines.next() != Some(HEADER) {
        return Err("C.8 Cargo graph has an invalid schema".into());
    }
    lines
        .filter(|line| !line.trim().is_empty())
        .enumerate()
        .map(|(index, line)| {
            let columns = split_csv(line, 7)
                .map_err(|error| format!("C.8 Cargo row {}: {error}", index + 2))?;
            Ok(GraphRow {
                package: columns[0].to_owned(),
                dependency: columns[1].to_owned(),
                kind: columns[2].to_owned(),
                optional: columns[3]
                    .parse()
                    .map_err(|_| format!("invalid optional flag at row {}", index + 2))?,
                disposition: columns[4].to_owned(),
                destination_owner: columns[5].to_owned(),
                phase: columns[6].to_owned(),
            })
        })
        .collect()
}

struct GraphRow {
    package: String,
    dependency: String,
    kind: String,
    optional: bool,
    disposition: String,
    destination_owner: String,
    phase: String,
}

impl GraphRow {
    fn edge(&self) -> GraphEdge {
        GraphEdge {
            package: self.package.clone(),
            dependency: self.dependency.clone(),
            kind: self.kind.clone(),
            optional: self.optional,
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
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
