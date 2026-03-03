//! Feature pipeline executor.
//!
//! DOMAIN: Orchestrates the feature evaluation lifecycle:
//! policy pre-validation → input parsing → execution →
//! finalization → post-invariants → audit.
//!
//! INVARIANTS:
//! - Every feature goes through the pipeline, even pass-throughs
//! - Policy pre-checks fail fast before any topology mutation
//! - Post-invariants only run on success
//! - `OperationResult` is the canonical metadata transport
//!
//! DEPENDENCIES: forge-core (KernelError, OperationResult, DecisionLog),
//! contract types, features/traits, context, finalization

use std::collections::HashMap;

use forge_core::envelope::OperationResult;
use forge_core::tracing::{DecisionSink, TraceAdjunctSet};
use forge_core::KernelError;
use forge_signal::facade::NodeId;

use super::super::contracts::contract::{AuditLevel, FeatureInputs};
use super::super::contracts::feature_trait::Feature;
use super::super::output::solid_envelope::SolidEnvelope;
use super::invariants::validate_invariant;
use crate::configuration::facade::{resolve_config, KernelConfig};
use crate::context::facade::ModelingContext;
use crate::context::scope::OperationScope;
use super::super::facade::{OperationFinalizer, TopologyHashBoundary};
use crate::proof::checkpoint::schema::ValidationConfig;

/// Feature pipeline executor.
///
/// Wraps every feature evaluation with compiler-enforced contract stages.
/// Even features with no policies or invariants go through this —
/// the pipeline degrades to a no-op for simple cases (adapter-by-default).
///
/// Returns `OperationResult<SolidEnvelope>` — the envelope carries all
/// audit metadata (decisions, warnings, metrics, lineage, hashes) while
/// the inner value is the domain result.  Pre-execution failures
/// (missing policy, invalid inputs) short-circuit with `Err(KernelError)`.
pub struct FeaturePipeline;

impl FeaturePipeline {
    /// Execute a feature through the full pipeline.
    ///
    /// Stages:
    /// 1. Pre-validate required policies (fail-fast)
    /// 2. Parse + validate typed inputs
    /// 3. Snapshot topology hash before execution
    /// 4. Execute business logic with trace span
    /// 5. Finalize — drain decisions + metadata into OperationResult
    /// 6. Post-validate invariants (success only)
    /// 7. Audit emission (based on audit level)
    ///
    /// Returns `Ok(OperationResult<FeatureOutput>)` on success, carrying
    /// the full audit trail in the envelope.  Returns `Err(KernelError)`
    /// for pre-execution failures, execution errors, or post-invariant
    /// violations.
    pub fn execute<F: Feature>(
        feature: &F,
        raw_inputs: &HashMap<NodeId, SolidEnvelope>,
        session_config: &KernelConfig,
    ) -> Result<OperationResult<SolidEnvelope>, KernelError> {
        let mut ctx = ModelingContext::from_config(session_config.clone());

        // 1. Resolve configuration (needed for policy pre-check and execution)
        let resolved_config = resolve_config(
            session_config,
            None,
            feature
                .config_overrides()
                .as_ref()
                .map(|o| (o, Some(feature.feature_kind().to_string()))),
            None,
        )?;

        // 2. Pre-validate required policies (fail-fast before any mutation)
        for policy in feature.required_policies() {
            resolved_config.validate_policy_configured(policy)?;
        }

        // 3. Parse + validate typed inputs
        let inputs = feature.parse_inputs(raw_inputs)?;
        inputs.validate()?;

        // 4. Snapshot topology hash before execution
        let hash_before = compute_input_hash(raw_inputs);

        // 5. Execute business logic with trace span
        let span_id = ctx.start_span(feature.feature_kind());

        let start = std::time::Instant::now();
        let result = {
            let mut scope = OperationScope::new(&resolved_config, &mut ctx);
            feature.execute_typed(&inputs, &mut scope)
        };
        let duration_micros = start.elapsed().as_micros() as u64;

        ctx.end_span(span_id, duration_micros);

        // Propagate execution errors before finalization
        let mut sub_envelope = result?;

        // 6. Post-validate invariants (success only)
        let validation_config = ValidationConfig {
            checkpoints: resolved_config.config().validation.checkpoints.clone(),
            include_geometric: resolved_config.config().validation.include_geometric,
            entity_limit: resolved_config.config().validation.entity_limit,
        };
        for invariant in feature.post_invariants() {
            validate_invariant(
                sub_envelope.get_value().topology(),
                sub_envelope.get_value().geometry(),
                invariant,
                &validation_config,
            )?;
        }

        // 7. Compute hash_after from the output
        let hash_after = forge_topo::transactions::compute_arena_topology_hash(sub_envelope.get_value().topology().arena());

        // 8. Finalize — drain decisions + metadata from ctx into envelope
        let hashes = TopologyHashBoundary {
            before: Some(hash_before),
            after: Some(hash_after),
        };
        {
            let mut finalizer = OperationFinalizer::new(&mut ctx);
            let _finalization = finalizer.collect_success(
                &mut sub_envelope,
                TraceAdjunctSet::new(),
                hashes,
                None,
            );
        }

        // 9. Audit-level filtering
        match feature.audit_level() {
            AuditLevel::None => {
                sub_envelope.set_decision_log(forge_core::DecisionLog::new());
            }
            AuditLevel::Summary => {
                emit_feature_summary(feature, &mut sub_envelope);
            }
            AuditLevel::Full => {
                emit_feature_audit(feature, &mut sub_envelope);
            }
        }

        Ok(sub_envelope)
    }
}

