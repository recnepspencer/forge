//! Operation finalization contract (Phase 2).
//!
//! Collector split for top-level operation boundary handling:
//! - drain `ModelingContext` decision/sub-op metadata exactly once
//! - merge into `OperationResult`
//! - attach typed trace adjuncts (deterministically ordered)
//! - set explicit topology hash boundary fields

use forge_core::envelope::{LineageDelta, OperationMetrics, OperationResult};
use forge_core::tracing::{compute_trace_fingerprint, TraceAdjunctSet, TraceFingerprint};

use crate::context::facade::ModelingContext;
use crate::observability::facade::SpanOutput;

/// Finalization path status (typed; avoids stringly status flags in callers).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FinalizationStatus {
    Success,
    Error,
}

impl FinalizationStatus {
    fn as_trace_status(self) -> &'static str {
        match self {
            FinalizationStatus::Success => "ok",
            FinalizationStatus::Error => "error",
        }
    }
}

/// Topology hash boundary values for finalization.
///
/// Phase 2 explicitly treats these as topology-state hashes (not a composite
/// kernel-state hash) until a `KernelState` fingerprint contract exists.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TopologyHashBoundary {
    pub before: Option<u128>,
    pub after: Option<u128>,
}

/// Aggregate counts/drained summaries captured during finalization.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DrainedMetadataCounts {
    pub warnings: usize,
    pub validation_results: usize,
    pub extra_summaries: usize,
}

/// Deterministic finalization summary (collect phase output).
#[derive(Debug, Clone)]
pub struct FinalizationSummary {
    pub status: FinalizationStatus,
    pub trace_fingerprint: TraceFingerprint,
    pub adjunct_count: usize,
    pub topology_state_hash_before: Option<u128>,
    pub topology_state_hash_after: Option<u128>,
    pub drained_metadata_counts: DrainedMetadataCounts,
    pub drained_metrics: OperationMetrics,
    pub drained_lineage_delta: LineageDelta,
    pub drained_accumulated_error_budget: f64,
    pub trace_emitted: bool,
}

/// Deterministic collected finalization artifact bundle (pre-emit).
#[derive(Debug, Clone)]
pub struct CollectedFinalization {
    pub summary: FinalizationSummary,
    decision_log: forge_core::DecisionLog,
    adjuncts: TraceAdjunctSet,
}

/// Finalizer reuse error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FinalizationError {
    AlreadyFinalized,
}

/// Single-use operation finalizer boundary object.
pub struct OperationFinalizer<'a> {
    ctx: &'a mut ModelingContext,
    used: bool,
}

impl<'a> OperationFinalizer<'a> {
    pub fn new(ctx: &'a mut ModelingContext) -> Self {
        Self { ctx, used: false }
    }

    pub fn collect_success<T>(
        &mut self,
        envelope: &mut OperationResult<T>,
        adjuncts: TraceAdjunctSet,
        hashes: TopologyHashBoundary,
        span_output: Option<SpanOutput>,
    ) -> Result<CollectedFinalization, FinalizationError> {
        self.collect(
            FinalizationStatus::Success,
            envelope,
            adjuncts,
            hashes,
            span_output,
        )
    }

    pub fn collect_error<T>(
        &mut self,
        envelope: &mut OperationResult<T>,
        adjuncts: TraceAdjunctSet,
        hashes: TopologyHashBoundary,
        span_output: Option<SpanOutput>,
    ) -> Result<CollectedFinalization, FinalizationError> {
        self.collect(
            FinalizationStatus::Error,
            envelope,
            adjuncts,
            hashes,
            span_output,
        )
    }

