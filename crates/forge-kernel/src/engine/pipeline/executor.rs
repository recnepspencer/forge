//! Feature pipeline executor.
//!
//! DOMAIN: Orchestrates the feature evaluation lifecycle:
//! policy pre-validation -> input parsing -> execution ->
//! subscriber finalization -> post-invariants -> audit.
//!
//! INVARIANTS:
//! - Every feature goes through the pipeline, even pass-throughs
//! - Policy pre-checks fail fast before any topology mutation
//! - Post-invariants only run on success
//! - `OperationResult` is the canonical metadata transport

use std::collections::HashMap;

use forge_core::envelope::OperationResult;
use forge_core::tracing::DecisionSink;
use forge_core::KernelError;
use forge_signal::facade::{CheckpointBarrier, NodeId};

use super::super::contracts::contract::{ConditioningMode, FeatureInputs};
use super::super::contracts::feature_trait::Feature;
use super::super::operation_space::operation_space::OperationSpace;
use super::super::output::solid_envelope::SolidEnvelope;
use super::conditioning_guard::ConditioningGuard;
use super::fingerprint::compute_pipeline_fingerprint;
use super::invariants::validate_invariant;
use crate::configuration::facade::{resolve_config, KernelConfig};
use crate::context::scope::OperationScope;
use crate::engine::transaction::data::feature_event::{FeatureInvocationId, KernelFeatureEvent};
use crate::engine::transaction::data::feature_execution_config::FeatureExecutionConfig;
use crate::engine::transaction::data::operation_outputs::OperationEnvelopeOutput;
use crate::engine::transaction::data::subscriber_data_id::KernelSubscriberDataId;
use crate::engine::transaction::logic::feature_event_runtime::{
    FeatureEventRuntime, FeatureEventRuntimeContext,
};
use crate::proof::ValidationConfig;

/// Feature pipeline executor.
///
/// Wraps every feature evaluation with compiler-enforced contract stages.
/// Even features with no policies or invariants go through this -
/// the pipeline degrades to a no-op for simple cases (adapter-by-default).
///
/// Returns `OperationResult<SolidEnvelope>` - the envelope carries all
/// audit metadata (decisions, warnings, metrics, lineage, hashes) while
/// the inner value is the domain result.
pub struct FeaturePipeline;

