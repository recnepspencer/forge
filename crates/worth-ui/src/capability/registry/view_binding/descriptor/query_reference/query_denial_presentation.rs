#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QueryDenialPresentation {
    Hidden,
    AdvisoryText,
    StructuredStatus,
}

impl QueryDenialPresentation {
    pub fn hidden() -> Self {
        Self::Hidden
    }

    pub fn advisory_text() -> Self {
        Self::AdvisoryText
    }

    pub fn structured_status() -> Self {
        Self::StructuredStatus
    }

    pub(crate) fn digest_basis(&self) -> &'static str {
        match self {
            Self::Hidden => "hidden",
            Self::AdvisoryText => "advisory_text",
            Self::StructuredStatus => "structured_status",
        }
    }
}
