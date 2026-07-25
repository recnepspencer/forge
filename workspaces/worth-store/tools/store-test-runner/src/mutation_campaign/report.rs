use std::{
    collections::BTreeSet,
    io::Write,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

use serde::Serialize;
use sha2::{Digest, Sha256};

use super::{evidence::MutationObservation, source_inventory::MutationSourceBinding};

#[cfg(feature = "physical-work-evidence")]
mod reader;

pub(crate) const MUTATION_EVIDENCE_REPORT_SCHEMA: &str = "worth.store.c5_1.mutation-evidence.v2";
const ARTIFACT_OWNER_SCHEMA: &str = "worth.store.c5_1.mutation-artifacts.v1";
const LEGACY_REPORT_SCHEMA: &str = "worth.store.c5_1.mutation-evidence.v1";
const ARTIFACT_OWNER_MARKER: &str = ".worth-store-mutation-evidence-owner";
static SESSION_SEQUENCE: AtomicU64 = AtomicU64::new(1);

#[derive(Serialize)]
struct MutationEvidenceReport<'evidence> {
    schema: &'static str,
    source: &'evidence MutationSourceBinding,
    observations: &'evidence [MutationObservation],
}

pub(super) struct MutationEvidenceSession {
    report: PathBuf,
    staging: PathBuf,
    published_artifacts: PathBuf,
    source: MutationSourceBinding,
    seen: BTreeSet<u8>,
    artifacts_moved: bool,
    published: bool,
}

impl MutationEvidenceSession {
    pub(super) fn begin(path: &Path, source: MutationSourceBinding) -> Result<Self, String> {
        let report = normalized_report(path)?;
        let published_artifacts = published_artifact_directory(&report)?;
        validate_owned_artifacts(&published_artifacts, &report)?;
        remove_prior_report(&report)?;
        remove_owned_artifacts(&published_artifacts)?;
        let staging = staging_artifact_directory(&report)?;
        std::fs::create_dir(&staging).map_err(|error| {
            format!(
                "cannot create mutation artifact directory {}: {error}",
                staging.display()
            )
        })?;
        std::fs::write(
            staging.join(ARTIFACT_OWNER_MARKER),
            owner_marker(&report).as_bytes(),
        )
        .map_err(|error| format!("cannot write mutation artifact owner marker: {error}"))?;
        Ok(Self {
            report,
            staging,
            published_artifacts,
            source,
            seen: BTreeSet::new(),
            artifacts_moved: false,
            published: false,
        })
    }

    pub(super) fn retain_binary(
        &mut self,
        observation: &mut MutationObservation,
    ) -> Result<(), String> {
        if !self.seen.insert(observation.id) {
            return Err(format!(
                "mutation report received duplicate mutant {}",
                observation.id
            ));
        }
        let source = absolute(Path::new(&observation.binary_binding))?;
        let extension = source.extension().and_then(|value| value.to_str());
        let file_name = match extension {
            Some(extension) => format!(
                "mutant-{:02}-{}.{}",
                observation.id, observation.binary_sha256, extension
            ),
            None => format!("mutant-{:02}-{}", observation.id, observation.binary_sha256),
        };
        let staged = self.staging.join(&file_name);
        std::fs::copy(&source, &staged).map_err(|error| {
            format!(
                "cannot retain mutant {} binary {}: {error}",
                observation.id,
                source.display()
            )
        })?;
        if hash_file(&staged)? != observation.binary_sha256 {
            return Err(format!(
                "retained mutant {} binary changed while copying",
                observation.id
            ));
        }
        observation.binary_binding = self
            .published_artifacts
            .join(file_name)
            .display()
            .to_string();
        Ok(())
    }

    pub(super) fn publish(
        mut self,
        observations: &[MutationObservation],
        current_source: &MutationSourceBinding,
    ) -> Result<(), String> {
        if &self.source != current_source {
            return Err("mutation campaign source changed before publication".into());
        }
        let observation_ids = observations
            .iter()
            .map(|observation| observation.id)
            .collect::<BTreeSet<_>>();
        if observation_ids != self.seen || observation_ids.len() != observations.len() {
            return Err("mutation report observations do not match retained binaries".into());
        }
        let pending = self.staging.join("report.pending.json");
        let mut file = std::fs::File::create(&pending)
            .map_err(|error| format!("cannot create {}: {error}", pending.display()))?;
        file.write_all(&encode(&self.source, observations)?)
            .map_err(|error| format!("cannot write {}: {error}", pending.display()))?;
        file.sync_all()
            .map_err(|error| format!("cannot synchronize {}: {error}", pending.display()))?;
        drop(file);
        std::fs::rename(&self.staging, &self.published_artifacts).map_err(|error| {
            format!(
                "cannot publish mutation artifacts {}: {error}",
                self.published_artifacts.display()
            )
        })?;
        self.artifacts_moved = true;
        let published_report = self.published_artifacts.join("report.pending.json");
        std::fs::rename(&published_report, &self.report).map_err(|error| {
            format!(
                "cannot publish mutation report {}: {error}",
                self.report.display()
            )
        })?;
        self.published = true;
        Ok(())
    }
}

