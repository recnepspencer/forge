#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct TopologyQueryBackedReadFamilyAdmissionError {
    detail: String,
}

impl TopologyQueryBackedReadFamilyAdmissionError {
    pub(crate) fn new(detail: impl Into<String>) -> Self {
        Self {
            detail: detail.into(),
        }
    }

    #[cfg(test)]
    pub(crate) fn detail(&self) -> &str {
        &self.detail
    }
}

pub(crate) fn require_optional_match(
    label: &str,
    observed: Option<&str>,
    expected: Option<&str>,
) -> Result<(), TopologyQueryBackedReadFamilyAdmissionError> {
    if observed != expected {
        return Err(TopologyQueryBackedReadFamilyAdmissionError::new(format!(
            "query-backed route admission rejected mismatched {label}: expected {:?}, observed {:?}",
            expected, observed
        )));
    }
    Ok(())
}

pub(crate) fn require_string_match(
    label: &str,
    observed: &str,
    expected: &str,
) -> Result<(), TopologyQueryBackedReadFamilyAdmissionError> {
    if observed != expected {
        return Err(TopologyQueryBackedReadFamilyAdmissionError::new(format!(
            "query-backed route admission rejected mismatched {label}: expected {expected}, observed {observed}",
        )));
    }
    Ok(())
}
