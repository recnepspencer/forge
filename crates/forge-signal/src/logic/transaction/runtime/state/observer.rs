use crate::data::comparator::TierPolicyResolver;
use crate::data::error::SignalError;
use crate::data::graph::{EvaluationStrategy, GraphMaterializer, GraphObserver};
use crate::data::handle::NodeId;
use crate::data::proof::{FrontierExecutionSummary, InvalidationTraceRecord};
use crate::data::telemetry::{EvaluationTelemetry, InvalidationTelemetry};
use crate::diagnostics::access::RuntimeDiagnostics;
use crate::diagnostics::facts::ProvenanceFact;
use crate::diagnostics::history::ExecutionInspector;
use crate::diagnostics::lineage::{LineageArtifactId, SynthesizedLineageChain};
use crate::diagnostics::policy::SignalRuntimePolicy;
use crate::diagnostics::profile::DiagnosticsTier;
use crate::diagnostics::summary::{
    ExecutionHistorySummary, GraphSummary, TemporalDiagnosticsSummary,
};
use crate::diagnostics::{
    FailureSummary, FlowSummary, ReplayView, RollbackDiagnostic, SynthesizedReplaySlice,
};
use crate::logic::explain::{explain_with_policy_resolver, NodeExplanation};
use crate::logic::transaction::ObservationBoundarySummary;
use crate::presentation::metrics::RuntimeMetrics;
use crate::state::{SignalBranchHandle, SignalBranchId, SignalSnapshotId};

use super::runtime_observation::{MatchingObserverSet, ObservationRegistrySummary};
use super::runtime_state::SignalRuntime;
use super::CheckpointRecord;

pub struct RuntimeObserver<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    runtime: &'a SignalRuntime<D, I, E, Ctx, T>,
}

pub struct RuntimeMaterializer<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    runtime: &'a SignalRuntime<D, I, E, Ctx, T>,
}

