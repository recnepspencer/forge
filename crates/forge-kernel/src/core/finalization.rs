//! Operation finalization contract (Phase 2).
//!
//! Collector/emit split for top-level operation boundary handling:
//! - drain `ModelingContext` decision/sub-op metadata exactly once
//! - merge into `OperationResult`
//! - attach typed trace adjuncts (deterministically ordered)
//! - set explicit topology hash boundary fields
//! - optionally emit trace artifacts

use std::path::PathBuf;

use forge_core::envelope::{LineageDelta, OperationMetrics, OperationResult};
use forge_core::tracing::{
    compute_trace_fingerprint, try_write_trace_file_with_adjuncts, TraceAdjunctSet,
    TraceFingerprint, TracePersistenceError,
};

use super::ModelingContext;

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

/// Options controlling emission side effects after deterministic collection.
#[derive(Debug, Clone, Default)]
pub struct FinalizationEmitOptions {
    pub trace_dir: Option<PathBuf>,
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

/// Typed emit failure for finalization sinks.
#[derive(Debug)]
pub enum FinalizationEmitError {
    TracePersistence(TracePersistenceError),
}

impl From<TracePersistenceError> for FinalizationEmitError {
    fn from(value: TracePersistenceError) -> Self { Self::TracePersistence(value) }
}

impl CollectedFinalization {
    /// Emit trace artifacts (optional) using explicit sink options.
    ///
    /// This does not mutate the collected summary; callers may clone/update their
    /// own bookkeeping if they need to record emit success separately.
    pub fn emit(&self, options: &FinalizationEmitOptions) -> Result<(), FinalizationEmitError> {
        if let Some(dir) = &options.trace_dir {
            let state_hash = self
                .summary
                .topology_state_hash_after
                .or(self.summary.topology_state_hash_before)
                .unwrap_or(0);
            try_write_trace_file_with_adjuncts(
                dir,
                &self.decision_log,
                self.adjuncts.records(),
                state_hash,
                self.summary.status.as_trace_status(),
            )?;
        }
        Ok(())
    }
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
    ) -> Result<CollectedFinalization, FinalizationError> {
        self.collect(FinalizationStatus::Success, envelope, adjuncts, hashes)
    }

