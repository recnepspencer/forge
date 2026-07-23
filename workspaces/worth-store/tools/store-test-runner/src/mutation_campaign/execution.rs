use std::path::{Path, PathBuf};
use std::process::Command;

use sha2::{Digest, Sha256};

use super::catalog::{ControlledMutation, MutationTarget};
use super::evidence::MutationObservation;
use super::sandbox::MutationSandbox;

pub(super) fn execute(
    sandbox: &MutationSandbox,
    mutation: &ControlledMutation,
) -> Result<MutationObservation, String> {
    let source = sandbox.workspace().join(mutation.source);
    let original = std::fs::read_to_string(&source)
        .map_err(|error| format!("cannot read mutation source {}: {error}", source.display()))?;
    let needle = mutation.source_needle(&original);
    let replacement = mutation.source_replacement(&original);
    let occurrences = original.matches(needle.as_ref()).count();
    if occurrences != 1 {
        return Err(format!(
            "mutant {} requires one exact source seam in {}, found {occurrences}",
            mutation.id,
            source.display()
        ));
    }
    let mutated = original.replacen(needle.as_ref(), replacement.as_ref(), 1);
    std::fs::write(&source, &mutated)
        .map_err(|error| format!("cannot install mutant {}: {error}", mutation.id))?;
    let result = run_test(sandbox, mutation);
    std::fs::write(&source, &original)
        .map_err(|error| format!("cannot restore mutant {} source: {error}", mutation.id))?;
    let output = result?;
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
        return Err(format!(
            "mutant {} did not reach a runtime assertion:\n{}",
            mutation.id,
            tail(&combined, 30)
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
    let binary = test_binary(&output.stdout).ok_or_else(|| {
        format!(
            "mutant {} runtime failure omitted the executed binary path",
            mutation.id
        )
    })?;
    let actual_failing_predicate = actual_failing_predicate(&combined, mutation.id)?;
    if actual_failing_predicate != mutation.predicate {
        return Err(format!(
            "mutant {} failed predicate `{actual_failing_predicate}` instead of `{}`",
            mutation.id, mutation.predicate
        ));
    }
    Ok(MutationObservation {
        id: mutation.id,
        source_binding: mutation.source,
        source_sha256: hash(original.as_bytes()),
        mutant_sha256: hash(mutated.as_bytes()),
        binary_binding: binary.display().to_string(),
        binary_sha256: hash_file(&binary)?,
        profile_binding: "test",
        scenario_binding: mutation.selector,
        expected_failing_predicate: mutation.predicate,
        actual_failing_predicate,
        localization: panic_localization(&combined),
    })
}

fn actual_failing_predicate(output: &str, mutant: u8) -> Result<String, String> {
    let predicates = output
        .match_indices("C5_PREDICATE:")
        .map(|(offset, marker)| {
            output[offset + marker.len()..]
                .chars()
                .take_while(|character| character.is_ascii_lowercase() || *character == '-')
                .collect::<String>()
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
    sandbox: &MutationSandbox,
    mutation: &ControlledMutation,
) -> Result<std::process::Output, String> {
    let mut command = Command::new("cargo");
    command.args(["test", "-j", "1", "-p", mutation.package]);
    match mutation.target {
        MutationTarget::Library => {
            command.arg("--lib");
        }
        MutationTarget::Integration(target) => {
            command.args(["--test", target]);
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
        .current_dir(sandbox.workspace())
        .env("CARGO_TARGET_DIR", sandbox.target());
    command
        .output()
        .map_err(|error| format!("cannot execute mutant {}: {error}", mutation.id))
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

#[cfg(test)]
mod tests {
    use super::{actual_failing_predicate, test_binary};

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
    }

    #[test]
    fn repeated_nested_process_marker_is_one_causal_predicate() {
        let output = "child C5_PREDICATE:current-truth\nparent C5_PREDICATE:current-truth";
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
}
