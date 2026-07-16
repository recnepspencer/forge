use std::ops::{Deref, DerefMut};

use crate::data::graph::{EvaluationStrategy, SignalGraph};
use crate::data::handle::NodeId;
use crate::data::resource::{
    AdmittedResourceCompletion, AsyncDenialId, DeniedResourceCompletion,
    DeniedResourcePolicyRestoreCompatibility, DependencyChangeResourceRevalidationProof,
    FulfilledLifecycleResourceRevalidationProof, InFlightResourceRequest,
    LoweredResourceDescriptor, ObserverDemandResourceRevalidationProof, RawCompletionEnvelope,
    ResourceBoundaryPerformanceEnvelope, ResourceBranchRestoreReport, ResourceCancellationReason,
    ResourceCancellationReport, ResourceCompletionAdmissionReport,
    ResourceCompletionBatchAdmissionReport, ResourceCompletionCommitReport,
    ResourceCompletionDenialStagingReport, ResourceCompletionRollbackReport,
    ResourceCompletionStagingReport, ResourceDeclarationReport, ResourceDiagnosticsExpansionBudget,
    ResourceDiagnosticsExpansionDenial, ResourceDiagnosticsSummary, ResourceLifecycleClass,
    ResourceLifecycleRetentionCompactionReport, ResourceNodeDeclaration, ResourceNodeId,
    ResourcePolicyCompatibilityReport, ResourcePolicyRestoreCompatibilityProof,
    ResourceRejectionReason, ResourceRejectionReport, ResourceReplayAvailabilityClass,
    ResourceReplayAvailabilityDenialClass, ResourceReplayAvailabilityReport,
    ResourceReplayReconstructionReport, ResourceRequestAdmissionReport, ResourceRequestHandle,
    ResourceRequestId, ResourceRequestIntent, ResourceRetainedDeniedCompletionAvailability,
    ResourceRetainedHistoryAvailability, ResourceRetainedRetryLineageAvailability,
    ResourceRetentionCompactionBudget, ResourceRetryAdmissionReport, ResourceRetryDenialClass,
    ResourceRetryOrdinal, ResourceRetryReason, ResourceRetryScheduleReport,
    ResourceRevalidationDenialClass, ResourceRevalidationIntent, ResourceRevalidationReport,
    ResourceRuntimeSummary, ResourceRuntimeSummaryReadReport, ResourceTimeoutDeadlineAuthority,
    ResourceTimeoutDecisionPlan, ResourceTimeoutHeartbeatExtensionReport,
    ResourceTimeoutOutcomeClass, ResourceTimeoutReport, RetainedResourceRetryLineage,
    StagedDeniedResourceCompletionEffect, StagedResourceCompletionEffect,
    TerminalStateResourceRevalidationProof,
};
use crate::data::telemetry::{RuntimeTelemetry, TransactionTelemetry};
use crate::data::temporal::{
    ReadyTemporalWake, ScheduledTemporalWake, TemporalCondition, TemporalDuration,
    TemporalWakeOwner, TemporalWakeRetirementReason,
};
use crate::logic::checkpoint::CheckpointRuntime;
use crate::logic::events::EventBus;
use crate::logic::transaction::{CommittedObservationEventSummary, ObservationBoundaryOutcome};
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

#[derive(Debug, Clone)]
struct ResolvedTimeoutAdmission {
    timeout_duration: TemporalDuration,
    due_tick: crate::data::temporal::ClockTick,
    outcome_class: ResourceTimeoutOutcomeClass,
    deadline_authority: ResourceTimeoutDeadlineAuthority,
    decision_digest: crate::data::resource::ResourcePolicyDigest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RetryTimeoutAdmissionResolution {
    Disabled,
    InheritedDeadlineExhausted,
}
use super::resource::ResourceRuntimeState;
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
    pub(in crate::logic::transaction::runtime) resource: ResourceRuntimeState,
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
        restored.rollback_packet_resource_count = restored
            .rollback_packet_resource_count
            .max(current.rollback_packet_resource_count);
        restored.rollback_packet_temporal_count = restored
            .rollback_packet_temporal_count
            .max(current.rollback_packet_temporal_count);
        restored.move_transfer_count = restored
            .move_transfer_count
            .max(current.move_transfer_count);
        restored.explicit_fork_count = restored
            .explicit_fork_count
            .max(current.explicit_fork_count);
        restored.explicit_snapshot_fork_count = restored
            .explicit_snapshot_fork_count
            .max(current.explicit_snapshot_fork_count);
        restored.explicit_fork_denial_count = restored
            .explicit_fork_denial_count
            .max(current.explicit_fork_denial_count);
        restored.restore_transfer_count = restored
            .restore_transfer_count
            .max(current.restore_transfer_count);
        restored.heavy_capture_count = restored
            .heavy_capture_count
            .max(current.heavy_capture_count);
        restored.branch_basis_production_count = restored
            .branch_basis_production_count
            .max(current.branch_basis_production_count);
        restored.branch_basis_validation_count = restored
            .branch_basis_validation_count
            .max(current.branch_basis_validation_count);
        restored.branch_basis_denial_count = restored
            .branch_basis_denial_count
            .max(current.branch_basis_denial_count);
        restored.branch_basis_stale_count = restored
            .branch_basis_stale_count
            .max(current.branch_basis_stale_count);
        restored.branch_retirement_plan_count = restored
            .branch_retirement_plan_count
            .max(current.branch_retirement_plan_count);
        restored.branch_retirement_execution_count = restored
            .branch_retirement_execution_count
            .max(current.branch_retirement_execution_count);
        restored.branch_retirement_denial_count = restored
            .branch_retirement_denial_count
            .max(current.branch_retirement_denial_count);
        restored.branch_retirement_reclaimed_branch_state_count = restored
            .branch_retirement_reclaimed_branch_state_count
            .max(current.branch_retirement_reclaimed_branch_state_count);
        restored.branch_retirement_reclaimed_snapshot_state_count = restored
            .branch_retirement_reclaimed_snapshot_state_count
            .max(current.branch_retirement_reclaimed_snapshot_state_count);
        restored.branch_retirement_reclaimed_runtime_meta_count = restored
            .branch_retirement_reclaimed_runtime_meta_count
            .max(current.branch_retirement_reclaimed_runtime_meta_count);
        restored.branch_retirement_retained_proof_count = restored
            .branch_retirement_retained_proof_count
            .max(current.branch_retirement_retained_proof_count);
        restored.branch_targeted_transaction_plan_count = restored
            .branch_targeted_transaction_plan_count
            .max(current.branch_targeted_transaction_plan_count);
        restored.branch_targeted_transaction_execution_count = restored
            .branch_targeted_transaction_execution_count
            .max(current.branch_targeted_transaction_execution_count);
        restored.branch_targeted_transaction_denial_count = restored
            .branch_targeted_transaction_denial_count
            .max(current.branch_targeted_transaction_denial_count);
        restored.branch_targeted_transaction_stale_count = restored
            .branch_targeted_transaction_stale_count
            .max(current.branch_targeted_transaction_stale_count);
        restored.branch_targeted_transaction_active_switch_avoided_count = restored
            .branch_targeted_transaction_active_switch_avoided_count
            .max(current.branch_targeted_transaction_active_switch_avoided_count);
        restored.branch_targeted_transaction_touched_node_count = restored
            .branch_targeted_transaction_touched_node_count
            .max(current.branch_targeted_transaction_touched_node_count);
        restored.branch_local_suppressed_observation_count = restored
            .branch_local_suppressed_observation_count
            .max(current.branch_local_suppressed_observation_count);
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
            resource: ResourceRuntimeState::default(),
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

    pub fn resource_runtime_summary(&self) -> ResourceRuntimeSummary {
        self.resource.summary()
    }

    pub fn resource_runtime_summary_read_report(&mut self) -> ResourceRuntimeSummaryReadReport {
        self.resource
            .summary_read_report(&mut self.telemetry.resource)
    }

    pub fn compact_resource_lifecycle_history(
        &mut self,
        max_reclaimed: u32,
    ) -> ResourceLifecycleRetentionCompactionReport {
        self.resource
            .compact_lifecycle_history(max_reclaimed, &mut self.telemetry.resource)
    }

    pub fn compact_resource_lifecycle_history_with_retained_limit(
        &mut self,
        max_reclaimed: u32,
        retained_history_limit: u32,
    ) -> ResourceLifecycleRetentionCompactionReport {
        self.resource.compact_lifecycle_history_with_retained_limit(
            max_reclaimed,
            Some(retained_history_limit),
            &mut self.telemetry.resource,
        )
    }

    pub fn compact_resource_lifecycle_history_with_budget(
        &mut self,
        max_reclaimed: u32,
        budget: ResourceRetentionCompactionBudget,
    ) -> ResourceLifecycleRetentionCompactionReport {
        self.resource.compact_lifecycle_history_with_budget(
            max_reclaimed,
            budget,
            &mut self.telemetry.resource,
        )
    }

    pub fn retained_history_availability_for_request(
        &self,
        request_id: ResourceRequestId,
    ) -> Option<&ResourceRetainedHistoryAvailability> {
        self.resource
            .retained_history_availability_for_request(request_id)
    }

    pub fn retained_denied_completion_availability(
        &self,
        denial_id: AsyncDenialId,
    ) -> Option<&ResourceRetainedDeniedCompletionAvailability> {
        self.resource
            .retained_denied_completion_availability(denial_id)
    }

    pub fn retained_retry_lineage(
        &self,
        retry_ordinal: ResourceRetryOrdinal,
    ) -> Option<&RetainedResourceRetryLineage> {
        self.resource.retained_retry_lineage(retry_ordinal)
    }

    pub fn retained_retry_lineage_availability(
        &self,
        retry_ordinal: ResourceRetryOrdinal,
    ) -> Option<&ResourceRetainedRetryLineageAvailability> {
        self.resource
            .retained_retry_lineage_availability(retry_ordinal)
    }

    pub fn resource_descriptor_for_node(
        &self,
        node: ResourceNodeId,
    ) -> Option<&LoweredResourceDescriptor> {
        self.resource.descriptor_for_node(node)
    }

    pub fn classify_resource_policy_compatibility(
        &mut self,
        declaration: &ResourceNodeDeclaration,
    ) -> Result<ResourcePolicyCompatibilityReport, crate::data::error::SignalError> {
        self.resource
            .classify_policy_compatibility(declaration, &mut self.telemetry.resource)
    }

    pub fn admit_resource_policy_restore_compatibility(
        &mut self,
        declaration: &ResourceNodeDeclaration,
    ) -> Result<
        Result<ResourcePolicyRestoreCompatibilityProof, DeniedResourcePolicyRestoreCompatibility>,
        crate::data::error::SignalError,
    > {
        self.resource
            .admit_policy_restore_compatibility(declaration, &mut self.telemetry.resource)
    }

    pub fn resource_replay_availability(
        &mut self,
        declaration: &ResourceNodeDeclaration,
    ) -> Result<ResourceReplayAvailabilityReport, crate::data::error::SignalError> {
        self.resource_replay_availability_with_optional_cold_reconstruction_budget(
            declaration,
            None,
        )
    }

    pub fn resource_replay_availability_with_cold_reconstruction_budget(
        &mut self,
        declaration: &ResourceNodeDeclaration,
        budget: ResourceDiagnosticsExpansionBudget,
    ) -> Result<ResourceReplayAvailabilityReport, crate::data::error::SignalError> {
        self.resource_replay_availability_with_optional_cold_reconstruction_budget(
            declaration,
            Some(budget),
        )
    }

    fn resource_replay_availability_with_optional_cold_reconstruction_budget(
        &mut self,
        declaration: &ResourceNodeDeclaration,
        budget: Option<ResourceDiagnosticsExpansionBudget>,
    ) -> Result<ResourceReplayAvailabilityReport, crate::data::error::SignalError> {
        let summary_read = self.resource_runtime_summary_read_report();
        let compatibility = self.admit_resource_policy_restore_compatibility(declaration)?;
        let unavailable_count = summary_read
            .summary()
            .retained_history_unavailable_count()
            .saturating_add(
                summary_read
                    .summary()
                    .retained_denied_completion_unavailable_count(),
            )
            .saturating_add(
                summary_read
                    .summary()
                    .retained_retry_lineage_unavailable_count(),
            ) as u32;

        self.telemetry
            .resource
            .resource_replay_availability_decision_count += 1;

        let (
            class,
            denial_class,
            restore_compatibility,
            restore_compatibility_denial,
            diagnostics_summary,
            diagnostics_denial,
        ) = match compatibility {
            Ok(proof) if unavailable_count == 0 => {
                self.telemetry
                    .resource
                    .resource_replay_availability_retained_count += 1;
                (
                    ResourceReplayAvailabilityClass::Retained,
                    None,
                    Some(proof),
                    None,
                    None,
                    None,
                )
            }
            Ok(proof) if proof.replay_decision_class().denies_unavailable_history() => {
                self.telemetry
                    .resource
                    .resource_replay_availability_denied_count += 1;
                self.telemetry
                    .resource
                    .resource_replay_budget_history_unavailable_count += 1;
                (
                    ResourceReplayAvailabilityClass::Denied,
                    Some(ResourceReplayAvailabilityDenialClass::BudgetHistoryUnavailable),
                    Some(proof),
                    None,
                    None,
                    None,
                )
            }
            Ok(proof) => match budget {
                None => {
                    self.telemetry
                        .resource
                        .resource_replay_availability_omitted_count += 1;
                    (
                        ResourceReplayAvailabilityClass::Omitted,
                        None,
                        Some(proof),
                        None,
                        None,
                        None,
                    )
                }
                Some(budget) => match self.try_resource_diagnostics_summary(budget) {
                    Ok(summary) => {
                        self.telemetry
                            .resource
                            .resource_replay_availability_reconstructed_count += 1;
                        (
                            ResourceReplayAvailabilityClass::Reconstructed,
                            None,
                            Some(proof),
                            None,
                            Some(summary),
                            None,
                        )
                    }
                    Err(denial) => {
                        self.telemetry
                            .resource
                            .resource_replay_availability_unavailable_count += 1;
                        (
                            ResourceReplayAvailabilityClass::Unavailable,
                            None,
                            Some(proof),
                            None,
                            None,
                            Some(denial),
                        )
                    }
                },
            },
            Err(denial) => {
                self.telemetry
                    .resource
                    .resource_replay_availability_denied_count += 1;
                (
                    ResourceReplayAvailabilityClass::Denied,
                    Some(ResourceReplayAvailabilityDenialClass::RestoreCompatibilityDenied),
                    None,
                    Some(denial),
                    None,
                    None,
                )
            }
        };

        let compatibility_width = restore_compatibility
            .as_ref()
            .map(|proof| proof.compatibility().compared_width())
            .or_else(|| {
                restore_compatibility_denial
                    .as_ref()
                    .map(|denial| denial.compatibility().compared_width())
            })
            .unwrap_or(0);
        let diagnostics_width = diagnostics_summary
            .as_ref()
            .map(|summary| summary.performance().input_width())
            .or_else(|| {
                diagnostics_denial
                    .as_ref()
                    .map(|denial| denial.performance().input_width())
            })
            .unwrap_or(0);
        let performance = ResourceBoundaryPerformanceEnvelope::replay_availability(
            summary_read
                .performance()
                .input_width()
                .saturating_add(compatibility_width)
                .saturating_add(diagnostics_width),
            u32::from(class != ResourceReplayAvailabilityClass::Denied),
            u32::from(class == ResourceReplayAvailabilityClass::Denied),
            u32::from(diagnostics_summary.is_some() || diagnostics_denial.is_some()),
        );
        let performance = {
            self.telemetry
                .resource
                .record_boundary_performance_envelope(performance);
            performance
        };

        Ok(ResourceReplayAvailabilityReport::new(
            class,
            denial_class,
            summary_read,
            restore_compatibility,
            restore_compatibility_denial,
            diagnostics_summary,
            diagnostics_denial,
            performance,
        ))
    }

    pub fn latest_resource_branch_restore_report(&self) -> Option<ResourceBranchRestoreReport> {
        self.resource.latest_branch_restore_report()
    }

    pub fn reconstruct_resource_replay_summary(&mut self) -> ResourceReplayReconstructionReport {
        self.resource
            .reconstruct_replay_summary(&mut self.telemetry.resource)
    }

    pub fn resource_diagnostics_summary_with_cold_reconstruction_budget(
        &mut self,
        budget: ResourceDiagnosticsExpansionBudget,
    ) -> Result<ResourceDiagnosticsSummary, ResourceDiagnosticsExpansionDenial> {
        self.try_resource_diagnostics_summary(budget)
    }

    pub fn resource_diagnostics_summary_with_unbounded_cold_reconstruction(
        &mut self,
    ) -> ResourceDiagnosticsSummary {
        self.resource_diagnostics_summary_with_cold_reconstruction_budget(
            ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(u32::MAX),
        )
        .expect("unbounded resource diagnostics budget should admit replay reconstruction")
    }

    pub fn try_resource_diagnostics_summary(
        &mut self,
        budget: ResourceDiagnosticsExpansionBudget,
    ) -> Result<ResourceDiagnosticsSummary, ResourceDiagnosticsExpansionDenial> {
        let runtime_summary = self.resource_runtime_summary();
        let latest_branch_restore_report = self.latest_resource_branch_restore_report();
        let estimated_replay_width = self.resource.replay_reconstruction_width();
        let estimated_forensic_width = estimated_replay_width;
        let branch_restore_width = u32::from(latest_branch_restore_report.is_some());
        let effective_policy = self.resource.effective_diagnostics_policy();
        self.telemetry
            .resource
            .resource_diagnostics_policy_decision_count += 1;
        if let Some(class) = match effective_policy.class() {
            crate::data::resource::ResourceDiagnosticsDecisionClass::RetainedOnly => Some(
                crate::data::resource::ResourceDiagnosticsExpansionDenialClass::PolicyRetainedOnly,
            ),
            crate::data::resource::ResourceDiagnosticsDecisionClass::DenyColdExpansion => Some(
                crate::data::resource::ResourceDiagnosticsExpansionDenialClass::PolicyColdReconstructionDisabled,
            ),
            crate::data::resource::ResourceDiagnosticsDecisionClass::BudgetedExpansion => {
                let policy_limit = effective_policy
                    .max_replay_reconstruction_width()
                    .unwrap_or(u32::MAX);
                if estimated_replay_width > policy_limit {
                    Some(
                        crate::data::resource::ResourceDiagnosticsExpansionDenialClass::PolicyReplayReconstructionBudgetExceeded,
                    )
                } else {
                    budget.denial_class(estimated_replay_width, estimated_forensic_width)
                }
            }
            crate::data::resource::ResourceDiagnosticsDecisionClass::ForensicExpansionBudget => {
                let policy_replay_limit = effective_policy
                    .max_replay_reconstruction_width()
                    .unwrap_or(u32::MAX);
                let policy_forensic_limit = effective_policy
                    .max_forensic_reconstruction_width()
                    .unwrap_or(u32::MAX);
                if estimated_replay_width > policy_replay_limit {
                    Some(
                        crate::data::resource::ResourceDiagnosticsExpansionDenialClass::PolicyReplayReconstructionBudgetExceeded,
                    )
                } else if estimated_forensic_width > policy_forensic_limit {
                    Some(
                        crate::data::resource::ResourceDiagnosticsExpansionDenialClass::PolicyForensicReconstructionBudgetExceeded,
                    )
                } else {
                    budget.denial_class(estimated_replay_width, estimated_forensic_width)
                }
            }
        } {
            let performance = ResourceBoundaryPerformanceEnvelope::diagnostics_expansion_denied(
                1_u32.saturating_add(effective_policy.descriptor_width()),
                estimated_replay_width,
                branch_restore_width,
            );
            self.telemetry.resource.resource_diagnostics_expansion_count += 1;
            self.telemetry
                .resource
                .resource_diagnostics_expansion_input_width = self
                .telemetry
                .resource
                .resource_diagnostics_expansion_input_width
                .max(performance.input_width() as u64);
            self.telemetry
                .resource
                .record_boundary_performance_envelope(performance);
            return Err(ResourceDiagnosticsExpansionDenial::new(
                class,
                effective_policy.class(),
                budget,
                estimated_replay_width,
                estimated_forensic_width,
                performance,
                effective_policy.decision_digest().clone(),
            ));
        }
        let replay_reconstruction = self.reconstruct_resource_replay_summary();
        let replay_reconstruction_width = replay_reconstruction
            .descriptor_width()
            .saturating_add(replay_reconstruction.lifecycle_summary_width())
            .saturating_add(replay_reconstruction.denied_completion_width())
            .saturating_add(replay_reconstruction.in_flight_width());
        let performance = ResourceBoundaryPerformanceEnvelope::diagnostics_expansion(
            1_u32.saturating_add(effective_policy.descriptor_width()),
            replay_reconstruction_width,
            branch_restore_width,
        );
        self.telemetry.resource.resource_diagnostics_expansion_count += 1;
        self.telemetry
            .resource
            .resource_diagnostics_expansion_input_width = self
            .telemetry
            .resource
            .resource_diagnostics_expansion_input_width
            .max(performance.input_width() as u64);
        self.telemetry
            .resource
            .resource_diagnostics_cold_reconstruction_count += 1;
        self.telemetry
            .resource
            .record_boundary_performance_envelope(performance);
        Ok(ResourceDiagnosticsSummary::new(
            runtime_summary,
            latest_branch_restore_report,
            replay_reconstruction,
            budget,
            effective_policy.class(),
            performance,
            effective_policy.decision_digest().clone(),
        ))
    }

    pub fn in_flight_resource_request(
        &mut self,
        handle: ResourceRequestHandle,
    ) -> Option<&InFlightResourceRequest> {
        self.resource
            .in_flight_request(handle, &mut self.telemetry.resource)
    }

    pub fn declare_resource_node(
        &mut self,
        declaration: ResourceNodeDeclaration,
    ) -> Result<ResourceDeclarationReport, crate::data::error::SignalError> {
        if !self.graph.is_alive(declaration.node().node()) {
            self.telemetry.resource.resource_non_live_owner_denial_count += 1;
            return Err(crate::data::error::SignalError::invalid_input(format!(
                "cannot declare resource node for non-live owner {}",
                declaration.node().node()
            )));
        }

        self.resource
            .declare_resource_node(declaration, &mut self.telemetry.resource)
    }

    fn resolve_timeout_admission(
        &mut self,
        resource_node: ResourceNodeId,
        timeout_plan: &ResourceTimeoutDecisionPlan,
        generation_started_tick: crate::data::temporal::ClockTick,
        transaction_deadline: Option<TemporalDuration>,
    ) -> Result<Option<ResolvedTimeoutAdmission>, crate::data::error::SignalError> {
        self.telemetry
            .resource
            .resource_timeout_policy_decision_count += 1;
        let current_tick = self.clock_basis().current_tick();
        let (timeout_duration, deadline_authority) = match timeout_plan.class() {
            crate::data::resource::ResourceTimeoutDecisionClass::Disabled => return Ok(None),
            crate::data::resource::ResourceTimeoutDecisionClass::TransactionInheritedDeadline => {
                let Some(deadline) = transaction_deadline else {
                    return Err(crate::data::error::SignalError::invalid_input(format!(
                        "resource node {} requires a transaction-inherited deadline",
                        resource_node.node()
                    )));
                };
                (
                    deadline,
                    ResourceTimeoutDeadlineAuthority::TransactionIntent,
                )
            }
            crate::data::resource::ResourceTimeoutDecisionClass::RuntimeInheritedDeadline => {
                let Some(deadline) = self.config.resource_runtime_deadline() else {
                    return Err(crate::data::error::SignalError::invalid_input(format!(
                        "resource node {} requires a runtime-inherited deadline",
                        resource_node.node()
                    )));
                };
                (deadline, ResourceTimeoutDeadlineAuthority::RuntimeConfig)
            }
            _ => {
                let Some(timeout_duration) =
                    timeout_plan.timeout_for_lineage(current_tick, generation_started_tick)
                else {
                    return Ok(None);
                };
                (
                    timeout_duration,
                    ResourceTimeoutDeadlineAuthority::Descriptor,
                )
            }
        };
        if !matches!(
            deadline_authority,
            ResourceTimeoutDeadlineAuthority::Descriptor
        ) {
            self.telemetry.resource.resource_deadline_inherited_count += 1;
        }
        let due_tick = crate::data::temporal::ClockTick::new(
            current_tick.get().saturating_add(timeout_duration.get()),
        );
        let decision_digest = crate::data::resource::ResourcePolicyDigest::new(format!(
            "resolved-timeout-decision:{}:{}:{}:{}",
            timeout_plan.decision_digest().as_str(),
            timeout_duration.get(),
            timeout_plan.outcome_class().as_str(),
            deadline_authority.as_str()
        ));
        Ok(Some(ResolvedTimeoutAdmission {
            timeout_duration,
            due_tick,
            outcome_class: timeout_plan.outcome_class(),
            deadline_authority,
            decision_digest,
        }))
    }

    fn schedule_resource_timeout_wake(
        &mut self,
        resource_node: ResourceNodeId,
        resolved_timeout: &ResolvedTimeoutAdmission,
    ) -> Result<Option<ScheduledTemporalWake>, crate::data::error::SignalError> {
        let timeout = resolved_timeout.timeout_duration;
        let condition = TemporalCondition::after(timeout.get())?;
        self.schedule_owned_temporal_wake(
            TemporalWakeOwner::ResourceNode(resource_node.node()),
            condition,
            resolved_timeout.due_tick,
        )
        .map(Some)
    }

    fn schedule_resource_stale_after_wake(
        &mut self,
        resource_node: ResourceNodeId,
        stale_after: TemporalDuration,
    ) -> Result<Option<ScheduledTemporalWake>, crate::data::error::SignalError> {
        let due_tick = crate::data::temporal::ClockTick::new(
            self.clock_basis()
                .current_tick()
                .get()
                .saturating_add(stale_after.get()),
        );
        self.schedule_owned_temporal_wake(
            TemporalWakeOwner::ResourceNode(resource_node.node()),
            TemporalCondition::after(stale_after.get())?,
            due_tick,
        )
        .map(Some)
    }

    fn resolve_retry_timeout_admission(
        &mut self,
        in_flight: InFlightResourceRequest,
        timeout_plan: &ResourceTimeoutDecisionPlan,
    ) -> Result<
        Result<ResolvedTimeoutAdmission, RetryTimeoutAdmissionResolution>,
        crate::data::error::SignalError,
    > {
        match in_flight.timeout_deadline_authority() {
            ResourceTimeoutDeadlineAuthority::Descriptor => Ok(self
                .resolve_timeout_admission(
                    in_flight.node(),
                    timeout_plan,
                    in_flight.generation_started_tick(),
                    None,
                )?
                .ok_or(RetryTimeoutAdmissionResolution::Disabled)),
            ResourceTimeoutDeadlineAuthority::TransactionIntent
            | ResourceTimeoutDeadlineAuthority::RuntimeConfig => {
                let Some(due_tick) = in_flight.timeout_due_tick() else {
                    return Err(crate::data::error::SignalError::invalid_input(format!(
                        "resource request {} lost inherited deadline due tick",
                        in_flight.handle().request_id().get()
                    )));
                };
                let current_tick = self.clock_basis().current_tick();
                let remaining = due_tick.get().saturating_sub(current_tick.get());
                if remaining == 0 {
                    return Ok(Err(
                        RetryTimeoutAdmissionResolution::InheritedDeadlineExhausted,
                    ));
                }
                let timeout_duration = TemporalDuration::temporal_duration(remaining)
                    .expect("positive inherited deadline remainder must stay valid");
                let decision_digest = crate::data::resource::ResourcePolicyDigest::new(format!(
                    "resolved-timeout-decision:{}:{}:{}:{}",
                    timeout_plan.decision_digest().as_str(),
                    timeout_duration.get(),
                    timeout_plan.outcome_class().as_str(),
                    in_flight.timeout_deadline_authority().as_str()
                ));
                Ok(Ok(ResolvedTimeoutAdmission {
                    timeout_duration,
                    due_tick,
                    outcome_class: timeout_plan.outcome_class(),
                    deadline_authority: in_flight.timeout_deadline_authority(),
                    decision_digest,
                }))
            }
        }
    }

    fn dispose_resource_timeout_wake(&mut self, scheduled_timeout_wake: &ScheduledTemporalWake) {
        let _ = self.retire_temporal_wake(
            scheduled_timeout_wake.id(),
            TemporalWakeRetirementReason::Disposed,
        );
    }

    fn dispose_resource_stale_after_wake(
        &mut self,
        scheduled_stale_after_wake: &ScheduledTemporalWake,
    ) {
        let _ = self.retire_temporal_wake(
            scheduled_stale_after_wake.id(),
            TemporalWakeRetirementReason::Disposed,
        );
    }

    fn retire_superseded_resource_timeout_wake(
        &mut self,
        prior_timeout_wake: Option<crate::data::temporal::TemporalWakeId>,
        scheduled_timeout_wake: Option<&ScheduledTemporalWake>,
    ) -> Result<(), crate::data::error::SignalError> {
        if let Some(wake_id) = prior_timeout_wake {
            if let Err(err) =
                self.retire_temporal_wake(wake_id, TemporalWakeRetirementReason::Superseded)
            {
                if let Some(wake) = scheduled_timeout_wake {
                    self.dispose_resource_timeout_wake(wake);
                }
                return Err(err);
            }
        }
        Ok(())
    }

    fn retire_superseded_resource_stale_after_wake(
        &mut self,
        prior_stale_after_wake: Option<crate::data::temporal::TemporalWakeId>,
        scheduled_stale_after_wake: Option<&ScheduledTemporalWake>,
    ) -> Result<(), crate::data::error::SignalError> {
        if let Some(wake_id) = prior_stale_after_wake {
            if let Err(err) =
                self.retire_temporal_wake(wake_id, TemporalWakeRetirementReason::Superseded)
            {
                if let Some(wake) = scheduled_stale_after_wake {
                    self.dispose_resource_stale_after_wake(wake);
                }
                return Err(err);
            }
        }
        Ok(())
    }

    fn retire_superseded_resource_retry_wake(
        &mut self,
        prior_retry_wake: Option<crate::data::temporal::TemporalWakeId>,
    ) -> Result<(), crate::data::error::SignalError> {
        if let Some(wake_id) = prior_retry_wake {
            self.retire_temporal_wake(wake_id, TemporalWakeRetirementReason::Superseded)?;
        }
        Ok(())
    }

    pub fn admit_resource_request(
        &mut self,
        intent: ResourceRequestIntent,
    ) -> Result<ResourceRequestAdmissionReport, crate::data::error::SignalError> {
        let resource_node = intent.node();
        if !self.graph.is_alive(intent.node().node()) {
            self.telemetry.resource.resource_non_live_owner_denial_count += 1;
            return Err(crate::data::error::SignalError::invalid_input(format!(
                "cannot admit resource request for non-live owner {}",
                intent.node().node()
            )));
        }

        let current_tick = self.clock_basis().current_tick();
        let timeout_plan = self
            .resource
            .descriptor_for_node(resource_node)
            .map(|descriptor| descriptor.timeout_decision_plan().clone())
            .unwrap_or_else(|| LoweredResourceDescriptor::default_timeout_decision_plan());
        let prior_stale_after_wake = self
            .resource
            .active_stale_after_wake_for_node(resource_node);
        let prior_retry_wake = self.resource.pending_retry_wake_for_node(resource_node);
        let resolved_timeout = self.resolve_timeout_admission(
            resource_node,
            &timeout_plan,
            current_tick,
            intent.transaction_deadline(),
        )?;
        let prior_timeout_wake = self.resource.active_timeout_wake_for_node(resource_node);
        let scheduled_timeout_wake = resolved_timeout
            .as_ref()
            .map(|resolved| self.schedule_resource_timeout_wake(resource_node, resolved))
            .transpose()?
            .flatten();
        self.retire_superseded_resource_timeout_wake(
            prior_timeout_wake,
            scheduled_timeout_wake.as_ref(),
        )?;
        self.retire_superseded_resource_stale_after_wake(prior_stale_after_wake, None)?;
        self.retire_superseded_resource_retry_wake(prior_retry_wake)?;
        let _ = self.resource.clear_pending_retry_for_node(resource_node);
        let report = match self.resource.admit_resource_request(
            intent,
            self.graph.current_branch().id,
            current_tick,
            true,
            resolved_timeout.map(|resolved| {
                (
                    resolved.timeout_duration,
                    resolved.due_tick,
                    resolved.outcome_class,
                    resolved.deadline_authority,
                    resolved.decision_digest,
                )
            }),
            &mut self.telemetry.resource,
        ) {
            Ok(report) => report,
            Err(err) => {
                if let Some(wake) = scheduled_timeout_wake.as_ref() {
                    self.dispose_resource_timeout_wake(wake);
                }
                return Err(err);
            }
        };

        if let Some(wake) = scheduled_timeout_wake {
            if let Err(err) = self.resource.attach_timeout_wake(
                report.admitted_request().handle(),
                wake.id(),
                &mut self.telemetry.resource,
            ) {
                self.dispose_resource_timeout_wake(&wake);
                return Err(err);
            }
        }

        Ok(report)
    }

    pub fn admit_resource_completion(
        &mut self,
        completion: RawCompletionEnvelope,
    ) -> ResourceCompletionAdmissionReport {
        self.resource
            .admit_resource_completion(completion, &mut self.telemetry.resource)
    }

    pub fn admit_resource_completion_batch(
        &mut self,
        completions: impl IntoIterator<Item = RawCompletionEnvelope>,
    ) -> ResourceCompletionBatchAdmissionReport {
        self.resource
            .admit_resource_completion_batch(completions, &mut self.telemetry.resource)
    }

    pub fn stage_admitted_resource_completion(
        &mut self,
        admitted: AdmittedResourceCompletion,
    ) -> Result<ResourceCompletionStagingReport, crate::data::error::SignalError> {
        self.resource
            .stage_admitted_resource_completion(admitted, &mut self.telemetry.resource)
    }

    pub fn stage_denied_resource_completion(
        &mut self,
        denied: DeniedResourceCompletion,
    ) -> Result<ResourceCompletionDenialStagingReport, crate::data::error::SignalError> {
        self.resource
            .stage_denied_resource_completion(denied, &mut self.telemetry.resource)
    }

    pub fn rollback_staged_resource_completion(
        &mut self,
        staged: StagedResourceCompletionEffect,
    ) -> ResourceCompletionRollbackReport {
        self.resource
            .rollback_staged_resource_completion(staged, &mut self.telemetry.resource)
    }

    pub fn rollback_staged_denied_resource_completion(
        &mut self,
        staged: StagedDeniedResourceCompletionEffect,
    ) -> ResourceCompletionRollbackReport {
        self.resource
            .rollback_staged_denied_resource_completion(staged, &mut self.telemetry.resource)
    }

    pub fn commit_staged_resource_completion(
        &mut self,
        staged: StagedResourceCompletionEffect,
    ) -> Result<ResourceCompletionCommitReport, crate::data::error::SignalError> {
        let handle = staged.handle();
        if let Some(wake_id) = self.resource.active_timeout_wake_for_handle(handle) {
            self.retire_temporal_wake(wake_id, TemporalWakeRetirementReason::Consumed)?;
        }
        let report = self
            .resource
            .commit_staged_resource_completion(staged, &mut self.telemetry.resource)?;
        let node = report.lifecycle().node();
        let prior_stale_after_wake = self.resource.active_stale_after_wake_for_node(node);
        let scheduled_stale_after_wake = self
            .resource
            .descriptor_for_node(node)
            .and_then(|descriptor| {
                let revalidation_plan = descriptor.revalidation_decision_plan();
                if !revalidation_plan.permits_stale_after_revalidation() {
                    return None;
                }
                descriptor.stale_after_decision_plan().stale_after()
            })
            .map(|stale_after| self.schedule_resource_stale_after_wake(node, stale_after))
            .transpose()?
            .flatten();
        self.retire_superseded_resource_stale_after_wake(
            prior_stale_after_wake,
            scheduled_stale_after_wake.as_ref(),
        )?;
        if let Some(wake) = scheduled_stale_after_wake {
            self.resource.attach_stale_after_wake(node, wake.id());
        }
        Ok(report)
    }

