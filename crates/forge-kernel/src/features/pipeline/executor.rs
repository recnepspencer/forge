//! Feature pipeline executor.
//!
//! DOMAIN: Orchestrates the feature evaluation lifecycle:
//! policy pre-validation → input parsing → execution → post-invariants → audit.
//!
//! INVARIANTS:
//! - Every feature goes through the pipeline, even pass-throughs
//! - Policy pre-checks fail fast before any topology mutation
//! - Post-invariants only run on success
//!
//! DEPENDENCIES: forge-core (KernelError, DecisionLog), contract types,
//! features/traits (Feature, FeatureOutput), core/context (ModelingContext)

use std::collections::HashMap;

use forge_core::KernelError;
use forge_signal::handles::NodeId;
use forge_topo::validate::ValidationLevel;

use super::contract::{AuditLevel, FeatureInputs, InvariantKind};
use crate::features::traits::{Feature, FeatureOutput};
use crate::core::ModelingContext;

/// Feature pipeline executor.
///
/// Wraps every feature evaluation with compiler-enforced contract stages.
/// Even features with no policies or invariants go through this —
/// the pipeline degrades to a no-op for simple cases (adapter-by-default).
pub struct FeaturePipeline;

impl FeaturePipeline {
    /// Execute a feature through the full pipeline.
    ///
    /// Stages:
    /// 1. Pre-validate required policies (fail-fast)
    /// 2. Parse + validate typed inputs
    /// 3. Execute business logic
    /// 4. Post-validate invariants (success only)
    /// 5. Audit emission (based on audit level)
    pub fn execute<F: Feature>(
        feature: &F,
        raw_inputs: &HashMap<NodeId, FeatureOutput>,
        ctx: &mut ModelingContext,
    ) -> Result<FeatureOutput, KernelError> {
        // 1. Pre-validate policies (fail-fast)
        for policy in feature.required_policies() {
            ctx.validate_policy_configured(policy)?;
        }

        // 2. Parse + validate typed inputs
        let inputs = feature.parse_inputs(raw_inputs)?;
        inputs.validate()?;

        // 3. Execute business logic (wrapped in a span for tracing)
        let span_id = ctx.get_decision_log_mut().start_span(feature.feature_kind());
        let start = std::time::Instant::now();
        let result = feature.execute_typed(&inputs, ctx);
        let duration_micros = start.elapsed().as_micros() as u64;
        ctx.get_decision_log_mut().end_span(span_id, duration_micros);

        // 4. Post-validate invariants (only on success)
        if let Ok(ref output) = result {
            for inv in feature.post_invariants() {
                validate_invariant(output, inv)?;
            }
        }

        // 5. Audit (explicit match, not >= comparison)
        if result.is_ok() {
            match feature.audit_level() {
                AuditLevel::Full | AuditLevel::Summary => {
                    let audit_span = ctx.get_decision_log_mut().start_span("audit");
                    ctx.get_decision_log_mut().end_span(audit_span, 0);
                }
                AuditLevel::None => {}
            }
        }

        result
    }
}

/// Validate a post-execution invariant against the feature output.
fn validate_invariant(output: &FeatureOutput, kind: &InvariantKind) -> Result<(), KernelError> {
    match kind {
        InvariantKind::ManifoldEdges => {
            forge_topo::validate::validate_topology(
                output.topology.arena(),
                ValidationLevel::Full,
            )
        }
        InvariantKind::G1Continuity => {
            // Future: validate geometric invariants with geometry state
            Ok(())
        }
        InvariantKind::NoSelfIntersection => {
            // Future: spatial self-intersection test via BVH
            Ok(())
        }
        InvariantKind::NoSliverFaces => {
            // Future: delegate to forge_kernel::analysis::sliver::analyze_slivers
            Ok(())
        }
    }
}
