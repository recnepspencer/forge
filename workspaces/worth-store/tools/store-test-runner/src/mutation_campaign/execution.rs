use std::path::{Path, PathBuf};
use std::process::Command;

use sha2::{Digest, Sha256};

use super::catalog::{ControlledMutation, MutationTarget};
use super::evidence::{MutationExecutionClass, MutationExecutionEvidence, MutationObservation};
use super::source_replacement::InstalledSourceMutation;

const NESTED_EXECUTABLE_MARKER: &str = "CONTROLLED_MUTATION_EXECUTABLE ";

pub(super) fn execute(
    workspace: &Path,
    mutation: &ControlledMutation,
) -> Result<MutationObservation, String> {
    let mut installed = InstalledSourceMutation::apply(workspace, mutation)?;
    let result = run_test(workspace, mutation);
    installed.restore_exact(mutation)?;
    let failure = classify_failure(result?, mutation)?;
    build_observation(&installed, mutation, failure)
}

struct ControlledFailure {
    combined: String,
    binary: PathBuf,
    predicate: String,
    execution_class: MutationExecutionClass,
    execution_elapsed: std::time::Duration,
}

struct ExecutedControlledCase {
    output: std::process::Output,
    class: MutationExecutionClass,
    elapsed: std::time::Duration,
}

fn classify_failure(
    executed: ExecutedControlledCase,
    mutation: &ControlledMutation,
) -> Result<ControlledFailure, String> {
    let output = executed.output;
    let combined = format!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    if output.status.success() {
        return Err(classify_successful_execution(&combined, mutation));
    }
    if combined.contains("could not compile")
        || combined.contains("error[E")
        || !combined.contains("test result: FAILED")
    {
        let diagnostics = compiler_diagnostics(&combined);
        return Err(format!(
            "mutant {} did not reach a runtime assertion:\n{}",
            mutation.id,
            diagnostics.unwrap_or_else(|| tail(&combined, 30))
        ));
    }
    let failure_line = format!("test {} ... FAILED", mutation.selector);
    if !combined.contains(&failure_line) {
        return Err(format!(
            "mutant {} failed outside causal selector `{}`:\n{}",
            mutation.id,
            mutation.selector,
            tail(&combined, 30)
        ));
    }
    let binary = executed_binary(&output.stdout, &combined, mutation)?;
    let predicate = actual_failing_predicate(&combined, mutation.id)?;
    if predicate != mutation.predicate {
        return Err(format!(
            "mutant {} failed predicate `{predicate}` instead of `{}`",
            mutation.id, mutation.predicate
        ));
    }
    Ok(ControlledFailure {
        combined,
        binary,
        predicate,
        execution_class: executed.class,
        execution_elapsed: executed.elapsed,
    })
}

fn classify_successful_execution(output: &str, mutation: &ControlledMutation) -> String {
    match executed_test_count(output) {
        None => format!(
            "mutant {} selector `{}` emitted no test-result summary",
            mutation.id, mutation.selector
        ),
        Some(0) => format!(
            "mutant {} selector `{}` executed zero tests",
            mutation.id, mutation.selector
        ),
        Some(_) => format!(
            "mutant {} survived predicate `{}`",
            mutation.id, mutation.predicate
        ),
    }
}

fn executed_test_count(output: &str) -> Option<u64> {
    let summaries = output
        .lines()
        .filter(|line| line.contains("test result:"))
        .collect::<Vec<_>>();
    if summaries.is_empty() {
        return None;
    }
    summaries.into_iter().try_fold(0_u64, |total, line| {
        let passed = test_result_field(line, " passed;")?;
        let failed = test_result_field(line, " failed;")?;
        total.checked_add(passed)?.checked_add(failed)
    })
}

fn test_result_field(line: &str, suffix: &str) -> Option<u64> {
    line.split_once(suffix)?
        .0
        .split_whitespace()
        .next_back()?
        .parse()
        .ok()
}

fn build_observation(
    installed: &InstalledSourceMutation,
    mutation: &ControlledMutation,
    failure: ControlledFailure,
) -> Result<MutationObservation, String> {
    let execution =
        MutationExecutionEvidence::bind(failure.execution_class, failure.execution_elapsed)?;
    Ok(MutationObservation {
        id: mutation.id,
        source_binding: mutation.source.to_owned(),
        source_sha256: hash(installed.original()),
        mutant_sha256: hash(installed.mutated()),
        binary_binding: failure.binary.display().to_string(),
        binary_sha256: hash_file(&failure.binary)?,
        profile_binding: "test".to_owned(),
        scenario_binding: mutation.selector.to_owned(),
        expected_failing_predicate: mutation.predicate.to_owned(),
        actual_failing_predicate: failure.predicate,
        localization: panic_localization(&failure.combined),
        execution,
    })
}

fn actual_failing_predicate(output: &str, mutant: u8) -> Result<String, String> {
    let predicates = ["C5_PREDICATE:", "MUTANT_PREDICATE:"]
        .into_iter()
        .flat_map(|marker| {
            output.match_indices(marker).map(move |(offset, _)| {
                output[offset + marker.len()..]
                    .chars()
                    .take_while(|character| {
                        character.is_ascii_lowercase()
                            || character.is_ascii_digit()
                            || *character == '-'
                    })
                    .collect::<String>()
            })
        })
        .filter(|predicate| !predicate.is_empty())
        .collect::<std::collections::BTreeSet<_>>();
    match predicates.len() {
        1 => Ok(predicates.into_iter().next().unwrap()),
        0 => Err(format!(
            "mutant {mutant} reached a runtime failure without causal predicate evidence:\n{}",
            tail(output, 30)
        )),
        _ => Err(format!(
            "mutant {mutant} reached multiple causal predicates: {predicates:?}"
        )),
    }
}