impl<'a, D, I, E, Ctx, T> RuntimeObserver<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(crate) fn new(runtime: &'a SignalRuntime<D, I, E, Ctx, T>) -> Self {
        Self { runtime }
    }

    pub fn graph(&self) -> GraphObserver<'a> {
        self.runtime.graph.observe()
    }

    pub fn materialize(&self) -> RuntimeMaterializer<'a, D, I, E, Ctx, T> {
        RuntimeMaterializer {
            runtime: self.runtime,
        }
    }

    pub fn explain(&self, node: NodeId) -> Result<NodeExplanation, SignalError> {
        let resolver = TierPolicyResolver::new(
            self.runtime.config.node_meta(),
            self.runtime.config.tier_policies(),
            self.runtime.config.fallback_comparator(),
        );
        explain_with_policy_resolver(&self.runtime.graph, node, &resolver)
    }

    pub fn metrics(&self) -> RuntimeMetrics {
        let graph_metrics = self.graph().metrics();
        RuntimeMetrics {
            evaluation: merge_evaluation_telemetry(
                graph_metrics.evaluation,
                self.runtime.telemetry.evaluation,
            ),
            invalidation: merge_invalidation_telemetry(
                graph_metrics.invalidation,
                self.runtime.telemetry.invalidation,
            ),
            transaction: self.runtime.telemetry.transaction,
            planner: graph_metrics.planner,
            execution: graph_metrics.execution,
            storage: graph_metrics.storage,
            checkpoint: self.composed_checkpoint_telemetry(),
            temporal: self.runtime.telemetry.temporal,
            host_computed: graph_metrics.host_computed,
        }
    }

    pub fn checkpoint_record(&self) -> CheckpointRecord {
        CheckpointRecord::from_checkpoint_telemetry(self.composed_checkpoint_telemetry())
    }

    pub fn diagnostics_summary(&self, profile: DiagnosticsTier) -> GraphSummary {
        self.graph().diagnostics_summary(profile)
    }

    pub fn temporal_diagnostics_summary(
        &self,
        profile: DiagnosticsTier,
    ) -> TemporalDiagnosticsSummary {
        TemporalDiagnosticsSummary::from_artifact(
            profile,
            self.runtime.temporal.frontier_snapshot(),
            crate::logic::transaction::TemporalReconstructabilityArtifact::from_temporal_state(
                &self.runtime.temporal,
            ),
            self.runtime.telemetry.temporal,
        )
    }

    pub fn temporal_diagnostics_summary_now(&self) -> TemporalDiagnosticsSummary {
        self.temporal_diagnostics_summary(self.diagnostics_profile())
    }

    pub fn diagnostics(&self) -> RuntimeDiagnostics<'a> {
        crate::diagnostics::access::diagnostics_for_runtime(self.runtime)
    }

    pub fn diagnostics_profile(&self) -> DiagnosticsTier {
        self.graph().diagnostics_profile()
    }

    pub fn observation_summary(&self) -> ObservationRegistrySummary {
        self.runtime.observations.summary()
    }

    pub fn matching_observers_for_node(&self, node: NodeId) -> MatchingObserverSet {
        self.runtime.observations.matching_observers_for_node(node)
    }

    fn composed_checkpoint_telemetry(&self) -> crate::data::telemetry::CheckpointTelemetry {
        crate::data::telemetry::CheckpointTelemetry {
            event_flushes: self.runtime.event_bus.telemetry().checkpoint.event_flushes,
            event_flush_nanos: self
                .runtime
                .event_bus
                .telemetry()
                .checkpoint
                .event_flush_nanos,
            checkpoint_flushes: self
                .runtime
                .checkpoint
                .telemetry()
                .checkpoint
                .checkpoint_flushes,
            checkpoint_flush_nanos: self
                .runtime
                .checkpoint
                .telemetry()
                .checkpoint
                .checkpoint_flush_nanos,
            rollback_count: self.runtime.event_bus.telemetry().checkpoint.rollback_count,
            snapshot_restore_count: self.runtime.telemetry.checkpoint.snapshot_restore_count,
            snapshot_restore_apply_active_policy_count: self
                .runtime
                .telemetry
                .checkpoint
                .snapshot_restore_apply_active_policy_count,
            snapshot_restore_shared_delta_node_count: self
                .runtime
                .telemetry
                .checkpoint
                .snapshot_restore_shared_delta_node_count,
            snapshot_restore_coarse_reason_count: self
                .runtime
                .telemetry
                .checkpoint
                .snapshot_restore_coarse_reason_count,
            checkpoint_size: self.runtime.telemetry.checkpoint.checkpoint_size,
            journal_replay_span: self.runtime.telemetry.checkpoint.journal_replay_span,
            restore_authority_breadth: self.runtime.telemetry.checkpoint.restore_authority_breadth,
            restore_required_derived_breadth: self
                .runtime
                .telemetry
                .checkpoint
                .restore_required_derived_breadth,
            restore_diagnostic_richness_breadth: self
                .runtime
                .telemetry
                .checkpoint
                .restore_diagnostic_richness_breadth,
        }
    }

    pub fn runtime_policy(&self) -> SignalRuntimePolicy {
        self.graph().runtime_policy()
    }

    pub fn evaluation_strategy(&self) -> EvaluationStrategy {
        self.graph().evaluation_strategy()
    }

    pub fn replay_for_node(&self, node: NodeId) -> ReplayView {
        self.graph().replay_for_node(node).to_owned_slice()
    }

    pub fn replay_for_artifact(&self, artifact_id: LineageArtifactId) -> ReplayView {
        self.graph()
            .replay_for_artifact(artifact_id)
            .to_owned_slice()
    }

    pub fn replay_from_cursor(&self, start: crate::diagnostics::ReplayCursor) -> ReplayView {
        self.graph().replay_from_cursor(start).to_owned_slice()
    }

    pub fn replay_between(
        &self,
        start: crate::diagnostics::ReplayCursor,
        end: crate::diagnostics::ReplayCursor,
    ) -> ReplayView {
        self.graph().replay_between(start, end).to_owned_slice()
    }

    pub fn replay_around_snapshot(&self, snapshot_id: SignalSnapshotId) -> ReplayView {
        self.graph()
            .replay_around_snapshot(snapshot_id)
            .to_owned_slice()
    }

    pub fn replay_for_branch(&self, branch_id: SignalBranchId) -> ReplayView {
        self.runtime
            .branches
            .replay_graph(
                branch_id,
                self.runtime.graph.current_branch().id,
                &self.runtime.graph,
            )
            .map(|graph| {
                graph
                    .observe()
                    .replay_for_branch(branch_id)
                    .to_owned_slice()
            })
            .unwrap_or_default()
    }

    pub fn replay_where(
        &self,
        predicate: impl FnMut(&crate::diagnostics::ReplayEvent) -> bool,
    ) -> SynthesizedReplaySlice {
        self.graph().replay_where(predicate)
    }

    pub fn current_lineage_artifact(&self, node: NodeId) -> Option<LineageArtifactId> {
        self.graph().current_lineage_artifact(node)
    }

    pub fn lineage_chain_for_node(&self, node: NodeId) -> SynthesizedLineageChain {
        self.graph().lineage_chain_for_node(node)
    }

    pub fn lineage_chain_for_artifact(
        &self,
        artifact_id: LineageArtifactId,
    ) -> SynthesizedLineageChain {
        self.graph().lineage_chain_for_artifact(artifact_id)
    }

    pub fn execution_history_summary(&self, profile: DiagnosticsTier) -> ExecutionHistorySummary {
        self.graph().execution_history_summary(profile)
    }

    pub fn inspect_execution(&self) -> ExecutionInspector<'a> {
        self.graph().inspect_execution()
    }

    pub fn latest_flow_diagnostics(&self) -> Option<&'a FlowSummary> {
        self.graph().latest_flow_diagnostics()
    }

    pub fn latest_failure_diagnostics(&self) -> Option<&'a FailureSummary> {
        self.graph().latest_failure_diagnostics()
    }

    pub fn latest_rollback_diagnostics(&self) -> Option<&'a RollbackDiagnostic> {
        self.graph().latest_rollback_diagnostics()
    }

    pub fn latest_observation_summary(&self) -> Option<&'a ObservationBoundarySummary> {
        self.runtime.graph.diagnostics_state().latest_observation()
    }

    pub fn latest_frontier_execution_summary(&self) -> Option<&'a FrontierExecutionSummary> {
        self.graph().latest_frontier_execution_summary()
    }

    pub fn latest_invalidation_trace_records(&self) -> &'a [InvalidationTraceRecord] {
        self.graph().latest_invalidation_trace_records()
    }

    pub fn recent_execution_history_diagnostics(
        &self,
    ) -> &'a std::collections::VecDeque<ExecutionHistorySummary> {
        self.graph().recent_execution_history_diagnostics()
    }

    pub fn to_dot(&self) -> String {
        self.graph().to_dot()
    }

    pub fn current_branch(&self) -> SignalBranchHandle {
        self.graph().current_branch()
    }

    pub fn known_branches(&self) -> Vec<SignalBranchHandle> {
        self.graph().known_branches()
    }

    pub fn branch_handle(&self, branch_id: SignalBranchId) -> Option<SignalBranchHandle> {
        self.graph()
            .branch_handle(branch_id)
            .or_else(|| self.runtime.branches.branch_handle(branch_id))
    }

    pub fn branch_ancestry(&self, branch_id: SignalBranchId) -> Vec<SignalBranchHandle> {
        if self.graph().branch_handle(branch_id).is_some() {
            self.graph().branch_ancestry(branch_id)
        } else {
            self.runtime.branches.branch_ancestry(branch_id)
        }
    }

    pub fn branch_head_snapshot_id(&self, branch_id: SignalBranchId) -> Option<SignalSnapshotId> {
        self.graph()
            .branch_head_snapshot_id(branch_id)
            .or_else(|| self.runtime.branches.branch_head_snapshot_id(branch_id))
    }
}

