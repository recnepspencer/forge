use std::{
    collections::BTreeSet,
    io::Write,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

use serde::Serialize;
#[cfg(feature = "physical-work-evidence")]
use sha2::{Digest, Sha256};

use super::{
    evidence::MutationObservation, source_inventory::MutationSourceBinding, MutationCampaignScope,
};

mod c8_record {
    pub(super) use crate::mutation_campaign::c8_retained_record::*;
}

#[cfg(feature = "physical-work-evidence")]
mod reader;

pub(crate) const MUTATION_EVIDENCE_REPORT_SCHEMA: &str =
    "worth.store.controlled-mutation-evidence.v5";
static SESSION_SEQUENCE: AtomicU64 = AtomicU64::new(1);

#[derive(Serialize)]
struct MutationEvidenceReport<'evidence> {
    schema: &'static str,
    scope: MutationCampaignScope,
    source: &'evidence MutationSourceBinding,
    observations: &'evidence [MutationObservation],
}

pub(super) struct MutationEvidenceSession {
    report: PathBuf,
    pending: PathBuf,
    source: MutationSourceBinding,
    scope: MutationCampaignScope,
    c8_source_closure: Option<String>,
    published: bool,
}

impl MutationEvidenceSession {
    pub(super) fn begin(
        path: &Path,
        source: MutationSourceBinding,
        scope: MutationCampaignScope,
        workspace: &Path,
    ) -> Result<Self, String> {
        let report = normalized_report(path)?;
        remove_prior_report(&report)?;
        let pending = pending_report(&report)?;
        Ok(Self {
            report,
            pending,
            source,
            scope,
            c8_source_closure: matches!(scope, MutationCampaignScope::C8Closure)
                .then(|| c8_record::phase_eight_source_closure(workspace))
                .transpose()?,
            published: false,
        })
    }

    pub(super) fn publish(
        mut self,
        observations: &[MutationObservation],
        current_source: &MutationSourceBinding,
    ) -> Result<(), String> {
        if &self.source != current_source {
            return Err("mutation campaign source changed before publication".into());
        }
        let identities = observations
            .iter()
            .map(|observation| observation.id)
            .collect::<BTreeSet<_>>();
        if identities.len() != observations.len() {
            return Err("mutation report contains duplicate mutant identities".into());
        }
        let encoded = match self.c8_source_closure.as_deref() {
            Some(source_closure) => c8_record::encode(&self.source, source_closure, observations)?,
            None => encode(self.scope, &self.source, observations)?,
        };
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&self.pending)
            .map_err(|error| format!("cannot create {}: {error}", self.pending.display()))?;
        file.write_all(&encoded)
            .map_err(|error| format!("cannot write {}: {error}", self.pending.display()))?;
        file.sync_all()
            .map_err(|error| format!("cannot synchronize {}: {error}", self.pending.display()))?;
        drop(file);
        std::fs::rename(&self.pending, &self.report).map_err(|error| {
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
            let _ = std::fs::remove_file(&self.pending);
        }
    }
}

fn encode(
    scope: MutationCampaignScope,
    source: &MutationSourceBinding,
    observations: &[MutationObservation],
) -> Result<Vec<u8>, String> {
    serde_json::to_vec_pretty(&MutationEvidenceReport {
        schema: MUTATION_EVIDENCE_REPORT_SCHEMA,
        scope,
        source,
        observations,
    })
    .map_err(|error| format!("cannot encode mutation report: {error}"))
}

fn normalized_report(path: &Path) -> Result<PathBuf, String> {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .map(|current| current.join(path))
            .map_err(|error| format!("cannot resolve current directory: {error}"))?
    };
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

fn pending_report(report: &Path) -> Result<PathBuf, String> {
    let parent = report
        .parent()
        .ok_or_else(|| format!("mutation report {} has no parent", report.display()))?;
    let name = report
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| "mutation report filename must be Unicode".to_owned())?;
    for _ in 0..32 {
        let sequence = SESSION_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let candidate = parent.join(format!(
            ".{name}.pending.{}.{}",
            std::process::id(),
            sequence
        ));
        if !candidate.exists() {
            return Ok(candidate);
        }
    }
    Err("cannot allocate a unique pending mutation report".into())
}

#[cfg(feature = "physical-work-evidence")]
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
    reader::load_evidence(report, workspace, MutationCampaignScope::PhysicalWork)
}

#[cfg(feature = "physical-work-evidence")]
pub(super) fn load_bounded_residency_evidence(
    report: &Path,
    workspace: &Path,
) -> Result<Vec<worth_store::physical_runtime::PhysicalWorkMutantLocalization>, String> {
    reader::load_evidence(report, workspace, MutationCampaignScope::BoundedResidency)
}

#[cfg(feature = "physical-work-evidence")]
#[allow(dead_code)]
pub(super) fn load_c8_closure_record(
    report: &Path,
    workspace: &Path,
) -> Result<super::c8_retained_record::RetainedC8CampaignRecord, String> {
    c8_record::load(report, workspace, super::catalog::c8_closure_mutations())
}

#[cfg(test)]
mod tests;
