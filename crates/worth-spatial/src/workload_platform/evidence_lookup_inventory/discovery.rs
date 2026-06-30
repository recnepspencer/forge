use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct EvidenceLookupDiscoveredSurface {
    source_path: String,
    evidence: String,
    test_support: bool,
}

impl EvidenceLookupDiscoveredSurface {
    pub(crate) fn new(
        source_path: impl Into<String>,
        evidence: impl Into<String>,
        test_support: bool,
    ) -> Self {
        Self {
            source_path: source_path.into(),
            evidence: evidence.into(),
            test_support,
        }
    }

    pub(crate) fn source_path(&self) -> &str {
        &self.source_path
    }

    pub(crate) fn evidence(&self) -> &str {
        &self.evidence
    }

    pub(crate) const fn is_test_support(&self) -> bool {
        self.test_support
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct EvidenceLookupDiscoveredSurfaceReport {
    surfaces: Vec<EvidenceLookupDiscoveredSurface>,
}

impl EvidenceLookupDiscoveredSurfaceReport {
    pub(crate) fn new(surfaces: Vec<EvidenceLookupDiscoveredSurface>) -> Self {
        Self { surfaces }
    }

    pub(crate) fn surfaces(&self) -> &[EvidenceLookupDiscoveredSurface] {
        &self.surfaces
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct EvidenceLookupDiscoveryScanRoot {
    source_path: String,
    classified_source_path: Option<String>,
    test_support: bool,
}

impl EvidenceLookupDiscoveryScanRoot {
    pub(crate) fn production(source_path: impl Into<String>) -> Self {
        Self {
            source_path: source_path.into(),
            classified_source_path: None,
            test_support: false,
        }
    }

    #[cfg(test)]
    pub(crate) fn test_support(source_path: impl Into<String>) -> Self {
        Self {
            source_path: source_path.into(),
            classified_source_path: None,
            test_support: true,
        }
    }

    #[cfg(test)]
    pub(crate) fn classified_as(mut self, source_path: impl Into<String>) -> Self {
        self.classified_source_path = Some(source_path.into());
        self
    }
}

pub(crate) fn current_evidence_lookup_discovered_surface_report(
) -> EvidenceLookupDiscoveredSurfaceReport {
    EvidenceLookupDiscoveredSurfaceReport::new(discover_evidence_lookup_surfaces_from_source())
}

#[cfg(test)]
pub(crate) fn evidence_lookup_discovered_surface_report_for_roots(
    scan_roots: &[EvidenceLookupDiscoveryScanRoot],
) -> EvidenceLookupDiscoveredSurfaceReport {
    EvidenceLookupDiscoveredSurfaceReport::new(discover_evidence_lookup_surfaces_from_roots(
        scan_roots,
    ))
}

fn discover_evidence_lookup_surfaces_from_source() -> Vec<EvidenceLookupDiscoveredSurface> {
    let scan_roots = EVIDENCE_LOOKUP_DISCOVERY_SCAN_ROOTS
        .iter()
        .map(|source_path| EvidenceLookupDiscoveryScanRoot::production(*source_path))
        .collect::<Vec<_>>();
    discover_evidence_lookup_surfaces_from_roots(&scan_roots)
}

fn discover_evidence_lookup_surfaces_from_roots(
    scan_roots: &[EvidenceLookupDiscoveryScanRoot],
) -> Vec<EvidenceLookupDiscoveredSurface> {
    let mut discovered_by_source = BTreeMap::new();
    for scan_root in scan_roots {
        for file in rust_files_below(&workspace_root().join(&scan_root.source_path)) {
            let Some(evidence) = evidence_lookup_shape_for_file(&file) else {
                continue;
            };
            let relative_file = workspace_relative_path(&file);
            let classified = classified_source_for_file(&relative_file, scan_root);
            discovered_by_source
                .entry(classified.source_path.clone())
                .or_insert_with(|| {
                    EvidenceLookupDiscoveredSurface::new(
                        classified.source_path,
                        evidence,
                        classified.test_support,
                    )
                });
        }
    }
    discovered_by_source.into_values().collect()
}

fn evidence_lookup_shape_for_file(file: &Path) -> Option<String> {
    let source = fs::read_to_string(file).ok()?;
    let matched_shape = evidence_lookup_shape_in_source(&source)?;
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
    workspace_relative_path(path)
        .contains("crates/worth-spatial/src/workload_platform/evidence_lookup_inventory")
}

fn classified_source_for_file(
    relative_file: &str,
    scan_root: &EvidenceLookupDiscoveryScanRoot,
) -> ClassifiedDiscoveredSource {
    if let Some(source_path) = &scan_root.classified_source_path {
        return ClassifiedDiscoveredSource {
            source_path: source_path.clone(),
            test_support: scan_root.test_support,
        };
    }
    for root in EVIDENCE_LOOKUP_COVERED_DISCOVERY_ROOTS {
        if relative_file.starts_with(root.source_path) {
            return ClassifiedDiscoveredSource {
                source_path: root.source_path.to_string(),
                test_support: root.test_support || scan_root.test_support,
            };
        }
    }
    ClassifiedDiscoveredSource {
        source_path: relative_file.to_string(),
        test_support: scan_root.test_support
            || relative_file.contains("/tests/")
            || relative_file.ends_with("/tests.rs"),
    }
}

fn evidence_lookup_shape_in_source(source: &str) -> Option<&'static str> {
    let normalized = source.to_ascii_lowercase();
    EVIDENCE_LOOKUP_DISCOVERY_SHAPES
        .iter()
        .copied()
        .find(|shape| normalized.contains(shape))
}

fn workspace_relative_path(path: &Path) -> String {
    let root = workspace_root();
    path.strip_prefix(&root)
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

struct EvidenceLookupDiscoveryRoot {
    source_path: &'static str,
    test_support: bool,
}

const fn production_root(source_path: &'static str) -> EvidenceLookupDiscoveryRoot {
    EvidenceLookupDiscoveryRoot {
        source_path,
        test_support: false,
    }
}

const fn test_support_root(source_path: &'static str) -> EvidenceLookupDiscoveryRoot {
    EvidenceLookupDiscoveryRoot {
        source_path,
        test_support: true,
    }
}

const EVIDENCE_LOOKUP_DISCOVERY_SCAN_ROOTS: &[&str] = &[
    "crates/worth-spatial/src/facade/workload_vocabulary/mod.rs",
    "crates/worth-spatial/src/workload_platform/evidence_ledger/ledger.rs",
    "crates/worth-spatial/src/workload_platform/evidence_ledger/stage_index/product.rs",
    "crates/worth-spatial/src/workload_platform/evidence_ledger/row.rs",
    "crates/worth-spatial/src/workload_platform/evidence_ledger/guard.rs",
    "crates/worth-spatial/src/workload_platform/evidence_ledger/stage_links",
    "crates/worth-spatial/src/workload_platform/evidence_ledger/stage_index",
    "crates/worth-spatial/src/workload_platform/evidence_ledger/spatial_touch_admission",
    "crates/worth-spatial/src/certification/workload_evidence.rs",
    "crates/worth-spatial/src/workload_platform/planar_boolean_edge_splitting",
    "crates/worth-spatial/src/workload_platform/planar_boolean_loop_reconstruction/test_support",
    "crates/worth-spatial/src/query_adoption.rs",
    "crates/worth-kernel/src/workload_composition",
];

const EVIDENCE_LOOKUP_COVERED_DISCOVERY_ROOTS: &[EvidenceLookupDiscoveryRoot] = &[
    production_root("crates/worth-spatial/src/facade/workload_vocabulary/mod.rs"),
    production_root("crates/worth-spatial/src/workload_platform/evidence_ledger/ledger.rs"),
    production_root("crates/worth-spatial/src/workload_platform/evidence_ledger/stage_index/product.rs"),
    production_root("crates/worth-spatial/src/workload_platform/evidence_ledger/row.rs"),
    production_root("crates/worth-spatial/src/workload_platform/evidence_ledger"),
    production_root("crates/worth-spatial/src/certification/workload_evidence.rs"),
    production_root("crates/worth-spatial/src/workload_platform/planar_boolean_edge_splitting"),
    test_support_root("crates/worth-spatial/src/workload_platform/planar_boolean_loop_reconstruction/test_support"),
    production_root("crates/worth-spatial/src/query_adoption.rs"),
    production_root("crates/worth-spatial/src/workload_platform/evidence_ledger/spatial_touch_admission"),
    production_root("crates/worth-kernel/src/workload_composition"),
];

const EVIDENCE_LOOKUP_DISCOVERY_SHAPES: &[&str] = &[
    "evidence_identity",
    "evidence lookup",
    "evidence row",
    "evidence vector",
    "nearby",
    "receipt lookup",
    "row_for_stage",
    "stage index",
];
