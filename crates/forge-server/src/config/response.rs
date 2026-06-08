use forge_foundational::facade::DiagnosticRichnessProfile;

use crate::response::ForgeServerResponseTransform;

#[derive(Clone, Debug)]
pub struct ForgeServerResponseConfig {
    default_success_transform: ForgeServerResponseTransform,
    default_denial_transform: ForgeServerResponseTransform,
    success_minimum_diagnostics_profile: DiagnosticRichnessProfile,
    denial_minimum_diagnostics_profile: DiagnosticRichnessProfile,
}

impl ForgeServerResponseConfig {
    pub fn builder() -> ForgeServerResponseConfigBuilder {
        ForgeServerResponseConfigBuilder::default()
    }

    pub fn default_success_transform(&self) -> ForgeServerResponseTransform {
        self.default_success_transform
    }

    pub fn default_denial_transform(&self) -> ForgeServerResponseTransform {
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
pub struct ForgeServerResponseConfigBuilder {
    default_success_transform: ForgeServerResponseTransform,
    default_denial_transform: ForgeServerResponseTransform,
    success_minimum_diagnostics_profile: DiagnosticRichnessProfile,
    denial_minimum_diagnostics_profile: DiagnosticRichnessProfile,
}

impl Default for ForgeServerResponseConfigBuilder {
    fn default() -> Self {
        Self {
            default_success_transform: ForgeServerResponseTransform::ForgeNative,
            default_denial_transform: ForgeServerResponseTransform::CompatHttp,
            success_minimum_diagnostics_profile: DiagnosticRichnessProfile::OperationalMinimal,
            denial_minimum_diagnostics_profile: DiagnosticRichnessProfile::OperationalMinimal,
        }
    }
}

impl ForgeServerResponseConfigBuilder {
    pub fn with_default_success_transform(
        mut self,
        transform: ForgeServerResponseTransform,
    ) -> Self {
        self.default_success_transform = transform;
        self
    }

    pub fn with_default_denial_transform(
        mut self,
        transform: ForgeServerResponseTransform,
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

    pub fn build(self) -> Result<ForgeServerResponseConfig, ForgeServerResponseConfigError> {
        Ok(ForgeServerResponseConfig {
            default_success_transform: self.default_success_transform,
            default_denial_transform: self.default_denial_transform,
            success_minimum_diagnostics_profile: self.success_minimum_diagnostics_profile,
            denial_minimum_diagnostics_profile: self.denial_minimum_diagnostics_profile,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerResponseConfigError {}
