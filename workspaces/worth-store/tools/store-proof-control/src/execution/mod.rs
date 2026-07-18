use std::path::Path;
use std::process::Command;
use std::time::Instant;

use serde::{Deserialize, Serialize};

use crate::selection::SelectedProofExecutionPlan;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutedProofRun {
    pub plan_digest: String,
    pub attempt_identity: String,
    pub attempt_started_unix_millis: u128,
    pub attempted_units: usize,
    pub completed_units: usize,
    pub behavioral_verdict: String,
    pub failed_unit: Option<String>,
    pub unit_verdicts: Vec<ProofUnitExecutionVerdict>,
    pub process_counts: ProofRunProcessCounts,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofUnitExecutionVerdict {
    pub unit_identity: String,
    pub case_filter: Option<String>,
    pub process_model: String,
    pub behavioral_verdict: String,
    pub elapsed_millis: u128,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofRunProcessCounts {
    pub cargo_processes_launched: usize,
    pub test_or_check_processes_requested: usize,
    pub declared_subprocess_units: usize,
    pub compiler_process_observation: String,
    pub linker_process_observation: String,
    pub child_process_observation: String,
}

pub fn execute(
    workspace_root: &Path,
    plan: &SelectedProofExecutionPlan,
) -> Result<ExecutedProofRun, String> {
    let attempt_started_unix_millis = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|error| format!("system clock precedes Unix epoch: {error}"))?
        .as_millis();
    let attempt_identity = format!(
        "{}-{attempt_started_unix_millis}-{}",
        &plan.plan_digest[..16],
        std::process::id()
    );
    let mut completed_units = 0;
    let mut unit_verdicts = Vec::new();
    for unit in &plan.units {
        let arguments = unit.cargo_arguments(plan.request.mode());
        let started = Instant::now();
        let mut command = Command::new("cargo");
        command.args(&arguments).current_dir(workspace_root);
        if let Some(seed) = plan.request.seed() {
            command.env("WORTH_STORE_PROOF_SEED", seed.to_string());
        }
        if let Some(backend) = plan.request.backend() {
            command.env("WORTH_STORE_BACKEND_PROFILE", backend);
        }
        let status = command
            .status()
            .map_err(|error| format!("could not launch cargo for {}: {error}", unit.target_name))?;
        let unit_identity = format!("{}::{}", unit.package, unit.target_name);
        unit_verdicts.push(ProofUnitExecutionVerdict {
            unit_identity: unit_identity.clone(),
            case_filter: unit.case_filter.clone(),
            process_model: unit.process_model.clone(),
            behavioral_verdict: if status.success() { "passed" } else { "failed" }.to_owned(),
            elapsed_millis: started.elapsed().as_millis(),
        });
        if !status.success() {
            return Ok(ExecutedProofRun {
                plan_digest: plan.plan_digest.clone(),
                attempt_identity,
                attempt_started_unix_millis,
                attempted_units: plan.units.len(),
                completed_units,
                behavioral_verdict: "failed".to_owned(),
                failed_unit: Some(unit_identity),
                process_counts: process_counts(&unit_verdicts),
                unit_verdicts,
            });
        }
        completed_units += 1;
    }
    Ok(ExecutedProofRun {
        plan_digest: plan.plan_digest.clone(),
        attempt_identity,
        attempt_started_unix_millis,
        attempted_units: plan.units.len(),
        completed_units,
        behavioral_verdict: "passed".to_owned(),
        failed_unit: None,
        process_counts: process_counts(&unit_verdicts),
        unit_verdicts,
    })
}

fn process_counts(verdicts: &[ProofUnitExecutionVerdict]) -> ProofRunProcessCounts {
    ProofRunProcessCounts {
        cargo_processes_launched: verdicts.len(),
        test_or_check_processes_requested: verdicts.len(),
        declared_subprocess_units: verdicts
            .iter()
            .filter(|verdict| verdict.process_model != "libtest-process")
            .count(),
        compiler_process_observation: "not-observed-before-phase-10-runner".to_owned(),
        linker_process_observation: "not-observed-before-phase-10-runner".to_owned(),
        child_process_observation: "declared-not-externally-counted-before-phase-8-and-phase-10"
            .to_owned(),
    }
}
