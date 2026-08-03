use std::path::{Path, PathBuf};
use std::process::Command;

use sha2::{Digest, Sha256};

use super::catalog::{ControlledMutation, MutationTarget};
use super::evidence::MutationObservation;
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
}

fn classify_failure(
    output: std::process::Output,
    mutation: &ControlledMutation,
) -> Result<ControlledFailure, String> {
    let combined = format!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    if output.status.success() {
        return Err(format!(
            "mutant {} survived predicate `{}`",
            mutation.id, mutation.predicate
        ));
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
    })
}

fn build_observation(
    installed: &InstalledSourceMutation,
    mutation: &ControlledMutation,
    failure: ControlledFailure,
) -> Result<MutationObservation, String> {
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
) -> Result<std::process::Output, String> {
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
    super::process_execution::run(&mut command, mutation.id)
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
            "nested mutation execution emitted {} executable bindings",
            markers.len()
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
mod tests {
    use super::{actual_failing_predicate, compiler_diagnostics, nested_executable, test_binary};

    #[test]
    fn mutation_causality_requires_one_runtime_predicate_marker() {
        assert_eq!(
            actual_failing_predicate("panic: C5_PREDICATE:page-layout", 6).unwrap(),
            "page-layout"
        );
        assert!(actual_failing_predicate("unrelated panic", 6).is_err());
        assert!(actual_failing_predicate(
            "C5_PREDICATE:page-layout C5_PREDICATE:batch-atomicity",
            6,
        )
        .is_err());
        assert_eq!(
            actual_failing_predicate("panic: C5_PREDICATE:local-physical-work-scheduler", 43,)
                .unwrap(),
            "local-physical-work-scheduler"
        );
        assert_eq!(
            actual_failing_predicate(
                "panic: MUTANT_PREDICATE:stale-residency-generation-consumed",
                49,
            )
            .unwrap(),
            "stale-residency-generation-consumed"
        );
        assert!(actual_failing_predicate(
            "C5_PREDICATE:page-layout MUTANT_PREDICATE:foreign-dirty-frame-claimed",
            50,
        )
        .is_err());
    }

    #[test]
    fn repeated_nested_process_marker_is_one_causal_predicate() {
        let output = "child C5_PREDICATE:current-truth\nparent MUTANT_PREDICATE:current-truth";
        assert_eq!(
            actual_failing_predicate(output, 8).unwrap(),
            "current-truth"
        );
    }

    #[test]
    fn cargo_json_binds_the_executed_binary_without_platform_text_parsing() {
        let output =
            br#"{"reason":"compiler-artifact","executable":"C:\\target\\debug\\deps\\proof.exe"}
{"reason":"build-finished","success":true}"#;
        assert_eq!(
            test_binary(output).unwrap(),
            std::path::PathBuf::from(r"C:\target\debug\deps\proof.exe")
        );
    }

    #[test]
    fn nested_execution_binds_the_actual_child_binary_from_json() {
        let output =
            r#"CONTROLLED_MUTATION_EXECUTABLE "C:\\target\\physical_store_work_courtroom.exe""#;
        assert_eq!(
            nested_executable(output).unwrap(),
            std::path::PathBuf::from(r"C:\target\physical_store_work_courtroom.exe")
        );
        assert!(nested_executable("").is_err());
        assert!(nested_executable(&format!("{output}\n{output}")).is_err());
    }

    #[test]
    fn cargo_json_preserves_compiler_diagnostics_ahead_of_trailing_artifacts() {
        let diagnostic = r#"{"reason":"compiler-message","message":{"message":"missing authority","rendered":"error[E0425]: cannot find value `authority`\n"}}"#;
        let mut output = format!("not-json\n{diagnostic}\n");
        for ordinal in 0..40 {
            output.push_str(&format!(
                "{{\"reason\":\"compiler-artifact\",\"target\":{{\"name\":\"artifact-{ordinal}\"}}}}\n"
            ));
        }
        assert_eq!(
            compiler_diagnostics(&output).unwrap(),
            "error[E0425]: cannot find value `authority`"
        );
    }

    #[test]
    fn cargo_json_diagnostic_extraction_ignores_malformed_and_empty_messages() {
        let output = r#"not-json
{"reason":"compiler-message","message":{"message":"","rendered":""}}
{"reason":"build-finished","success":false}"#;
        assert!(compiler_diagnostics(output).is_none());
    }
}
