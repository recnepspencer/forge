#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SettingMigrationPosture {
    NotRuntimeMigrated,
    MigrationArtifactDeferred,
    ClaimsAuthoritativeDomainTruth,
}

impl SettingMigrationPosture {
    pub fn not_runtime_migrated() -> Self {
        Self::NotRuntimeMigrated
    }

    pub fn migration_artifact_deferred() -> Self {
        Self::MigrationArtifactDeferred
    }

    pub fn claims_authoritative_domain_truth_for_diagnostics() -> Self {
        Self::ClaimsAuthoritativeDomainTruth
    }

    pub(crate) fn claims_domain_truth(&self) -> bool {
        matches!(self, Self::ClaimsAuthoritativeDomainTruth)
    }

    pub(crate) fn digest_basis(&self) -> &'static str {
        match self {
            Self::NotRuntimeMigrated => "not_runtime_migrated",
            Self::MigrationArtifactDeferred => "migration_artifact_deferred",
            Self::ClaimsAuthoritativeDomainTruth => "claims_authoritative_domain_truth",
        }
    }
}
