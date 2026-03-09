use crate::data::diagnostics::RelationalDiagnosticsProfile;
use crate::data::schema::RelationalSchemaRegistry;
use crate::logic::commit::CommitAuthorityContract;
use crate::logic::planning::{PlanningContract, RelationalExecutionModel};
use crate::logic::runtime::{InvariantCatalog, RelationalRuntime, RelationalRuntimeConfig};

#[derive(Debug, Clone)]
pub struct RelationalRuntimeBuilder {
    config: RelationalRuntimeConfig,
}

impl Default for RelationalRuntimeBuilder {
    fn default() -> Self {
        Self {
            config: RelationalRuntimeConfig::default(),
        }
    }
}

impl RelationalRuntimeBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn runtime_name(mut self, runtime_name: impl Into<String>) -> Self {
        self.config.runtime_name = runtime_name.into();
        self
    }

    pub fn execution_model(mut self, execution_model: RelationalExecutionModel) -> Self {
        self.config.execution_model = execution_model;
        self
    }

    pub fn planning(mut self, planning: PlanningContract) -> Self {
        self.config.planning = planning;
        self
    }

    pub fn commit_authority(mut self, commit_authority: CommitAuthorityContract) -> Self {
        self.config.commit_authority = commit_authority;
        self
    }

    pub fn diagnostics(mut self, diagnostics: RelationalDiagnosticsProfile) -> Self {
        self.config.diagnostics = diagnostics;
        self
    }

    pub fn schema_registry(mut self, schema_registry: RelationalSchemaRegistry) -> Self {
        self.config.schema_registry = schema_registry;
        self
    }

    pub fn invariant_catalog(mut self, invariant_catalog: InvariantCatalog) -> Self {
        self.config.invariant_catalog = invariant_catalog;
        self
    }

    pub fn entity_capacity(mut self, capacity: usize) -> Self {
        self.config.initial_entity_capacity = capacity;
        self
    }

    pub fn relation_capacity(mut self, capacity: usize) -> Self {
        self.config.initial_relation_capacity = capacity;
        self
    }

    pub fn build(self) -> RelationalRuntime {
        RelationalRuntime::new(self.config)
    }
}
