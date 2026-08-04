use crate::data::graph::SignalGraph;

use crate::data::telemetry::RuntimeTelemetry;

use crate::logic::checkpoint::CheckpointRuntime;

use crate::logic::events::EventBus;

use crate::schema::data::SignalSchemaRegistry;

use super::super::super::super::config::SignalRuntimeConfig;

use super::super::super::branching::BranchManager;

use super::super::super::merge::{
    FrozenAspectMergePolicyRegistry, FrozenConflictIsolationRegistry, FrozenConflictPolicyRegistry,
    FrozenDeletionPolicyRegistry, FrozenIdentityMatcherRegistry, FrozenMergeBaseStrategyRegistry,
    FrozenMergeStrategyRegistry, FrozenSourceOnlyPolicyRegistry,
};

use super::super::super::resource::ResourceRuntimeState;

use super::super::super::runtime_observation::RuntimeObservationRegistry;

use super::super::super::temporal::TemporalRuntimeState;

use super::super::SignalRuntime;

impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(crate) fn new(
        graph: SignalGraph,
        mut schema_registry: SignalSchemaRegistry,
        checkpoint: CheckpointRuntime<D, I>,
        event_bus: EventBus<E, D, Ctx>,
    ) -> Self {
        if schema_registry.is_empty() {
            schema_registry = graph.schema_registry().clone();
        }
        let mut config = SignalRuntimeConfig::default();
        config.sync_graph_capacity(&graph);
        Self {
            config,
            graph,
            schema_registry,
            merge_strategy_registry: FrozenMergeStrategyRegistry::built_in(),
            merge_base_strategy_registry: FrozenMergeBaseStrategyRegistry::built_in(),
            aspect_merge_policy_registry: FrozenAspectMergePolicyRegistry::built_in(),
            conflict_isolation_registry: FrozenConflictIsolationRegistry::built_in(),
            conflict_policy_registry: FrozenConflictPolicyRegistry::built_in(),
            identity_matcher_registry: FrozenIdentityMatcherRegistry::built_in(),
            source_only_policy_registry: FrozenSourceOnlyPolicyRegistry::built_in(),
            deletion_policy_registry: FrozenDeletionPolicyRegistry::built_in(),
            checkpoint,
            event_bus,
            observations: RuntimeObservationRegistry::default(),
            resource: ResourceRuntimeState::default(),
            temporal: TemporalRuntimeState::default(),
            telemetry: RuntimeTelemetry::default(),
            branches: BranchManager::<D, I, T>::new(),
        }
    }
}
