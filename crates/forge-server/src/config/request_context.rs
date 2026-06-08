use forge_foundational::facade::DiagnosticRichnessProfile;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerRequestContextConfig {
    default_diagnostics_profile: DiagnosticRichnessProfile,
    maximum_diagnostics_profile: DiagnosticRichnessProfile,
    branch_targeting_enabled: bool,
    preview_targeting_enabled: bool,
}

impl ForgeServerRequestContextConfig {
    pub fn builder() -> ForgeServerRequestContextConfigBuilder {
        ForgeServerRequestContextConfigBuilder::default()
    }

    pub fn default_diagnostics_profile(&self) -> DiagnosticRichnessProfile {
        self.default_diagnostics_profile
    }

    pub fn maximum_diagnostics_profile(&self) -> DiagnosticRichnessProfile {
        self.maximum_diagnostics_profile
    }

    pub fn branch_targeting_enabled(&self) -> bool {
        self.branch_targeting_enabled
    }

    pub fn preview_targeting_enabled(&self) -> bool {
        self.preview_targeting_enabled
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerRequestContextConfigBuilder {
    default_diagnostics_profile: DiagnosticRichnessProfile,
    maximum_diagnostics_profile: DiagnosticRichnessProfile,
    branch_targeting_enabled: bool,
    preview_targeting_enabled: bool,
}

impl Default for ForgeServerRequestContextConfigBuilder {
    fn default() -> Self {
        Self {
            default_diagnostics_profile: DiagnosticRichnessProfile::Standard,
            maximum_diagnostics_profile: DiagnosticRichnessProfile::Forensic,
            branch_targeting_enabled: true,
            preview_targeting_enabled: false,
        }
    }
}

impl ForgeServerRequestContextConfigBuilder {
    pub fn with_default_diagnostics_profile(mut self, profile: DiagnosticRichnessProfile) -> Self {
        self.default_diagnostics_profile = profile;
        self
    }

    pub fn with_maximum_diagnostics_profile(mut self, profile: DiagnosticRichnessProfile) -> Self {
        self.maximum_diagnostics_profile = profile;
        self
    }

    pub fn with_branch_targeting_enabled(mut self, enabled: bool) -> Self {
        self.branch_targeting_enabled = enabled;
        self
    }

    pub fn with_preview_targeting_enabled(mut self, enabled: bool) -> Self {
        self.preview_targeting_enabled = enabled;
        self
    }

    pub fn build(
        self,
    ) -> Result<ForgeServerRequestContextConfig, ForgeServerRequestContextConfigError> {
        if self.default_diagnostics_profile > self.maximum_diagnostics_profile {
            return Err(ForgeServerRequestContextConfigError::DefaultDiagnosticsExceedsMaximum);
        }

        Ok(ForgeServerRequestContextConfig {
            default_diagnostics_profile: self.default_diagnostics_profile,
            maximum_diagnostics_profile: self.maximum_diagnostics_profile,
            branch_targeting_enabled: self.branch_targeting_enabled,
            preview_targeting_enabled: self.preview_targeting_enabled,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerRequestContextConfigError {
    DefaultDiagnosticsExceedsMaximum,
}