impl FeaturePipeline {
    /// Execute a feature through the full pipeline.
    pub fn execute<F: Feature>(
        feature: &F,
        mut raw_inputs: HashMap<NodeId, SolidEnvelope>,
        session_config: &KernelConfig,
    ) -> Result<OperationResult<SolidEnvelope>, KernelError> {
        let resolved_config = resolve_config(
            session_config,
            None,
            feature
                .config_overrides()
                .as_ref()
                .map(|o| (o, Some(feature.feature_kind().to_string()))),
            None,
        )?;

        let execution_config = FeatureExecutionConfig {
            feature_kind: feature.feature_kind(),
            audit_level: feature.audit_level(),
            validation: ValidationConfig {
                checkpoints: resolved_config.config().validation.checkpoints.clone(),
                include_geometric: resolved_config.config().validation.include_geometric,
                entity_limit: resolved_config.config().validation.entity_limit,
            },
        };

        let mut runtime_ctx = FeatureEventRuntimeContext::from_config(session_config.clone());
        let mut event_runtime = FeatureEventRuntime::new()?;

        for policy in feature.required_policies() {
            resolved_config.validate_policy_configured(policy)?;
        }

        let conditioning_mode = feature.conditioning_mode();
        let hash_before = compute_pipeline_fingerprint(
            &raw_inputs,
            execution_config.feature_kind,
            conditioning_mode,
            resolved_config.config().tolerance.spatial_tolerance,
            resolved_config.config().tolerance.model_scale_mm,
            resolved_config.config().tolerance.min_edge_length,
            resolved_config.config().diagnostics.fingerprint_detail,
        );

        let op_space = match conditioning_mode {
            ConditioningMode::None => OperationSpace::identity(),
            ConditioningMode::UnaryAnalysis | ConditioningMode::BinaryAnalysis => {
                let space = OperationSpace::analyze_envelopes(
                    raw_inputs.values(),
                    resolved_config.tolerance_config().get_min_edge_length(),
                );
                if space.is_active() {
                    for env in raw_inputs.values_mut() {
                        space.transform_store(env.geometry_mut());
                    }
                }
                space
            }
        };

        let inputs = feature.parse_inputs(raw_inputs)?;
        inputs.validate()?;

        let invocation_id = FeatureInvocationId::new(1);
        event_runtime.begin(&mut runtime_ctx)?;
        event_runtime.emit(KernelFeatureEvent::OperationStarted {
            feature_kind: execution_config.feature_kind,
            invocation_id,
            audit_level: execution_config.audit_level,
            state_hash_before: hash_before,
        });

        let span_id = runtime_ctx
            .modeling_context
            .start_span(execution_config.feature_kind);
        let start = std::time::Instant::now();
        let result = {
            let mut scope = OperationScope::with_conditioning(
                &resolved_config,
                &mut runtime_ctx.modeling_context,
                &op_space,
            );
            feature.execute_typed(inputs, &mut scope)
        };
        let duration_micros = start.elapsed().as_micros() as u64;
        runtime_ctx.modeling_context.end_span(span_id, duration_micros);

        let mut sub_envelope = match result {
            Ok(value) => value,
            Err(err) => {
                event_runtime.emit(KernelFeatureEvent::OperationFailed {
                    invocation_id,
                    error_summary: err.to_string(),
                });
                event_runtime.rollback(&mut runtime_ctx);
                return Err(err);
            }
        };

        if let Some(guard) =
            ConditioningGuard::new(&op_space, sub_envelope.get_value_mut().geometry_mut())
        {
            guard.defuse();
        }

        let hash_after = forge_topo::transactions::compute_arena_topology_hash(
            sub_envelope.get_value().topology().arena(),
        );
        event_runtime.emit(KernelFeatureEvent::OperationCompleted {
            invocation_id,
            duration_micros,
            state_hash_after: hash_after,
        });
        event_runtime.flush(CheckpointBarrier::PerOperation, &mut runtime_ctx)?;

        let operation_output = event_runtime
            .event_bus()
            .context()
            .committed::<OperationEnvelopeOutput>(KernelSubscriberDataId::OperationEnvelope)
            .ok_or_else(|| KernelError::InternalError {
                message: "OperationEnvelope output missing after feature event flush".to_string(),
                context: None,
            })?;
        apply_operation_output(&mut sub_envelope, operation_output);

        for invariant in feature.post_invariants() {
            validate_invariant(
                sub_envelope.get_value().topology(),
                sub_envelope.get_value().geometry(),
                invariant,
                &execution_config.validation,
            )?;
        }

        Ok(sub_envelope)
    }
}

fn apply_operation_output(
    envelope: &mut OperationResult<SolidEnvelope>,
    output: &OperationEnvelopeOutput,
) {
    let mut merged_log = envelope.get_decision_log().clone();
    merged_log.merge(output.decision_log.clone());
    envelope.set_decision_log(merged_log);
    for warning in output.warnings.iter().cloned() {
        envelope.add_warning(warning);
    }
    let mut merged_metrics = envelope.get_metrics().clone();
    merged_metrics.accumulate(&output.metrics);
    envelope.set_metrics(merged_metrics);

    let mut merged_lineage = envelope.get_lineage_delta().clone();
    merged_lineage.accumulate(&output.lineage_delta);
    envelope.set_lineage_delta(merged_lineage);

    envelope.consume_budget(output.accumulated_error_budget);
    envelope.set_state_hash_before(output.state_hash_before);
    envelope.set_state_hash_after(output.state_hash_after);
    for summary in output.extra_summaries.iter().cloned() {
        envelope.add_extra_summary(summary);
    }
}