fn merge_evaluation_telemetry(
    graph: EvaluationTelemetry,
    runtime: EvaluationTelemetry,
) -> EvaluationTelemetry {
    EvaluationTelemetry {
        evaluation_calls: graph.evaluation_calls + runtime.evaluation_calls,
        evaluation_nanos: graph.evaluation_nanos + runtime.evaluation_nanos,
        nodes_evaluated: graph.nodes_evaluated + runtime.nodes_evaluated,
        nodes_recomputed: graph.nodes_recomputed + runtime.nodes_recomputed,
        reuse_eligibility_checks_attempted: graph.reuse_eligibility_checks_attempted
            + runtime.reuse_eligibility_checks_attempted,
        fresh_compute_count: graph.fresh_compute_count + runtime.fresh_compute_count,
        output_suppressed_count: graph.output_suppressed_count + runtime.output_suppressed_count,
        memoized_reuse_count: graph.memoized_reuse_count + runtime.memoized_reuse_count,
        snapshot_restore_reuse_count: graph.snapshot_restore_reuse_count
            + runtime.snapshot_restore_reuse_count,
        reconciliation_adoption_count: graph.reconciliation_adoption_count
            + runtime.reconciliation_adoption_count,
        cross_identity_reuse_count: graph.cross_identity_reuse_count
            + runtime.cross_identity_reuse_count,
        partial_artifact_splice_count: graph.partial_artifact_splice_count
            + runtime.partial_artifact_splice_count,
        reuse_rejected_unsupported_strategy_count: graph.reuse_rejected_unsupported_strategy_count
            + runtime.reuse_rejected_unsupported_strategy_count,
        reuse_rejected_contract_strategy_count: graph.reuse_rejected_contract_strategy_count
            + runtime.reuse_rejected_contract_strategy_count,
        reuse_rejected_boundary_mismatch_count: graph.reuse_rejected_boundary_mismatch_count
            + runtime.reuse_rejected_boundary_mismatch_count,
        reuse_rejected_missing_prior_context_count: graph
            .reuse_rejected_missing_prior_context_count
            + runtime.reuse_rejected_missing_prior_context_count,
        reuse_rejected_persistent_correspondence_missing_count: graph
            .reuse_rejected_persistent_correspondence_missing_count
            + runtime.reuse_rejected_persistent_correspondence_missing_count,
        reuse_rejected_persistent_correspondence_invalid_count: graph
            .reuse_rejected_persistent_correspondence_invalid_count
            + runtime.reuse_rejected_persistent_correspondence_invalid_count,
        reuse_rejected_composition_region_count: graph.reuse_rejected_composition_region_count
            + runtime.reuse_rejected_composition_region_count,
        reuse_rejected_mixed_basis_insufficiency_count: graph
            .reuse_rejected_mixed_basis_insufficiency_count
            + runtime.reuse_rejected_mixed_basis_insufficiency_count,
        reuse_dependency_comparison_breadth: graph.reuse_dependency_comparison_breadth
            + runtime.reuse_dependency_comparison_breadth,
        reuse_cold_certification_materialization_count: graph
            .reuse_cold_certification_materialization_count
            + runtime.reuse_cold_certification_materialization_count,
        skipped_by_comparator: graph.skipped_by_comparator + runtime.skipped_by_comparator,
        suppressed_downstream_propagations: graph.suppressed_downstream_propagations
            + runtime.suppressed_downstream_propagations,
        output_identity_unchanged_count: graph.output_identity_unchanged_count
            + runtime.output_identity_unchanged_count,
        memoization_hits: graph.memoization_hits + runtime.memoization_hits,
        memoization_misses: graph.memoization_misses + runtime.memoization_misses,
        condition_skip_count: graph.condition_skip_count + runtime.condition_skip_count,
        ondemand_deferred_count: graph.ondemand_deferred_count + runtime.ondemand_deferred_count,
        debounce_deferred_count: graph.debounce_deferred_count + runtime.debounce_deferred_count,
        evaluation_stack_peak: graph
            .evaluation_stack_peak
            .max(runtime.evaluation_stack_peak),
    }
}

