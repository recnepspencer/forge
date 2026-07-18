use serde::{Deserialize, Serialize};

use super::ExpectedCompilerDenial;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CheckedCompilerDiagnostic {
    pub level: String,
    pub code: Option<String>,
    pub message: String,
    pub rendered: String,
}

pub(super) fn checked_diagnostics(
    stdout: &[u8],
    workspace_root: &str,
) -> Vec<CheckedCompilerDiagnostic> {
    String::from_utf8_lossy(stdout)
        .lines()
        .filter_map(|line| serde_json::from_str::<serde_json::Value>(line).ok())
        .filter(|event| event["reason"].as_str() == Some("compiler-message"))
        .filter_map(|event| {
            let message = &event["message"];
            (message["level"].as_str() == Some("error")).then(|| CheckedCompilerDiagnostic {
                level: "error".to_owned(),
                code: message["code"]["code"].as_str().map(str::to_owned),
                message: normalize(
                    message["message"].as_str().unwrap_or_default(),
                    workspace_root,
                ),
                rendered: normalize(
                    message["rendered"].as_str().unwrap_or_default(),
                    workspace_root,
                ),
            })
        })
        .collect()
}

pub(super) fn validate_denial(
    expected: &ExpectedCompilerDenial,
    diagnostics: &[CheckedCompilerDiagnostic],
    stderr: &str,
) -> Result<(), String> {
    if diagnostics.is_empty() {
        return Err(format!(
            "Cargo failed without a structured compiler error; stderr:\n{stderr}"
        ));
    }
    let checked_text = diagnostics
        .iter()
        .flat_map(|diagnostic| [&diagnostic.message, &diagnostic.rendered])
        .cloned()
        .chain(std::iter::once(stderr.to_owned()))
        .collect::<Vec<_>>()
        .join("\n");
    for forbidden in &expected.forbidden_setup_fragments {
        if checked_text.contains(forbidden) {
            return Err(format!(
                "compiler denial was caused by forbidden setup failure {forbidden:?}"
            ));
        }
    }
    if !expected.error_codes.is_empty()
        && !diagnostics.iter().any(|diagnostic| {
            diagnostic
                .code
                .as_ref()
                .is_some_and(|code| expected.error_codes.contains(code))
        })
    {
        return Err(format!(
            "compiler denial omitted every declared error code {:?}",
            expected.error_codes
        ));
    }
    for fragment in &expected.required_semantic_fragments {
        if !checked_text.contains(fragment) {
            return Err(format!(
                "compiler denial missed semantic fragment {fragment:?}"
            ));
        }
    }
    Ok(())
}

fn normalize(value: &str, workspace_root: &str) -> String {
    value
        .replace(workspace_root, "<workspace>")
        .replace('\\', "/")
}
