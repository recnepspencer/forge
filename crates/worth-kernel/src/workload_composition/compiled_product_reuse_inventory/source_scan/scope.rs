use std::fs;
use std::path::{Path, PathBuf};

use crate::workload_composition::compiled_product_reuse_inventory::error::CompiledProductReuseInventoryError;

const PHASE_SCOPE_ROOTS: [&str; 10] = [
    "crates/worth-kernel/src/workload_composition/public_closeout",
    "crates/worth-kernel/src/workload_composition/worth_workload",
    "crates/worth-kernel/src/replay_undo_consumer_cutover/public_closeout",
    "crates/worth-topo/src/derived_topology/invalidation_plan/catalog",
    "crates/worth-topo/src/derived_topology/invalidation_plan/selection",
    "crates/worth-topo/src/derived_topology/compiled_product_consumer_cutover/topology_derived_cluster",
    "crates/worth-topo/src/projection/runtime_boundary/read_execution",
    "crates/worth-spatial/src/workload_platform/evidence_lookup_index_product",
    "crates/worth-spatial/src/workload_platform/evidence_lookup_public_closeout",
    "crates/worth-spatial/src/workload_platform/retained_replay_workload",
];

const EXCLUDED_PATH_SEGMENTS: [&str; 6] = [
    "/compiled_product_reuse_inventory/",
    "/conflict_batch_admission_inventory/",
    "/source_firewall/",
    "/tests/",
    "/certification/",
    "/test_support/",
];

pub(crate) fn workspace_root() -> Result<PathBuf, CompiledProductReuseInventoryError> {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .map(Path::to_path_buf)
        .ok_or_else(|| {
            CompiledProductReuseInventoryError::SourceScanFailure(
                "worth-kernel should live under workspace/crates/worth-kernel".to_string(),
            )
        })
}

pub(super) fn scope_files(
    workspace_root: &Path,
) -> Result<Vec<(PathBuf, String)>, CompiledProductReuseInventoryError> {
    let mut files = Vec::new();
    for root in PHASE_SCOPE_ROOTS {
        visit_scope_root(workspace_root, root, &mut files)?;
    }
    files.sort_by(|left, right| left.1.cmp(&right.1));
    Ok(files)
}

fn visit_scope_root(
    workspace_root: &Path,
    relative_root: &str,
    files: &mut Vec<(PathBuf, String)>,
) -> Result<(), CompiledProductReuseInventoryError> {
    let root = workspace_root.join(relative_root);
    if !root.exists() {
        return Ok(());
    }
    visit_directory(workspace_root, &root, files)
}

fn visit_directory(
    workspace_root: &Path,
    directory: &Path,
    files: &mut Vec<(PathBuf, String)>,
) -> Result<(), CompiledProductReuseInventoryError> {
    for entry in fs::read_dir(directory).map_err(|error| {
        CompiledProductReuseInventoryError::SourceScanFailure(format!(
            "cannot read {}: {error}",
            directory.display()
        ))
    })? {
        let entry = entry.map_err(|error| {
            CompiledProductReuseInventoryError::SourceScanFailure(format!(
                "cannot read {} entry: {error}",
                directory.display()
            ))
        })?;
        let path = entry.path();
        let relative_path = path
            .strip_prefix(workspace_root)
            .map_err(|error| {
                CompiledProductReuseInventoryError::SourceScanFailure(format!(
                    "cannot relativize {} against {}: {error}",
                    path.display(),
                    workspace_root.display()
                ))
            })?
            .to_string_lossy()
            .replace('\\', "/");
        if should_skip(&relative_path) {
            continue;
        }
        if path.is_dir() {
            visit_directory(workspace_root, &path, files)?;
        } else if path.extension().and_then(|value| value.to_str()) == Some("rs") {
            files.push((path, relative_path));
        }
    }
    Ok(())
}

fn should_skip(relative_path: &str) -> bool {
    relative_path.ends_with("_tests.rs")
        || EXCLUDED_PATH_SEGMENTS
            .iter()
            .any(|segment| relative_path.contains(segment))
}
