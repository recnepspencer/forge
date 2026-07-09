use super::error::{EvidenceReportError, EvidenceReportErrorKind};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct EvidenceReportScope {
    value: String,
}

impl EvidenceReportScope {
    pub fn new(value: impl Into<String>) -> Result<Self, EvidenceReportError> {
        let value = value.into();
        validate_scope(&value)?;
        Ok(Self { value })
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}

fn validate_scope(value: &str) -> Result<(), EvidenceReportError> {
    if value.is_empty() {
        return Err(EvidenceReportError::new(
            EvidenceReportErrorKind::EmptyScope,
            "evidence report scope must not be empty",
        ));
    }

    for segment in value.split('.') {
        if segment.is_empty()
            || !segment
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_')
        {
            return Err(EvidenceReportError::new(
                EvidenceReportErrorKind::InvalidScopeSegment,
                format!("invalid evidence report scope segment `{segment}`"),
            ));
        }
    }

    Ok(())
}