    pub fn revalidate_resource_node(
        &mut self,
        intent: ResourceRevalidationIntent,
    ) -> Result<ResourceRevalidationReport, crate::data::error::SignalError> {
        let resource_node = intent.node();
        if !self.graph.is_alive(resource_node.node()) {
            self.telemetry.resource.resource_non_live_owner_denial_count += 1;
            return Err(crate::data::error::SignalError::invalid_input(format!(
                "cannot revalidate resource node for non-live owner {}",
                resource_node.node()
            )));
        }

        let current_tick = self.clock_basis().current_tick();
        let revalidation_descriptor = self.resource.descriptor_for_node(resource_node);
        let timeout_plan = revalidation_descriptor
            .map(|descriptor| descriptor.timeout_decision_plan().clone())
            .unwrap_or_else(|| LoweredResourceDescriptor::default_timeout_decision_plan());
        let revalidation_decision_digest = revalidation_descriptor
            .map(|descriptor| {
                descriptor
                    .revalidation_decision_plan()
                    .decision_digest()
                    .clone()
            })
            .unwrap_or_else(|| {
                crate::data::resource::ResourcePolicyDigest::new(
                    "resource-policy-revalidation-plan:undeclared",
                )
            });
        let freshness_decision =
            crate::data::resource::ResourceRevalidationFreshnessDecision::explicit_intent(
                revalidation_decision_digest.clone(),
            );
        let resolved_timeout = self.resolve_timeout_admission(
            resource_node,
            &timeout_plan,
            current_tick,
            intent.transaction_deadline(),
        )?;
        let prior_timeout_wake = self.resource.active_timeout_wake_for_node(resource_node);
        let prior_stale_after_wake = self
            .resource
            .active_stale_after_wake_for_node(resource_node);
        let scheduled_timeout_wake = resolved_timeout
            .as_ref()
            .map(|resolved| self.schedule_resource_timeout_wake(resource_node, resolved))
            .transpose()?
            .flatten();
        self.retire_superseded_resource_timeout_wake(
            prior_timeout_wake,
            scheduled_timeout_wake.as_ref(),
        )?;
        self.retire_superseded_resource_stale_after_wake(prior_stale_after_wake, None)?;

        let report = self.resource.admit_resource_revalidation(
            intent,
            self.graph.current_branch().id,
            current_tick,
            true,
            revalidation_decision_digest,
            freshness_decision,
            None,
            None,
            None,
            None,
            None,
            None,
            resolved_timeout.map(|resolved| {
                (
                    resolved.timeout_duration,
                    resolved.due_tick,
                    resolved.outcome_class,
                    resolved.deadline_authority,
                    resolved.decision_digest,
                )
            }),
            &mut self.telemetry.resource,
        );
        if let Some(wake) = scheduled_timeout_wake {
            if let Some(admitted) = report.admitted_revalidation() {
                if let Err(err) = self.resource.attach_timeout_wake(
                    admitted.admitted_request().handle(),
                    wake.id(),
                    &mut self.telemetry.resource,
                ) {
                    self.dispose_resource_timeout_wake(&wake);
                    return Err(err);
                }
            } else {
                self.dispose_resource_timeout_wake(&wake);
            }
        }
        Ok(report)
    }

    pub fn prove_active_resource_revalidation_handle(
        &mut self,
        handle: ResourceRequestHandle,
    ) -> Result<
        crate::data::resource::ActiveResourceRevalidationProof,
        crate::data::error::SignalError,
    > {
        self.resource
            .prove_active_resource_revalidation_handle(handle, &mut self.telemetry.resource)
            .ok_or_else(|| {
                crate::data::error::SignalError::invalid_input(format!(
                    "cannot mint active revalidation proof for stale or non-active request {}",
                    handle.request_id().get()
                ))
            })
    }

    pub fn force_revalidate_resource_node(
        &mut self,
        proof: crate::data::resource::ActiveResourceRevalidationProof,
    ) -> Result<ResourceRevalidationReport, crate::data::error::SignalError> {
        let resource_node = proof.node();
        if !self.graph.is_alive(resource_node.node()) {
            self.telemetry.resource.resource_non_live_owner_denial_count += 1;
            return Err(crate::data::error::SignalError::invalid_input(format!(
                "cannot force revalidate non-live resource node {}",
                resource_node.node()
            )));
        }

        let current_tick = self.clock_basis().current_tick();
        self.telemetry
            .resource
            .resource_revalidation_policy_decision_count += 1;
        self.telemetry.resource.resource_forced_revalidation_count += 1;
        if let Some(class) = self
            .resource
            .validate_forced_resource_revalidation_proof(&proof)
        {
            return Ok(self.resource.deny_forced_revalidation_for_report(
                proof.node(),
                proof.handle(),
                class,
                &mut self.telemetry.resource,
            ));
        }
        let timeout_plan = self
            .resource
            .descriptor_for_node(resource_node)
            .map(|descriptor| descriptor.timeout_decision_plan().clone())
            .unwrap_or_else(|| LoweredResourceDescriptor::default_timeout_decision_plan());
        let resolved_timeout =
            self.resolve_timeout_admission(resource_node, &timeout_plan, current_tick, None)?;
        let prior_timeout_wake = self.resource.active_timeout_wake_for_node(resource_node);
        let prior_stale_after_wake = self
            .resource
            .active_stale_after_wake_for_node(resource_node);
        let scheduled_timeout_wake = resolved_timeout
            .as_ref()
            .map(|resolved| self.schedule_resource_timeout_wake(resource_node, resolved))
            .transpose()?
            .flatten();
        self.retire_superseded_resource_timeout_wake(
            prior_timeout_wake,
            scheduled_timeout_wake.as_ref(),
        )?;
        self.retire_superseded_resource_stale_after_wake(prior_stale_after_wake, None)?;

        let report = self.resource.admit_forced_resource_revalidation(
            &proof,
            self.graph.current_branch().id,
            current_tick,
            resolved_timeout.map(|resolved| {
                (
                    resolved.timeout_duration,
                    resolved.due_tick,
                    resolved.outcome_class,
                    resolved.deadline_authority,
                    resolved.decision_digest,
                )
            }),
            &mut self.telemetry.resource,
        );
        if let Some(wake) = scheduled_timeout_wake {
            if let Some(admitted) = report.admitted_revalidation() {
                if let Err(err) = self.resource.attach_timeout_wake(
                    admitted.admitted_request().handle(),
                    wake.id(),
                    &mut self.telemetry.resource,
                ) {
                    self.dispose_resource_timeout_wake(&wake);
                    return Err(err);
                }
            } else {
                self.dispose_resource_timeout_wake(&wake);
            }
        }
        Ok(report)
    }

    pub fn prove_dependency_change_resource_revalidation(
        &mut self,
        node: ResourceNodeId,
    ) -> Result<DependencyChangeResourceRevalidationProof, crate::data::error::SignalError> {
        if !self.graph.is_alive(node.node()) {
            self.telemetry.resource.resource_non_live_owner_denial_count += 1;
            return Err(crate::data::error::SignalError::invalid_input(format!(
                "cannot mint dependency-change revalidation proof for non-live resource node {}",
                node.node()
            )));
        }
        let node_state = self.graph.get_state(node.node())?;
        self.resource
            .prove_dependency_change_resource_revalidation(
                node,
                node_state,
                &mut self.telemetry.resource,
            )
            .ok_or_else(|| {
                crate::data::error::SignalError::invalid_input(format!(
                    "cannot mint dependency-change revalidation proof for clean or policy-disabled resource node {} in state {:?}",
                    node.node(),
                    node_state
                ))
            })
    }

    fn latest_committed_observer_demand_event_for_node(
        &self,
        node: ResourceNodeId,
    ) -> Option<&CommittedObservationEventSummary> {
        self.observe()
            .latest_observation_summary()?
            .boundary_events
            .iter()
            .find(|event| {
                event.outcome == ObservationBoundaryOutcome::Delivered
                    && event.trigger_matched
                    && event
                        .matched_nodes
                        .iter()
                        .any(|matched| matched == node.node())
            })
    }

    fn observer_demand_observation_digest(event: &CommittedObservationEventSummary) -> String {
        let policy = format!(
            "{:?}:{:?}",
            event.policy.trigger(),
            event.policy.delivery_mode()
        );
        let matched_nodes = event
            .matched_nodes
            .iter()
            .map(|node| format!("{}:{}", node.index(), node.generation()))
            .collect::<Vec<_>>();
        ResourceRuntimeState::observer_demand_observation_digest(
            event.observer_id.get(),
            event.handle_id.get(),
            &policy,
            &matched_nodes,
            event.touched,
            event.recomputed,
            event.meaningful_change,
            event.trigger_matched,
            matches!(event.outcome, ObservationBoundaryOutcome::Delivered),
        )
    }

    pub fn prove_observer_demand_resource_revalidation(
        &mut self,
        node: ResourceNodeId,
    ) -> Result<ObserverDemandResourceRevalidationProof, crate::data::error::SignalError> {
        if !self.graph.is_alive(node.node()) {
            self.telemetry.resource.resource_non_live_owner_denial_count += 1;
            return Err(crate::data::error::SignalError::invalid_input(format!(
                "cannot mint observer-demand revalidation proof for non-live resource node {}",
                node.node()
            )));
        }
        self.telemetry
            .resource
            .resource_revalidation_observer_demand_proof_check_count += 1;
        let descriptor = self.resource.descriptor_for_node(node).ok_or_else(|| {
            crate::data::error::SignalError::invalid_input(format!(
                "cannot mint observer-demand revalidation proof for undeclared resource node {}",
                node.node()
            ))
        })?;
        if !descriptor
            .revalidation_decision_plan()
            .permits_observer_demand_revalidation()
        {
            return Err(crate::data::error::SignalError::invalid_input(format!(
                "cannot mint observer-demand revalidation proof for policy-disabled resource node {}",
                node.node()
            )));
        }
        let event = self
            .latest_committed_observer_demand_event_for_node(node)
            .ok_or_else(|| {
                crate::data::error::SignalError::invalid_input(format!(
                    "cannot mint observer-demand revalidation proof without committed matching observation for resource node {}",
                    node.node()
                ))
            })?;
        Ok(ObserverDemandResourceRevalidationProof::new(
            node,
            event.observer_id.get(),
            event.handle_id.get(),
            Self::observer_demand_observation_digest(event),
            descriptor
                .revalidation_decision_plan()
                .decision_digest()
                .clone(),
        ))
    }

