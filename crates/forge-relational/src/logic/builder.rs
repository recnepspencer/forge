use crate::config::data::{
    AdjacencyPolicy, CascadeDeletePolicy, CompiledLanePolicy, CrossContextPolicy,
    DurabilityPolicy, DurableLogPolicy, MvccConfig, PublicationConfig,
    RelationalConfigOverride, RelationalRuntimeProfile, StorageLayoutConfig,
    VisibilityCachePolicy,
};
use crate::diagnostics::data::RelationalDiagnosticsProfile;
use crate::durability::data::{DurabilityMode, DurableStoreLayout};
use crate::logic::runtime::{RelationalRuntime, RelationalRuntimeConfig};
use crate::payloads::data::PayloadPolicy;
use crate::schema::data::RelationalSchemaRegistry;
use crate::symbols::data::SymbolPolicy;
use crate::validation::data::InvariantCatalog;

#[derive(Debug, Clone)]
pub struct RelationalRuntimeBuilder {
    profile: RelationalRuntimeProfile,
    config_override: RelationalConfigOverride,
}

impl Default for RelationalRuntimeBuilder {
    fn default() -> Self {
        Self {
            profile: RelationalRuntimeProfile::CertificationCore,
            config_override: RelationalConfigOverride::default(),
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
        self.config_override.execution.runtime_name = Some(runtime_name.into());
        self
    }

    pub fn execution_model(
        mut self,
        execution_model: crate::logic::planning::RelationalExecutionModel,
    ) -> Self {
        self.config_override.execution.execution_model = Some(execution_model);
        self
    }

    pub fn planning(
        mut self,
        planning: crate::logic::planning::PlanningContract,
    ) -> Self {
        self.config_override.execution.planning = Some(planning);
        self
    }

    pub fn commit_authority(
        mut self,
        commit_authority: crate::logic::commit::CommitAuthorityContract,
    ) -> Self {
        self.config_override.execution.commit_authority = Some(commit_authority);
        self
    }

    pub fn durability_mode(mut self, durability_mode: DurabilityMode) -> Self {
        self.config_override.durability.mode = Some(durability_mode);
        self
    }

    pub fn diagnostics(mut self, diagnostics: RelationalDiagnosticsProfile) -> Self {
        self.config_override.diagnostics.profile = Some(diagnostics);
        self
    }

    pub fn schema_registry(mut self, schema_registry: RelationalSchemaRegistry) -> Self {
        self.config_override.schema.registry = Some(schema_registry);
        self
    }

    pub fn invariant_catalog(mut self, invariant_catalog: InvariantCatalog) -> Self {
        self.config_override.schema.invariant_catalog = Some(invariant_catalog);
        self
    }

    pub fn entity_capacity(mut self, capacity: usize) -> Self {
        self.config_override.storage.initial_entity_capacity = Some(capacity);
        self
    }

    pub fn relation_capacity(mut self, capacity: usize) -> Self {
        self.config_override.storage.initial_relation_capacity = Some(capacity);
        self
    }

    pub fn mvcc(mut self, mvcc: MvccConfig) -> Self {
        self.config_override.storage.mvcc = Some(mvcc);
        self
    }

    pub fn storage_layout(mut self, storage_layout: StorageLayoutConfig) -> Self {
        self.config_override.storage.layout = Some(storage_layout);
        self
    }

    pub fn publication(mut self, publication: PublicationConfig) -> Self {
        self.config_override.publication.policy = Some(publication);
        self
    }

    pub fn payload_policy(mut self, payload_policy: PayloadPolicy) -> Self {
        self.config_override.storage.payload_policy = Some(payload_policy);
        self
    }

    pub fn symbol_policy(mut self, symbol_policy: SymbolPolicy) -> Self {
        self.config_override.identity.symbol_policy = Some(symbol_policy);
        self
    }

    pub fn visibility_cache_policy(
        mut self,
        visibility_cache_policy: VisibilityCachePolicy,
    ) -> Self {
        self.config_override.visibility.cache_policy = Some(visibility_cache_policy);
        self
    }

    pub fn durable_log_policy(mut self, durable_log_policy: DurableLogPolicy) -> Self {
        self.config_override.durability.log = Some(durable_log_policy);
        self
    }

    pub fn durability_policy(mut self, durability_policy: DurabilityPolicy) -> Self {
        self.config_override.durability.policy = Some(durability_policy);
        self
    }

    pub fn durable_store_layout(mut self, durable_store_layout: DurableStoreLayout) -> Self {
        self.config_override.durability.store_layout = Some(durable_store_layout);
        self
    }

    pub fn adjacency_policy(mut self, adjacency_policy: AdjacencyPolicy) -> Self {
        self.config_override.storage.adjacency_policy = Some(adjacency_policy);
        self
    }

    pub fn cross_context_policy(mut self, cross_context_policy: CrossContextPolicy) -> Self {
        self.config_override.storage.cross_context_policy = Some(cross_context_policy);
        self
    }

    pub fn cascade_delete_policy(mut self, cascade_delete_policy: CascadeDeletePolicy) -> Self {
        self.config_override.storage.cascade_delete_policy = Some(cascade_delete_policy);
        self
    }

    pub fn compiled_lane_policy(mut self, compiled_lane_policy: CompiledLanePolicy) -> Self {
        self.config_override.execution.compiled_lane_policy = Some(compiled_lane_policy);
        self
    }

    pub fn build(self) -> RelationalRuntime {
        RelationalRuntime::new(RelationalRuntimeConfig::resolved(
            self.profile,
            self.config_override,
        ))
    }
}
