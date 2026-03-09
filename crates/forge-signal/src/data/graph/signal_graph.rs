//! Arena-based signal graph with dependency storage.

use serde::{Deserialize, Serialize};
use std::time::Instant;

use crate::data::aspect::Aspect;
use crate::data::comparator::{
    DefaultComparatorPolicyResolver, DefaultComparatorResolver, VersionComparatorPolicy,
};
use crate::data::dependency::DependencyEdge;
use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::data::node::{NodeEntry, NodeEvaluationConfig, NodeState};
use crate::data::output::{PartitionInterner, PartitionSubscription};
use crate::data::telemetry::RuntimeTelemetry;
use crate::data::trace::CausalityMetadata;
use crate::diagnostics::access::GraphDiagnostics;
use crate::diagnostics::history::ExecutionInspector;
use crate::diagnostics::profile::DiagnosticsProfile;
use crate::diagnostics::recorder::DiagnosticsRecorder;
use crate::diagnostics::state::DiagnosticsState;
use crate::diagnostics::summary::{ExecutionHistorySummary, GraphSummary};
use crate::diagnostics::{FailureSummary, FlowSummary, RollbackDiagnostic};
use crate::logic::evaluation::EvaluationRequestMode;
use crate::logic::explain::{dependency_chain_to, explain, NodeExplanation};
use crate::logic::planner::{
    build_evaluation_plan, execute_prepared_plan, execute_prepared_plan_with_policy,
    EvaluationPlan, ExecutionReport, StageExecutor,
};
use crate::logic::prepared::{ExecutionReadView, PreparedEvaluation};
use crate::presentation::dot::to_dot;
use crate::presentation::metrics::GraphMetrics;

use super::node_builder::NodeBuilder;
use super::scratch::{ScratchLeaseKind, TraversalScratch};
use super::slot::Slot;

/// The reactive signal graph.
///
/// An arena of `NodeEntry` values with dependency edges.
/// Nodes are allocated with generational handles (`NodeId`) for
/// safe, stale-proof access.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalGraph {
    /// Arena slots (generational).
    nodes: Vec<Slot>,
    /// Free list of vacant slot indices for reuse.
    free_list: Vec<u32>,
    /// Count of tombstoned nodes awaiting GC.
    tombstone_count: u32,
    /// Threshold for triggering a GC epoch.
    gc_threshold: u32,
    /// Reusable traversal scratch to avoid hot-path allocations.
    #[serde(skip, default)]
    scratch: TraversalScratch,
    /// Active scratch lease, if any.
    #[serde(skip, default)]
    scratch_lease: Option<ScratchLeaseKind>,
    /// Lightweight runtime counters for evaluation/invalidation behavior.
    #[serde(skip, default)]
    telemetry: RuntimeTelemetry,
    /// Interned partition tokens/details for efficient scoped comparisons.
    #[serde(default)]
    partition_interner: PartitionInterner,
    /// Bounded diagnostics retention and pending causal flow state.
    #[serde(skip, default)]
    diagnostics: DiagnosticsState,
}

