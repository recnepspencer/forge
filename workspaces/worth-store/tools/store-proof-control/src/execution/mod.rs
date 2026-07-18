use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

use serde::{Deserialize, Serialize};

use crate::selection::{
    ProofFailurePolicy, SelectedProofExecutionPlan, StructuralPreflightReference,
};
use crate::structural_preflight::{forge_root, require_fresh, PreflightEvidenceFreshness};
use worth_store_test_support::structural_preflight::StructuralPreflightEvidence;

mod attempt;
mod cargo_artifact;
mod command_attempt;
#[cfg(test)]
mod command_attempt_tests;
mod external_observer;
mod formal_evidence;
#[cfg(test)]
mod formal_evidence_tests;
mod observation;
mod process_evidence;
mod run_integrity;
mod schedule;
mod ui_evidence;

pub use attempt::{ProofAttemptOutcome, ProofRunAttempt, ProofUnitExecutionVerdict};
pub use cargo_artifact::{
    CargoArtifactEquivalenceIdentity, CargoArtifactSemanticIdentity, ObservedCargoArtifact,
};
pub use external_observer::{ExternalObservationReceipt, ExternalObservedProcess};
pub use formal_evidence::FormalToolEvidenceReference;
pub use observation::ObservedProofRunCost;
pub use process_evidence::ProcessProbeEvidenceReference;
pub use schedule::ProofExecutionSchedule;
pub use ui_evidence::UiProofEvidenceReference;

