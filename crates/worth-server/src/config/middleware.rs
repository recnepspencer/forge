use worth_foundational::DiagnosticRichnessProfile;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerMiddlewareConfig {
    compat_http_maximum_diagnostics_profile: DiagnosticRichnessProfile,
    preview_branch_authorization_enabled: bool,
    query_mutation_enabled: bool,
}

impl WorthServerMiddlewareConfig {
    pub fn builder() -> WorthServerMiddlewareConfigBuilder {
        WorthServerMiddlewareConfigBuilder::default()
    }

    pub fn compat_http_maximum_diagnostics_profile(&self) -> DiagnosticRichnessProfile {
        self.compat_http_maximum_diagnostics_profile
    }

    pub fn preview_branch_authorization_enabled(&self) -> bool {
        self.preview_branch_authorization_enabled
    }

    pub fn query_mutation_enabled(&self) -> bool {
        self.query_mutation_enabled
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerMiddlewareConfigBuilder {
    compat_http_maximum_diagnostics_profile: DiagnosticRichnessProfile,
    preview_branch_authorization_enabled: bool,
    query_mutation_enabled: bool,
}

impl Default for WorthServerMiddlewareConfigBuilder {
    fn default() -> Self {
        Self {
            compat_http_maximum_diagnostics_profile: DiagnosticRichnessProfile::Standard,
            preview_branch_authorization_enabled: true,
            query_mutation_enabled: false,
        }
    }
}

impl WorthServerMiddlewareConfigBuilder {
    pub fn with_compat_http_maximum_diagnostics_profile(
        mut self,
        profile: DiagnosticRichnessProfile,
    ) -> Self {
        self.compat_http_maximum_diagnostics_profile = profile;
        self
    }

    pub fn with_preview_branch_authorization_enabled(mut self, enabled: bool) -> Self {
        self.preview_branch_authorization_enabled = enabled;
        self
    }

    pub fn with_query_mutation_enabled(mut self, enabled: bool) -> Self {
        self.query_mutation_enabled = enabled;
        self
    }

    pub fn build(self) -> Result<WorthServerMiddlewareConfig, WorthServerMiddlewareConfigError> {
        Ok(WorthServerMiddlewareConfig {
            compat_http_maximum_diagnostics_profile: self.compat_http_maximum_diagnostics_profile,
            preview_branch_authorization_enabled: self.preview_branch_authorization_enabled,
            query_mutation_enabled: self.query_mutation_enabled,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthServerMiddlewareConfigError {}
