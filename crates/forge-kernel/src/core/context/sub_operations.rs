//! Sub-operation absorption and metadata draining.
//!
//! DOMAIN: Pulling metadata from child OperationResult envelopes into the context.
//! INVARIANTS: Absorption is a true drain — child metadata is removed to prevent double-counting.

use forge_core::envelope::{KernelWarning, OperationMetrics, OperationResult};
use forge_core::tracing::{TraceAdjunctRecord, TraceAdjunctSet};
use forge_core::{DecisionId, DecisionLog};

use super::schema::{ModelingContext, SubOperationMetadata};

impl ModelingContext {
    /// Warnings absorbed from sub-operation envelopes.
    pub fn get_sub_warnings(&self) -> &[KernelWarning] {
        &self.sub_warnings
    }

    /// Aggregated metrics absorbed from sub-operation envelopes.
    pub fn get_sub_metrics(&self) -> &OperationMetrics {
        &self.sub_metrics
    }

    /// Aggregated lineage delta absorbed from sub-operation envelopes.
    pub fn get_sub_lineage_delta(&self) -> &forge_core::envelope::LineageDelta {
        &self.sub_lineage_delta
    }

    /// Aggregated error budget absorbed from sub-operation envelopes.
    pub fn get_sub_accumulated_error_budget(&self) -> f64 {
        self.sub_accumulated_error_budget
    }

    /// Typed adjunct payloads accumulated alongside the decision log.
    pub fn get_trace_adjuncts(&self) -> &TraceAdjunctSet {
        &self.trace_adjuncts
    }

    /// Append one adjunct payload to the context's typed trace adjunct sink.
    pub fn push_trace_adjunct(&mut self, record: TraceAdjunctRecord) {
        self.trace_adjuncts.insert(record);
    }

    /// Drain typed adjunct payloads and reset the adjunct sink.
    pub fn take_trace_adjuncts(&mut self) -> TraceAdjunctSet {
        std::mem::take(&mut self.trace_adjuncts)
    }

    /// Drain all aggregated sub-operation metadata and reset the sink.
    pub fn take_sub_metadata(&mut self) -> SubOperationMetadata {
        SubOperationMetadata {
            warnings: std::mem::take(&mut self.sub_warnings),
            metrics: std::mem::take(&mut self.sub_metrics),
            lineage_delta: std::mem::take(&mut self.sub_lineage_delta),
            accumulated_error_budget: std::mem::take(&mut self.sub_accumulated_error_budget),
        }
    }

    /// Pull all metadata from an `OperationResult` sub-operation into this context.
    ///
    /// Use this when the current function returns a plain value/result (not an
    /// `OperationResult`) but must preserve the sub-operation's audit trail.
    ///
    /// This is a true drain: absorbed metadata is removed from `op` to prevent
    /// accidental double-counting if the caller reuses the child envelope.
    pub fn absorb_sub_result<U>(&mut self, op: &mut OperationResult<U>) {
        if crate::core::tracing::KernelSpan::is_active() {
            crate::core::tracing::KernelSpan::merge_decision_log(op.take_decision_log());
            crate::core::tracing::KernelSpan::extend_warnings(op.take_warnings());
            let metrics = op.take_metrics();
            crate::core::tracing::KernelSpan::add_metrics(metrics.clone());
            crate::core::tracing::KernelSpan::record_lineage_delta(op.take_lineage_delta());

            // Still accumulate the budget here for the check_budget function
            self.sub_accumulated_error_budget += op.take_accumulated_budget();
            return;
        }

        self.decision_log.merge(op.take_decision_log());
        self.sub_warnings.extend(op.take_warnings());

        let metrics = op.take_metrics();
        self.sub_metrics.duration += metrics.duration;
        self.sub_metrics.entities_created += metrics.entities_created;
        self.sub_metrics.entities_deleted += metrics.entities_deleted;
        self.sub_metrics.entities_modified += metrics.entities_modified;
        self.sub_metrics.exact_predicate_calls += metrics.exact_predicate_calls;
        self.sub_metrics.policy_decisions_made += metrics.policy_decisions_made;

        let lineage = op.take_lineage_delta();
        self.sub_lineage_delta.faces_created += lineage.faces_created;
        self.sub_lineage_delta.faces_deleted += lineage.faces_deleted;
        self.sub_lineage_delta.half_edges_created += lineage.half_edges_created;
        self.sub_lineage_delta.half_edges_deleted += lineage.half_edges_deleted;
        self.sub_lineage_delta.vertices_created += lineage.vertices_created;
        self.sub_lineage_delta.vertices_deleted += lineage.vertices_deleted;
        self.sub_lineage_delta.loops_created += lineage.loops_created;
        self.sub_lineage_delta.loops_deleted += lineage.loops_deleted;
        self.sub_lineage_delta.edges_created += lineage.edges_created;
        self.sub_lineage_delta.edges_deleted += lineage.edges_deleted;
        self.sub_lineage_delta.shells_created += lineage.shells_created;
        self.sub_lineage_delta.shells_deleted += lineage.shells_deleted;
        self.sub_lineage_delta.solids_created += lineage.solids_created;
        self.sub_lineage_delta.solids_deleted += lineage.solids_deleted;

        self.sub_accumulated_error_budget += op.take_accumulated_budget();
    }

    /// Take ownership of the decision log, replacing it with an empty one.
    ///
    /// Marks the log as drained so `Drop` knows the success path was taken.
    pub fn take_decision_log(&mut self) -> DecisionLog {
        self.log_drained = true;
        std::mem::take(&mut self.decision_log)
    }
}
