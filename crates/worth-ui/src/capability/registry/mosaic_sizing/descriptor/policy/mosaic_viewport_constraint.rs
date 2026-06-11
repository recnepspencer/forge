#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MosaicViewportConstraint {
    NotViewportRelative,
    ClampToViewport,
    AdmitViewportRelative,
    MissingForDiagnostics,
}

impl MosaicViewportConstraint {
    pub fn not_viewport_relative() -> Self {
        Self::NotViewportRelative
    }

    pub fn clamp_to_viewport() -> Self {
        Self::ClampToViewport
    }

    pub fn admit_viewport_relative() -> Self {
        Self::AdmitViewportRelative
    }

    pub fn missing_for_diagnostics() -> Self {
        Self::MissingForDiagnostics
    }

    pub(crate) fn is_missing(&self) -> bool {
        matches!(self, Self::MissingForDiagnostics)
    }

    pub(crate) fn digest_basis(&self) -> &'static str {
        match self {
            Self::NotViewportRelative => "not_viewport_relative",
            Self::ClampToViewport => "clamp_to_viewport",
            Self::AdmitViewportRelative => "admit_viewport_relative",
            Self::MissingForDiagnostics => "missing",
        }
    }
}
