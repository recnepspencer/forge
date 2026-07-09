use super::*;
use crate::intent_admission::{
    admit_authoritative_execution, admit_effect_execution, intent_runtime_facade_family,
    WorthQueryAdmittedIntentExecutionHandoff, WorthQueryAuthoritativeIntentExecutionBinding,
    WorthQueryAuthoritativeIntentExecutionHandoff, WorthQueryEffectTriggeredIntentExecutionBinding,
    WorthQueryEffectTriggeredIntentExecutionHandoff,
};

impl WorthQueryRuntime {
    pub(crate) fn prepare_authoritative_intent_execution_binding(
        &self,
        handoff: WorthQueryAuthoritativeIntentExecutionHandoff,
    ) -> WorthQueryAuthoritativeIntentExecutionBinding {
        WorthQueryAuthoritativeIntentExecutionBinding::from_handoff(handoff)
    }

    pub(crate) fn prepare_effect_intent_execution_binding(
        &self,
        handoff: WorthQueryEffectTriggeredIntentExecutionHandoff,
        pending_delivery: &WorthQueryEffectDelivery,
    ) -> WorthQueryEffectTriggeredIntentExecutionBinding {
        WorthQueryEffectTriggeredIntentExecutionBinding::from_handoff_and_delivery(
            handoff,
            pending_delivery,
        )
    }

    pub(crate) fn execute_authoritative_intent_execution_binding(
        &mut self,
        binding: WorthQueryAuthoritativeIntentExecutionBinding,
    ) -> Result<WorthQueryIntentReceipt, WorthQueryRuntimeError> {
        self.admit_facade_family(intent_runtime_facade_family(
            binding.declaration().source_lane(),
        ))?;
        self.admit_facade_family(WorthQueryRuntimeFacadeFamily::Intent)?;
        let declaration = binding.declaration().clone();
        let execution = self.backend.execute_intent(&declaration)?;
        admit_authoritative_execution(binding.handoff(), &execution).map_err(|violation| {
            let handoff = WorthQueryAdmittedIntentExecutionHandoff::from(binding.handoff().clone());
            let decision_trace_envelope =
                WorthQueryIntentDecisionTraceEnvelope::for_execution_violation(
                    &handoff, &execution, &violation,
                );
            let snapshot_evidence_identity = execution
                .mutation_receipt()
                .snapshot_identity
                .evidence_identity();
            let execution_provenance =
                WorthQueryIntentExecutionProvenance::for_shared_execution_typed_parts(
                    binding.family(),
                    binding.entrypoint(),
                    binding.execution_seam(),
                    binding.handoff().decision_digest(),
                    binding.handoff().handoff_digest(),
                    binding.binding_digest(),
                    execution.outcome_digest(),
                    &snapshot_evidence_identity,
                );
            self.intent_violation_error(
                &declaration,
                violation,
                Some(&execution),
                Some(decision_trace_envelope),
                Some(execution_provenance),
            )
        })?;
        let summary = classify_receipt_mutation_summary(execution.mutation_receipt());
        let handoff = WorthQueryAdmittedIntentExecutionHandoff::from(binding.handoff().clone());
        let snapshot_evidence_identity = execution
            .mutation_receipt()
            .snapshot_identity
            .evidence_identity();
        let execution_provenance =
            WorthQueryIntentExecutionProvenance::for_shared_execution_typed_parts(
                binding.family(),
                binding.entrypoint(),
                binding.execution_seam(),
                binding.handoff().decision_digest(),
                binding.handoff().handoff_digest(),
                binding.binding_digest(),
                execution.outcome_digest(),
                &snapshot_evidence_identity,
            );
        let decision_trace_envelope =
            WorthQueryIntentDecisionTraceEnvelope::for_admitted_execution(&handoff, &execution);
        let write_receipt = self
            .route_authoritative_mutation_receipt(
                execution.mutation_receipt().clone(),
                summary.0,
                summary.1,
                summary.2,
                None,
                None,
                None,
                Vec::new(),
                None,
                None,
                Vec::new(),
                None,
                WorthQueryMutationMetadata::default(),
                Some(decision_trace_envelope.clone()),
                Some(execution_provenance.clone()),
                None,
            )
            .map_err(|error| {
                self.intent_execution_routing_error(
                    &declaration,
                    &execution,
                    execution_provenance.clone(),
                    decision_trace_envelope.clone(),
                    error,
                )
            })?;
        Ok(WorthQueryIntentReceipt::from_authoritative_binding(
            &binding,
            &declaration,
            execution,
            &write_receipt,
        ))
    }

    pub(crate) fn execute_effect_intent_execution_binding(
        &mut self,
        binding: WorthQueryEffectTriggeredIntentExecutionBinding,
    ) -> Result<WorthQueryEffectIntentReceipt, WorthQueryRuntimeError> {
        self.admit_facade_family(intent_runtime_facade_family(
            binding.declaration().source_lane(),
        ))?;
        self.admit_facade_family(WorthQueryRuntimeFacadeFamily::Intent)?;
        let handoff = WorthQueryAdmittedIntentExecutionHandoff::from(binding.handoff().clone());
        let (pending_index, pending_delivery) = self
            .pending_effect_write_delivery_for_binding(&binding)
            .map_err(|error| {
                self.admitted_handoff_violation_error(
                    &handoff,
                    "pending-write-intent-binding",
                    error.to_string(),
                )
            })?;
        let declaration = binding.declaration().clone();
        self.admit_effect_write_intent_graph_obligation_boundary(&handoff, &pending_delivery)?;
        let execution = self.backend.execute_intent(&declaration)?;
        admit_effect_execution(binding.handoff(), &execution).map_err(|violation| {
            let decision_trace_envelope =
                WorthQueryIntentDecisionTraceEnvelope::for_execution_violation(
                    &handoff, &execution, &violation,
                );
            let snapshot_evidence_identity = execution
                .mutation_receipt()
                .snapshot_identity
                .evidence_identity();
            let execution_provenance =
                WorthQueryIntentExecutionProvenance::for_shared_execution_typed_parts(
                    binding.family(),
                    binding.entrypoint(),
                    binding.execution_seam(),
                    binding.handoff().decision_digest(),
                    binding.handoff().handoff_digest(),
                    binding.binding_digest(),
                    execution.outcome_digest(),
                    &snapshot_evidence_identity,
                );
            self.intent_violation_error(
                &declaration,
                violation,
                Some(&execution),
                Some(decision_trace_envelope),
                Some(execution_provenance),
            )
        })?;
        let summary = classify_receipt_mutation_summary(execution.mutation_receipt());
        let snapshot_evidence_identity = execution
            .mutation_receipt()
            .snapshot_identity
            .evidence_identity();
        let execution_provenance =
            WorthQueryIntentExecutionProvenance::for_shared_execution_typed_parts(
                binding.family(),
                binding.entrypoint(),
                binding.execution_seam(),
                binding.handoff().decision_digest(),
                binding.handoff().handoff_digest(),
                binding.binding_digest(),
                execution.outcome_digest(),
                &snapshot_evidence_identity,
            );
        let decision_trace_envelope =
            WorthQueryIntentDecisionTraceEnvelope::for_admitted_execution(&handoff, &execution);
        let write_receipt = self
            .route_authoritative_mutation_receipt(
                execution.mutation_receipt().clone(),
                summary.0,
                summary.1,
                summary.2,
                None,
                None,
                None,
                Vec::new(),
                None,
                None,
                Vec::new(),
                None,
                WorthQueryMutationMetadata::default(),
                Some(decision_trace_envelope.clone()),
                Some(execution_provenance.clone()),
                None,
            )
            .map_err(|error| {
                self.intent_execution_routing_error(
                    &declaration,
                    &execution,
                    execution_provenance.clone(),
                    decision_trace_envelope.clone(),
                    error,
                )
            })?;
        let intent_receipt = WorthQueryIntentReceipt::from_effect_binding(
            &binding,
            &declaration,
            execution,
            &write_receipt,
        );
        let effect_target = WorthQueryEffectTarget::from_name(pending_delivery.effect_name());
        self.remove_pending_effect_delivery(&effect_target, pending_index, &pending_delivery);
        Ok(WorthQueryEffectIntentReceipt::new(
            &pending_delivery,
            intent_receipt,
        ))
    }

    fn pending_effect_write_delivery_for_binding(
        &self,
        binding: &WorthQueryEffectTriggeredIntentExecutionBinding,
    ) -> Result<(usize, WorthQueryEffectDelivery), WorthQueryRuntimeError> {
        let effect_target = WorthQueryEffectTarget::from_name(binding.effect_name());
        let runtime = self.effects.get(&effect_target).ok_or_else(|| {
            WorthQueryRuntimeError::MissingEffect(binding.effect_name().to_string())
        })?;
        runtime
            .deliveries
            .iter()
            .enumerate()
            .find(|(_, delivery)| {
                delivery.family() == &WorthQueryEffectDeliveryFamily::PendingWriteIntent
                    && binding.matches_pending_delivery(delivery)
            })
            .map(|(index, delivery)| (index, delivery.clone()))
            .ok_or_else(|| {
                WorthQueryRuntimeError::MissingPendingWriteIntent(binding.effect_name().to_string())
            })
    }

    fn admit_effect_write_intent_graph_obligation_boundary(
        &self,
        _handoff: &WorthQueryAdmittedIntentExecutionHandoff,
        pending_delivery: &WorthQueryEffectDelivery,
    ) -> Result<(), WorthQueryRuntimeError> {
        if self
            .graph_obligation_registration_catalog()
            .registration_count()
            == 0
        {
            return Ok(());
        }
        Err(
            WorthQueryRuntimeError::GraphObligationEffectTouchDescriptorMissing {
                effect_name: pending_delivery.effect_name().to_string(),
            },
        )
    }
}
