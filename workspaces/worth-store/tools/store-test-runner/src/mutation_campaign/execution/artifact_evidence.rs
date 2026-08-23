#[cfg(all(test, feature = "physical-work-evidence"))]
use std::path::Path;
use std::path::PathBuf;

use super::super::catalog::{ControlledMutation, MutationTarget};

const NESTED_EXECUTABLE_MARKER: &str = "CONTROLLED_MUTATION_EXECUTABLE ";

pub(super) fn executed_binary(
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

pub(super) fn nested_executable(output: &str) -> Result<PathBuf, String> {
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

pub(super) fn test_binary(output: &[u8]) -> Option<PathBuf> {
    output.split(|byte| *byte == b'\n').find_map(|line| {
        let message: serde_json::Value = serde_json::from_slice(line).ok()?;
        if message.get("reason")?.as_str()? != "compiler-artifact" {
            return None;
        }
        message.get("executable")?.as_str().map(PathBuf::from)
    })
}

pub(super) fn compiler_diagnostics(output: &str) -> Option<String> {
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

pub(super) fn tail(value: &str, lines: usize) -> String {
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
