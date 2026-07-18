use std::path::Path;
use std::process::Command;
use std::time::Instant;

use serde::{Deserialize, Serialize};

use crate::selection::SelectedProofExecutionPlan;
use crate::structural_preflight::{forge_root, require_fresh, PreflightEvidenceFreshness};
use worth_store_test_support::structural_preflight::{
    StructuralPreflightEvidence, STRUCTURAL_PREFLIGHT_BUNDLE_ENV,
};

mod ui_evidence;
mod process_evidence;
pub use process_evidence::ProcessProbeEvidenceReference;
pub use ui_evidence::UiProofEvidenceReference;

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
    pub structural_preflight_evidence_identity: String,
    pub structural_preflight_bundle_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofUnitExecutionVerdict {
    pub unit_identity: String,
    pub case_filter: Option<String>,
    pub process_model: String,
    pub behavioral_verdict: String,
    pub elapsed_millis: u128,
    pub ui_proof_evidence: Vec<UiProofEvidenceReference>,
    pub process_probe_evidence: Vec<ProcessProbeEvidenceReference>,
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
    let preflight = validate_preflight(workspace_root, plan)?;
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
    for (unit_index, unit) in plan.units.iter().enumerate() {
        let arguments = unit.cargo_arguments(plan.request.mode());
        let started = Instant::now();
        let unit_identity = format!("{}::{}", unit.package, unit.target_name);
        let ui_evidence_root = ui_evidence::attempt_root(
            workspace_root,
            &attempt_identity,
            unit_index,
            &unit_identity,
        );
        let process_evidence_root = process_evidence::attempt_root(
            workspace_root,
            &attempt_identity,
            unit_index,
            &unit_identity,
        );
        let mut command = Command::new("cargo");
        command
            .args(&arguments)
            .current_dir(workspace_root)
            .env(
                worth_store_test_support::compiler_boundary::UI_EVIDENCE_ROOT_ENV,
                &ui_evidence_root,
            )
            .env(STRUCTURAL_PREFLIGHT_BUNDLE_ENV, &preflight.bundle_path)
            .env(
                process_evidence::PROCESS_PROBE_EVIDENCE_ROOT_ENV,
                &process_evidence_root,
            );
        if let Some(seed) = plan.request.seed() {
            command.env("WORTH_STORE_PROOF_SEED", seed.to_string());
        }
        if let Some(backend) = plan.request.backend() {
            command.env("WORTH_STORE_BACKEND_PROFILE", backend);
        }
        let status = command
            .status()
            .map_err(|error| format!("could not launch cargo for {}: {error}", unit.target_name))?;
        let ui_proof_evidence = ui_evidence::collect(
            workspace_root,
            &ui_evidence_root,
            &unit_identity,
            unit.process_model == "compiler-boundary-suite" && status.success(),
        )?;
        let process_probe_evidence = process_evidence::collect(
            workspace_root,
            &process_evidence_root,
            &unit_identity,
            status.success()
                && matches!(
                    unit.process_model.as_str(),
                    "libtest-with-fresh-child-process" | "libtest-with-declared-subprocesses"
                ),
        )?;
        unit_verdicts.push(ProofUnitExecutionVerdict {
            unit_identity: unit_identity.clone(),
            case_filter: unit.case_filter.clone(),
            process_model: unit.process_model.clone(),
            behavioral_verdict: if status.success() { "passed" } else { "failed" }.to_owned(),
            elapsed_millis: started.elapsed().as_millis(),
            ui_proof_evidence,
            process_probe_evidence,
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
                structural_preflight_evidence_identity: preflight.evidence_identity.clone(),
                structural_preflight_bundle_path: preflight.bundle_path.clone(),
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
        structural_preflight_evidence_identity: preflight.evidence_identity,
        structural_preflight_bundle_path: preflight.bundle_path,
        unit_verdicts,
    })
}

fn validate_preflight(
    workspace_root: &Path,
    plan: &SelectedProofExecutionPlan,
) -> Result<ValidatedPreflight, String> {
    let bundle_path = std::path::PathBuf::from(&plan.structural_preflight.bundle_path);
    let evidence: StructuralPreflightEvidence = crate::evidence::read_json(&bundle_path)?;
    if evidence.evidence_identity.0 != plan.structural_preflight.evidence_identity {
        return Err(format!(
            "preflight bundle identity mismatch: plan={} bundle={}",
            plan.structural_preflight.evidence_identity, evidence.evidence_identity.0
        ));
    }
    if !evidence.failures().is_empty() {
        return Err("behavioral execution cannot consume a failed preflight bundle".to_owned());
    }
    let forge_root = forge_root(workspace_root)?;
    match require_fresh(&forge_root, &evidence)? {
        PreflightEvidenceFreshness::Fresh { .. } => Ok(ValidatedPreflight {
            evidence_identity: evidence.evidence_identity.0,
            bundle_path: bundle_path.to_string_lossy().replace('\\', "/"),
        }),
        PreflightEvidenceFreshness::Stale { failures } => Err(format!(
            "structural preflight is stale:\n  - {}",
            failures
                .iter()
                .map(|failure| format!(
                    "{:?}: {} ({})",
                    failure.predicate,
                    failure.message,
                    failure.invalidated_inputs.join(", ")
                ))
                .collect::<Vec<_>>()
                .join("\n  - ")
        )),
    }
}

struct ValidatedPreflight {
    evidence_identity: String,
    bundle_path: String,
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