/// Compute a combined topology hash from all input SolidEnvelopes.
fn compute_input_hash(inputs: &HashMap<NodeId, SolidEnvelope>) -> u128 {
    let mut combined: u128 = 0;
    for output in inputs.values() {
        let h = forge_topo::transactions::compute_arena_topology_hash(output.topology().arena());
        combined = combined.wrapping_add(h);
    }
    combined
}

/// Emit a full audit trace for the feature execution.
///
/// Records an "audit/{feature_kind}" span in the envelope's decision log
/// with per-decision detail. The span itself carries no duration (it's a
/// metadata annotation, not a timed operation). Downstream trace viewers
/// can filter on "audit/" spans to find audit records.
fn emit_feature_audit<F: Feature>(feature: &F, envelope: &mut OperationResult<SolidEnvelope>) {
    let decision_count = envelope.get_decision_log().len();
    let warning_count = envelope.get_warnings().len();

    // Build per-decision breakdown summary.
    let decision_details: Vec<String> = envelope
        .get_decision_log()
        .decisions()
        .map(|d| {
            format!(
                "{:?}/tier={:?}/margin={:.2e}",
                d.get_kind(),
                d.get_tier(),
                d.get_margin(),
            )
        })
        .collect();

    let detail = format!(
        "full: {} decisions, {} warnings | [{}]",
        decision_count,
        warning_count,
        decision_details.join(", "),
    );

    // Record audit span directly in the envelope's decision log.
    let span_name = format!("audit/{}", feature.feature_kind());
    let log = envelope.get_decision_log_mut();
    let span_id = log.start_span(&span_name);
    log.end_span(span_id, 0);

    // Also record as extra summary for structured access.
    envelope.add_extra_summary(detail);
}

/// Emit a summary audit trace for the feature execution.
///
/// Records an "audit/{feature_kind}" span with aggregate counts only.
/// Per-decision detail is still available in the envelope's decision log
/// but the `AuditLevel::Summary` contract signals that downstream
/// consumers should not rely on per-decision inspection.
fn emit_feature_summary<F: Feature>(feature: &F, envelope: &mut OperationResult<SolidEnvelope>) {
    let summary = format!(
        "summary: {} decisions, {} warnings",
        envelope.get_decision_log().len(),
        envelope.get_warnings().len(),
    );

    let span_name = format!("audit/{}", feature.feature_kind());
    let log = envelope.get_decision_log_mut();
    let span_id = log.start_span(&span_name);
    log.end_span(span_id, 0);

    envelope.add_extra_summary(summary);
}
