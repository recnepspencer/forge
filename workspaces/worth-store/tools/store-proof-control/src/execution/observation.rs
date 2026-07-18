use std::collections::BTreeMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::discovery::{observe_artifact_footprint, ObservedArtifactFootprint};
use crate::selection::SelectedProofExecutionPlan;

use super::{ObservedCargoArtifact, ProofRunAttempt};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservedTargetRootCost {
    pub target_root: String,
    pub before: ObservedArtifactFootprint,
    pub after: ObservedArtifactFootprint,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservedProofRunCost {
    pub target_roots: Vec<ObservedTargetRootCost>,
    pub cargo_processes_launched: usize,
    pub test_or_check_processes_requested: usize,
    pub declared_subprocess_evidence: usize,
    pub externally_observed_processes: usize,
    pub externally_observed_compilers: usize,
    pub externally_observed_linkers: usize,
    pub peak_observed_descendants: usize,
    pub observer_authorities: Vec<String>,
    pub cargo_compiler_artifact_messages: usize,
    pub freshly_compiled_cargo_artifacts: usize,
    pub reused_cargo_artifacts: usize,
    pub linked_executable_artifacts: Vec<String>,
    pub compiler_process_observation: String,
    pub linker_process_observation: String,
    pub child_process_observation: String,
}

pub(crate) fn observe_before(
    plan: &SelectedProofExecutionPlan,
) -> Result<BTreeMap<String, ObservedArtifactFootprint>, String> {
    unique_target_roots(plan)
        .into_iter()
        .map(|root| {
            let footprint = observe_artifact_footprint(Path::new(&root))?;
            Ok((root, footprint))
        })
        .collect()
}

pub(crate) fn finish_observation(
    before: BTreeMap<String, ObservedArtifactFootprint>,
    attempts: &[ProofRunAttempt],
) -> Result<ObservedProofRunCost, String> {
    let mut target_roots = Vec::new();
    for (root, before) in before {
        target_roots.push(ObservedTargetRootCost {
            after: observe_artifact_footprint(Path::new(&root))?,
            target_root: root,
            before,
        });
    }
    let mut linked_executable_artifacts: Vec<_> = attempts
        .iter()
        .flat_map(|attempt| attempt.linked_executable_artifacts.clone())
        .collect();
    linked_executable_artifacts.sort();
    linked_executable_artifacts.dedup();
    let linked_executable_count = linked_executable_artifacts.len();
    let cargo_compiler_artifact_messages = attempts
        .iter()
        .map(|attempt| attempt.cargo_compiler_artifact_messages)
        .sum();
    let freshly_compiled_cargo_artifacts = attempts
        .iter()
        .flat_map(|attempt| &attempt.observed_cargo_artifacts)
        .filter(|artifact| !artifact.fresh)
        .count();
    let reused_cargo_artifacts = attempts
        .iter()
        .flat_map(|attempt| &attempt.observed_cargo_artifacts)
        .filter(|artifact| artifact.fresh)
        .count();
    let declared_subprocess_evidence = attempts
        .iter()
        .map(|attempt| attempt.process_probe_evidence.len())
        .sum();
    let receipts: Vec<_> = attempts
        .iter()
        .filter_map(|attempt| attempt.external_observation.as_ref())
        .collect();
    let externally_observed_processes = receipts
        .iter()
        .map(|receipt| receipt.observed_processes.len())
        .sum();
    let externally_observed_compilers = receipts
        .iter()
        .flat_map(|receipt| &receipt.observed_processes)
        .filter(|process| process.classifications.contains("compiler"))
        .count();
    let externally_observed_linkers = receipts
        .iter()
        .flat_map(|receipt| &receipt.observed_processes)
        .filter(|process| process.classifications.contains("linker"))
        .count();
    let peak_observed_descendants = receipts
        .iter()
        .map(|receipt| receipt.peak_observed_descendants)
        .max()
        .unwrap_or(0);
    let mut observer_authorities: Vec<_> = receipts
        .iter()
        .map(|receipt| receipt.observer_authority.clone())
        .collect();
    observer_authorities.sort();
    observer_authorities.dedup();
    Ok(ObservedProofRunCost {
        target_roots,
        cargo_processes_launched: attempts.len(),
        test_or_check_processes_requested: attempts.len(),
        declared_subprocess_evidence,
        externally_observed_processes,
        externally_observed_compilers,
        externally_observed_linkers,
        peak_observed_descendants,
        observer_authorities,
        cargo_compiler_artifact_messages,
        freshly_compiled_cargo_artifacts,
        reused_cargo_artifacts,
        linked_executable_artifacts,
        compiler_process_observation: format!(
            "external observer sampled {externally_observed_compilers} compiler processes; Cargo emitted {cargo_compiler_artifact_messages} exact completed compiler-artifact messages"
        ),
        linker_process_observation: format!(
            "external observer sampled {externally_observed_linkers} linker processes; Cargo emitted {} linked executable artifacts",
            linked_executable_count
        ),
        child_process_observation: format!(
            "external observer sampled {externally_observed_processes} root/descendant processes with peak {peak_observed_descendants}; role-bound probes emitted {declared_subprocess_evidence} receipts"
        ),
    })
}

pub(crate) fn cargo_artifacts(stdout_path: &Path) -> Result<Vec<ObservedCargoArtifact>, String> {
    super::cargo_artifact::read_cargo_artifacts(stdout_path)
}

fn unique_target_roots(plan: &SelectedProofExecutionPlan) -> Vec<String> {
    let mut roots: Vec<_> = plan
        .units
        .iter()
        .map(|unit| unit.resources.target_root.clone())
        .collect();
    roots.sort();
    roots.dedup();
    roots
}
