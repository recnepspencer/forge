//! Operation finalizer — the drain-and-commit boundary object.
//!
//! DOMAIN: Single-use finalizer that drains `ModelingContext` metadata
//! exactly once and merges it into an `OperationResult` envelope.
//!
//! INVARIANTS:
//! - `collect()` can only be called once (enforced by `used` flag)
//! - Decision log, adjuncts, and sub-op metadata are drained exactly once
//! - Merge uses `accumulate()` — never manual field-by-field addition

use forge_core::envelope::OperationResult;
use forge_core::tracing::{compute_trace_fingerprint, TraceAdjunctSet};

use crate::context::facade::ModelingContext;

use super::super::data::summary::{
    CollectedFinalization, DrainedMetadataCounts, FinalizationError, FinalizationStatus,
    FinalizationSummary, TopologyHashBoundary,
};

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
        _span_output: Option<()>,
    ) -> Result<CollectedFinalization, FinalizationError> {
        self.collect(FinalizationStatus::Success, envelope, adjuncts, hashes)
    }

    pub fn collect_error<T>(
        &mut self,
        envelope: &mut OperationResult<T>,
        adjuncts: TraceAdjunctSet,
        hashes: TopologyHashBoundary,
        _span_output: Option<()>,
    ) -> Result<CollectedFinalization, FinalizationError> {
        self.collect(FinalizationStatus::Error, envelope, adjuncts, hashes)
    }

    fn collect<T>(
        &mut self,
        status: FinalizationStatus,
        envelope: &mut OperationResult<T>,
        adjuncts: TraceAdjunctSet,
        hashes: TopologyHashBoundary,
    ) -> Result<CollectedFinalization, FinalizationError> {
        if self.used {
            return Err(FinalizationError::AlreadyFinalized);
        }
        self.used = true;

        // Drain context trace first and merge into envelope exactly once.
        let ctx_log = self.ctx.take_decision_log();
        envelope.get_decision_log_mut().merge(ctx_log);

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
        metrics.accumulate(&drained.metrics);
        envelope.set_metrics(metrics);

        let mut lineage = envelope.take_lineage_delta();
        lineage.accumulate(&drained.lineage_delta);
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