impl Drop for MutationEvidenceSession {
    fn drop(&mut self) {
        if !self.published {
            let artifacts = if self.artifacts_moved {
                &self.published_artifacts
            } else {
                &self.staging
            };
            let _ = std::fs::remove_dir_all(artifacts);
        }
    }
}

fn encode(
    source: &MutationSourceBinding,
    observations: &[MutationObservation],
) -> Result<Vec<u8>, String> {
    serde_json::to_vec_pretty(&MutationEvidenceReport {
        schema: MUTATION_EVIDENCE_REPORT_SCHEMA,
        source,
        observations,
    })
    .map_err(|error| format!("cannot encode mutation report: {error}"))
}

fn absolute(path: &Path) -> Result<PathBuf, String> {
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        std::env::current_dir()
            .map(|current| current.join(path))
            .map_err(|error| format!("cannot resolve current directory: {error}"))
    }
}

fn normalized_report(path: &Path) -> Result<PathBuf, String> {
    let absolute = absolute(path)?;
    let parent = absolute
        .parent()
        .ok_or_else(|| format!("mutation report {} has no parent", absolute.display()))?;
    std::fs::create_dir_all(parent)
        .map_err(|error| format!("cannot create {}: {error}", parent.display()))?;
    let parent = parent
        .canonicalize()
        .map_err(|error| format!("cannot canonicalize {}: {error}", parent.display()))?;
    let name = absolute
        .file_name()
        .ok_or_else(|| "mutation report has no filename".to_owned())?;
    Ok(parent.join(name))
}

fn remove_prior_report(report: &Path) -> Result<(), String> {
    match std::fs::symlink_metadata(report) {
        Ok(metadata) if metadata.is_dir() => Err(format!(
            "mutation report path {} is a directory",
            report.display()
        )),
        Ok(_) => std::fs::remove_file(report)
            .map_err(|error| format!("cannot invalidate {}: {error}", report.display())),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(format!(
            "cannot inspect mutation report {}: {error}",
            report.display()
        )),
    }
}

fn published_artifact_directory(report: &Path) -> Result<PathBuf, String> {
    let (parent, name) = report_parent_and_name(report)?;
    let parent = parent
        .canonicalize()
        .map_err(|error| format!("cannot canonicalize {}: {error}", parent.display()))?;
    Ok(parent.join(format!("{name}.artifacts.current")))
}

fn staging_artifact_directory(report: &Path) -> Result<PathBuf, String> {
    let (parent, name) = report_parent_and_name(report)?;
    for _ in 0..32 {
        let sequence = SESSION_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let candidate = parent.join(format!(
            "{name}.artifacts.pending.{}.{}",
            std::process::id(),
            sequence
        ));
        if !candidate.exists() {
            return Ok(candidate);
        }
    }
    Err("cannot allocate a unique mutation artifact staging directory".into())
}

fn report_parent_and_name(report: &Path) -> Result<(&Path, &str), String> {
    let parent = report
        .parent()
        .ok_or_else(|| format!("mutation report {} has no parent", report.display()))?;
    let name = report
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| "mutation report filename must be Unicode".to_owned())?;
    Ok((parent, name))
}

fn validate_owned_artifacts(artifacts: &Path, report: &Path) -> Result<(), String> {
    match std::fs::symlink_metadata(artifacts) {
        Ok(metadata) if !metadata.is_dir() => Err(format!(
            "mutation artifact path {} is not an owned directory",
            artifacts.display()
        )),
        Ok(_) => {
            let marker =
                std::fs::read_to_string(artifacts.join(ARTIFACT_OWNER_MARKER)).map_err(|_| {
                    format!(
                        "refusing to replace unmarked mutation artifacts {}",
                        artifacts.display()
                    )
                })?;
            if marker == owner_marker(report) || marker == legacy_owner_marker(report) {
                Ok(())
            } else {
                Err(format!(
                    "refusing to replace foreign mutation artifacts {}",
                    artifacts.display()
                ))
            }
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(format!(
            "cannot inspect mutation artifacts {}: {error}",
            artifacts.display()
        )),
    }
}

fn remove_owned_artifacts(artifacts: &Path) -> Result<(), String> {
    match std::fs::remove_dir_all(artifacts) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(format!(
            "cannot replace mutation artifacts {}: {error}",
            artifacts.display()
        )),
    }
}

fn owner_marker(report: &Path) -> String {
    format!("{ARTIFACT_OWNER_SCHEMA}\n{}\n", report.display())
}

fn legacy_owner_marker(report: &Path) -> String {
    format!("{LEGACY_REPORT_SCHEMA}\n{}\n", report.display())
}

fn hash_file(path: &Path) -> Result<String, String> {
    let bytes =
        std::fs::read(path).map_err(|error| format!("cannot hash {}: {error}", path.display()))?;
    Ok(Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect())
}

#[cfg(feature = "physical-work-evidence")]
pub(super) fn load_physical_work_evidence(
    report: &Path,
    workspace: &Path,
) -> Result<Vec<worth_store::physical_runtime::PhysicalWorkMutantLocalization>, String> {
    reader::load_physical_work_evidence(report, workspace)
}

#[cfg(test)]
mod tests;
