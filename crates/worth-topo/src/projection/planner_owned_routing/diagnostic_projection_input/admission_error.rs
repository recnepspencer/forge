#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyDerivedReadDiagnosticInputAdmissionError {
    detail: String,
}

impl TopologyDerivedReadDiagnosticInputAdmissionError {
    pub(crate) fn new(detail: impl Into<String>) -> Self {
        Self {
            detail: detail.into(),
        }
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

pub(super) fn require_string_match(
    label: &str,
    observed: &str,
    expected: &str,
) -> Result<(), TopologyDerivedReadDiagnosticInputAdmissionError> {
    if observed != expected {
        return Err(TopologyDerivedReadDiagnosticInputAdmissionError::new(format!(
            "derived-read diagnostic input rejected mismatched {label}: expected {expected}, observed {observed}",
        )));
    }
    Ok(())
}

pub(super) fn require_optional_match(
    label: &str,
    observed: Option<&str>,
    expected: Option<&str>,
) -> Result<(), TopologyDerivedReadDiagnosticInputAdmissionError> {
    if observed != expected {
        return Err(TopologyDerivedReadDiagnosticInputAdmissionError::new(format!(
            "derived-read diagnostic input rejected mismatched {label}: expected {:?}, observed {:?}",
            expected, observed
        )));
    }
    Ok(())
}