    pub fn revalidate_resource_node_for_dependency_change(
        &mut self,
        proof: DependencyChangeResourceRevalidationProof,
    ) -> Result<ResourceRevalidationReport, crate::data::error::SignalError> {
        let resource_node = proof.node();
        if !self.graph.is_alive(resource_node.node()) {
            self.telemetry.resource.resource_non_live_owner_denial_count += 1;
            return Err(crate::data::error::SignalError::invalid_input(format!(
                "cannot dependency-change revalidate non-live resource node {}",
                resource_node.node()
            )));
        }

        let current_tick = self.clock_basis().current_tick();
        self.telemetry
            .resource
            .resource_revalidation_policy_decision_count += 1;
        let current_node_state = self.graph.get_state(resource_node.node())?;
        if let Some(class) = self
            .resource
            .validate_dependency_change_resource_revalidation_proof(&proof, current_node_state)
        {
            return Ok(self.resource.deny_resource_revalidation_for_report(
                proof.node(),
                class,
                &mut self.telemetry.resource,
            ));
        }

        self.telemetry
            .resource
            .resource_dependency_change_revalidation_count += 1;
        let timeout_plan = self
            .resource
            .descriptor_for_node(resource_node)
            .map(|descriptor| descriptor.timeout_decision_plan().clone())
            .unwrap_or_else(|| LoweredResourceDescriptor::default_timeout_decision_plan());
        let resolved_timeout =
            self.resolve_timeout_admission(resource_node, &timeout_plan, current_tick, None)?;
        let prior_timeout_wake = self.resource.active_timeout_wake_for_node(resource_node);
        let prior_stale_after_wake = self
            .resource
            .active_stale_after_wake_for_node(resource_node);
        let scheduled_timeout_wake = resolved_timeout
            .as_ref()
            .map(|resolved| self.schedule_resource_timeout_wake(resource_node, resolved))
            .transpose()?
            .flatten();
        self.retire_superseded_resource_timeout_wake(
            prior_timeout_wake,
            scheduled_timeout_wake.as_ref(),
        )?;
        self.retire_superseded_resource_stale_after_wake(prior_stale_after_wake, None)?;

        let report = self.resource.admit_dependency_change_resource_revalidation(
            proof,
            self.graph.current_branch().id,
            current_tick,
            resolved_timeout.map(|resolved| {
                (
                    resolved.timeout_duration,
                    resolved.due_tick,
                    resolved.outcome_class,
                    resolved.deadline_authority,
                    resolved.decision_digest,
                )
            }),
            &mut self.telemetry.resource,
        );
        if let Some(wake) = scheduled_timeout_wake {
            if let Some(admitted) = report.admitted_revalidation() {
                if let Err(err) = self.resource.attach_timeout_wake(
                    admitted.admitted_request().handle(),
                    wake.id(),
                    &mut self.telemetry.resource,
                ) {
                    self.dispose_resource_timeout_wake(&wake);
                    return Err(err);
                }
            } else {
                self.dispose_resource_timeout_wake(&wake);
            }
        }
        Ok(report)
    }

    pub fn revalidate_resource_node_for_observer_demand(
        &mut self,
        proof: ObserverDemandResourceRevalidationProof,
    ) -> Result<ResourceRevalidationReport, crate::data::error::SignalError> {
        let resource_node = proof.node();
        if !self.graph.is_alive(resource_node.node()) {
            self.telemetry.resource.resource_non_live_owner_denial_count += 1;
            return Err(crate::data::error::SignalError::invalid_input(format!(
                "cannot observer-demand revalidate non-live resource node {}",
                resource_node.node()
            )));
        }

        let current_tick = self.clock_basis().current_tick();
        self.telemetry
            .resource
            .resource_revalidation_policy_decision_count += 1;
        let event = self.latest_committed_observer_demand_event_for_node(resource_node);
        if let Some(class) = self
            .resource
            .validate_observer_demand_resource_revalidation_proof(&proof)
            .or_else(|| match event {
                Some(event)
                    if proof.observer_id() == event.observer_id.get()
                        && proof.handle_id() == event.handle_id.get()
                        && proof.observation_digest()
                            == Self::observer_demand_observation_digest(event) =>
                {
                    None
                }
                _ => Some(ResourceRevalidationDenialClass::ObserverDemandProofMismatch),
            })
        {
            return Ok(self.resource.deny_resource_revalidation_for_report(
                proof.node(),
                class,
                &mut self.telemetry.resource,
            ));
        }

        self.telemetry
            .resource
            .resource_observer_demand_revalidation_count += 1;
        let timeout_plan = self
            .resource
            .descriptor_for_node(resource_node)
            .map(|descriptor| descriptor.timeout_decision_plan().clone())
            .unwrap_or_else(|| LoweredResourceDescriptor::default_timeout_decision_plan());
        let resolved_timeout =
            self.resolve_timeout_admission(resource_node, &timeout_plan, current_tick, None)?;
        let prior_timeout_wake = self.resource.active_timeout_wake_for_node(resource_node);
        let prior_stale_after_wake = self
            .resource
            .active_stale_after_wake_for_node(resource_node);
        let scheduled_timeout_wake = resolved_timeout
            .as_ref()
            .map(|resolved| self.schedule_resource_timeout_wake(resource_node, resolved))
            .transpose()?
            .flatten();
        self.retire_superseded_resource_timeout_wake(
            prior_timeout_wake,
            scheduled_timeout_wake.as_ref(),
        )?;
        self.retire_superseded_resource_stale_after_wake(prior_stale_after_wake, None)?;

        let report = self.resource.admit_observer_demand_resource_revalidation(
            proof,
            self.graph.current_branch().id,
            current_tick,
            resolved_timeout.map(|resolved| {
                (
                    resolved.timeout_duration,
                    resolved.due_tick,
                    resolved.outcome_class,
                    resolved.deadline_authority,
                    resolved.decision_digest,
                )
            }),
            &mut self.telemetry.resource,
        );
        if let Some(wake) = scheduled_timeout_wake {
            if let Some(admitted) = report.admitted_revalidation() {
                if let Err(err) = self.resource.attach_timeout_wake(
                    admitted.admitted_request().handle(),
                    wake.id(),
                    &mut self.telemetry.resource,
                ) {
                    self.dispose_resource_timeout_wake(&wake);
                    return Err(err);
                }
            } else {
                self.dispose_resource_timeout_wake(&wake);
            }
        }
        Ok(report)
    }

    pub fn prove_terminal_state_resource_revalidation(
        &mut self,
        node: ResourceNodeId,
    ) -> Result<TerminalStateResourceRevalidationProof, crate::data::error::SignalError> {
        if !self.graph.is_alive(node.node()) {
            self.telemetry.resource.resource_non_live_owner_denial_count += 1;
            return Err(crate::data::error::SignalError::invalid_input(format!(
                "cannot mint terminal-state revalidation proof for non-live resource node {}",
                node.node()
            )));
        }
        let lifecycle = self
            .resource
            .lifecycle_summary_for_node(node)
            .map(|summary| summary.lifecycle())
            .unwrap_or(ResourceLifecycleClass::Unrequested);
        self.resource
            .prove_terminal_state_resource_revalidation(node, &mut self.telemetry.resource)
            .ok_or_else(|| {
                crate::data::error::SignalError::invalid_input(format!(
                    "cannot mint terminal-state revalidation proof for resource node {} in lifecycle {:?}",
                    node.node(),
                    lifecycle
                ))
            })
    }

    pub fn revalidate_resource_node_for_terminal_state(
        &mut self,
        proof: TerminalStateResourceRevalidationProof,
    ) -> Result<ResourceRevalidationReport, crate::data::error::SignalError> {
        let resource_node = proof.node();
        if !self.graph.is_alive(resource_node.node()) {
            self.telemetry.resource.resource_non_live_owner_denial_count += 1;
            return Err(crate::data::error::SignalError::invalid_input(format!(
                "cannot terminal-state revalidate non-live resource node {}",
                resource_node.node()
            )));
        }

        let current_tick = self.clock_basis().current_tick();
        self.telemetry
            .resource
            .resource_revalidation_policy_decision_count += 1;
        if let Some(class) = self
            .resource
            .validate_terminal_state_resource_revalidation_proof(&proof)
        {
            return Ok(self.resource.deny_resource_revalidation_for_report(
                proof.node(),
                class,
                &mut self.telemetry.resource,
            ));
        }

        self.telemetry
            .resource
            .resource_terminal_state_revalidation_count += 1;
        let timeout_plan = self
            .resource
            .descriptor_for_node(resource_node)
            .map(|descriptor| descriptor.timeout_decision_plan().clone())
            .unwrap_or_else(|| LoweredResourceDescriptor::default_timeout_decision_plan());
        let resolved_timeout =
            self.resolve_timeout_admission(resource_node, &timeout_plan, current_tick, None)?;
        let prior_timeout_wake = self.resource.active_timeout_wake_for_node(resource_node);
        let prior_stale_after_wake = self
            .resource
            .active_stale_after_wake_for_node(resource_node);
        let scheduled_timeout_wake = resolved_timeout
            .as_ref()
            .map(|resolved| self.schedule_resource_timeout_wake(resource_node, resolved))
            .transpose()?
            .flatten();
        self.retire_superseded_resource_timeout_wake(
            prior_timeout_wake,
            scheduled_timeout_wake.as_ref(),
        )?;
        self.retire_superseded_resource_stale_after_wake(prior_stale_after_wake, None)?;

        let report = self.resource.admit_terminal_state_resource_revalidation(
            proof,
            self.graph.current_branch().id,
            current_tick,
            resolved_timeout.map(|resolved| {
                (
                    resolved.timeout_duration,
                    resolved.due_tick,
                    resolved.outcome_class,
                    resolved.deadline_authority,
                    resolved.decision_digest,
                )
            }),
            &mut self.telemetry.resource,
        );
        if let Some(wake) = scheduled_timeout_wake {
            if let Some(admitted) = report.admitted_revalidation() {
                if let Err(err) = self.resource.attach_timeout_wake(
                    admitted.admitted_request().handle(),
                    wake.id(),
                    &mut self.telemetry.resource,
                ) {
                    self.dispose_resource_timeout_wake(&wake);
                    return Err(err);
                }
            } else {
                self.dispose_resource_timeout_wake(&wake);
            }
        }
        Ok(report)
    }

