use crate::data::config::{
    MvccConfig, PublicationConfig, RelationalConfigOverride, RelationalRuntimeProfile,
    StorageLayoutConfig,
};
use crate::data::diagnostics::RelationalDiagnosticsProfile;
use crate::data::durability::DurabilityMode;
use crate::data::schema::RelationalSchemaRegistry;
use crate::logic::commit::CommitAuthorityContract;
use crate::logic::planning::{PlanningContract, RelationalExecutionModel};
use crate::logic::runtime::{InvariantCatalog, RelationalRuntime, RelationalRuntimeConfig};

#[derive(Debug, Clone)]
pub struct RelationalRuntimeBuilder {
    profile: RelationalRuntimeProfile,
    config_override: RelationalConfigOverride,
    diagnostics: Option<RelationalDiagnosticsProfile>,
    schema_registry: Option<RelationalSchemaRegistry>,
    invariant_catalog: Option<InvariantCatalog>,
    execution_model: Option<RelationalExecutionModel>,
    planning: Option<PlanningContract>,
    commit_authority: Option<CommitAuthorityContract>,
    durability_mode: Option<DurabilityMode>,
}

impl Default for RelationalRuntimeBuilder {
    fn default() -> Self {
        Self {
            profile: RelationalRuntimeProfile::CertificationCore,
            config_override: RelationalConfigOverride::default(),
            diagnostics: None,
            schema_registry: None,
            invariant_catalog: None,
            execution_model: None,
            planning: None,
            commit_authority: None,
            durability_mode: None,
        }
    }
}

impl RelationalRuntimeBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn profile(mut self, profile: RelationalRuntimeProfile) -> Self {
        self.profile = profile;
        self
    }

    pub fn runtime_name(mut self, runtime_name: impl Into<String>) -> Self {
        self.config_override.runtime_name = Some(runtime_name.into());
        self
    }

    pub fn execution_model(mut self, execution_model: RelationalExecutionModel) -> Self {
        self.execution_model = Some(execution_model);
        self
    }

    pub fn planning(mut self, planning: PlanningContract) -> Self {
        self.planning = Some(planning);
        self
    }

    pub fn commit_authority(mut self, commit_authority: CommitAuthorityContract) -> Self {
        self.commit_authority = Some(commit_authority);
        self
    }

    pub fn durability_mode(mut self, durability_mode: DurabilityMode) -> Self {
        self.durability_mode = Some(durability_mode);
        self
    }

    pub fn diagnostics(mut self, diagnostics: RelationalDiagnosticsProfile) -> Self {
        self.diagnostics = Some(diagnostics);
        self
    }

    pub fn schema_registry(mut self, schema_registry: RelationalSchemaRegistry) -> Self {
        self.schema_registry = Some(schema_registry);
        self
    }

    pub fn invariant_catalog(mut self, invariant_catalog: InvariantCatalog) -> Self {
        self.invariant_catalog = Some(invariant_catalog);
        self
    }

    pub fn entity_capacity(mut self, capacity: usize) -> Self {
        self.config_override.initial_entity_capacity = Some(capacity);
        self
    }

    pub fn relation_capacity(mut self, capacity: usize) -> Self {
        self.config_override.initial_relation_capacity = Some(capacity);
        self
    }

    pub fn mvcc(mut self, mvcc: MvccConfig) -> Self {
        self.config_override.mvcc = Some(mvcc);
        self
    }

    pub fn storage_layout(mut self, storage_layout: StorageLayoutConfig) -> Self {
        self.config_override.storage_layout = Some(storage_layout);
        self
    }

    pub fn publication(mut self, publication: PublicationConfig) -> Self {
        self.config_override.publication = Some(publication);
        self
    }

    pub fn build(self) -> RelationalRuntime {
        let mut config = RelationalRuntimeConfig::resolved(self.profile, self.config_override);
        if let Some(execution_model) = self.execution_model {
            config.execution_model = execution_model;
        }
        if let Some(planning) = self.planning {
            config.planning = planning;
        }
        if let Some(commit_authority) = self.commit_authority {
            config.commit_authority = commit_authority;
        }
        if let Some(diagnostics) = self.diagnostics {
            config.diagnostics = diagnostics;
        }
        if let Some(schema_registry) = self.schema_registry {
            config.schema_registry = schema_registry;
        }
        if let Some(invariant_catalog) = self.invariant_catalog {
            config.invariant_catalog = invariant_catalog;
        }
        if let Some(durability_mode) = self.durability_mode {
            config.durability_mode = durability_mode;
        }
        RelationalRuntime::new(config)
    }
}
