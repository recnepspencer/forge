use super::Diagnostic;

pub(crate) fn render_human(diagnostics: &[Diagnostic]) -> String {
    diagnostics
        .iter()
        .map(|diagnostic| {
            format!(
                "{} {}: {}\nbelongs: {}",
                diagnostic.code().as_str(),
                diagnostic.subject(),
                diagnostic.message(),
                diagnostic.legal_home()
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub(crate) fn render_json(diagnostics: &[Diagnostic]) -> Result<String, String> {
    serde_json::to_string_pretty(diagnostics)
        .map_err(|error| format!("serialize diagnostics to json: {error}"))
}
