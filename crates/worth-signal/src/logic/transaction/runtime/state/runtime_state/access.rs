use crate::data::graph::{EvaluationStrategy, SignalGraph};

use crate::data::telemetry::RuntimeTelemetry;

use crate::logic::checkpoint::CheckpointRuntime;

use crate::logic::events::EventBus;

use crate::schema::data::SignalSchemaRegistry;

use super::super::super::config::SignalRuntimeConfig;

use super::super::merge::{
    FrozenAspectMergePolicyRegistry, FrozenConflictIsolationRegistry, FrozenConflictPolicyRegistry,
    FrozenDeletionPolicyRegistry, FrozenIdentityMatcherRegistry, FrozenMergeBaseStrategyRegistry,
    FrozenMergeStrategyRegistry, FrozenSourceOnlyPolicyRegistry,
};

use super::super::observer::RuntimeObserver;

use super::super::runtime_observation::RuntimeObservationRegistry;

use super::{SignalGraphMut, SignalRuntime};

impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn config(&self) -> &SignalRuntimeConfig<T> {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut SignalRuntimeConfig<T> {
        self.config.sync_graph_capacity(&self.graph);
        &mut self.config
    }

    pub fn graph(&self) -> &SignalGraph {
        &self.graph
    }

    pub fn schema_registry(&self) -> &SignalSchemaRegistry {
        &self.schema_registry
    }

    pub fn merge_strategy_registry(&self) -> &FrozenMergeStrategyRegistry {
        &self.merge_strategy_registry
    }

    pub fn merge_base_strategy_registry(&self) -> &FrozenMergeBaseStrategyRegistry {
        &self.merge_base_strategy_registry
    }

    pub fn aspect_merge_policy_registry(&self) -> &FrozenAspectMergePolicyRegistry {
        &self.aspect_merge_policy_registry
    }

    pub fn conflict_policy_registry(&self) -> &FrozenConflictPolicyRegistry {
        &self.conflict_policy_registry
    }

    pub fn conflict_isolation_registry(&self) -> &FrozenConflictIsolationRegistry {
        &self.conflict_isolation_registry
    }

    pub fn identity_matcher_registry(&self) -> &FrozenIdentityMatcherRegistry {
        &self.identity_matcher_registry
    }

    pub fn source_only_policy_registry(&self) -> &FrozenSourceOnlyPolicyRegistry {
        &self.source_only_policy_registry
    }

    pub fn deletion_policy_registry(&self) -> &FrozenDeletionPolicyRegistry {
        &self.deletion_policy_registry
    }

    pub fn validate_schema_bindings(&self) -> Result<(), crate::data::error::SignalError> {
        self.graph
            .validate_schema_bindings_against(&self.schema_registry)
    }

    pub fn validate_merge_semantics(&self) -> Result<(), crate::data::error::SignalError> {
        self.graph.validate_merge_semantics_against(
            &self.schema_registry,
            &self.merge_strategy_registry,
            &self.aspect_merge_policy_registry,
            &self.conflict_isolation_registry,
            &self.conflict_policy_registry,
            &self.identity_matcher_registry,
            &self.source_only_policy_registry,
            &self.deletion_policy_registry,
        )
    }

    pub fn observe(&self) -> RuntimeObserver<'_, D, I, E, Ctx, T> {
        RuntimeObserver::new(self)
    }

    pub fn derive_evaluation_strategy(&self) -> EvaluationStrategy {
        self.graph.derive_evaluation_strategy()
    }

    pub fn graph_mut(&mut self) -> SignalGraphMut<'_, D, I, E, Ctx, T> {
        self.config.sync_graph_capacity(&self.graph);
        SignalGraphMut { runtime: self }
    }

    pub fn clear_live_branch_mutation_residue(&mut self) {
        self.graph.clear_branch_mutation_nodes();
    }

    pub fn checkpoint(&self) -> &CheckpointRuntime<D, I> {
        &self.checkpoint
    }

    pub fn event_bus(&self) -> &EventBus<E, D, Ctx> {
        &self.event_bus
    }

    pub fn event_bus_mut(&mut self) -> &mut EventBus<E, D, Ctx> {
        &mut self.event_bus
    }

    pub fn observations(&self) -> &RuntimeObservationRegistry<D, I, E, Ctx, T> {
        &self.observations
    }

    pub fn observations_mut(&mut self) -> &mut RuntimeObservationRegistry<D, I, E, Ctx, T> {
        &mut self.observations
    }

    pub fn telemetry(&self) -> &RuntimeTelemetry {
        &self.telemetry
    }
}
