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
//! contract types, features/traits, core/context, core/finalization

use std::collections::HashMap;

use forge_core::KernelError;
use forge_core::envelope::OperationResult;
use forge_core::tracing::TraceAdjunctSet;
use forge_signal::handles::NodeId;

use super::contract::{AuditLevel, FeatureInputs};
use super::errors::PipelineError;
use super::invariants::validate_invariant;
use crate::core::finalization::{OperationFinalizer, TopologyHashBoundary};
use crate::features::traits::{Feature, FeatureOutput};
use crate::core::ModelingContext;

/// Feature pipeline executor.
///
/// Wraps every feature evaluation with compiler-enforced contract stages.
/// Even features with no policies or invariants go through this —
/// the pipeline degrades to a no-op for simple cases (adapter-by-default).
///
/// Returns `OperationResult<FeatureOutput>` — the envelope carries all
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
        raw_inputs: &HashMap<NodeId, FeatureOutput>,
        ctx: &mut ModelingContext,
    ) -> Result<OperationResult<FeatureOutput>, KernelError> {
        // 1. Pre-validate policies (fail-fast with typed PipelineError)
        for policy in feature.required_policies() {
            ctx.validate_policy_configured(policy).map_err(|_| {
                KernelError::from(PipelineError::PolicyNotConfigured {
                    kind: policy.clone(),
                    feature: feature.feature_kind().to_string(),
                })
            })?;
        }

        // 2. Parse + validate typed inputs
        let inputs = feature.parse_inputs(raw_inputs)?;
        inputs.validate()?;

        // 3. Compute OperationSpace from input geometry (immutable lens).
        //    Steps call ctx.get_operation_space().to_local(point) —
        //    geometry stays immutable in world space throughout.
        let op_space = compute_operation_space(raw_inputs, ctx);
        ctx.set_operation_space(op_space);

        // 4. Snapshot topology hash before execution
        let hash_before = compute_input_hash(raw_inputs);

        // 5. Execute business logic with trace span
        let span_id = ctx.get_decision_log_mut().start_span(feature.feature_kind());
        let start = std::time::Instant::now();
        let result = feature.execute_typed(&inputs, ctx);
        let duration_micros = start.elapsed().as_micros() as u64;
        ctx.get_decision_log_mut().end_span(span_id, duration_micros);

        // Propagate execution errors before finalization — the envelope
        // is only useful when we have a valid FeatureOutput to carry.
        let raw_output = result?;

        // 6. KernelDraft commit-validate (Doctrine D6).
        //    Wrap the output in a transactional draft and commit. This runs
        //    commit-time topology validation (manifold edges, Euler formula).
        //    If commit fails, no invalid topology escapes the pipeline.
        let kernel_state = crate::core::KernelState::new(
            raw_output.topology,
            raw_output.geometry,
        );
        let draft = crate::core::KernelDraft::new(kernel_state);
        let committed = draft.commit()?;
        let (committed_topo, committed_geom) = committed.into_parts();
        let output = FeatureOutput {
            topology: committed_topo,
            geometry: committed_geom,
        };

        // 7. Compute hash_after from the committed output
        let hash_after = forge_topo::hashing::compute_arena_topology_hash(
            output.topology.arena(),
        );

        // 6. Finalize — drain decisions + metadata from ctx into envelope.
        //    The finalizer calls ctx.take_decision_log(), so after this block
        //    the envelope owns the canonical decision log — not ctx.
        let hashes = TopologyHashBoundary {
            before: Some(hash_before),
            after: Some(hash_after),
        };
        let mut envelope = OperationResult::new(output);
        {
            let mut finalizer = OperationFinalizer::new(ctx);
            let _finalization = finalizer.collect_success(
                &mut envelope,
                TraceAdjunctSet::new(),
                hashes,
            );
            // finalizer drops here — ctx borrow released
        }

        // 7. Post-validate invariants
        let output_ref = envelope.get_value();
        let validation_config = ctx.get_validation_config();
        for inv in feature.post_invariants() {
            validate_invariant(
                &output_ref.topology,
                &output_ref.geometry,
                inv,
                &validation_config,
            )?;
        }

        // 8. Audit (explicit match, not >= comparison)
        match feature.audit_level() {
            AuditLevel::Full => {
                emit_feature_audit(feature, &mut envelope);
            }
            AuditLevel::Summary => {
                emit_feature_summary(feature, &mut envelope);
            }
            AuditLevel::None => {}
        }

        // 9. Reset OperationSpace for next feature
        ctx.set_operation_space(crate::core::OperationSpace::identity());

        Ok(envelope)
    }
}

/// Compute a combined topology hash from all input FeatureOutputs.
fn compute_input_hash(inputs: &HashMap<NodeId, FeatureOutput>) -> u128 {
    let mut combined: u128 = 0;
    for output in inputs.values() {
        let h = forge_topo::hashing::compute_arena_topology_hash(output.topology.arena());
        combined = combined.wrapping_add(h);
    }
    combined
}

/// Compute the OperationSpace from feature inputs.
///
/// Collects all vertex positions from input geometry and analyzes
/// whether a local coordinate transform is needed for numerical safety.
fn compute_operation_space(
    inputs: &HashMap<NodeId, FeatureOutput>,
    ctx: &ModelingContext,
) -> crate::core::OperationSpace {
    let mut all_points: Vec<[f64; 3]> = Vec::new();
    for output in inputs.values() {
        for (v_id, _) in output.topology.arena().iter_vertices() {
            if let Some(pos) = output.geometry.get_vertex_position(v_id) {
                all_points.push(*pos);
            }
        }
    }
    if all_points.is_empty() {
        return crate::core::OperationSpace::identity();
    }
    let feature_tol = ctx.get_tolerance_config().scaled_vertex_tolerance();
    crate::core::OperationSpace::from_points(&all_points, feature_tol)
}

/// Emit a full audit trace for the feature execution.
///
/// Records an "audit/{feature_kind}" span in the envelope's decision log
/// with per-decision detail. The span itself carries no duration (it's a
/// metadata annotation, not a timed operation). Downstream trace viewers
/// can filter on "audit/" spans to find audit records.
fn emit_feature_audit<F: Feature>(
    feature: &F,
    envelope: &mut OperationResult<FeatureOutput>,
) {
    let decision_count = envelope.get_decision_log().len();
    let warning_count = envelope.get_warnings().len();

    // Build per-decision breakdown summary.
    let decision_details: Vec<String> = envelope.get_decision_log().decisions().map(|d| {
        format!(
            "{:?}/tier={:?}/margin={:.2e}",
            d.get_kind(),
            d.get_tier(),
            d.get_margin(),
        )
    }).collect();

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
fn emit_feature_summary<F: Feature>(
    feature: &F,
    envelope: &mut OperationResult<FeatureOutput>,
) {
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

