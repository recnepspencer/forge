#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SettingOwnershipMetadata {
    PlatformRuntimeConfig,
    ApplicationRuntimeConfig,
    PluginRuntimeConfig,
    ClaimsAuthoritativeDomainTruth,
}

impl SettingOwnershipMetadata {
    pub fn platform_runtime_config() -> Self {
        Self::PlatformRuntimeConfig
    }

    pub fn application_runtime_config() -> Self {
        Self::ApplicationRuntimeConfig
    }

    pub fn plugin_runtime_config() -> Self {
        Self::PluginRuntimeConfig
    }

    pub fn claims_authoritative_domain_truth_for_diagnostics() -> Self {
        Self::ClaimsAuthoritativeDomainTruth
    }

    pub(crate) fn claims_domain_truth(&self) -> bool {
        matches!(self, Self::ClaimsAuthoritativeDomainTruth)
    }

    pub(crate) fn digest_basis(&self) -> &'static str {
        match self {
            Self::PlatformRuntimeConfig => "platform_runtime_config",
            Self::ApplicationRuntimeConfig => "application_runtime_config",
            Self::PluginRuntimeConfig => "plugin_runtime_config",
            Self::ClaimsAuthoritativeDomainTruth => "claims_authoritative_domain_truth",
        }
    }
}