    pub fn prove_fulfilled_lifecycle_resource_revalidation(
        &mut self,
        node: ResourceNodeId,
    ) -> Result<FulfilledLifecycleResourceRevalidationProof, crate::data::error::SignalError> {
        if !self.graph.is_alive(node.node()) {
            self.telemetry.resource.resource_non_live_owner_denial_count += 1;
            return Err(crate::data::error::SignalError::invalid_input(format!(
                "cannot mint fulfilled-lifecycle revalidation proof for non-live resource node {}",
                node.node()
            )));
        }
        let lifecycle = self
            .resource
            .lifecycle_summary_for_node(node)
            .map(|summary| summary.lifecycle())
            .unwrap_or(ResourceLifecycleClass::Unrequested);
        self.resource
            .prove_fulfilled_lifecycle_resource_revalidation(node, &mut self.telemetry.resource)
            .ok_or_else(|| {
                crate::data::error::SignalError::invalid_input(format!(
                    "cannot mint fulfilled-lifecycle revalidation proof for resource node {} in lifecycle {:?}",
                    node.node(),
                    lifecycle
                ))
            })
    }

    pub fn revalidate_resource_node_for_fulfilled_lifecycle(
        &mut self,
        proof: FulfilledLifecycleResourceRevalidationProof,
    ) -> Result<ResourceRevalidationReport, crate::data::error::SignalError> {
        let resource_node = proof.node();
        if !self.graph.is_alive(resource_node.node()) {
            self.telemetry.resource.resource_non_live_owner_denial_count += 1;
            return Err(crate::data::error::SignalError::invalid_input(format!(
                "cannot fulfilled-lifecycle revalidate non-live resource node {}",
                resource_node.node()
            )));
        }

        let current_tick = self.clock_basis().current_tick();
        self.telemetry
            .resource
            .resource_revalidation_policy_decision_count += 1;
        if let Some(class) = self
            .resource
            .validate_fulfilled_lifecycle_resource_revalidation_proof(&proof)
        {
            return Ok(self.resource.deny_resource_revalidation_for_report(
                proof.node(),
                class,
                &mut self.telemetry.resource,
            ));
        }

        self.telemetry
            .resource
            .resource_fulfilled_lifecycle_revalidation_count += 1;
        let timeout_plan = self
            .resource
            .descriptor_for_node(resource_node)
            .map(|descriptor| descriptor.timeout_decision_plan().clone())
            .unwrap_or_else(|| LoweredResourceDescriptor::default_timeout_decision_plan());
        let resolved_timeout =
            self.resolve_timeout_admission(resource_node, &timeout_plan, current_tick, None)?;
        let prior_timeout_wake = self.resource.active_timeout_wake_for_node(resource_node);
        let prior_stale_after_wake = self
            .resource
            .active_stale_after_wake_for_node(resource_node);
        let scheduled_timeout_wake = resolved_timeout
            .as_ref()
            .map(|resolved| self.schedule_resource_timeout_wake(resource_node, resolved))
            .transpose()?
            .flatten();
        self.retire_superseded_resource_timeout_wake(
            prior_timeout_wake,
            scheduled_timeout_wake.as_ref(),
        )?;
        self.retire_superseded_resource_stale_after_wake(prior_stale_after_wake, None)?;

        let report = self
            .resource
            .admit_fulfilled_lifecycle_resource_revalidation(
                proof,
                self.graph.current_branch().id,
                current_tick,
                resolved_timeout.map(|resolved| {
                    (
                        resolved.timeout_duration,
                        resolved.due_tick,
                        resolved.outcome_class,
                        resolved.deadline_authority,
                        resolved.decision_digest,
                    )
                }),
                &mut self.telemetry.resource,
            );
        if let Some(wake) = scheduled_timeout_wake {
            if let Some(admitted) = report.admitted_revalidation() {
                if let Err(err) = self.resource.attach_timeout_wake(
                    admitted.admitted_request().handle(),
                    wake.id(),
                    &mut self.telemetry.resource,
                ) {
                    self.dispose_resource_timeout_wake(&wake);
                    return Err(err);
                }
            } else {
                self.dispose_resource_timeout_wake(&wake);
            }
        }
        Ok(report)
    }

    pub fn active_resource_stale_after_wake_for_node(
        &self,
        node: ResourceNodeId,
    ) -> Option<crate::data::temporal::TemporalWakeId> {
        self.resource.active_stale_after_wake_for_node(node)
    }

    pub fn admit_stale_after_resource_revalidation(
        &mut self,
        node: ResourceNodeId,
        ready_wake: ReadyTemporalWake,
    ) -> Result<ResourceRevalidationReport, crate::data::error::SignalError> {
        if !self.graph.is_alive(node.node()) {
            self.telemetry.resource.resource_non_live_owner_denial_count += 1;
            return Err(crate::data::error::SignalError::invalid_input(format!(
                "cannot stale-after revalidate non-live resource node {}",
                node.node()
            )));
        }

        self.telemetry
            .resource
            .resource_revalidation_policy_decision_count += 1;
        let validation = self
            .resource
            .validate_stale_after_resource_revalidation(node, &ready_wake);
        let wake_id = ready_wake.id();
        let matched_active_wake = self
            .resource
            .active_stale_after_wake_for_node(node)
            .is_some_and(|active| active == wake_id);
        if matched_active_wake {
            self.retire_temporal_wake(wake_id, TemporalWakeRetirementReason::Consumed)?;
            self.resource.clear_stale_after_wake_for_node(node);
        }
        if let Some(class) = validation {
            return Ok(self.resource.deny_resource_revalidation_for_report(
                node,
                class,
                &mut self.telemetry.resource,
            ));
        }

        self.telemetry
            .resource
            .resource_stale_after_revalidation_count += 1;
        let current_tick = self.clock_basis().current_tick();
        let revalidation_descriptor = self.resource.descriptor_for_node(node);
        let timeout_plan = revalidation_descriptor
            .map(|descriptor| descriptor.timeout_decision_plan().clone())
            .unwrap_or_else(|| LoweredResourceDescriptor::default_timeout_decision_plan());
        let revalidation_decision_digest = revalidation_descriptor
            .map(|descriptor| {
                descriptor
                    .revalidation_decision_plan()
                    .decision_digest()
                    .clone()
            })
            .unwrap_or_else(|| {
                crate::data::resource::ResourcePolicyDigest::new(
                    "resource-policy-revalidation-plan:undeclared",
                )
            });
        let resolved_timeout =
            self.resolve_timeout_admission(node, &timeout_plan, current_tick, None)?;
        let prior_timeout_wake = self.resource.active_timeout_wake_for_node(node);
        let scheduled_timeout_wake = resolved_timeout
            .as_ref()
            .map(|resolved| self.schedule_resource_timeout_wake(node, resolved))
            .transpose()?
            .flatten();
        self.retire_superseded_resource_timeout_wake(
            prior_timeout_wake,
            scheduled_timeout_wake.as_ref(),
        )?;

        let report = self.resource.admit_resource_revalidation(
            ResourceRevalidationIntent::new(node),
            self.graph.current_branch().id,
            current_tick,
            false,
            revalidation_decision_digest,
            crate::data::resource::ResourceRevalidationFreshnessDecision::stale_after(
                node,
                ready_wake.id(),
                self.resource
                    .descriptor_for_node(node)
                    .map(|descriptor| {
                        descriptor
                            .revalidation_decision_plan()
                            .decision_digest()
                            .clone()
                    })
                    .unwrap_or_else(|| {
                        crate::data::resource::ResourcePolicyDigest::new(
                            "resource-policy-revalidation-plan:undeclared",
                        )
                    }),
            ),
            None,
            None,
            None,
            None,
            None,
            Some(ready_wake),
            resolved_timeout.map(|resolved| {
                (
                    resolved.timeout_duration,
                    resolved.due_tick,
                    resolved.outcome_class,
                    resolved.deadline_authority,
                    resolved.decision_digest,
                )
            }),
            &mut self.telemetry.resource,
        );
        if let Some(wake) = scheduled_timeout_wake {
            if let Some(admitted) = report.admitted_revalidation() {
                if let Err(err) = self.resource.attach_timeout_wake(
                    admitted.admitted_request().handle(),
                    wake.id(),
                    &mut self.telemetry.resource,
                ) {
                    self.dispose_resource_timeout_wake(&wake);
                    return Err(err);
                }
            } else {
                self.dispose_resource_timeout_wake(&wake);
            }
        }
        Ok(report)
    }