impl<'a, D, I, E, Ctx, T> RuntimeMaterializer<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn graph(&self) -> GraphMaterializer<'a> {
        self.runtime.graph.observe().materialize()
    }

    pub fn retained_explanation_artifact(&self, node: NodeId) -> Option<NodeExplanation> {
        self.graph().retained_explanation_artifact(node)
    }

    pub fn materialize_historical_artifact_record(
        &self,
        node: NodeId,
    ) -> Result<Option<crate::data::trace::HistoricalArtifactRecord>, SignalError> {
        self.graph().materialize_historical_artifact_record(node)
    }

    pub fn materialize_trace_summary(
        &self,
        node: NodeId,
    ) -> Result<Option<crate::data::trace::TraceSummary>, SignalError> {
        self.graph().materialize_trace_summary(node)
    }

    pub fn reconstruct_explanation_artifact(
        &self,
        node: NodeId,
    ) -> Result<NodeExplanation, SignalError> {
        self.graph().reconstruct_explanation_artifact(node)
    }

    pub fn materialize_explanation_artifact(
        &self,
        node: NodeId,
    ) -> Result<
        (
            Option<NodeExplanation>,
            crate::diagnostics::policy::DiagnosticsAvailability,
        ),
        SignalError,
    > {
        self.graph().materialize_explanation_artifact(node)
    }

    pub fn retained_provenance_artifact(&self, node: NodeId) -> Option<ProvenanceFact> {
        self.graph().retained_provenance_artifact(node)
    }

    pub fn reconstruct_provenance_artifact(
        &self,
        node: NodeId,
    ) -> Result<ProvenanceFact, SignalError> {
        self.graph().reconstruct_provenance_artifact(node)
    }

    pub fn materialize_provenance_artifact(
        &self,
        node: NodeId,
    ) -> Result<
        (
            Option<ProvenanceFact>,
            crate::diagnostics::policy::DiagnosticsAvailability,
        ),
        SignalError,
    > {
        self.graph().materialize_provenance_artifact(node)
    }
}

