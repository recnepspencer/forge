use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use super::closeout_report::WorthGraphAuthorityCloseoutViolation;
use super::closeout_types::WorthGraphAuthorityDeletionClassCloseoutEvidence;
use super::gate_report_types::WorthGraphAuthorityGateReport;
use super::types::{WorthGraphAuthorityDeletionLedgerRow, WorthGraphAuthorityDeletionTarget};

pub(crate) fn closeout_deletion_class_evidence(
    gate: &WorthGraphAuthorityGateReport,
    audited_source_paths: &[String],
    required_targets: &[WorthGraphAuthorityDeletionTarget],
) -> Result<
    Vec<WorthGraphAuthorityDeletionClassCloseoutEvidence>,
    WorthGraphAuthorityCloseoutViolation,
> {
    let mut evidence = Vec::with_capacity(required_targets.len());
    for target in required_targets {
        let rows: Vec<_> = gate
            .deletion_ledger()
            .iter()
            .filter(|row| row.deletion_target() == *target)
            .collect();
        if rows.is_empty() {
            return Err(WorthGraphAuthorityCloseoutViolation::MissingDeletionTargetClass(*target));
        }
        let affected_files = affected_files_for_rows(&rows, audited_source_paths);
        let affected_source_lines = affected_files
            .iter()
            .map(|path| source_line_count(path))
            .sum();
        evidence.push(WorthGraphAuthorityDeletionClassCloseoutEvidence::new(
            *target,
            rows.len(),
            affected_files.len(),
            affected_source_lines,
        ));
    }
    Ok(evidence)
}

fn affected_files_for_rows(
    rows: &[&WorthGraphAuthorityDeletionLedgerRow],
    audited_source_paths: &[String],
) -> BTreeSet<PathBuf> {
    let workspace = workspace_root();
    let mut affected_files = BTreeSet::new();
    for row in rows {
        let source_path = row.source_path();
        for audited_source_path in audited_source_paths {
            if source_matches(audited_source_path, source_path) {
                affected_files.insert(workspace.join(audited_source_path));
            }
        }
        if affected_files
            .iter()
            .any(|path| path_matches(path, source_path))
        {
            continue;
        }
        collect_direct_source_path(&workspace.join(source_path), &mut affected_files);
    }
    affected_files
}

fn source_matches(audited_source_path: &str, source_path: &str) -> bool {
    audited_source_path == source_path
        || audited_source_path
            .strip_prefix(source_path)
            .is_some_and(|suffix| suffix.starts_with('/'))
}

fn path_matches(path: &Path, source_path: &str) -> bool {
    path.to_string_lossy()
        .replace('\\', "/")
        .ends_with(source_path)
}

fn collect_direct_source_path(path: &Path, affected_files: &mut BTreeSet<PathBuf>) {
    if path.is_file() {
        affected_files.insert(path.to_path_buf());
        return;
    }
    if !path.is_dir() {
        return;
    }
    let entries = std::fs::read_dir(path)
        .unwrap_or_else(|error| panic!("failed to read closeout source path {path:?}: {error}"));
    for entry in entries {
        let entry = entry.expect("failed to read closeout source directory entry");
        let path = entry.path();
        if path.is_dir() {
            collect_direct_source_path(&path, affected_files);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            affected_files.insert(path);
        }
    }
}

fn source_line_count(path: &Path) -> usize {
    std::fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("failed to read closeout source path {path:?}: {error}"))
        .lines()
        .count()
}

fn workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("worth-kernel should live two levels below the workspace root")
}
