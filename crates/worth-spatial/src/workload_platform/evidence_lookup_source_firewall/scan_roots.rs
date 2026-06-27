use std::fs;
use std::path::{Path, PathBuf};

use super::covered_root::{
    EvidenceLookupSourceFirewallCoveredRoot, EvidenceLookupSourceFirewallCoveredRootKind,
};
use super::error::{EvidenceLookupSourceFirewallError, EvidenceLookupSourceFirewallErrorKind};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SourceFirewallRecord {
    pub(crate) source_path: String,
    pub(crate) source: String,
    pub(crate) test_support: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SourceFirewallSnapshot {
    covered_root_inventory: Vec<EvidenceLookupSourceFirewallCoveredRoot>,
    scanned_root_count: usize,
    scanned_file_count: usize,
    records: Vec<SourceFirewallRecord>,
}

#[derive(Clone, Copy)]
struct SourceFirewallScanRoot {
    source_path: &'static str,
    kind: EvidenceLookupSourceFirewallCoveredRootKind,
    test_support: bool,
}

impl SourceFirewallSnapshot {
    pub(crate) fn covered_root_inventory(&self) -> &[EvidenceLookupSourceFirewallCoveredRoot] {
        &self.covered_root_inventory
    }

    pub(crate) fn covered_roots(&self) -> Vec<String> {
        self.covered_root_inventory
            .iter()
            .map(|root| root.source_path().to_string())
            .collect()
    }

    pub(crate) fn scanned_root_count(&self) -> usize {
        self.scanned_root_count
    }

    pub(crate) fn scanned_file_count(&self) -> usize {
        self.scanned_file_count
    }

    pub(crate) fn records(&self) -> &[SourceFirewallRecord] {
        &self.records
    }
}

pub(crate) fn current_source_firewall_snapshot(
) -> Result<SourceFirewallSnapshot, EvidenceLookupSourceFirewallError> {
    source_firewall_snapshot_for_workspace_root(&workspace_root())
}

pub(crate) fn source_firewall_snapshot_for_workspace_root(
    workspace_root: &Path,
) -> Result<SourceFirewallSnapshot, EvidenceLookupSourceFirewallError> {
    let mut scanned_files = 0;
    let mut records = Vec::new();
    let covered_root_inventory = covered_root_inventory();
    for root in SOURCE_FIREWALL_SCAN_ROOTS {
        let absolute_root = workspace_root.join(root.source_path);
        if !absolute_root.exists() {
            return Err(EvidenceLookupSourceFirewallError::new(
                EvidenceLookupSourceFirewallErrorKind::MissingScanRoot,
                root.source_path,
            ));
        }
        collect_records_below(
            workspace_root,
            &absolute_root,
            root.test_support,
            &mut scanned_files,
            &mut records,
        );
    }
    Ok(SourceFirewallSnapshot {
        covered_root_inventory,
        scanned_root_count: SOURCE_FIREWALL_SCAN_ROOTS.len(),
        scanned_file_count: scanned_files,
        records,
    })
}

pub(crate) fn covered_root_inventory() -> Vec<EvidenceLookupSourceFirewallCoveredRoot> {
    SOURCE_FIREWALL_SCAN_ROOTS
        .iter()
        .map(|root| EvidenceLookupSourceFirewallCoveredRoot::new(root.source_path, root.kind))
        .collect()
}

fn collect_records_below(
    workspace_root: &Path,
    root: &Path,
    test_support: bool,
    scanned_files: &mut usize,
    records: &mut Vec<SourceFirewallRecord>,
) {
    for file in rust_files_below(root) {
        *scanned_files += 1;
        let Ok(source) = fs::read_to_string(&file) else {
            continue;
        };
        let source_path = workspace_relative_path(workspace_root, &file);
        records.push(SourceFirewallRecord {
            test_support: test_support || looks_like_test_support_path(&source_path),
            source_path,
            source,
        });
    }
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
        if path.is_dir() {
            files.extend(rust_files_below(&path));
        } else if path.extension().and_then(|extension| extension.to_str()) == Some("rs") {
            files.push(path);
        }
    }
    files
}

fn workspace_relative_path(workspace_root: &Path, path: &Path) -> String {
    path.strip_prefix(workspace_root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
}

fn looks_like_test_support_path(source_path: &str) -> bool {
    source_path.contains("/test_support/")
        || source_path.contains("/tests/")
        || source_path.ends_with("/tests.rs")
        || source_path.contains("_tests.rs")
}

const fn production_root(
    source_path: &'static str,
    kind: EvidenceLookupSourceFirewallCoveredRootKind,
) -> SourceFirewallScanRoot {
    SourceFirewallScanRoot {
        source_path,
        kind,
        test_support: false,
    }
}

const SOURCE_FIREWALL_SCAN_ROOTS: &[SourceFirewallScanRoot] = &[
    production_root(
        "crates/worth-spatial/src/facade/workload_vocabulary/mod.rs",
        EvidenceLookupSourceFirewallCoveredRootKind::PublicFacadeVocabulary,
    ),
    production_root(
        "crates/worth-spatial/src/workload_platform/evidence_ledger/ledger.rs",
        EvidenceLookupSourceFirewallCoveredRootKind::LegacyLedgerSurface,
    ),
    production_root(
        "crates/worth-spatial/src/workload_platform/evidence_ledger/stage_index/product.rs",
        EvidenceLookupSourceFirewallCoveredRootKind::LegacyStageIndexSurface,
    ),
    production_root(
        "crates/worth-spatial/src/workload_platform/evidence_ledger/row.rs",
        EvidenceLookupSourceFirewallCoveredRootKind::RawEvidenceRowSurface,
    ),
    production_root(
        "crates/worth-spatial/src/workload_platform/evidence_ledger/spatial_touch_admission",
        EvidenceLookupSourceFirewallCoveredRootKind::SpatialTouchAdmissionLane,
    ),
    production_root(
        "crates/worth-spatial/src/workload_platform/evidence_ledger/surface_inventory/rows.rs",
        EvidenceLookupSourceFirewallCoveredRootKind::DocumentationReportCodec,
    ),
    production_root(
        "crates/worth-spatial/src/certification/workload_evidence.rs",
        EvidenceLookupSourceFirewallCoveredRootKind::CertificationCodec,
    ),
    production_root(
        "crates/worth-spatial/src/query_adoption.rs",
        EvidenceLookupSourceFirewallCoveredRootKind::QueryAdoptionSurface,
    ),
    production_root(
        "crates/worth-kernel/src/workload_composition",
        EvidenceLookupSourceFirewallCoveredRootKind::KernelResidueSurface,
    ),
];
