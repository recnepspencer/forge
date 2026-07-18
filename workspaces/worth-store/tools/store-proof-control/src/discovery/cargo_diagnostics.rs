pub(super) fn rendered_failure(stdout: &[u8], stderr: &[u8]) -> String {
    let mut diagnostics = Vec::new();
    for line in String::from_utf8_lossy(stdout).lines() {
        let Ok(event) = serde_json::from_str::<serde_json::Value>(line) else {
            continue;
        };
        if event["reason"].as_str() != Some("compiler-message") {
            continue;
        }
        if let Some(rendered) = event["message"]["rendered"].as_str() {
            diagnostics.push(rendered.trim().to_owned());
        }
    }
    let cargo_stderr = String::from_utf8_lossy(stderr);
    if !cargo_stderr.trim().is_empty() {
        diagnostics.push(cargo_stderr.trim().to_owned());
    }
    if diagnostics.is_empty() {
        "Cargo failed without a rendered compiler diagnostic".to_owned()
    } else {
        diagnostics.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::rendered_failure;

    #[test]
    fn cargo_json_failures_preserve_rendered_diagnostics() {
        let event =
            br#"{"reason":"compiler-message","message":{"rendered":"error: honest failure\n"}}"#;
        let rendered = rendered_failure(event, b"cargo stopped");
        assert!(rendered.contains("error: honest failure"));
        assert!(rendered.contains("cargo stopped"));
    }
}