    fn collect<T>(
        &mut self,
        status: FinalizationStatus,
        envelope: &mut OperationResult<T>,
        adjuncts: TraceAdjunctSet,
        hashes: TopologyHashBoundary,
        span_output: Option<SpanOutput>,
    ) -> Result<CollectedFinalization, FinalizationError> {
        if self.used {
            return Err(FinalizationError::AlreadyFinalized);
        }
        self.used = true;

        // Drain context trace first and merge into envelope exactly once.
        let ctx_log = self.ctx.take_decision_log();
        envelope.get_decision_log_mut().merge(ctx_log);

        // If a span output was provided, merge its decisions, warnings, and metrics directly.
        if let Some(mut span) = span_output {
            envelope
                .get_decision_log_mut()
                .merge(std::mem::take(&mut span.decision_log));

            for w in span.warnings {
                envelope.add_warning(w);
            }

            let mut current_metrics = envelope.take_metrics();
            current_metrics.duration += span.metrics.duration;
            current_metrics.entities_created += span.metrics.entities_created;
            current_metrics.entities_deleted += span.metrics.entities_deleted;
            current_metrics.entities_modified += span.metrics.entities_modified;
            current_metrics.exact_predicate_calls += span.metrics.exact_predicate_calls;
            current_metrics.policy_decisions_made += span.metrics.policy_decisions_made;
            envelope.set_metrics(current_metrics);

            let mut current_lineage = envelope.take_lineage_delta();
            current_lineage.faces_created += span.lineage_delta.faces_created;
            current_lineage.faces_deleted += span.lineage_delta.faces_deleted;
            current_lineage.vertices_created += span.lineage_delta.vertices_created;
            current_lineage.vertices_deleted += span.lineage_delta.vertices_deleted;
            current_lineage.edges_created += span.lineage_delta.edges_created;
            current_lineage.edges_deleted += span.lineage_delta.edges_deleted;
            current_lineage.half_edges_created += span.lineage_delta.half_edges_created;
            current_lineage.half_edges_deleted += span.lineage_delta.half_edges_deleted;
            current_lineage.loops_created += span.lineage_delta.loops_created;
            current_lineage.loops_deleted += span.lineage_delta.loops_deleted;
            current_lineage.shells_created += span.lineage_delta.shells_created;
            current_lineage.shells_deleted += span.lineage_delta.shells_deleted;
            current_lineage.solids_created += span.lineage_delta.solids_created;
            current_lineage.solids_deleted += span.lineage_delta.solids_deleted;
            envelope.set_lineage_delta(current_lineage);
        }

        // Drain typed adjunct sink from the context and merge with caller-supplied adjuncts.
        let mut merged_adjunct_records = self.ctx.take_trace_adjuncts().into_records();
        merged_adjunct_records.extend(adjuncts.into_records());
        let adjuncts = TraceAdjunctSet::from_records(merged_adjunct_records);

        // Drain sub-op metadata sink and merge into envelope exactly once.
        let drained = self.ctx.take_sub_metadata();
        let drained_counts = DrainedMetadataCounts {
            warnings: drained.warnings.len(),
            validation_results: 0,
            extra_summaries: 0,
        };

        for w in drained.warnings.iter().cloned() {
            envelope.add_warning(w);
        }

        let mut metrics = envelope.take_metrics();
        metrics.duration += drained.metrics.duration;
        metrics.entities_created += drained.metrics.entities_created;
        metrics.entities_deleted += drained.metrics.entities_deleted;
        metrics.entities_modified += drained.metrics.entities_modified;
        metrics.exact_predicate_calls += drained.metrics.exact_predicate_calls;
        metrics.policy_decisions_made += drained.metrics.policy_decisions_made;
        envelope.set_metrics(metrics);

        let mut lineage = envelope.take_lineage_delta();
        lineage.faces_created += drained.lineage_delta.faces_created;
        lineage.faces_deleted += drained.lineage_delta.faces_deleted;
        lineage.half_edges_created += drained.lineage_delta.half_edges_created;
        lineage.half_edges_deleted += drained.lineage_delta.half_edges_deleted;
        lineage.vertices_created += drained.lineage_delta.vertices_created;
        lineage.vertices_deleted += drained.lineage_delta.vertices_deleted;
        lineage.loops_created += drained.lineage_delta.loops_created;
        lineage.loops_deleted += drained.lineage_delta.loops_deleted;
        lineage.edges_created += drained.lineage_delta.edges_created;
        lineage.edges_deleted += drained.lineage_delta.edges_deleted;
        lineage.shells_created += drained.lineage_delta.shells_created;
        lineage.shells_deleted += drained.lineage_delta.shells_deleted;
        lineage.solids_created += drained.lineage_delta.solids_created;
        lineage.solids_deleted += drained.lineage_delta.solids_deleted;
        envelope.set_lineage_delta(lineage);

        envelope.consume_budget(drained.accumulated_error_budget);

        if let Some(before) = hashes.before {
            envelope.set_state_hash_before(before);
        }
        if let Some(after) = hashes.after {
            envelope.set_state_hash_after(after);
        }

        let trace_fingerprint = compute_trace_fingerprint(envelope.get_decision_log());
        let summary = FinalizationSummary {
            status,
            trace_fingerprint,
            adjunct_count: adjuncts.records().len(),
            topology_state_hash_before: hashes.before,
            topology_state_hash_after: hashes.after,
            drained_metadata_counts: drained_counts,
            drained_metrics: drained.metrics.clone(),
            drained_lineage_delta: drained.lineage_delta.clone(),
            drained_accumulated_error_budget: drained.accumulated_error_budget,
            trace_emitted: false,
        };

        Ok(CollectedFinalization {
            summary,
            decision_log: envelope.get_decision_log().clone(),
            adjuncts,
        })
    }
}
