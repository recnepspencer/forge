use std::ops::{Deref, DerefMut};

use crate::data::graph::{EvaluationStrategy, SignalGraph};
use crate::data::handle::NodeId;
use crate::data::telemetry::{RuntimeTelemetry, TransactionTelemetry};
use crate::logic::checkpoint::CheckpointRuntime;
use crate::logic::events::EventBus;
use crate::schema::data::SignalSchemaRegistry;
use crate::state::{SignalBranchHandle, SignalBranchId};

use super::super::config::SignalRuntimeConfig;
use super::branching::{BranchAncestryState, BranchManager, BranchState};
use super::builder::SignalRuntimeBuilder;
use super::merge::{
    BranchMutationLedger, FrozenAspectMergePolicyRegistry, FrozenConflictIsolationRegistry,
    FrozenConflictPolicyRegistry, FrozenDeletionPolicyRegistry, FrozenIdentityMatcherRegistry,
    FrozenMergeBaseStrategyRegistry, FrozenMergeStrategyRegistry, FrozenSourceOnlyPolicyRegistry,
};
use super::observer::RuntimeObserver;
use super::reconstructability::{AuthorityState, DerivedState};
use super::runtime_observation::RuntimeObservationRegistry;
use super::temporal::TemporalRuntimeState;

#[derive(Debug)]
pub(in crate::logic::transaction::runtime) struct HeavyCaptureWitness(());

#[derive(Debug)]
pub(in crate::logic::transaction::runtime) struct AuthorityTransferPacket<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    branch_id: SignalBranchId,
    state: BranchState<D, I, T>,
}

impl<D, I, T> AuthorityTransferPacket<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn new(branch_id: SignalBranchId, state: BranchState<D, I, T>) -> Self {
        Self { branch_id, state }
    }

    pub fn branch_id(&self) -> SignalBranchId {
        self.branch_id
    }

    pub fn into_state(self) -> BranchState<D, I, T> {
        self.state
    }
}

#[derive(Debug)]
pub(in crate::logic::transaction::runtime) struct RestoreTransferPacket<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    branch_id: SignalBranchId,
    state: BranchState<D, I, T>,
}

impl<D, I, T> RestoreTransferPacket<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn new(branch_id: SignalBranchId, state: BranchState<D, I, T>) -> Self {
        Self { branch_id, state }
    }

    pub fn branch_id(&self) -> SignalBranchId {
        self.branch_id
    }

    pub fn into_state(self) -> BranchState<D, I, T> {
        self.state
    }
}

#[derive(Debug)]
pub(in crate::logic::transaction::runtime) struct ExplicitBranchForkPacket<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    source_branch: SignalBranchId,
    branch_id: SignalBranchId,
    state: BranchState<D, I, T>,
}

impl<D, I, T> ExplicitBranchForkPacket<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn new(
        source_branch: SignalBranchId,
        branch_id: SignalBranchId,
        state: BranchState<D, I, T>,
    ) -> Self {
        Self {
            source_branch,
            branch_id,
            state,
        }
    }

    pub fn source_branch(&self) -> SignalBranchId {
        self.source_branch
    }

    pub fn branch_id(&self) -> SignalBranchId {
        self.branch_id
    }

    pub fn into_state(self) -> BranchState<D, I, T> {
        self.state
    }

    pub fn state(&self) -> &BranchState<D, I, T> {
        &self.state
    }
}

#[derive(Debug)]
pub(in crate::logic::transaction::runtime) enum BranchLifecycleTransfer<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    Move(AuthorityTransferPacket<D, I, T>),
    Restore(RestoreTransferPacket<D, I, T>),
}

