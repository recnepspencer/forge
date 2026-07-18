use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use super::super::cargo_surface::normalized;

pub(super) fn candidate_source_paths(
    workspace_root: &Path,
    package_sources: &BTreeMap<String, Vec<PathBuf>>,
) -> Result<BTreeSet<PathBuf>, String> {
    let all_sources: BTreeSet<_> = package_sources.values().flatten().cloned().collect();
    let tracked_tests = git_lines(
        workspace_root,
        &["grep", "-l", "-F", "#[test]", "--", "*.rs"],
        true,
    )?;
    let tracked_doctests = git_lines(
        workspace_root,
        &["grep", "-l", "-F", "```", "--", "*.rs"],
        true,
    )?;
    let tracked_included_doctests = git_lines(
        workspace_root,
        &["grep", "-l", "-F", "include_str!", "--", "*.rs"],
        true,
    )?;
    let untracked = git_lines(
        workspace_root,
        &["ls-files", "--others", "--exclude-standard", "--", "*.rs"],
        false,
    )?;
    let selected: BTreeSet<_> = tracked_tests
        .into_iter()
        .chain(tracked_doctests)
        .chain(tracked_included_doctests)
        .chain(untracked)
        .map(|path| normalized(&workspace_root.join(path)))
        .collect();
    Ok(all_sources
        .into_iter()
        .filter(|path| selected.contains(&normalized(path)) || is_ui_fixture(path))
        .collect())
}

pub(super) fn read_source_snapshot(
    paths: &BTreeSet<PathBuf>,
) -> Result<BTreeMap<String, String>, String> {
    let paths: Vec<_> = paths.iter().cloned().collect();
    let workers = std::thread::available_parallelism()
        .map(usize::from)
        .unwrap_or(1)
        .min(paths.len().max(1));
    let chunk_size = paths.len().div_ceil(workers).max(1);
    let batches = std::thread::scope(|scope| {
        paths
            .chunks(chunk_size)
            .map(|chunk| scope.spawn(move || read_batch(chunk)))
            .collect::<Vec<_>>()
            .into_iter()
            .map(|worker| {
                worker
                    .join()
                    .map_err(|_| "source snapshot worker panicked".to_owned())?
            })
            .collect::<Result<Vec<_>, String>>()
    })?;
    Ok(batches.into_iter().flatten().collect())
}

pub(super) fn rust_sources(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut pending = vec![root.to_path_buf()];
    let mut sources = Vec::new();
    while let Some(directory) = pending.pop() {
        for entry in fs::read_dir(&directory)
            .map_err(|error| format!("could not inspect {}: {error}", directory.display()))?
        {
            let entry = entry.map_err(|error| format!("could not inspect entry: {error}"))?;
            let path = entry.path();
            if path.file_name().is_some_and(|name| name == "target") {
                continue;
            }
            if entry.file_type().is_ok_and(|value| value.is_dir()) {
                pending.push(path);
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                sources.push(path);
            }
        }
    }
    sources.sort();
    Ok(sources)
}

pub(super) fn is_ui_fixture(path: &Path) -> bool {
    let path = normalized(path);
    (path.contains("/tests/ui/") || path.contains("/tests/compile_fail/"))
        && !path.ends_with("_runner.rs")
        && !path.contains("/src/")
}

fn read_batch(paths: &[PathBuf]) -> Result<Vec<(String, String)>, String> {
    paths
        .iter()
        .map(|path| {
            let text = fs::read_to_string(path)
                .map_err(|error| format!("could not read {}: {error}", path.display()))?;
            Ok((normalized(path), text))
        })
        .collect()
}

fn git_lines(
    workspace_root: &Path,
    arguments: &[&str],
    no_matches_is_success: bool,
) -> Result<Vec<String>, String> {
    let output = Command::new("git")
        .args(arguments)
        .current_dir(workspace_root)
        .output()
        .map_err(|error| format!("could not launch git source discovery: {error}"))?;
    if !output.status.success() && !(no_matches_is_success && output.status.code() == Some(1)) {
        return Err(format!(
            "git source discovery failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(str::to_owned)
        .collect())
}