fn run_test(
    workspace: &Path,
    mutation: &ControlledMutation,
) -> Result<ExecutedControlledCase, String> {
    let class = execution_class(mutation.target);
    let mut command = Command::new("cargo");
    command.args(["test", "-j", "1", "-p", mutation.package]);
    match mutation.target {
        MutationTarget::Library => {
            command.arg("--lib");
        }
        MutationTarget::LibraryWithFeatures { features } => {
            command.arg("--lib").args(["--features", features]);
        }
        MutationTarget::Binary(target) => {
            command.args(["--bin", target]);
        }
        MutationTarget::Integration(target) => {
            command.args(["--test", target]);
        }
        MutationTarget::NestedExecutableLibrary { features } => {
            command.arg("--lib").args(["--features", features]);
        }
    }
    if mutation.package == "worth-store" {
        command.args(["--features", "certification-test-authority"]);
    }
    command
        .args([
            "--message-format",
            "json",
            mutation.selector,
            "--",
            "--exact",
            "--nocapture",
        ])
        .current_dir(workspace)
        .env("CARGO_TARGET_DIR", workspace.join("target"));
    let executed = super::process_execution::run(&mut command, mutation.id, class)?;
    Ok(ExecutedControlledCase {
        output: executed.output,
        class,
        elapsed: executed.elapsed,
    })
}

const fn execution_class(target: MutationTarget) -> MutationExecutionClass {
    match target {
        MutationTarget::NestedExecutableLibrary { .. } => {
            MutationExecutionClass::NestedExecutableCold
        }
        MutationTarget::Library
        | MutationTarget::LibraryWithFeatures { .. }
        | MutationTarget::Binary(_)
        | MutationTarget::Integration(_) => MutationExecutionClass::Ordinary,
    }
}

fn executed_binary(
    cargo_stdout: &[u8],
    combined: &str,
    mutation: &ControlledMutation,
) -> Result<PathBuf, String> {
    let binary = match mutation.target {
        MutationTarget::NestedExecutableLibrary { .. } => nested_executable(combined)?,
        MutationTarget::Library
        | MutationTarget::LibraryWithFeatures { .. }
        | MutationTarget::Binary(_)
        | MutationTarget::Integration(_) => test_binary(cargo_stdout).ok_or_else(|| {
            format!(
                "mutant {} runtime failure omitted the executed test binary path",
                mutation.id
            )
        })?,
    };
    if !binary.is_file() {
        return Err(format!(
            "mutant {} named an absent executed binary {}",
            mutation.id,
            binary.display()
        ));
    }
    Ok(binary)
}

fn nested_executable(output: &str) -> Result<PathBuf, String> {
    let markers = output
        .lines()
        .filter_map(|line| line.strip_prefix(NESTED_EXECUTABLE_MARKER))
        .collect::<Vec<_>>();
    let [encoded] = markers.as_slice() else {
        return Err(format!(
            "nested mutation execution emitted {} executable bindings:\n{}",
            markers.len(),
            tail(output, 24)
        ));
    };
    serde_json::from_str::<String>(encoded)
        .map(PathBuf::from)
        .map_err(|error| format!("nested mutation executable binding was malformed: {error}"))
}

#[cfg(all(test, feature = "physical-work-evidence"))]
pub(crate) fn emit_nested_executable(path: &Path) {
    let encoded = serde_json::to_string(&path.display().to_string())
        .expect("nested mutation executable path must encode");
    println!("{NESTED_EXECUTABLE_MARKER}{encoded}");
}

fn test_binary(output: &[u8]) -> Option<PathBuf> {
    output.split(|byte| *byte == b'\n').find_map(|line| {
        let message: serde_json::Value = serde_json::from_slice(line).ok()?;
        if message.get("reason")?.as_str()? != "compiler-artifact" {
            return None;
        }
        message.get("executable")?.as_str().map(PathBuf::from)
    })
}

fn panic_localization(output: &str) -> String {
    output
        .lines()
        .find(|line| line.contains("panicked at"))
        .unwrap_or("exact causal test returned runtime failure")
        .trim()
        .to_owned()
}

fn hash_file(path: &Path) -> Result<String, String> {
    let bytes = std::fs::read(path)
        .map_err(|error| format!("cannot hash executed binary {}: {error}", path.display()))?;
    Ok(hash(&bytes))
}

fn hash(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn tail(value: &str, lines: usize) -> String {
    value
        .lines()
        .rev()
        .take(lines)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>()
        .join("\n")
}

fn compiler_diagnostics(output: &str) -> Option<String> {
    let diagnostics = output
        .lines()
        .filter_map(|line| serde_json::from_str::<serde_json::Value>(line).ok())
        .filter(|message| {
            message.get("reason").and_then(|value| value.as_str()) == Some("compiler-message")
        })
        .filter_map(|message| {
            let diagnostic = message.get("message")?;
            diagnostic
                .get("rendered")
                .and_then(|value| value.as_str())
                .or_else(|| diagnostic.get("message").and_then(|value| value.as_str()))
                .map(str::trim)
                .filter(|rendered| !rendered.is_empty())
                .map(str::to_owned)
        })
        .collect::<Vec<_>>();
    (!diagnostics.is_empty()).then(|| diagnostics.join("\n"))
}

#[cfg(test)]
#[path = "execution/tests.rs"]
mod tests;
