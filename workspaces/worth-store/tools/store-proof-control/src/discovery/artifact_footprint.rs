use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use super::cargo_surface::normalized;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservedArtifactFootprint {
    pub target_root: String,
    pub observation_status: String,
    pub file_count: u64,
    pub logical_bytes: u64,
    pub produced_executables: u64,
    pub pdb_files: u64,
    pub rlib_files: u64,
    pub rmeta_files: u64,
    pub incremental_directories: u64,
    pub extension_counts: BTreeMap<String, u64>,
}

impl ObservedArtifactFootprint {
    pub fn not_observed(target_root: &str) -> Self {
        Self {
            target_root: target_root.to_owned(),
            observation_status: "not_observed".to_owned(),
            file_count: 0,
            logical_bytes: 0,
            produced_executables: 0,
            pdb_files: 0,
            rlib_files: 0,
            rmeta_files: 0,
            incremental_directories: 0,
            extension_counts: BTreeMap::new(),
        }
    }
}

pub fn observe_artifact_footprint(target_root: &Path) -> Result<ObservedArtifactFootprint, String> {
    if !target_root.exists() {
        let mut absent = ObservedArtifactFootprint::not_observed(&normalized(target_root));
        absent.observation_status = "target_root_absent".to_owned();
        return Ok(absent);
    }
    let mut footprint = ObservedArtifactFootprint::not_observed(&normalized(target_root));
    footprint.observation_status = "historical_target_observed".to_owned();
    let mut pending = vec![PathBuf::from(target_root)];
    while let Some(directory) = pending.pop() {
        if directory
            .file_name()
            .is_some_and(|name| name == "incremental")
        {
            footprint.incremental_directories += 1;
        }
        let entries = fs::read_dir(&directory)
            .map_err(|error| format!("could not inspect {}: {error}", directory.display()))?;
        for entry in entries {
            let entry = entry.map_err(|error| {
                format!(
                    "could not inspect entry under {}: {error}",
                    directory.display()
                )
            })?;
            let file_type = entry.file_type().map_err(|error| {
                format!("could not classify {}: {error}", entry.path().display())
            })?;
            if file_type.is_dir() {
                pending.push(entry.path());
                continue;
            }
            if !file_type.is_file() {
                continue;
            }
            footprint.file_count += 1;
            footprint.logical_bytes += entry.metadata().map(|value| value.len()).unwrap_or(0);
            let extension = entry
                .path()
                .extension()
                .and_then(|value| value.to_str())
                .unwrap_or("<none>")
                .to_ascii_lowercase();
            *footprint
                .extension_counts
                .entry(extension.clone())
                .or_default() += 1;
            match extension.as_str() {
                "exe" => footprint.produced_executables += 1,
                "pdb" => footprint.pdb_files += 1,
                "rlib" => footprint.rlib_files += 1,
                "rmeta" => footprint.rmeta_files += 1,
                _ => {}
            }
        }
    }
    Ok(footprint)
}
