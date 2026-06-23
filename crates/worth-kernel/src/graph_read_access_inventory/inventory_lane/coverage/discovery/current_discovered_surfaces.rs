use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use super::{WorthGraphReadAccessDiscoveredSurface, WorthGraphReadAccessDiscoveredSurfaceReport};

pub(super) fn current_worth_graph_read_access_discovered_surface_report(
) -> WorthGraphReadAccessDiscoveredSurfaceReport {
    WorthGraphReadAccessDiscoveredSurfaceReport::new(discover_graph_read_surfaces_from_source())
}

fn discover_graph_read_surfaces_from_source() -> Vec<WorthGraphReadAccessDiscoveredSurface> {
    let mut discovered_by_source = BTreeMap::new();
    for scan_root in GRAPH_READ_DISCOVERY_SCAN_ROOTS {
        for file in rust_files_below(&workspace_root().join(scan_root)) {
            let Some(evidence) = graph_read_evidence_for_file(&file) else {
                continue;
            };
            let relative_file = workspace_relative_path(&file);
            let classified_source = classified_source_for_file(&relative_file);
            let source_path = classified_source.source_path;
            let test_support = classified_source.test_support;
            discovered_by_source
                .entry(source_path.clone())
                .or_insert_with(|| {
                    WorthGraphReadAccessDiscoveredSurface::new(source_path, evidence, test_support)
                });
        }
    }

    discovered_by_source.into_values().collect()
}

fn graph_read_evidence_for_file(file: &Path) -> Option<String> {
    let source = fs::read_to_string(file).ok()?;
    let matched_shape = graph_read_shape_in_source(&source)?;
    Some(format!(
        "{} contains `{matched_shape}`",
        workspace_relative_path(file)
    ))
}

fn rust_files_below(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if root.is_file() {
        if root.extension().and_then(|extension| extension.to_str()) == Some("rs") {
            files.push(root.to_path_buf());
        }
        return files;
    }

    let Ok(entries) = fs::read_dir(root) else {
        return files;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if ignored_discovery_path(&path) {
            continue;
        }
        if path.is_dir() {
            files.extend(rust_files_below(&path));
        } else if path.extension().and_then(|extension| extension.to_str()) == Some("rs") {
            files.push(path);
        }
    }
    files
}

fn ignored_discovery_path(path: &Path) -> bool {
    let path_text = workspace_relative_path(path);
    path_text.contains("crates/worth-kernel/src/graph_read_access_inventory")
}

fn classified_source_for_file(relative_file: &str) -> ClassifiedDiscoveredSource {
    for covered_root in GRAPH_READ_COVERED_DISCOVERY_ROOTS {
        if relative_file.starts_with(covered_root.source_path) {
            return ClassifiedDiscoveredSource {
                source_path: covered_root.source_path.to_string(),
                test_support: covered_root.test_support,
            };
        }
    }

    ClassifiedDiscoveredSource {
        source_path: relative_file.to_string(),
        test_support: relative_file.contains("/tests/") || relative_file.ends_with("/tests.rs"),
    }
}

fn graph_read_shape_in_source(source: &str) -> Option<&'static str> {
    let normalized_source = source.to_ascii_lowercase();
    GRAPH_READ_DISCOVERY_SHAPES
        .iter()
        .copied()
        .find(|shape| normalized_source.contains(shape))
}

fn workspace_relative_path(path: &Path) -> String {
    let workspace_root = workspace_root();
    path.strip_prefix(&workspace_root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
}

struct ClassifiedDiscoveredSource {
    source_path: String,
    test_support: bool,
}

struct WorthGraphReadAccessDiscoveryRoot {
    source_path: &'static str,
    test_support: bool,
}

const GRAPH_READ_DISCOVERY_SCAN_ROOTS: &[&str] = &[
    "crates/worth-topo/src/projection/read_views/domain",
    "crates/worth-topo/src/projection/read_views/domain/read_proof",
    "crates/worth-topo/src/projection/runtime_boundary/read_execution",
    "crates/worth-topo/src/certification/projection_closeout/tests/topology_reads",
    "crates/worth-spatial/src/workload_platform/evidence_ledger",
    "crates/worth-spatial/src/workload_platform/planar_boolean_loop_reconstruction",
    "crates/worth-spatial/src/workload_platform/planar_boolean_loop_reconstruction/test_support",
    "crates/worth-spatial/src/workload_platform/planar_boolean_events",
    "crates/worth-kernel/src/binding",
    "crates/worth-kernel/src/query_adoption/graph_read_access",
    "crates/worth-kernel/src/workload_composition",
];

const GRAPH_READ_COVERED_DISCOVERY_ROOTS: &[WorthGraphReadAccessDiscoveryRoot] = &[
    production_root("crates/worth-topo/src/projection/read_views/domain/read_proof"),
    production_root("crates/worth-topo/src/projection/read_views/domain"),
    production_root("crates/worth-topo/src/projection/runtime_boundary/read_execution"),
    production_root("crates/worth-kernel/src/query_adoption/graph_read_access"),
    production_root("crates/worth-spatial/src/workload_platform/evidence_ledger"),
    test_support_root(
        "crates/worth-spatial/src/workload_platform/planar_boolean_loop_reconstruction/test_support",
    ),
    production_root("crates/worth-spatial/src/workload_platform/planar_boolean_loop_reconstruction"),
    production_root("crates/worth-spatial/src/workload_platform/planar_boolean_events"),
    test_support_root("crates/worth-topo/src/certification/projection_closeout/tests/topology_reads"),
    test_support_root("crates/worth-kernel/src/binding/tests"),
    production_root("crates/worth-kernel/src/workload_composition"),
    production_root("crates/worth-kernel/src/binding"),
];

const fn production_root(source_path: &'static str) -> WorthGraphReadAccessDiscoveryRoot {
    WorthGraphReadAccessDiscoveryRoot {
        source_path,
        test_support: false,
    }
}

const fn test_support_root(source_path: &'static str) -> WorthGraphReadAccessDiscoveryRoot {
    WorthGraphReadAccessDiscoveryRoot {
        source_path,
        test_support: true,
    }
}

const GRAPH_READ_DISCOVERY_SHAPES: &[&str] = &[
    "adjacency",
    "broad scan",
    "fabricated",
    "frontier",
    "graph_read",
    "local cache",
    "local topology",
    "neighborhood",
    "no-n-plus-one",
    "read-proof",
    "read receipt",
    "relation loop",
    "relationship",
];
