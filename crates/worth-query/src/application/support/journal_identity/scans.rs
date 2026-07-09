use std::path::{Path, PathBuf};

use super::inventory::worth_query_journal_identity_inventory;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryJournalIdentityScanFailure {
    path: String,
    pattern: &'static str,
}

impl WorthQueryJournalIdentityScanFailure {
    fn new(path: impl Into<String>, pattern: &'static str) -> Self {
        Self {
            path: path.into(),
            pattern,
        }
    }
}

pub fn scan_journal_identity_forbidden_patterns(
    workspace_root: &Path,
) -> Vec<WorthQueryJournalIdentityScanFailure> {
    let forbidden_patterns = [
        ".rsplit",
        ".rsplit_once",
        ".split",
        ".split_once",
        "strip_prefix",
        "trim_start_matches",
        "chars().filter",
        "Regex",
        "commit_identity().to_string",
        "terminal_projection_for_reporting().parse",
        "terminal_projection_for_reporting",
        "parse::<u64>",
        "parse::<",
    ];
    worth_query_journal_identity_inventory()
        .iter()
        .flat_map(|row| scan_patterns(workspace_root, row.path(), &forbidden_patterns))
        .collect()
}

pub fn scan_journal_identity_required_pattern_failures(
    workspace_root: &Path,
) -> Vec<WorthQueryJournalIdentityScanFailure> {
    worth_query_journal_identity_inventory()
        .iter()
        .flat_map(|row| {
            row.required_patterns()
                .iter()
                .filter(|pattern| !path_contains(workspace_root, row.path(), pattern))
                .map(|pattern| WorthQueryJournalIdentityScanFailure::new(row.path(), *pattern))
                .collect::<Vec<_>>()
        })
        .collect()
}

fn scan_patterns(
    workspace_root: &Path,
    relative_path: &str,
    patterns: &[&'static str],
) -> Vec<WorthQueryJournalIdentityScanFailure> {
    let Some(contents) = read_relative_path(workspace_root, relative_path) else {
        return vec![WorthQueryJournalIdentityScanFailure::new(
            relative_path,
            "missing inventory path",
        )];
    };
    patterns
        .iter()
        .filter(|pattern| contents.contains(**pattern))
        .map(|pattern| WorthQueryJournalIdentityScanFailure::new(relative_path, *pattern))
        .collect()
}

fn path_contains(workspace_root: &Path, relative_path: &str, pattern: &str) -> bool {
    read_relative_path(workspace_root, relative_path)
        .map(|contents| contents.contains(pattern))
        .unwrap_or(false)
}

fn read_relative_path(workspace_root: &Path, relative_path: &str) -> Option<String> {
    std::fs::read_to_string(path_from_workspace_root(workspace_root, relative_path)).ok()
}

fn path_from_workspace_root(workspace_root: &Path, relative_path: &str) -> PathBuf {
    workspace_root.join(relative_path)
}