    pub fn cancel_resource_request(
        &mut self,
        handle: ResourceRequestHandle,
        reason: ResourceCancellationReason,
    ) -> Result<ResourceCancellationReport, crate::data::error::SignalError> {
        for wake_id in self
            .resource
            .active_timeout_wakes_for_cancellation_footprint(handle)
        {
            self.retire_temporal_wake(wake_id, TemporalWakeRetirementReason::Cancelled)?;
        }
        let report =
            self.resource
                .cancel_resource_request(handle, reason, &mut self.telemetry.resource);
        Ok(report)
    }

    pub fn reject_resource_request(
        &mut self,
        handle: ResourceRequestHandle,
        reason: ResourceRejectionReason,
    ) -> Result<ResourceRejectionReport, crate::data::error::SignalError> {
        if let Some(wake_id) = self.resource.active_timeout_wake_for_handle(handle) {
            self.retire_temporal_wake(wake_id, TemporalWakeRetirementReason::Consumed)?;
        }
        let report =
            self.resource
                .reject_resource_request(handle, reason, &mut self.telemetry.resource);
        Ok(report)
    }

    pub fn admit_resource_timeout(
        &mut self,
        handle: ResourceRequestHandle,
        ready_wake: ReadyTemporalWake,
    ) -> Result<ResourceTimeoutReport, crate::data::error::SignalError> {
        let wake_id = ready_wake.id();
        if self
            .resource
            .active_timeout_wake_for_handle(handle)
            .is_some_and(|active| active == wake_id)
        {
            self.retire_temporal_wake(wake_id, TemporalWakeRetirementReason::Consumed)?;
        }
        let report =
            self.resource
                .admit_resource_timeout(handle, ready_wake, &mut self.telemetry.resource);
        Ok(report)
    }

    pub fn extend_resource_timeout_heartbeat(
        &mut self,
        handle: ResourceRequestHandle,
    ) -> Result<ResourceTimeoutHeartbeatExtensionReport, crate::data::error::SignalError> {
        let (node, previous_timeout_wake_id, extension_duration) = match self
            .resource
            .timeout_heartbeat_extension_candidate(handle, &mut self.telemetry.resource)
        {
            Ok(candidate) => candidate,
            Err(class) => {
                return Ok(self.resource.deny_timeout_heartbeat_extension_for_report(
                    handle.request_id(),
                    class,
                    &mut self.telemetry.resource,
                ))
            }
        };
        let current_tick = self.clock_basis().current_tick();
        let due_tick = crate::data::temporal::ClockTick::new(
            current_tick.get().saturating_add(extension_duration.get()),
        );
        let extended_timeout_wake = self.schedule_owned_temporal_wake(
            TemporalWakeOwner::ResourceNode(node.node()),
            TemporalCondition::after(extension_duration.get())?,
            due_tick,
        )?;
        let report = self.resource.extend_timeout_heartbeat(
            handle,
            previous_timeout_wake_id,
            extended_timeout_wake.clone(),
            &mut self.telemetry.resource,
        );
        if report.extended_heartbeat().is_some() {
            self.retire_temporal_wake(
                previous_timeout_wake_id,
                TemporalWakeRetirementReason::Superseded,
            )?;
            return Ok(report);
        }
        let _ = self.retire_temporal_wake(
            extended_timeout_wake.id(),
            TemporalWakeRetirementReason::Disposed,
        );
        Ok(report)
    }

    pub fn schedule_resource_retry(
        &mut self,
        handle: ResourceRequestHandle,
        reason: ResourceRetryReason,
    ) -> Result<ResourceRetryScheduleReport, crate::data::error::SignalError> {
        let (delay, next_attempt, retry_decision_digest, retry_budget_charge) =
            match self.resource.retry_backoff_delay_for_handle(
                handle,
                self.clock_basis().current_tick(),
                &mut self.telemetry.resource,
            ) {
                Ok(delay) => delay,
                Err(class) => {
                    return Ok(self.resource.deny_resource_retry_schedule(
                        handle,
                        class,
                        &mut self.telemetry.resource,
                    ));
                }
            };
        let condition = TemporalCondition::after(delay.get())?;
        let current_tick = self.clock_basis().current_tick();
        let due_tick =
            crate::data::temporal::ClockTick::new(current_tick.get().saturating_add(delay.get()));
        let node = self
            .resource
            .in_flight_request(handle, &mut self.telemetry.resource)
            .map(|in_flight| in_flight.node())
            .ok_or_else(|| {
                crate::data::error::SignalError::invalid_input(format!(
                    "cannot schedule retry for unknown resource request {}",
                    handle.request_id().get()
                ))
            })?;
        let wake = self.schedule_owned_temporal_wake(
            TemporalWakeOwner::ResourceNode(node.node()),
            condition,
            due_tick,
        )?;
        let report = self.resource.schedule_resource_retry(
            handle,
            reason,
            wake.id(),
            next_attempt,
            delay,
            retry_decision_digest,
            retry_budget_charge,
            &mut self.telemetry.resource,
        );
        if report.denied_retry().is_some() {
            let _ = self.retire_temporal_wake(wake.id(), TemporalWakeRetirementReason::Disposed);
        }
        Ok(report)
    }

    pub fn admit_scheduled_resource_retry(
        &mut self,
        handle: ResourceRequestHandle,
        ready_wake: ReadyTemporalWake,
    ) -> Result<ResourceRetryAdmissionReport, crate::data::error::SignalError> {
        let retry_lineage = self.in_flight_resource_request(handle).cloned();
        let current_tick = self.clock_basis().current_tick();
        let resource_node = retry_lineage.as_ref().map(|in_flight| in_flight.node());
        let timeout_plan = resource_node
            .and_then(|node| self.resource.descriptor_for_node(node))
            .map(|descriptor| descriptor.timeout_decision_plan().clone())
            .unwrap_or_else(|| LoweredResourceDescriptor::default_timeout_decision_plan());
        let prior_timeout_wake =
            resource_node.and_then(|node| self.resource.active_timeout_wake_for_node(node));
        let prior_stale_after_wake =
            resource_node.and_then(|node| self.resource.active_stale_after_wake_for_node(node));
        let timeout_resolution = match retry_lineage {
            Some(in_flight) => {
                Some(self.resolve_retry_timeout_admission(in_flight, &timeout_plan)?)
            }
            None => None,
        };
        let scheduled_timeout_wake = match (resource_node, timeout_resolution.as_ref()) {
            (Some(node), Some(Ok(resolved))) => {
                self.schedule_resource_timeout_wake(node, resolved)?
            }
            _ => None,
        };
        if !matches!(
            timeout_resolution,
            Some(Err(
                RetryTimeoutAdmissionResolution::InheritedDeadlineExhausted
            ))
        ) {
            self.retire_superseded_resource_timeout_wake(
                prior_timeout_wake,
                scheduled_timeout_wake.as_ref(),
            )?;
            self.retire_superseded_resource_stale_after_wake(prior_stale_after_wake, None)?;
        }

        let wake_id = ready_wake.id();
        if self
            .resource
            .pending_retry_wake_for_handle(handle)
            .is_some_and(|pending| pending == wake_id)
        {
            self.retire_temporal_wake(wake_id, TemporalWakeRetirementReason::Consumed)?;
        }

        if matches!(
            timeout_resolution,
            Some(Err(
                RetryTimeoutAdmissionResolution::InheritedDeadlineExhausted
            ))
        ) {
            return Ok(self.resource.deny_resource_retry_admission_for_report(
                handle,
                ResourceRetryDenialClass::RetryTimeoutWindowExhausted,
                &mut self.telemetry.resource,
            ));
        }

        let resolved_timeout = match timeout_resolution {
            Some(Ok(resolved)) => Some(resolved),
            Some(Err(RetryTimeoutAdmissionResolution::Disabled)) | None => None,
            Some(Err(RetryTimeoutAdmissionResolution::InheritedDeadlineExhausted)) => None,
        };
        let report = self.resource.admit_scheduled_resource_retry(
            handle,
            ready_wake,
            self.graph.current_branch().id,
            current_tick,
            resolved_timeout.map(|resolved| {
                (
                    resolved.timeout_duration,
                    resolved.due_tick,
                    resolved.outcome_class,
                    resolved.deadline_authority,
                    resolved.decision_digest,
                )
            }),
            &mut self.telemetry.resource,
        );

        if let Some(wake) = scheduled_timeout_wake {
            if let Some(admitted) = report.admitted_retry() {
                if let Err(err) = self.resource.attach_timeout_wake(
                    admitted.admitted_request().handle(),
                    wake.id(),
                    &mut self.telemetry.resource,
                ) {
                    self.dispose_resource_timeout_wake(&wake);
                    return Err(err);
                }
            } else {
                self.dispose_resource_timeout_wake(&wake);
            }
        }

        Ok(report)
    }

    pub fn telemetry(&self) -> &RuntimeTelemetry {
        &self.telemetry
    }

    pub(super) fn capture_full_authority_state(&self) -> AuthorityState<T> {
        AuthorityState::capture(&self.graph, &self.config)
    }

    pub(super) fn capture_full_derived_state(&self) -> DerivedState<D, I> {
        DerivedState::capture(
            &self.checkpoint,
            &self.resource,
            &self.temporal,
            &self.telemetry,
        )
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
            resource: std::mem::take(&mut self.resource),
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
            &mut self.resource,
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