/// Full runtime surface for transactional evaluation, diagnostics, replay, and
/// keyed or tier-aware execution.
pub struct SignalRuntime<D, I, E, Ctx, T = ()>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(in crate::logic::transaction::runtime) config: SignalRuntimeConfig<T>,
    pub(in crate::logic::transaction::runtime) graph: SignalGraph,
    pub(in crate::logic::transaction::runtime) schema_registry: SignalSchemaRegistry,
    pub(in crate::logic::transaction::runtime) merge_strategy_registry: FrozenMergeStrategyRegistry,
    pub(in crate::logic::transaction::runtime) merge_base_strategy_registry:
        FrozenMergeBaseStrategyRegistry,
    pub(in crate::logic::transaction::runtime) aspect_merge_policy_registry:
        FrozenAspectMergePolicyRegistry,
    pub(in crate::logic::transaction::runtime) conflict_isolation_registry:
        FrozenConflictIsolationRegistry,
    pub(in crate::logic::transaction::runtime) conflict_policy_registry:
        FrozenConflictPolicyRegistry,
    pub(in crate::logic::transaction::runtime) identity_matcher_registry:
        FrozenIdentityMatcherRegistry,
    pub(in crate::logic::transaction::runtime) source_only_policy_registry:
        FrozenSourceOnlyPolicyRegistry,
    pub(in crate::logic::transaction::runtime) deletion_policy_registry:
        FrozenDeletionPolicyRegistry,
    pub(in crate::logic::transaction::runtime) checkpoint: CheckpointRuntime<D, I>,
    pub(in crate::logic::transaction::runtime) event_bus: EventBus<E, D, Ctx>,
    pub(in crate::logic::transaction::runtime) observations:
        RuntimeObservationRegistry<D, I, E, Ctx, T>,
    pub(in crate::logic::transaction::runtime) temporal: TemporalRuntimeState,
    pub(in crate::logic::transaction::runtime) telemetry: RuntimeTelemetry,
    pub(in crate::logic::transaction::runtime) branches: BranchManager<D, I, T>,
}

pub struct SignalGraphMut<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    runtime: &'a mut SignalRuntime<D, I, E, Ctx, T>,
}

impl<D, I, E, Ctx, T> SignalGraphMut<'_, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn unregister_node(
        &mut self,
        node: NodeId,
    ) -> Result<crate::data::temporal::TemporalWakeRetirementBatch, crate::data::error::SignalError>
    {
        self.runtime.unregister_node(node)
    }

    pub fn replace_node_from_checkpoint_image(
        &mut self,
        node: NodeId,
        image: crate::data::node::CheckpointNodeImage,
    ) -> Result<crate::data::temporal::TemporalWakeRetirementBatch, crate::data::error::SignalError>
    {
        self.runtime.replace_node_from_checkpoint_image(node, image)
    }

    pub fn replace_node_evaluation_config(
        &mut self,
        node: NodeId,
        eval_config: crate::data::node::NodeEvaluationConfig,
    ) -> Result<crate::data::temporal::TemporalWakeRetirementBatch, crate::data::error::SignalError>
    {
        self.runtime
            .replace_node_evaluation_config(node, eval_config)
    }
}

impl<D, I, E, Ctx, T> Deref for SignalGraphMut<'_, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    type Target = SignalGraph;

    fn deref(&self) -> &Self::Target {
        &self.runtime.graph
    }
}

impl<D, I, E, Ctx, T> DerefMut for SignalGraphMut<'_, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.runtime.graph
    }
}

impl<D, I, E, Ctx, T> Drop for SignalGraphMut<'_, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    fn drop(&mut self) {
        self.runtime
            .config
            .prune_stale_node_meta(&self.runtime.graph);
    }
}

impl SignalRuntime<(), (), (), (), ()> {
    /// Create a runtime builder from a graph.
    ///
    /// Use this when you need abnormal setup, not for the normal path.
    pub fn builder(
        graph: SignalGraph,
    ) -> SignalRuntimeBuilder<super::builder::Missing, super::builder::Missing, (), (), (), (), ()>
    {
        SignalRuntimeBuilder::new(graph)
    }

    /// Build a runtime with the recommended default setup for a typed app context.
    pub fn build_for<Ctx>(graph: SignalGraph) -> SignalRuntime<(), (), (), Ctx, ()> {
        SignalRuntime::<(), (), (), Ctx, ()>::build(graph)
    }

    /// Build a runtime with the recommended default setup and a first-class schema registry.
    pub fn build_for_with_schema<Ctx>(
        graph: SignalGraph,
        schema_registry: SignalSchemaRegistry,
    ) -> SignalRuntime<(), (), (), Ctx, ()> {
        SignalRuntime::<(), (), (), Ctx, ()>::build_with_schema(graph, schema_registry)
    }