pub(crate) fn observe_external_request(request_path: &Path) -> Result<(), String> {
    external_observer::observe_request(request_path)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutedProofRun {
    schema_version: u32,
    evidence_identity: String,
    pub plan_digest: String,
    pub schedule: ProofExecutionSchedule,
    pub run_identity: String,
    pub run_started_unix_millis: u128,
    pub planned_units: usize,
    pub executed_units: usize,
    pub passed_units: usize,
    pub failed_units: usize,
    pub skipped_units: Vec<SkippedProofExecutionUnit>,
    pub behavioral_verdict: String,
    pub failed_unit: Option<String>,
    pub unit_verdicts: Vec<ProofUnitExecutionVerdict>,
    pub attempts: Vec<ProofRunAttempt>,
    pub observed_cost: ObservedProofRunCost,
    pub structural_preflight_evidence_identity: String,
    pub structural_preflight_bundle_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkippedProofExecutionUnit {
    pub unit_identity: String,
    pub reason: String,
    pub blocking_units: Vec<String>,
}

pub fn execute(
    workspace_root: &Path,
    plan: &SelectedProofExecutionPlan,
) -> Result<ExecutedProofRun, String> {
    let preflight = validate_preflight(workspace_root, &plan.structural_preflight)?;
    execute_validated(workspace_root, plan, preflight)
}

fn execute_validated(
    workspace_root: &Path,
    plan: &SelectedProofExecutionPlan,
    preflight: ValidatedPreflight,
) -> Result<ExecutedProofRun, String> {
    let schedule = schedule::schedule(plan)?;
    let run_started_unix_millis = unix_millis()?;
    let run_identity = format!(
        "{}-{run_started_unix_millis}-{}-{}",
        &plan.plan_digest[..16],
        std::process::id(),
        RUN_NONCE.fetch_add(1, Ordering::Relaxed)
    );
    let before = observation::observe_before(plan)?;
    let mut attempts_by_unit = BTreeMap::new();
    let mut skipped = BTreeMap::new();
    let mut failed = BTreeSet::new();

    for wave in &schedule.waves {
        let runnable: Vec<_> = wave
            .unit_indices
            .iter()
            .copied()
            .filter(|index| {
                classify_skip(plan, *index, &failed, &skipped).map_or(true, |denial| {
                    skipped.insert(*index, denial);
                    false
                })
            })
            .collect();
        if runnable.is_empty() {
            continue;
        }
        let results = std::thread::scope(|scope| {
            let preflight = &preflight;
            let run_identity = run_identity.as_str();
            let handles: Vec<_> = runnable
                .iter()
                .copied()
                .map(|index| {
                    let handle = scope.spawn(move || {
                        command_attempt::execute_unit(
                            workspace_root,
                            plan,
                            preflight,
                            run_identity,
                            index,
                        )
                    });
                    (index, handle)
                })
                .collect();
            handles
                .into_iter()
                .map(|(index, handle)| {
                    let result = handle
                        .join()
                        .map_err(|_| format!("execution worker panicked for unit {index}"))?;
                    result.map(|attempts| (index, attempts))
                })
                .collect::<Result<Vec<_>, String>>()
        })?;
        for (index, attempts) in results {
            let verdict = ProofUnitExecutionVerdict::from_attempts(
                plan.units[index].identity(),
                plan.units[index].case_filter.clone(),
                plan.units[index].process_model,
                &attempts,
            );
            if verdict.behavioral_verdict != "passed" {
                failed.insert(index);
            }
            attempts_by_unit.insert(index, attempts);
        }
    }

    let executed: BTreeSet<_> = attempts_by_unit.keys().copied().collect();
    classify_remaining_skips(plan, &executed, &failed, &mut skipped);
    let mut attempts = Vec::new();
    let mut unit_verdicts = Vec::new();
    for (index, unit_attempts) in attempts_by_unit {
        unit_verdicts.push(ProofUnitExecutionVerdict::from_attempts(
            plan.units[index].identity(),
            plan.units[index].case_filter.clone(),
            plan.units[index].process_model,
            &unit_attempts,
        ));
        attempts.extend(unit_attempts);
    }
    let observed_cost = observation::finish_observation(before, &attempts)?;
    let skipped_units = skipped.into_values().collect::<Vec<_>>();
    let passed_units = unit_verdicts
        .iter()
        .filter(|verdict| verdict.behavioral_verdict == "passed")
        .count();
    let failed_units = unit_verdicts.len() - passed_units;
    let behavioral_verdict = if failed_units == 0 && skipped_units.is_empty() {
        "passed"
    } else if unit_verdicts
        .iter()
        .any(|verdict| verdict.behavioral_verdict == "flaky-indeterminate")
    {
        "indeterminate"
    } else {
        "failed"
    };
    let failed_unit = unit_verdicts
        .iter()
        .find(|verdict| verdict.behavioral_verdict != "passed")
        .map(|verdict| verdict.unit_identity.clone());
    let mut run = ExecutedProofRun {
        schema_version: 1,
        evidence_identity: String::new(),
        plan_digest: plan.plan_digest.clone(),
        schedule,
        run_identity,
        run_started_unix_millis,
        planned_units: plan.units.len(),
        executed_units: unit_verdicts.len(),
        passed_units,
        failed_units,
        skipped_units,
        behavioral_verdict: behavioral_verdict.to_owned(),
        failed_unit,
        unit_verdicts,
        attempts,
        observed_cost,
        structural_preflight_evidence_identity: preflight.evidence_identity,
        structural_preflight_bundle_path: preflight.bundle_path,
    };
    run.seal()?;
    run.validate_integrity(plan)?;
    Ok(run)
}

fn classify_skip(
    plan: &SelectedProofExecutionPlan,
    index: usize,
    failed: &BTreeSet<usize>,
    skipped: &BTreeMap<usize, SkippedProofExecutionUnit>,
) -> Option<SkippedProofExecutionUnit> {
    let failed_identities: BTreeSet<_> = failed
        .iter()
        .map(|failed| plan.units[*failed].identity())
        .collect();
    let skipped_identities: BTreeSet<_> = skipped
        .values()
        .map(|unit| unit.unit_identity.clone())
        .collect();
    let blocking_units: Vec<_> = plan.units[index]
        .dependencies
        .iter()
        .filter(|dependency| {
            failed_identities.contains(*dependency) || skipped_identities.contains(*dependency)
        })
        .cloned()
        .collect();
    if !blocking_units.is_empty() {
        return Some(SkippedProofExecutionUnit {
            unit_identity: plan.units[index].identity(),
            reason: "dependency-failed".to_owned(),
            blocking_units,
        });
    }
    if plan.failure_policy == ProofFailurePolicy::StopAllAfterFailure && !failed.is_empty() {
        return Some(SkippedProofExecutionUnit {
            unit_identity: plan.units[index].identity(),
            reason: "product-fail-fast-policy".to_owned(),
            blocking_units: failed
                .iter()
                .map(|failed| plan.units[*failed].identity())
                .collect(),
        });
    }
    None
}

fn classify_remaining_skips(
    plan: &SelectedProofExecutionPlan,
    executed: &BTreeSet<usize>,
    failed: &BTreeSet<usize>,
    skipped: &mut BTreeMap<usize, SkippedProofExecutionUnit>,
) {
    loop {
        let mut changed = false;
        for index in 0..plan.units.len() {
            if executed.contains(&index) || skipped.contains_key(&index) {
                continue;
            }
            if let Some(denial) = classify_skip(plan, index, failed, skipped) {
                skipped.insert(index, denial);
                changed = true;
            }
        }
        if !changed {
            return;
        }
    }
}

fn validate_preflight(
    workspace_root: &Path,
    reference: &StructuralPreflightReference,
) -> Result<ValidatedPreflight, String> {
    let bundle_path = std::path::PathBuf::from(&reference.bundle_path);
    let evidence: StructuralPreflightEvidence = crate::evidence::read_json(&bundle_path)?;
    evidence
        .validate_integrity()
        .map_err(|denial| denial.to_string())?;
    if evidence.evidence_identity.0 != reference.evidence_identity {
        return Err(format!(
            "preflight bundle identity mismatch: plan={} bundle={}",
            reference.evidence_identity, evidence.evidence_identity.0
        ));
    }
    if !evidence.failures().is_empty() {
        return Err("behavioral execution cannot consume a failed preflight bundle".to_owned());
    }
    match require_fresh(&forge_root(workspace_root)?, &evidence)? {
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

pub(crate) struct ValidatedPreflight {
    evidence_identity: String,
    bundle_path: String,
}

fn unix_millis() -> Result<u128, String> {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .map_err(|error| format!("system clock precedes Unix epoch: {error}"))
}

static RUN_NONCE: AtomicU64 = AtomicU64::new(0);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::selection::ProofProcessModel;

    #[test]
    fn green_preflight_cannot_substitute_for_a_failed_behavioral_unit() {
        let accepted_preflight = ValidatedPreflight {
            evidence_identity: "green-preflight".to_owned(),
            bundle_path: "green-preflight.json".to_owned(),
        };
        let verdict = ProofUnitExecutionVerdict {
            unit_identity: "worth-store-certification::inverted-assertion".to_owned(),
            case_filter: None,
            process_model: ProofProcessModel::LibtestProcess,
            behavioral_verdict: "failed".to_owned(),
            elapsed_millis: 1,
            attempt_identities: vec!["failed-attempt".to_owned()],
            ui_proof_evidence: Vec::new(),
            process_probe_evidence: Vec::new(),
        };

        assert_eq!(accepted_preflight.evidence_identity, "green-preflight");
        assert_eq!(verdict.behavioral_verdict, "failed");
        assert_eq!(verdict.attempt_identities, ["failed-attempt"]);
    }
}
