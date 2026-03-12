use serde::{Deserialize, Serialize};

use super::overrides::RelationalConfigOverride;
use super::provenance::ConfigProvenance;
use super::sections::{
    DiagnosticsConfig, DurabilityConfig, ExecutionConfig, HistoryConfig, IdentityConfig,
    PublicationSection, SchemaConfig, StorageConfig, VisibilityConfig,
};
use super::RelationalRuntimeProfile;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationalRuntimeConfig {
    pub profile: RelationalRuntimeProfile,
    pub execution: ExecutionConfig,
    pub diagnostics: DiagnosticsConfig,
    pub history: HistoryConfig,
    pub schema: SchemaConfig,
    pub identity: IdentityConfig,
    pub storage: StorageConfig,
    pub visibility: VisibilityConfig,
    pub publication: PublicationSection,
    pub durability: DurabilityConfig,
    pub config_override: RelationalConfigOverride,
    pub config_provenance: ConfigProvenance,
}
