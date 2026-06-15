pub(crate) fn normalize_human_text(value: impl Into<String>) -> String {
    let value = value.into();
    let trimmed = value.trim();
    if trimmed.is_empty() {
        "A human-readable response is required before this outcome can be shown.".to_string()
    } else {
        trimmed.to_string()
    }
}

pub(crate) fn normalize_machine_identity(value: impl Into<String>) -> String {
    let value = value.into();
    let trimmed = value.trim();
    if trimmed.is_empty() {
        "missing-response-evidence".to_string()
    } else {
        trimmed.to_string()
    }
}

pub(crate) fn is_machine_token_only(value: &str) -> bool {
    let trimmed = value.trim();
    !trimmed.is_empty()
        && !trimmed.chars().any(char::is_whitespace)
        && trimmed.contains('-')
        && trimmed
            .chars()
            .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-')
}
