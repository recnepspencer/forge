use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use crate::evidence::{sha256_file, sha256_serialized, write_new_json};
use crate::selection::SelectedProofExecutionPlan;
use worth_store_test_support::structural_preflight::STRUCTURAL_PREFLIGHT_BUNDLE_ENV;

use super::attempt::{ProofAttemptLog, ProofAttemptOutcome, ProofRunAttempt};
use super::{
    external_observer, formal_evidence, observation, process_evidence, ui_evidence,
    ValidatedPreflight,
};

pub(crate) fn execute_unit(
    workspace_root: &Path,
    plan: &SelectedProofExecutionPlan,
    preflight: &ValidatedPreflight,
    run_identity: &str,
    unit_index: usize,
) -> Result<Vec<ProofRunAttempt>, String> {
    let unit = &plan.units[unit_index];
    let mut attempts = Vec::new();
    loop {
        let attempt = execute_attempt(
            workspace_root,
            plan,
            preflight,
            run_identity,
            unit_index,
            attempts.len(),
        )?;
        let should_retry = !attempt.outcome.passed()
            && unit
                .retry
                .admits(attempt.outcome.exit_code(), attempts.len() + 1);
        attempts.push(attempt);
        if !should_retry {
            return Ok(attempts);
        }
    }
}

fn execute_attempt(
    workspace_root: &Path,
    plan: &SelectedProofExecutionPlan,
    preflight: &ValidatedPreflight,
    run_identity: &str,
    unit_index: usize,
    ordinal: usize,
) -> Result<ProofRunAttempt, String> {
    let unit = &plan.units[unit_index];
    let unit_identity = unit.identity();
    let attempt_identity = sha256_serialized(&(
        "worth-store-proof-run-attempt-v1",
        &plan.plan_digest,
        run_identity,
        unit_index,
        ordinal,
        unit,
    ))?;
    let attempt_root = attempt_root(workspace_root, plan, run_identity);
    let log_root = attempt_root.join("logs");
    std::fs::create_dir_all(&log_root)
        .map_err(|error| format!("could not create {}: {error}", log_root.display()))?;
    let stdout_path = log_root.join(format!("{unit_index:04}-{ordinal:02}.stdout"));
    let stderr_path = log_root.join(format!("{unit_index:04}-{ordinal:02}.stderr"));
    let stdout = File::create(&stdout_path)
        .map_err(|error| format!("could not create {}: {error}", stdout_path.display()))?;
    let stderr = File::create(&stderr_path)
        .map_err(|error| format!("could not create {}: {error}", stderr_path.display()))?;
    let (program, arguments) = unit.command_line(plan.request.mode());
    let command_identity = sha256_serialized(&(
        "worth-store-proof-unit-execution-v2",
        &plan.plan_digest,
        unit_index,
        ordinal,
        unit,
    ))?;
    let ui_root = ui_evidence::attempt_root(
        workspace_root,
        &format!("{run_identity}-attempt-{ordinal}"),
        unit_index,
        &unit_identity,
    );
    let process_root = process_evidence::attempt_root(
        workspace_root,
        &format!("{run_identity}-attempt-{ordinal}"),
        unit_index,
        &unit_identity,
    );
    let started_unix_millis = unix_millis()?;
    let started = Instant::now();
    let mut evidence_denials = Vec::new();
    let mut observer = match external_observer::start(
        &attempt_root,
        command_identity.clone(),
        unit.resources.target_root.clone(),
    ) {
        Ok(observer) => Some(observer),
        Err(denial) => {
            evidence_denials.push(denial);
            None
        }
    };
    let formal_receipt_path = attempt_root.join(format!(
        "formal-tool-receipt-{unit_index:04}-{ordinal:02}.json"
    ));
    let mut command = Command::new(&program);
    command
        .args(&arguments)
        .current_dir(workspace_root)
        .stdout(Stdio::from(stdout))
        .stderr(Stdio::from(stderr))
        .env("CARGO_TARGET_DIR", &unit.resources.target_root)
        .envs(&unit.resources.environment)
        .env(
            worth_store_test_support::compiler_boundary::UI_EVIDENCE_ROOT_ENV,
            &ui_root,
        )
        .env(
            worth_store_test_support::compiler_boundary::UI_EXECUTION_IDENTITY_ENV,
            &command_identity,
        )
        .env(STRUCTURAL_PREFLIGHT_BUNDLE_ENV, &preflight.bundle_path)
        .env(
            process_evidence::PROCESS_PROBE_EVIDENCE_ROOT_ENV,
            &process_root,
        );
    if unit.process_model == crate::selection::ProofProcessModel::ExternalToolProcess {
        command.env("WORTH_STORE_FORMAL_EVIDENCE_PATH", &formal_receipt_path);
    }
    let mut outcome = match command.spawn() {
        Ok(mut child) => {
            if let Some(observer) = &observer {
                if let Err(denial) = observer.bind_root_process(child.id()) {
                    evidence_denials.push(denial);
                }
            }
            wait_for_child(&mut child, unit.timeout_millis)?
        }
        Err(error) => ProofAttemptOutcome::LaunchDenied {
            reason: format!("could not launch {program}: {error}"),
        },
    };
    let external_observation = match observer.take() {
        Some(observer) => match observer.finish() {
            Ok(receipt) => Some(receipt),
            Err(denial) => {
                evidence_denials.push(denial);
                None
            }
        },
        None => None,
    };
    let command_succeeded = outcome.passed();
    let ui_proof_evidence = match ui_evidence::collect(
        workspace_root,
        &ui_root,
        &unit_identity,
        &command_identity,
        unit.process_model.requires_ui_proof_evidence() && command_succeeded,
    ) {
        Ok(evidence) => evidence,
        Err(denial) => {
            evidence_denials.push(denial);
            Vec::new()
        }
    };
    let process_probe_evidence = match process_evidence::collect(
        workspace_root,
        &process_root,
        &unit_identity,
        command_succeeded && unit.process_model.requires_process_probe_evidence(),
    ) {
        Ok(evidence) => evidence,
        Err(denial) => {
            evidence_denials.push(denial);
            Vec::new()
        }
    };
    let formal_tool_evidence = match formal_evidence::collect(
        workspace_root,
        &formal_receipt_path,
        command_succeeded
            && unit.process_model == crate::selection::ProofProcessModel::ExternalToolProcess,
    ) {
        Ok(evidence) => evidence,
        Err(denial) => {
            evidence_denials.push(denial);
            None
        }
    };
    if command_succeeded && !evidence_denials.is_empty() {
        outcome = ProofAttemptOutcome::EvidenceDenied {
            denials: evidence_denials.clone(),
        };
    }
    let observed_cargo_artifacts = observation::cargo_artifacts(&stdout_path)?;
    let cargo_compiler_artifact_messages = observed_cargo_artifacts.len();
    let linked_executable_artifacts = observed_cargo_artifacts
        .iter()
        .filter_map(|artifact| artifact.executable.clone())
        .collect();
    let attempt = ProofRunAttempt {
        attempt_identity,
        plan_digest: plan.plan_digest.clone(),
        unit_identity,
        unit_index,
        ordinal,
        command: std::iter::once(program).chain(arguments).collect(),
        started_unix_millis,
        elapsed_millis: started.elapsed().as_millis(),
        outcome,
        stdout: log_evidence(&stdout_path)?,
        stderr: log_evidence(&stderr_path)?,
        cargo_compiler_artifact_messages,
        linked_executable_artifacts,
        observed_cargo_artifacts,
        evidence_denials,
        external_observation,
        formal_tool_evidence,
        ui_proof_evidence,
        process_probe_evidence,
    };
    write_new_json(
        &attempt_root.join("attempts").join(format!(
            "{}-{:02}.json",
            attempt.unit_index, attempt.ordinal
        )),
        &attempt,
    )?;
    Ok(attempt)
}