fn merge_invalidation_telemetry(
    graph: InvalidationTelemetry,
    runtime: InvalidationTelemetry,
) -> InvalidationTelemetry {
    InvalidationTelemetry {
        batch_width: graph.batch_width + runtime.batch_width,
        dirty_delta_breadth: graph.dirty_delta_breadth + runtime.dirty_delta_breadth,
        partition_aware_recomputations: graph.partition_aware_recomputations
            + runtime.partition_aware_recomputations,
        keyed_evaluation_count: graph.keyed_evaluation_count + runtime.keyed_evaluation_count,
        partition_scoped_invalidation_checks: graph.partition_scoped_invalidation_checks
            + runtime.partition_scoped_invalidation_checks,
        partition_match_dirty_count: graph.partition_match_dirty_count
            + runtime.partition_match_dirty_count,
        detail_match_dirty_count: graph.detail_match_dirty_count + runtime.detail_match_dirty_count,
        partition_scope_revert_clean_count: graph.partition_scope_revert_clean_count
            + runtime.partition_scope_revert_clean_count,
        partition_interner_growth_delta: graph.partition_interner_growth_delta
            + runtime.partition_interner_growth_delta,
        invalidation_nodes_visited: graph.invalidation_nodes_visited
            + runtime.invalidation_nodes_visited,
        narrowed_frontier_width: graph.narrowed_frontier_width + runtime.narrowed_frontier_width,
        transitive_frontier_width: graph.transitive_frontier_width
            + runtime.transitive_frontier_width,
        frontier_seed_count: graph.frontier_seed_count + runtime.frontier_seed_count,
        frontier_group_count: graph.frontier_group_count + runtime.frontier_group_count,
        frontier_direct_wave_count: graph.frontier_direct_wave_count
            + runtime.frontier_direct_wave_count,
        frontier_transitive_wave_count: graph.frontier_transitive_wave_count
            + runtime.frontier_transitive_wave_count,
        frontier_direct_dirty_count: graph.frontier_direct_dirty_count
            + runtime.frontier_direct_dirty_count,
        frontier_maybe_stale_count: graph.frontier_maybe_stale_count
            + runtime.frontier_maybe_stale_count,
        frontier_partition_match_count: graph.frontier_partition_match_count
            + runtime.frontier_partition_match_count,
        frontier_detail_match_count: graph.frontier_detail_match_count
            + runtime.frontier_detail_match_count,
        frontier_cycle_check_candidate_count: graph.frontier_cycle_check_candidate_count
            + runtime.frontier_cycle_check_candidate_count,
        frontier_cycle_check_visited_count: graph.frontier_cycle_check_visited_count
            + runtime.frontier_cycle_check_visited_count,
        frontier_trace_retained_count: graph.frontier_trace_retained_count
            + runtime.frontier_trace_retained_count,
        subscriber_repair_breadth: graph.subscriber_repair_breadth
            + runtime.subscriber_repair_breadth,
    }
}
