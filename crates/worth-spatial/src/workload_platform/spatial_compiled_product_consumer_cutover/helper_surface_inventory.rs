use std::fs;
use std::path::{Path, PathBuf};

const HELPER_NAME: &str = concat!("reuse_", "evidence_", "lookup_", "index_", "product");

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum DisplacedEvidenceIndexHelperSurfaceDisposition {
    CutoverAuthority,
    InventorySupport,
    TestSupport,
    OrdinaryCallerViolation,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DisplacedEvidenceIndexHelperSurfaceRow {
    source_path: String,
    mention_count: usize,
    disposition: DisplacedEvidenceIndexHelperSurfaceDisposition,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DisplacedEvidenceIndexHelperSurfaceInventory {
    rows: Vec<DisplacedEvidenceIndexHelperSurfaceRow>,
}

impl DisplacedEvidenceIndexHelperSurfaceInventory {
    pub fn rows(&self) -> &[DisplacedEvidenceIndexHelperSurfaceRow] {
        &self.rows
    }

    pub fn ordinary_caller_violations(&self) -> Vec<&DisplacedEvidenceIndexHelperSurfaceRow> {
        self.rows
            .iter()
            .filter(|row| {
                row.disposition()
                    == DisplacedEvidenceIndexHelperSurfaceDisposition::OrdinaryCallerViolation
            })
            .collect()
    }
}

impl DisplacedEvidenceIndexHelperSurfaceRow {
    pub fn source_path(&self) -> &str {
        &self.source_path
    }

    pub const fn mention_count(&self) -> usize {
        self.mention_count
    }

    pub const fn disposition(&self) -> DisplacedEvidenceIndexHelperSurfaceDisposition {
        self.disposition
    }
}

pub fn current_displaced_evidence_index_helper_surface_inventory(
) -> std::io::Result<DisplacedEvidenceIndexHelperSurfaceInventory> {
    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..");
    let mut rows = Vec::new();
    for root in PHASE_TWELVE_SCAN_ROOTS {
        collect_rows_below(&workspace_root, &workspace_root.join(root), &mut rows)?;
    }
    rows.sort_by(|left, right| left.source_path.cmp(&right.source_path));
    Ok(DisplacedEvidenceIndexHelperSurfaceInventory { rows })
}

fn collect_rows_below(
    workspace_root: &Path,
    root: &Path,
    rows: &mut Vec<DisplacedEvidenceIndexHelperSurfaceRow>,
) -> std::io::Result<()> {
    if root.is_file() {
        collect_row_for_file(workspace_root, root, rows)?;
        return Ok(());
    }
    for entry in fs::read_dir(root)? {
        let path = entry?.path();
        if path.is_dir() {
            collect_rows_below(workspace_root, &path, rows)?;
        } else {
            collect_row_for_file(workspace_root, &path, rows)?;
        }
    }
    Ok(())
}

fn collect_row_for_file(
    workspace_root: &Path,
    file: &Path,
    rows: &mut Vec<DisplacedEvidenceIndexHelperSurfaceRow>,
) -> std::io::Result<()> {
    if file.extension().and_then(|extension| extension.to_str()) != Some("rs") {
        return Ok(());
    }
    let source = fs::read_to_string(file)?;
    let mention_count = source.matches(HELPER_NAME).count();
    if mention_count == 0 {
        return Ok(());
    }
    let source_path = file
        .strip_prefix(workspace_root)
        .unwrap_or(file)
        .to_string_lossy()
        .replace('\\', "/");
    rows.push(DisplacedEvidenceIndexHelperSurfaceRow {
        disposition: classify_path(&source_path),
        mention_count,
        source_path,
    });
    Ok(())
}

fn classify_path(source_path: &str) -> DisplacedEvidenceIndexHelperSurfaceDisposition {
    if CUTOVER_AUTHORITY_PATHS.contains(&source_path) {
        return DisplacedEvidenceIndexHelperSurfaceDisposition::CutoverAuthority;
    }
    if source_path.contains("/compiled_product_reuse_inventory/") {
        return DisplacedEvidenceIndexHelperSurfaceDisposition::InventorySupport;
    }
    if source_path.ends_with("/helper_surface_inventory.rs") {
        return DisplacedEvidenceIndexHelperSurfaceDisposition::InventorySupport;
    }
    if source_path.contains("/tests/") || source_path.ends_with("/tests.rs") {
        return DisplacedEvidenceIndexHelperSurfaceDisposition::TestSupport;
    }
    DisplacedEvidenceIndexHelperSurfaceDisposition::OrdinaryCallerViolation
}

const PHASE_TWELVE_SCAN_ROOTS: &[&str] = &[
    "crates/worth-spatial/src/workload_platform",
    "crates/worth-spatial/src/facade",
    "crates/worth-kernel/src/workload_composition",
];

const CUTOVER_AUTHORITY_PATHS: &[&str] = &[
    "crates/worth-spatial/src/workload_platform/spatial_compiled_product_consumer_cutover/mod.rs",
    "crates/worth-spatial/src/workload_platform/spatial_compiled_product_consumer_cutover/spatial_consumer_cluster/evidence_index_lowering.rs",
    "crates/worth-spatial/src/workload_platform/spatial_compiled_product_consumer_cutover/spatial_consumer_cluster/mod.rs",
    "crates/worth-spatial/src/facade/spatial_compiled_product_consumer_cutover/mod.rs",
    "crates/worth-kernel/src/workload_composition/compiled_product_consumer_cutover/vertical_slice/lookup_consumed/reuse_resolution.rs",
];