fn wait_for_child(
    child: &mut std::process::Child,
    timeout_millis: u64,
) -> Result<ProofAttemptOutcome, String> {
    let started = Instant::now();
    loop {
        match child
            .try_wait()
            .map_err(|error| format!("could not observe cargo child: {error}"))?
        {
            Some(status) if status.success() => return Ok(ProofAttemptOutcome::Passed),
            Some(status) => {
                return Ok(ProofAttemptOutcome::Failed {
                    exit_code: status.code(),
                })
            }
            None if started.elapsed() >= Duration::from_millis(timeout_millis) => {
                return Ok(terminate_timed_out_child(child));
            }
            None => thread::sleep(Duration::from_millis(20)),
        }
    }
}

fn terminate_timed_out_child(child: &mut std::process::Child) -> ProofAttemptOutcome {
    match terminate_process_tree(child.id()) {
        Ok(()) => match child.wait() {
            Ok(_) => ProofAttemptOutcome::TimedOut,
            Err(error) => ProofAttemptOutcome::TerminationDenied {
                reason: format!("process tree terminated but cargo could not be reaped: {error}"),
            },
        },
        Err(tree_error) => {
            let child_error = child.kill().err().map(|error| error.to_string());
            let wait_error = child.wait().err().map(|error| error.to_string());
            ProofAttemptOutcome::TerminationDenied {
                reason: format!(
                    "{tree_error}; direct-child kill={}; reap={}",
                    child_error.as_deref().unwrap_or("ok"),
                    wait_error.as_deref().unwrap_or("ok")
                ),
            }
        }
    }
}

#[cfg(windows)]
fn terminate_process_tree(process_id: u32) -> Result<(), String> {
    let output = Command::new("taskkill")
        .args(["/PID", &process_id.to_string(), "/T", "/F"])
        .output()
        .map_err(|error| format!("could not launch taskkill: {error}"))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "taskkill could not terminate process tree {process_id}: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ))
    }
}

#[cfg(unix)]
fn terminate_process_tree(process_id: u32) -> Result<(), String> {
    let output = Command::new("pkill")
        .args(["-TERM", "-P", &process_id.to_string()])
        .output()
        .map_err(|error| format!("could not launch pkill: {error}"))?;
    if output.status.success() || output.status.code() == Some(1) {
        Ok(())
    } else {
        Err(format!(
            "pkill could not terminate descendants of {process_id}: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ))
    }
}

fn log_evidence(path: &Path) -> Result<ProofAttemptLog, String> {
    Ok(ProofAttemptLog {
        path: normalized(path),
        sha256: sha256_file(path)?,
        bytes: std::fs::metadata(path)
            .map_err(|error| format!("could not inspect {}: {error}", path.display()))?
            .len(),
    })
}

pub(crate) fn attempt_root(
    workspace_root: &Path,
    plan: &SelectedProofExecutionPlan,
    run_identity: &str,
) -> PathBuf {
    workspace_root
        .join(".store-proof/evidence/runs")
        .join(&plan.plan_digest)
        .join(run_identity)
}

fn unix_millis() -> Result<u128, String> {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .map_err(|error| format!("system clock precedes Unix epoch: {error}"))
}

fn normalized(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}