    /// Build a runtime with the richer development diagnostics preset for a typed app context.
    pub fn development_for<Ctx>(graph: SignalGraph) -> SignalRuntime<(), (), (), Ctx, ()> {
        SignalRuntime::<(), (), (), Ctx, ()>::development(graph)
    }

    /// Build a runtime with the richer development diagnostics preset and a first-class schema registry.
    pub fn development_for_with_schema<Ctx>(
        graph: SignalGraph,
        schema_registry: SignalSchemaRegistry,
    ) -> SignalRuntime<(), (), (), Ctx, ()> {
        SignalRuntime::<(), (), (), Ctx, ()>::development_with_schema(graph, schema_registry)
    }

    /// Build a runtime with the lean operational diagnostics preset for a typed app context.
    pub fn operational_for<Ctx>(graph: SignalGraph) -> SignalRuntime<(), (), (), Ctx, ()> {
        SignalRuntime::<(), (), (), Ctx, ()>::operational(graph)
    }

    /// Build a runtime with the lean operational diagnostics preset and a first-class schema registry.
    pub fn operational_for_with_schema<Ctx>(
        graph: SignalGraph,
        schema_registry: SignalSchemaRegistry,
    ) -> SignalRuntime<(), (), (), Ctx, ()> {
        SignalRuntime::<(), (), (), Ctx, ()>::operational_with_schema(graph, schema_registry)
    }

    /// Build a runtime with the web-development preset for a typed app context.
    pub fn web_development_for<Ctx>(graph: SignalGraph) -> SignalRuntime<(), (), (), Ctx, ()> {
        SignalRuntime::<(), (), (), Ctx, ()>::web_development(graph)
    }

    /// Build a runtime with the web-development preset and a first-class schema registry.
    pub fn web_development_for_with_schema<Ctx>(
        graph: SignalGraph,
        schema_registry: SignalSchemaRegistry,
    ) -> SignalRuntime<(), (), (), Ctx, ()> {
        SignalRuntime::<(), (), (), Ctx, ()>::web_development_with_schema(graph, schema_registry)
    }

    /// Build a runtime with the fintech preset for a typed app context.
    pub fn fintech_for<Ctx>(graph: SignalGraph) -> SignalRuntime<(), (), (), Ctx, ()> {
        SignalRuntime::<(), (), (), Ctx, ()>::fintech(graph)
    }

    /// Build a runtime with the fintech preset and a first-class schema registry.
    pub fn fintech_for_with_schema<Ctx>(
        graph: SignalGraph,
        schema_registry: SignalSchemaRegistry,
    ) -> SignalRuntime<(), (), (), Ctx, ()> {
        SignalRuntime::<(), (), (), Ctx, ()>::fintech_with_schema(graph, schema_registry)
    }

    /// Build a runtime with the heaviest forensic preset for a typed app context.
    pub fn forensic_for<Ctx>(graph: SignalGraph) -> SignalRuntime<(), (), (), Ctx, ()> {
        SignalRuntime::<(), (), (), Ctx, ()>::forensic(graph)
    }

    /// Build a runtime with the heaviest forensic preset and a first-class schema registry.
    pub fn forensic_for_with_schema<Ctx>(
        graph: SignalGraph,
        schema_registry: SignalSchemaRegistry,
    ) -> SignalRuntime<(), (), (), Ctx, ()> {
        SignalRuntime::<(), (), (), Ctx, ()>::forensic_with_schema(graph, schema_registry)
    }
}

impl<Ctx> SignalRuntime<(), (), (), Ctx, ()> {
    /// Build a runtime with the recommended default setup for a typed app context.
    ///
    /// This defaults to the richer development diagnostics profile rather than
    /// the lean operational one.
    pub fn build(graph: SignalGraph) -> Self {
        Self::development(graph)
    }

    /// Build a runtime with the recommended default setup and a first-class schema registry.
    pub fn build_with_schema(graph: SignalGraph, schema_registry: SignalSchemaRegistry) -> Self {
        Self::development_with_schema(graph, schema_registry)
    }

    /// Build a runtime with the development policy preset.
    pub fn development(graph: SignalGraph) -> Self {
        SignalRuntime::<(), (), (), (), ()>::builder(graph)
            .with_context::<Ctx>()
            .with_kernel_defaults()
            .development_policy()
            .build()
    }

