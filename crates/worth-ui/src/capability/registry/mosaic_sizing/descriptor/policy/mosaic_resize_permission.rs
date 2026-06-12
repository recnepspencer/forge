#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MosaicResizePermission {
    FixedByRuntime,
    UserResizable,
    ContentDriven,
    MissingForDiagnostics,
}

impl MosaicResizePermission {
    pub fn fixed_by_runtime() -> Self {
        Self::FixedByRuntime
    }

    pub fn user_resizable() -> Self {
        Self::UserResizable
    }

    pub fn content_driven() -> Self {
        Self::ContentDriven
    }

    pub fn missing_for_diagnostics() -> Self {
        Self::MissingForDiagnostics
    }

    pub(crate) fn is_missing(&self) -> bool {
        matches!(self, Self::MissingForDiagnostics)
    }

    pub(crate) fn digest_basis(&self) -> &'static str {
        match self {
            Self::FixedByRuntime => "fixed_by_runtime",
            Self::UserResizable => "user_resizable",
            Self::ContentDriven => "content_driven",
            Self::MissingForDiagnostics => "missing",
        }
    }
}
