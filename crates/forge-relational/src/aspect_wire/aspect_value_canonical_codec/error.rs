#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct AspectValueCanonicalCodecError {
    detail: String,
}

impl AspectValueCanonicalCodecError {
    pub(crate) fn new(detail: impl Into<String>) -> Self {
        Self {
            detail: detail.into(),
        }
    }

    pub(crate) fn detail(&self) -> &str {
        &self.detail
    }
}

impl std::fmt::Display for AspectValueCanonicalCodecError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.detail)
    }
}

impl std::error::Error for AspectValueCanonicalCodecError {}