    pub fn collect_error<T>(
        &mut self,
        envelope: &mut OperationResult<T>,
        adjuncts: TraceAdjunctSet,
        hashes: TopologyHashBoundary,
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

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::*;
    use forge_core::{DecisionContext, DecisionId, DecisionKind, DecisionTier, TracedDecision};

    #[test]
    fn finalizer_rejects_double_collect_and_prevents_double_counting() {
        let mut ctx = ModelingContext::new();
        ctx.log_decision(
            DecisionKind::Exact,
            DecisionTier::Deterministic,
            [0.0, 0.0, 0.0],
            0.0,
            1.0,
        );
        let mut sub = OperationResult::new(());
        let mut sub_metrics = forge_core::envelope::OperationMetrics::default();
        sub_metrics.entities_modified = 3;
        sub.set_metrics(sub_metrics);
        ctx.absorb_sub_result(&mut sub);

        let mut envelope = OperationResult::new("ok");
        let mut finalizer = OperationFinalizer::new(&mut ctx);
        let first = finalizer
            .collect_success(&mut envelope, TraceAdjunctSet::new(), TopologyHashBoundary::default())
            .expect("first finalization");
        assert_eq!(first.summary.drained_metrics.entities_modified, 3);
        assert_eq!(envelope.get_metrics().entities_modified, 3);
        assert_eq!(envelope.get_decision_log().len(), 1);

        let second = finalizer.collect_success(
            &mut envelope,
            TraceAdjunctSet::new(),
            TopologyHashBoundary::default(),
        );
        assert!(matches!(second, Err(FinalizationError::AlreadyFinalized)));
        assert_eq!(envelope.get_metrics().entities_modified, 3);
        assert_eq!(envelope.get_decision_log().len(), 1);
    }

    #[test]
    fn finalizer_collects_error_path_with_explicit_missing_after_hash() {
        let mut ctx = ModelingContext::new();
        ctx.get_decision_log_mut().record(TracedDecision::new(
            DecisionId(7),
            DecisionKind::Forced { reason: "test_error".into() },
            DecisionTier::Escalated,
            0.0,
            DecisionContext::Degeneracy { description: "fixture".into() },
        ));

        let mut envelope: OperationResult<Result<(), forge_core::KernelError>> =
            OperationResult::new(Err(forge_core::KernelError::InternalError {
                message: "boom".into(),
                context: None,
            }));
        let mut finalizer = OperationFinalizer::new(&mut ctx);
        let collected = finalizer
            .collect_error(
                &mut envelope,
                TraceAdjunctSet::new(),
                TopologyHashBoundary {
                    before: Some(123),
                    after: None,
                },
            )
            .expect("error finalization");

        assert_eq!(collected.summary.status, FinalizationStatus::Error);
        assert_eq!(collected.summary.topology_state_hash_before, Some(123));
        assert_eq!(collected.summary.topology_state_hash_after, None);
        assert_eq!(envelope.get_state_hash_before(), 123);
        assert_eq!(envelope.get_state_hash_after(), 0);
        assert_eq!(envelope.get_decision_log().len(), 1);
    }

    #[test]
    fn finalizer_collect_preserves_deterministic_adjunct_ordering() {
        let mut ctx = ModelingContext::new();
        let mut envelope = OperationResult::new("ok");
        let mut finalizer = OperationFinalizer::new(&mut ctx);
        let adjuncts = TraceAdjunctSet::from_records(vec![
            forge_core::tracing::TraceAdjunctRecord::new(
                DecisionId(2),
                "zeta",
                1,
                serde_json::json!({"x":1}),
            ),
            forge_core::tracing::TraceAdjunctRecord::new(
                DecisionId(1),
                "alpha",
                1,
                serde_json::json!({"x":2}),
            ),
        ]);
        let collected = finalizer
            .collect_success(&mut envelope, adjuncts.clone(), TopologyHashBoundary::default())
            .expect("collect");
        let keys: Vec<_> = collected
            .adjuncts
            .records()
            .iter()
            .map(|r| r.sort_key())
            .collect();
        assert_eq!(keys, vec![(1, "alpha", 1), (2, "zeta", 1)]);
        assert_eq!(collected.summary.adjunct_count, 2);
    }

    #[test]
    fn emit_failure_does_not_corrupt_collected_finalization_or_redrain_context() {
        let mut ctx = ModelingContext::new();
        ctx.log_decision(
            DecisionKind::Exact,
            DecisionTier::Deterministic,
            [0.0, 0.0, 0.0],
            0.0,
            1.0,
        );

        let mut envelope = OperationResult::new("ok");
        let mut finalizer = OperationFinalizer::new(&mut ctx);
        let collected = finalizer
            .collect_success(
                &mut envelope,
                TraceAdjunctSet::new(),
                TopologyHashBoundary { before: Some(11), after: Some(22) },
            )
            .expect("collect");

        let tmp = std::env::temp_dir();
        let uniq = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let file_path = tmp.join(format!("forge-finalizer-emit-failure-{uniq}.tmp"));
        File::create(&file_path).expect("create temp file");

        let err = collected
            .emit(&FinalizationEmitOptions {
                trace_dir: Some(file_path.clone()),
            })
            .expect_err("emit must fail when trace_dir points to a file");

        match err {
            FinalizationEmitError::TracePersistence(_) => {}
        }

        // Collected payload remains intact after emit failure.
        assert_eq!(collected.summary.topology_state_hash_before, Some(11));
        assert_eq!(collected.summary.topology_state_hash_after, Some(22));
        assert_eq!(collected.summary.trace_fingerprint.decision_ids, vec![1]);
        assert_eq!(envelope.get_decision_log().len(), 1);

        // Context was already drained at collect time; emit failure must not reintroduce state.
        assert!(ctx.get_decision_log_mut().is_empty());
        let sub = ctx.take_sub_metadata();
        assert!(sub.warnings.is_empty());
        assert_eq!(sub.metrics.duration, std::time::Duration::default());
        assert_eq!(sub.metrics.entities_created, 0);
        assert_eq!(sub.metrics.entities_deleted, 0);
        assert_eq!(sub.metrics.entities_modified, 0);
        assert_eq!(sub.metrics.exact_predicate_calls, 0);
        assert_eq!(sub.metrics.policy_decisions_made, 0);

        let _ = std::fs::remove_file(file_path);
    }
}
