use worth_foundational::facade::DiagnosticRichnessProfile;

use crate::response::WorthServerResponseTransform;

#[derive(Clone, Debug)]
pub struct WorthServerResponseConfig {
    default_success_transform: WorthServerResponseTransform,
    default_denial_transform: WorthServerResponseTransform,
    success_minimum_diagnostics_profile: DiagnosticRichnessProfile,
    denial_minimum_diagnostics_profile: DiagnosticRichnessProfile,
}

impl WorthServerResponseConfig {
    pub fn builder() -> WorthServerResponseConfigBuilder {
        WorthServerResponseConfigBuilder::default()
    }

    pub fn default_success_transform(&self) -> WorthServerResponseTransform {
        self.default_success_transform
    }

    pub fn default_denial_transform(&self) -> WorthServerResponseTransform {
        self.default_denial_transform
    }

    pub fn success_minimum_diagnostics_profile(&self) -> DiagnosticRichnessProfile {
        self.success_minimum_diagnostics_profile
    }

    pub fn denial_minimum_diagnostics_profile(&self) -> DiagnosticRichnessProfile {
        self.denial_minimum_diagnostics_profile
    }
}

#[derive(Clone, Debug)]
pub struct WorthServerResponseConfigBuilder {
    default_success_transform: WorthServerResponseTransform,
    default_denial_transform: WorthServerResponseTransform,
    success_minimum_diagnostics_profile: DiagnosticRichnessProfile,
    denial_minimum_diagnostics_profile: DiagnosticRichnessProfile,
}

impl Default for WorthServerResponseConfigBuilder {
    fn default() -> Self {
        Self {
            default_success_transform: WorthServerResponseTransform::WorthNative,
            default_denial_transform: WorthServerResponseTransform::CompatHttp,
            success_minimum_diagnostics_profile: DiagnosticRichnessProfile::OperationalMinimal,
            denial_minimum_diagnostics_profile: DiagnosticRichnessProfile::OperationalMinimal,
        }
    }
}

impl WorthServerResponseConfigBuilder {
    pub fn with_default_success_transform(
        mut self,
        transform: WorthServerResponseTransform,
    ) -> Self {
        self.default_success_transform = transform;
        self
    }

    pub fn with_default_denial_transform(
        mut self,
        transform: WorthServerResponseTransform,
    ) -> Self {
        self.default_denial_transform = transform;
        self
    }

    pub fn with_success_minimum_diagnostics_profile(
        mut self,
        profile: DiagnosticRichnessProfile,
    ) -> Self {
        self.success_minimum_diagnostics_profile = profile;
        self
    }

    pub fn with_denial_minimum_diagnostics_profile(
        mut self,
        profile: DiagnosticRichnessProfile,
    ) -> Self {
        self.denial_minimum_diagnostics_profile = profile;
        self
    }

    pub fn build(self) -> Result<WorthServerResponseConfig, WorthServerResponseConfigError> {
        Ok(WorthServerResponseConfig {
            default_success_transform: self.default_success_transform,
            default_denial_transform: self.default_denial_transform,
            success_minimum_diagnostics_profile: self.success_minimum_diagnostics_profile,
            denial_minimum_diagnostics_profile: self.denial_minimum_diagnostics_profile,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthServerResponseConfigError {}