impl Default for SignalGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl SignalGraph {
    /// Create an empty signal graph.
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            free_list: Vec::new(),
            tombstone_count: 0,
            gc_threshold: 1024,
            scratch: TraversalScratch::default(),
            scratch_lease: None,
            telemetry: RuntimeTelemetry::default(),
            partition_interner: PartitionInterner::default(),
            diagnostics: DiagnosticsState::default(),
        }
    }

    /// Create a signal graph with a custom GC threshold.
    pub fn with_gc_threshold(gc_threshold: u32) -> Self {
        Self {
            nodes: Vec::new(),
            free_list: Vec::new(),
            tombstone_count: 0,
            gc_threshold,
            scratch: TraversalScratch::default(),
            scratch_lease: None,
            telemetry: RuntimeTelemetry::default(),
            partition_interner: PartitionInterner::default(),
            diagnostics: DiagnosticsState::default(),
        }
    }

    pub(crate) fn acquire_scratch(
        &mut self,
        kind: ScratchLeaseKind,
    ) -> Result<TraversalScratch, SignalError> {
        if let Some(active) = self.scratch_lease {
            self.telemetry.scratch_reentry_error_count += 1;
            return Err(SignalError::invalid_input(format!(
                "signal scratch is already leased for {active:?}; re-entrant {kind:?} traversal is forbidden"
            )));
        }
        self.scratch_lease = Some(kind);
        Ok(std::mem::take(&mut self.scratch))
    }

    pub(crate) fn restore_scratch(
        &mut self,
        kind: ScratchLeaseKind,
        scratch: TraversalScratch,
    ) -> Result<(), SignalError> {
        match self.scratch_lease {
            Some(active) if active == kind => {
                self.scratch = scratch;
                self.scratch_lease = None;
                Ok(())
            }
            Some(active) => Err(SignalError::internal(format!(
                "signal scratch lease mismatch: expected {active:?}, restored {kind:?}"
            ))),
            None => Err(SignalError::internal(
                "signal scratch restore called without active lease",
            )),
        }
    }

    #[doc(hidden)]
    /// Low-level signal node allocation.
    pub fn create_node(&mut self) -> NodeId {
        let entry = NodeEntry::new();
        self.allocate_node(entry)
    }

    /// Start a fluent node builder.
    pub fn node(&mut self) -> NodeBuilder<'_> {
        NodeBuilder::new(self)
    }

    #[doc(hidden)]
    /// Low-level node allocation with explicit evaluation config.
    pub fn create_node_with_config(&mut self, config: NodeEvaluationConfig) -> NodeId {
        let mut entry = NodeEntry::new();
        entry.set_eval_config(config);
        self.allocate_node(entry)
    }

    fn allocate_node(&mut self, entry: NodeEntry) -> NodeId {
        if let Some(index) = self.free_list.pop() {
            let slot = &mut self.nodes[index as usize];
            let generation = slot.occupy(entry);
            return NodeId::new(index, generation);
        }

        let index = self.nodes.len() as u32;
        let mut slot = Slot::vacant();
        let generation = slot.occupy(entry);
        self.nodes.push(slot);
        NodeId::new(index, generation)
    }

    /// Wire a dependency: `downstream` reads `aspect` from `upstream`.
    pub fn add_dependency(
        &mut self,
        downstream: NodeId,
        upstream: NodeId,
        aspect: Aspect,
    ) -> Result<(), SignalError> {
        self.validate_handle(downstream)?;
        self.validate_handle(upstream)?;

        let edge = DependencyEdge::new(upstream, aspect);
        let inserted = self.get_entry_mut(downstream)?.add_dependency(edge);
        if inserted {
            self.get_entry_mut(upstream)?.add_subscriber(downstream);
        }
        Ok(())
    }

    /// Wire one dependency scoped to a whole partition token.
    pub fn add_partition_dependency(
        &mut self,
        downstream: NodeId,
        upstream: NodeId,
        aspect: Aspect,
        partition: impl Into<crate::data::output::PartitionToken>,
    ) -> Result<(), SignalError> {
        let scope = PartitionSubscription::whole_partition(partition);
        self.add_dependency_with_scope(downstream, upstream, aspect, scope)
    }

    /// Wire one detail-sensitive dependency scoped to one partition/detail pair.
    pub fn add_partition_detail_dependency(
        &mut self,
        downstream: NodeId,
        upstream: NodeId,
        aspect: Aspect,
        partition: impl Into<crate::data::output::PartitionToken>,
        detail: impl Into<String>,
    ) -> Result<(), SignalError> {
        let scope = PartitionSubscription::partition_and_detail(partition, detail);
        self.add_dependency_with_scope(downstream, upstream, aspect, scope)
    }

    fn add_dependency_with_scope(
        &mut self,
        downstream: NodeId,
        upstream: NodeId,
        aspect: Aspect,
        scope: PartitionSubscription,
    ) -> Result<(), SignalError> {
        self.validate_handle(downstream)?;
        self.validate_handle(upstream)?;
        let interned_scope = self.partition_interner.intern_subscription(&scope);
        let edge = DependencyEdge::with_scope(upstream, aspect, scope, interned_scope);
        let inserted = self.get_entry_mut(downstream)?.add_dependency(edge);
        if inserted {
            self.get_entry_mut(upstream)?.add_subscriber(downstream);
        }
        Ok(())
    }

    pub(crate) fn connect_dependency_capture(
        &mut self,
        downstream: NodeId,
        upstream: NodeId,
        aspect: Aspect,
        scope: Option<PartitionSubscription>,
    ) -> Result<bool, SignalError> {
        self.validate_handle(downstream)?;
        self.validate_handle(upstream)?;
        let edge = match scope {
            Some(scope) => {
                let interned_scope = self.partition_interner.intern_subscription(&scope);
                DependencyEdge::with_scope(upstream, aspect, scope, interned_scope)
            }
            None => DependencyEdge::new(upstream, aspect),
        };
        let inserted = self.get_entry_mut(downstream)?.add_dependency(edge);
        if inserted {
            self.get_entry_mut(upstream)?.add_subscriber(downstream);
        }
        Ok(inserted)
    }

    pub(crate) fn disconnect_dependency_edge(
        &mut self,
        downstream: NodeId,
        edge: DependencyEdge,
    ) -> Result<bool, SignalError> {
        self.validate_handle(downstream)?;
        self.validate_handle(edge.source())?;
        let removed = self
            .get_entry_mut(downstream)?
            .remove_dependency(edge.clone());
        if removed && !self.get_entry(downstream)?.has_dependency_on(edge.source()) {
            self.get_entry_mut(edge.source())?
                .remove_subscriber(downstream);
        }
        Ok(removed)
    }

    /// Remove one dependency edge from `downstream` to `upstream` for the specified aspect.
    pub fn remove_dependency(
        &mut self,
        downstream: NodeId,
        upstream: NodeId,
        aspect: Aspect,
    ) -> Result<(), SignalError> {
        self.validate_handle(downstream)?;
        self.validate_handle(upstream)?;

        let edge = DependencyEdge::new(upstream, aspect);
        let removed = self.get_entry_mut(downstream)?.remove_dependency(edge);
        if removed && !self.get_entry(downstream)?.has_dependency_on(upstream) {
            self.get_entry_mut(upstream)?.remove_subscriber(downstream);
        }
        Ok(())
    }

    /// Read the state of a node.
    pub fn get_state(&self, id: NodeId) -> Result<NodeState, SignalError> {
        Ok(*self.get_entry(id)?.get_state())
    }

    /// Read-only access to a node entry.
    pub fn get_entry(&self, id: NodeId) -> Result<&NodeEntry, SignalError> {
        self.validate_handle(id)?;
        let slot = &self.nodes[id.index() as usize];
        slot.data.as_ref().ok_or_else(|| stale_error(id))
    }

    /// Mutable access to a node entry.
    pub fn get_entry_mut(&mut self, id: NodeId) -> Result<&mut NodeEntry, SignalError> {
        self.validate_handle(id)?;
        let slot = &mut self.nodes[id.index() as usize];
        slot.data.as_mut().ok_or_else(|| stale_error(id))
    }

    /// Check whether a node handle is valid (alive and generation matches).
    pub fn is_alive(&self, id: NodeId) -> bool {
        let idx = id.index() as usize;
        if idx >= self.nodes.len() {
            return false;
        }
        let slot = &self.nodes[idx];
        slot.generation == id.generation() && slot.is_occupied()
    }

    /// The total number of active (non-tombstoned, occupied) nodes.
    pub fn active_node_count(&self) -> usize {
        self.nodes.iter().filter(|s| s.is_occupied()).count()
    }

    /// The number of allocated slots (including vacant ones).
    pub fn arena_capacity(&self) -> usize {
        self.nodes.len()
    }

    /// Resolve a live `NodeId` at slot index if occupied.
    pub(crate) fn live_node_id_at(&self, index: usize) -> Option<NodeId> {
        let slot = self.nodes.get(index)?;
        if !slot.is_occupied() {
            return None;
        }
        Some(NodeId::new(index as u32, slot.generation))
    }

    /// Replace full node entry payload for an existing live node.
    pub(crate) fn replace_entry(
        &mut self,
        id: NodeId,
        entry: NodeEntry,
    ) -> Result<(), SignalError> {
        let target = self.get_entry_mut(id)?;
        *target = entry;
        Ok(())
    }

    /// The count of tombstoned nodes awaiting GC.
    pub fn tombstone_count(&self) -> u32 {
        self.tombstone_count
    }

    /// The GC threshold.
    pub fn gc_threshold(&self) -> u32 {
        self.gc_threshold
    }

    /// Remove a node from the arena, severing all dependency edges.
    pub fn unregister_node(&mut self, id: NodeId) -> Result<(), SignalError> {
        self.validate_handle(id)?;
        let mut scratch = self.acquire_scratch(ScratchLeaseKind::Churn)?;
        scratch.node_buffer_a.clear();
        scratch.node_buffer_b.clear();

        {
            let entry = self.get_entry(id)?;
            scratch
                .node_buffer_a
                .extend(entry.get_dependencies().iter().map(|edge| edge.source()));
            scratch
                .node_buffer_b
                .extend(entry.get_subscribers().iter().copied());
        }

        for &source in &scratch.node_buffer_a {
            if self.is_alive(source) {
                self.get_entry_mut(source)?.remove_subscriber(id);
            }
        }

        for &subscriber in &scratch.node_buffer_b {
            if self.is_alive(subscriber) {
                self.get_entry_mut(subscriber)?.remove_dependencies_on(id);
                self.get_entry_mut(subscriber)?.set_state(NodeState::Dirty);
            }
        }

        debug_assert!(
            !self.free_list.contains(&id.index()),
            "free list already contained slot {} before unregister",
            id.index()
        );
        self.nodes[id.index() as usize].vacate();
        self.tombstone_count += 1;
        self.free_list.push(id.index());
        self.restore_scratch(ScratchLeaseKind::Churn, scratch)?;
        Ok(())
    }

    /// Run a garbage collection epoch.
    pub fn run_gc_epoch(&mut self) {
        let gc_start = Instant::now();
        let mut scratch = self
            .acquire_scratch(ScratchLeaseKind::Gc)
            .expect("GC scratch lease must succeed");
        let len = self.nodes.len();
        if scratch.gc_liveness_generations.len() < len {
            scratch.gc_liveness_generations.resize(len, 0);
        }
        scratch.gc_liveness_alive.clear_all();
        scratch.gc_liveness_alive.ensure_len(len);

        for (index, slot) in self.nodes.iter().enumerate() {
            scratch.gc_liveness_generations[index] = slot.generation;
            if slot.is_occupied() {
                scratch.gc_liveness_alive.mark(index);
            }
        }

        let generations = &scratch.gc_liveness_generations;
        let alive_bits = &scratch.gc_liveness_alive;
        let alive_checker = |node_id: NodeId| -> bool {
            let idx = node_id.index() as usize;
            idx < generations.len()
                && generations[idx] == node_id.generation()
                && alive_bits.contains(idx)
        };

        for slot in &mut self.nodes {
            if let Some(ref mut entry) = slot.data {
                entry.purge_stale_subscribers(alive_checker);
            }
        }

        self.tombstone_count = 0;
        self.restore_scratch(ScratchLeaseKind::Gc, scratch)
            .expect("GC scratch restore must succeed");
        self.telemetry.gc_epoch_count += 1;
        self.telemetry.gc_epoch_nanos += gc_start.elapsed().as_nanos();
    }

    /// Whether a GC epoch should be triggered.
    pub fn should_gc(&self) -> bool {
        self.tombstone_count >= self.gc_threshold
    }

    /// Validate that a handle refers to a live node.
    fn validate_handle(&self, id: NodeId) -> Result<(), SignalError> {
        let idx = id.index() as usize;
        if idx >= self.nodes.len() {
            return Err(stale_error(id));
        }
        let slot = &self.nodes[idx];
        if slot.generation != id.generation() || !slot.is_occupied() {
            return Err(stale_error(id));
        }
        Ok(())
    }

    /// Immutable telemetry snapshot.
    pub fn telemetry(&self) -> &RuntimeTelemetry {
        &self.telemetry
    }

    /// Mutable telemetry reference.
    pub fn telemetry_mut(&mut self) -> &mut RuntimeTelemetry {
        &mut self.telemetry
    }

    /// Reset runtime telemetry counters.
    pub fn reset_telemetry(&mut self) {
        self.telemetry = RuntimeTelemetry::default();
    }

    /// Structured explanation for one node based on committed graph state.
    pub fn explain(&self, node: NodeId) -> Result<NodeExplanation, SignalError> {
        explain(self, node)
    }

    /// Direct dependencies of one node.
    pub fn dependencies_of(&self, node: NodeId) -> Result<&[DependencyEdge], SignalError> {
        Ok(self.get_entry(node)?.get_dependencies())
    }

    /// Direct subscribers of one node.
    pub fn subscribers_of(&self, node: NodeId) -> Result<&[NodeId], SignalError> {
        Ok(self.get_entry(node)?.get_subscribers())
    }

    /// Whether `node` directly depends on `upstream` for the given aspect.
    pub fn depends_on(
        &self,
        node: NodeId,
        upstream: NodeId,
        aspect: Aspect,
    ) -> Result<bool, SignalError> {
        Ok(self
            .get_entry(node)?
            .get_dependencies()
            .iter()
            .any(|dependency| dependency.source() == upstream && dependency.aspect() == aspect))
    }

    /// Deterministic dependency path from `root` to `target` through subscriber edges.
    pub fn dependency_chain_to(
        &self,
        root: NodeId,
        target: NodeId,
    ) -> Result<Option<Vec<NodeId>>, SignalError> {
        dependency_chain_to(self, root, target)
    }

    /// Read the host-provided causality payload for one node.
    pub fn causality_of(&self, node: NodeId) -> Result<Option<&CausalityMetadata>, SignalError> {
        Ok(self.get_entry(node)?.get_causality())
    }

    /// Set or clear the host-provided causality payload for one node.
    pub fn set_causality(
        &mut self,
        node: NodeId,
        causality: Option<CausalityMetadata>,
    ) -> Result<(), SignalError> {
        self.get_entry_mut(node)?.set_causality(causality);
        Ok(())
    }

    /// Structured graph metrics snapshot.
    pub fn metrics(&self) -> GraphMetrics {
        GraphMetrics::from_runtime_telemetry(
            self.telemetry(),
            self.partition_interner.partition_count(),
        )
    }

    pub fn diagnostics_profile(&self) -> DiagnosticsProfile {
        self.diagnostics.profile()
    }

    pub fn set_diagnostics_profile(&mut self, profile: DiagnosticsProfile) {
        self.diagnostics.set_profile(profile);
    }

    /// Production diagnostics summary for the current graph state.
    pub fn diagnostics_summary(&self, profile: DiagnosticsProfile) -> GraphSummary {
        GraphSummary::from_graph(self, profile)
    }

    /// Central diagnostics facade for this graph.
    pub fn diagnostics(&self) -> GraphDiagnostics<'_> {
        GraphDiagnostics::new(self)
    }

    /// Production diagnostics summary for execution/trace history visible on the graph.
    pub fn execution_history_summary(
        &self,
        profile: DiagnosticsProfile,
    ) -> ExecutionHistorySummary {
        ExecutionHistorySummary::from_graph(self, profile)
    }

    /// Structured execution-history inspector for production diagnostics.
    pub fn inspect_execution(&self) -> ExecutionInspector<'_> {
        ExecutionInspector { graph: self }
    }

    pub fn latest_flow_diagnostics(&self) -> Option<&FlowSummary> {
        self.diagnostics.latest_flow()
    }

    pub fn latest_failure_diagnostics(&self) -> Option<&FailureSummary> {
        self.diagnostics.latest_failure()
    }

    pub fn latest_rollback_diagnostics(&self) -> Option<&RollbackDiagnostic> {
        self.diagnostics.latest_rollback()
    }

    pub fn recent_execution_history_diagnostics(
        &self,
    ) -> &std::collections::VecDeque<ExecutionHistorySummary> {
        self.diagnostics.recent_history()
    }

    /// Export this graph to Graphviz DOT.
    pub fn to_dot(&self) -> String {
        to_dot(self)
    }

    /// Build a deterministic staged execution plan for one or more targets.
    pub fn build_evaluation_plan(
        &self,
        targets: &[NodeId],
        request_mode: EvaluationRequestMode,
    ) -> Result<EvaluationPlan, SignalError> {
        build_evaluation_plan(self, targets, request_mode)
    }

    /// Execute one pre-built plan using the prepared-evaluation contract.
    pub fn execute_prepared_plan<F>(
        &mut self,
        plan: &EvaluationPlan,
        precompute: &F,
    ) -> Result<ExecutionReport, SignalError>
    where
        F: Fn(NodeId, &ExecutionReadView<'_>) -> Result<PreparedEvaluation, SignalError> + Sync,
    {
        execute_prepared_plan(self, plan, precompute)
    }

    /// Execute one pre-built prepared-evaluation plan with an explicit executor.
    pub fn execute_prepared_plan_with_executor<F>(
        &mut self,
        plan: &EvaluationPlan,
        precompute: &F,
        executor: StageExecutor,
    ) -> Result<ExecutionReport, SignalError>
    where
        F: Fn(NodeId, &ExecutionReadView<'_>) -> Result<PreparedEvaluation, SignalError> + Sync,
    {
        let mut comparator = DefaultComparatorResolver;
        let mut resolver = DefaultComparatorPolicyResolver {
            fallback: VersionComparatorPolicy::Exact,
            custom: &mut comparator,
        };
        execute_prepared_plan_with_policy(self, plan, precompute, &mut resolver, executor)
    }

    pub(crate) fn partition_interner_mut(&mut self) -> &mut PartitionInterner {
        &mut self.partition_interner
    }

    pub(crate) fn diagnostics_state(&self) -> &DiagnosticsState {
        &self.diagnostics
    }

    pub(crate) fn diagnostics_state_mut(&mut self) -> &mut DiagnosticsState {
        &mut self.diagnostics
    }

    pub(crate) fn note_change_input(
        &mut self,
        node: NodeId,
        aspect: Aspect,
        changed_regions: &[crate::data::output::ChangedRegion],
    ) {
        DiagnosticsRecorder::new(self).note_change_input(node, aspect, changed_regions);
    }

    pub(crate) fn record_invalidation_diagnostics(
        &mut self,
        invalidated_direct_subscribers: u32,
        maybe_stale_direct_subscribers: u32,
        partition_scoped_checks: u32,
    ) {
        DiagnosticsRecorder::new(self).record_invalidation_result(
            invalidated_direct_subscribers,
            maybe_stale_direct_subscribers,
            partition_scoped_checks,
        );
    }

    pub(crate) fn clear_pending_diagnostics_input(&mut self) {
        self.diagnostics.clear_pending_input();
    }
}

/// Produce a structured error for a stale or invalid node handle.
fn stale_error(id: NodeId) -> SignalError {
    SignalError::InvalidInput {
        message: format!("Stale or invalid signal node handle: {}", id),
        context: None,
    }
}