    /// Build a runtime with the development policy preset and a first-class schema registry.
    pub fn development_with_schema(
        graph: SignalGraph,
        schema_registry: SignalSchemaRegistry,
    ) -> Self {
        SignalRuntime::<(), (), (), (), ()>::builder(graph)
            .with_context::<Ctx>()
            .with_kernel_defaults()
            .schema_registry(schema_registry)
            .development_policy()
            .build()
    }

    /// Build a runtime with the operational policy preset.
    pub fn operational(graph: SignalGraph) -> Self {
        SignalRuntime::<(), (), (), (), ()>::builder(graph)
            .with_context::<Ctx>()
            .with_kernel_defaults()
            .operational_policy()
            .build()
    }

    /// Build a runtime with the operational policy preset and a first-class schema registry.
    pub fn operational_with_schema(
        graph: SignalGraph,
        schema_registry: SignalSchemaRegistry,
    ) -> Self {
        SignalRuntime::<(), (), (), (), ()>::builder(graph)
            .with_context::<Ctx>()
            .with_kernel_defaults()
            .schema_registry(schema_registry)
            .operational_policy()
            .build()
    }

    /// Build a runtime with the web-development policy preset.
    pub fn web_development(graph: SignalGraph) -> Self {
        SignalRuntime::<(), (), (), (), ()>::builder(graph)
            .with_context::<Ctx>()
            .with_kernel_defaults()
            .web_development_policy()
            .build()
    }

    /// Build a runtime with the web-development policy preset and a first-class schema registry.
    pub fn web_development_with_schema(
        graph: SignalGraph,
        schema_registry: SignalSchemaRegistry,
    ) -> Self {
        SignalRuntime::<(), (), (), (), ()>::builder(graph)
            .with_context::<Ctx>()
            .with_kernel_defaults()
            .schema_registry(schema_registry)
            .web_development_policy()
            .build()
    }

    /// Build a runtime with the fintech policy preset.
    pub fn fintech(graph: SignalGraph) -> Self {
        SignalRuntime::<(), (), (), (), ()>::builder(graph)
            .with_context::<Ctx>()
            .with_kernel_defaults()
            .fintech_policy()
            .build()
    }

    /// Build a runtime with the fintech policy preset and a first-class schema registry.
    pub fn fintech_with_schema(graph: SignalGraph, schema_registry: SignalSchemaRegistry) -> Self {
        SignalRuntime::<(), (), (), (), ()>::builder(graph)
            .with_context::<Ctx>()
            .with_kernel_defaults()
            .schema_registry(schema_registry)
            .fintech_policy()
            .build()
    }

    /// Build a runtime with the forensic policy preset.
    pub fn forensic(graph: SignalGraph) -> Self {
        SignalRuntime::<(), (), (), (), ()>::builder(graph)
            .with_context::<Ctx>()
            .with_kernel_defaults()
            .forensic_policy()
            .build()
    }

    /// Build a runtime with the forensic policy preset and a first-class schema registry.
    pub fn forensic_with_schema(graph: SignalGraph, schema_registry: SignalSchemaRegistry) -> Self {
        SignalRuntime::<(), (), (), (), ()>::builder(graph)
            .with_context::<Ctx>()
            .with_kernel_defaults()
            .schema_registry(schema_registry)
            .forensic_policy()
            .build()
    }
}

impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(in crate::logic::transaction::runtime::state) fn merge_global_transaction_telemetry(
        current: TransactionTelemetry,
        restored: &mut TransactionTelemetry,
    ) {
        restored.transaction_begin_count = restored
            .transaction_begin_count
            .max(current.transaction_begin_count);
        restored.transaction_commit_count = restored
            .transaction_commit_count
            .max(current.transaction_commit_count);
        restored.transaction_rollback_count = restored
            .transaction_rollback_count
            .max(current.transaction_rollback_count);
        restored.transaction_poison_count = restored
            .transaction_poison_count
            .max(current.transaction_poison_count);
        restored.rollback_packet_breadth = restored
            .rollback_packet_breadth
            .max(current.rollback_packet_breadth);
        restored.rollback_packet_config_count = restored
            .rollback_packet_config_count
            .max(current.rollback_packet_config_count);
        restored.rollback_packet_diagnostics_count = restored
            .rollback_packet_diagnostics_count
            .max(current.rollback_packet_diagnostics_count);
        restored.rollback_packet_graph_patch_count = restored
            .rollback_packet_graph_patch_count
            .max(current.rollback_packet_graph_patch_count);
        restored.rollback_packet_created_node_count = restored
            .rollback_packet_created_node_count
            .max(current.rollback_packet_created_node_count);
        restored.rollback_packet_subscriber_repair_count = restored
            .rollback_packet_subscriber_repair_count
            .max(current.rollback_packet_subscriber_repair_count);
        restored.move_transfer_count = restored
            .move_transfer_count
            .max(current.move_transfer_count);
        restored.explicit_fork_count = restored
            .explicit_fork_count
            .max(current.explicit_fork_count);
        restored.restore_transfer_count = restored
            .restore_transfer_count
            .max(current.restore_transfer_count);
        restored.heavy_capture_count = restored
            .heavy_capture_count
            .max(current.heavy_capture_count);
        restored.decision_log_event_count = restored
            .decision_log_event_count
            .max(current.decision_log_event_count);
        restored.staged_node_patch_count = restored
            .staged_node_patch_count
            .max(current.staged_node_patch_count);
        restored.max_touched_nodes_in_txn = restored
            .max_touched_nodes_in_txn
            .max(current.max_touched_nodes_in_txn);
        restored.transaction_mark_dirty_candidate_visits = restored
            .transaction_mark_dirty_candidate_visits
            .max(current.transaction_mark_dirty_candidate_visits);
        restored.staged_observation_candidate_count = restored
            .staged_observation_candidate_count
            .max(current.staged_observation_candidate_count);
        restored.staged_observation_match_count = restored
            .staged_observation_match_count
            .max(current.staged_observation_match_count);
        restored.classified_observation_count = restored
            .classified_observation_count
            .max(current.classified_observation_count);
        restored.observation_classification_breadth = restored
            .observation_classification_breadth
            .max(current.observation_classification_breadth);
        restored.delivered_observation_count = restored
            .delivered_observation_count
            .max(current.delivered_observation_count);
        restored.rollback_suppressed_observation_count = restored
            .rollback_suppressed_observation_count
            .max(current.rollback_suppressed_observation_count);
    }

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
            temporal: TemporalRuntimeState::default(),
            telemetry: RuntimeTelemetry::default(),
            branches: BranchManager::<D, I, T>::new(),
        }
    }

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

    pub(super) fn capture_full_authority_state(&self) -> AuthorityState<T> {
        AuthorityState::capture(&self.graph, &self.config)
    }

    pub(super) fn capture_full_derived_state(&self) -> DerivedState<D, I> {
        DerivedState::capture(&self.checkpoint, &self.temporal, &self.telemetry)
    }

    fn heavy_capture_witness(&mut self) -> HeavyCaptureWitness {
        self.telemetry.transaction.heavy_capture_count += 1;
        HeavyCaptureWitness(())
    }

    pub(super) fn capture_heavy_branch_state(&mut self) -> BranchState<D, I, T> {
        let _witness = self.heavy_capture_witness();
        let handle = self.graph.current_branch();
        let ancestry = self
            .branches
            .branch_ancestry_state(handle.id)
            .cloned()
            .unwrap_or(BranchAncestryState::new(
                handle.id,
                handle.parent_branch_id,
                handle.head_snapshot_id,
            ));
        let mut mutation_ledger = self
            .branches
            .branch_mutation_ledger(handle.id)
            .cloned()
            .unwrap_or_else(|| {
                BranchMutationLedger::default().with_baseline_snapshot(handle.head_snapshot_id)
            });
        mutation_ledger.absorb_records(self.graph.pending_branch_mutation_records());
        self.graph.clear_branch_mutation_nodes();
        self.branches.capture_active_state(
            self.capture_full_authority_state(),
            self.capture_full_derived_state(),
            ancestry,
            mutation_ledger,
        )
    }

    pub(super) fn take_heavy_active_branch_state(&mut self) -> BranchState<D, I, T> {
        let _witness = self.heavy_capture_witness();
        let handle = self.graph.current_branch();
        let ancestry = self
            .branches
            .branch_ancestry_state(handle.id)
            .cloned()
            .unwrap_or(BranchAncestryState::new(
                handle.id,
                handle.parent_branch_id,
                handle.head_snapshot_id,
            ));
        let mut mutation_ledger = self
            .branches
            .branch_mutation_ledger(handle.id)
            .cloned()
            .unwrap_or_else(|| {
                BranchMutationLedger::default().with_baseline_snapshot(handle.head_snapshot_id)
            });
        mutation_ledger.absorb_records(self.graph.pending_branch_mutation_records());
        self.graph.clear_branch_mutation_nodes();

        let authority = AuthorityState {
            graph: std::mem::take(&mut self.graph),
            config: std::mem::take(&mut self.config),
        };
        let checkpoint_policy = self.checkpoint.policy().clone();
        let derived = DerivedState {
            checkpoint: std::mem::replace(
                &mut self.checkpoint,
                CheckpointRuntime::new(checkpoint_policy),
            ),
            temporal: std::mem::take(&mut self.temporal),
            telemetry: std::mem::take(&mut self.telemetry),
        };
        self.branches
            .capture_active_state(authority, derived, ancestry, mutation_ledger)
    }

    fn load_branch_state(
        &mut self,
        packet: AuthorityTransferPacket<D, I, T>,
        count_temporal_restore: bool,
    ) -> Result<(), crate::data::error::SignalError> {
        let preserved_transaction = self.telemetry.transaction;
        let branch_id = packet.branch_id();
        let state = packet.into_state();
        if branch_id != state.ancestry().branch_id() {
            return Err(crate::data::error::SignalError::internal(format!(
                "branch lifecycle transfer mismatch: packet branch {} does not match state branch {}",
                branch_id.0,
                state.ancestry().branch_id().0
            )));
        }
        self.branches.restore_active_state(
            state,
            &mut self.graph,
            &mut self.config,
            &mut self.checkpoint,
            &mut self.temporal,
            &mut self.telemetry,
            count_temporal_restore,
        );
        Self::merge_global_transaction_telemetry(
            preserved_transaction,
            &mut self.telemetry.transaction,
        );
        Ok(())
    }

    fn load_restored_branch_state(
        &mut self,
        packet: RestoreTransferPacket<D, I, T>,
    ) -> Result<(), crate::data::error::SignalError> {
        self.telemetry.transaction.restore_transfer_count += 1;
        self.load_branch_state(
            AuthorityTransferPacket::new(packet.branch_id(), packet.into_state()),
            true,
        )
    }

    pub(super) fn apply_branch_lifecycle_transfer(
        &mut self,
        transfer: BranchLifecycleTransfer<D, I, T>,
    ) -> Result<(), crate::data::error::SignalError> {
        match transfer {
            BranchLifecycleTransfer::Move(packet) => self.load_branch_state(packet, false),
            BranchLifecycleTransfer::Restore(packet) => self.load_restored_branch_state(packet),
        }
    }

    pub(super) fn synchronize_branch_catalogs(
        &mut self,
        branch_catalog: std::collections::BTreeMap<SignalBranchId, SignalBranchHandle>,
    ) {
        let active_branch = self.graph.current_branch().id;
        self.branches
            .synchronize_catalogs(&branch_catalog, active_branch, &mut self.graph);
    }
}

#[cfg(test)]
mod tests {
    use crate::data::telemetry::TransactionTelemetry;

    use super::SignalRuntime;

    #[test]
    fn merge_global_transaction_telemetry_preserves_observation_counters() {
        let current = TransactionTelemetry {
            staged_observation_candidate_count: 11,
            staged_observation_match_count: 19,
            classified_observation_count: 7,
            observation_classification_breadth: 23,
            delivered_observation_count: 5,
            rollback_suppressed_observation_count: 3,
            ..TransactionTelemetry::default()
        };
        let mut restored = TransactionTelemetry::default();

        SignalRuntime::<(), (), (), (), ()>::merge_global_transaction_telemetry(
            current,
            &mut restored,
        );

        assert_eq!(restored.staged_observation_candidate_count, 11);
        assert_eq!(restored.staged_observation_match_count, 19);
        assert_eq!(restored.classified_observation_count, 7);
        assert_eq!(restored.observation_classification_breadth, 23);
        assert_eq!(restored.delivered_observation_count, 5);
        assert_eq!(restored.rollback_suppressed_observation_count, 3);
    }
}
