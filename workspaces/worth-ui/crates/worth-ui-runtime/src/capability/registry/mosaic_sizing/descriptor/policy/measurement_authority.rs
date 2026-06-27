#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MosaicMeasurementAuthority {
    RuntimeToken,
    DesignToken,
    UserPreference,
    ContentMeasurement,
    PlatformInternal,
    MissingForDiagnostics,
}

impl MosaicMeasurementAuthority {
    pub fn runtime_token() -> Self {
        Self::RuntimeToken
    }

    pub fn design_token() -> Self {
        Self::DesignToken
    }

    pub fn user_preference() -> Self {
        Self::UserPreference
    }

    pub fn content_measurement() -> Self {
        Self::ContentMeasurement
    }

    pub fn platform_internal() -> Self {
        Self::PlatformInternal
    }

    pub fn missing_for_diagnostics() -> Self {
        Self::MissingForDiagnostics
    }

    pub(crate) fn is_missing(&self) -> bool {
        matches!(self, Self::MissingForDiagnostics)
    }

    pub(crate) fn digest_basis(&self) -> &'static str {
        match self {
            Self::RuntimeToken => "runtime_token",
            Self::DesignToken => "design_token",
            Self::UserPreference => "user_preference",
            Self::ContentMeasurement => "content_measurement",
            Self::PlatformInternal => "platform_internal",
            Self::MissingForDiagnostics => "missing",
        }
    }
}
