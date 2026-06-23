use std::fs;
use std::path::{Path, PathBuf};

use crate::certification::error::TopologyCertificationError;
use crate::certification::shared::digest_rows;

use super::{
    TopologyQueryBoundaryCleanupArea, TopologyQueryBoundaryCleanupRow,
    TopologyQueryBoundaryCleanupStatus,
};

pub(super) fn closed_row(
    area: TopologyQueryBoundaryCleanupArea,
    reason: &str,
    designated_survivor: Option<&str>,
    evidence_paths: impl IntoIterator<Item = &'static str>,
) -> Result<TopologyQueryBoundaryCleanupRow, TopologyCertificationError> {
    let survivor = designated_survivor.map(str::to_string);
    let evidence = evidence_paths
        .into_iter()
        .map(|path| source_text(path).map(|source| format!("path:{path}\n{source}")))
        .collect::<Result<Vec<_>, _>>()?;
    let row_digest = digest_rows(evidence.into_iter());
    Ok(TopologyQueryBoundaryCleanupRow {
        area,
        status: TopologyQueryBoundaryCleanupStatus::Closed,
        reason: reason.to_string(),
        designated_survivor: survivor,
        row_digest,
    })
}

#[track_caller]
pub(super) fn ensure(condition: bool) -> Result<(), TopologyCertificationError> {
    if condition {
        Ok(())
    } else {
        let location = std::panic::Location::caller();
        Err(TopologyCertificationError::ReadView(format!(
            "worth-topo query boundary cleanup closeout structural proof failed at {}:{}",
            location.file(),
            location.line()
        )))
    }
}

#[track_caller]
pub(super) fn ensure_all(
    sources: &[String],
    predicate: impl Fn(&str) -> bool,
) -> Result<(), TopologyCertificationError> {
    ensure(sources.iter().all(|source| predicate(source)))
}

pub(super) fn source_text(relative: &str) -> Result<String, TopologyCertificationError> {
    fs::read_to_string(workspace_path(relative))
        .map_err(|error| TopologyCertificationError::ReadView(error.to_string()))
}

pub(super) fn domain_view_sources() -> Result<Vec<String>, TopologyCertificationError> {
    let mut sources = collect_rs_sources("src/projection/read_views/domain/views")?;
    sources.retain(|(path, _)| !path.ends_with("boundary_tests.rs") && !path.ends_with("mod.rs"));
    Ok(sources.into_iter().map(|(_, source)| source).collect())
}

pub(super) fn collect_rs_sources(
    relative: &str,
) -> Result<Vec<(String, String)>, TopologyCertificationError> {
    let dir = workspace_path(relative);
    let mut sources = fs::read_dir(&dir)
        .map_err(|error| TopologyCertificationError::ReadView(error.to_string()))?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "rs"))
        .map(|path| {
            let source = fs::read_to_string(&path)
                .map_err(|error| TopologyCertificationError::ReadView(error.to_string()))?;
            let relative = path
                .strip_prefix(workspace_root())
                .expect("source file should live inside workspace")
                .to_string_lossy()
                .replace('\\', "/");
            Ok((relative, source))
        })
        .collect::<Result<Vec<_>, TopologyCertificationError>>()?;
    sources.sort_by(|left, right| left.0.cmp(&right.0));
    Ok(sources)
}

pub(super) fn collect_rs_sources_recursive(
    relative: &str,
) -> Result<Vec<(String, String)>, TopologyCertificationError> {
    let mut sources = Vec::new();
    collect_rs_sources_recursive_from(&workspace_path(relative), &mut sources)?;
    sources.sort_by(|left, right| left.0.cmp(&right.0));
    Ok(sources)
}

fn collect_rs_sources_recursive_from(
    dir: &Path,
    sources: &mut Vec<(String, String)>,
) -> Result<(), TopologyCertificationError> {
    for entry in fs::read_dir(dir)
        .map_err(|error| TopologyCertificationError::ReadView(error.to_string()))?
    {
        let path = entry
            .map_err(|error| TopologyCertificationError::ReadView(error.to_string()))?
            .path();
        if path.is_dir() {
            collect_rs_sources_recursive_from(&path, sources)?;
            continue;
        }
        if path.extension().is_none_or(|ext| ext != "rs") {
            continue;
        }
        let source = fs::read_to_string(&path)
            .map_err(|error| TopologyCertificationError::ReadView(error.to_string()))?;
        let relative = path
            .strip_prefix(workspace_root())
            .expect("source file should live inside workspace")
            .to_string_lossy()
            .replace('\\', "/");
        sources.push((relative, source));
    }
    Ok(())
}

fn workspace_path(relative: &str) -> PathBuf {
    workspace_root().join(relative)
}

fn workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}
