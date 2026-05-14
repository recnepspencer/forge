use super::*;
use crate::intent_admission::{
    ForgeQueryAuthoritativeIntentExecutionHandoff, ForgeQueryEffectTriggeredIntentExecutionHandoff,
    ForgeQueryIntentDecisionTraceEnvelope, ForgeQueryIntentViolationDecision,
    ForgeQueryRawIntentAdmissionRequest, ForgeQueryRuntimeEffectWriteIntentAuthoring,
    ForgeQueryRuntimeIntentAuthoring,
};

impl ForgeQueryRuntime {
    pub fn intent(
        &mut self,
        declaration: ForgeQueryIntentDeclaration,
    ) -> ForgeQueryRuntimeIntentAuthoring<'_> {
        ForgeQueryRuntimeIntentAuthoring::new(self, declaration)
    }

    pub fn next_effect_write_intent<T>(
        &mut self,
        effect: &ForgeQueryEffectHandle<T>,
        strategy_version: impl Into<String>,
        input_contract: impl Into<String>,
    ) -> ForgeQueryRuntimeEffectWriteIntentAuthoring<'_> {
        ForgeQueryRuntimeEffectWriteIntentAuthoring::new(
            self,
            effect,
            strategy_version.into(),
            input_contract.into(),
        )
    }

    pub fn execute_intent(
        &mut self,
        declaration: ForgeQueryIntentDeclaration,
    ) -> Result<ForgeQueryIntentReceipt, ForgeQueryRuntimeError> {
        let handoff = self.admit_authoritative_intent_for_execution(declaration)?;
        let binding = self.prepare_authoritative_intent_execution_binding(handoff);
        self.execute_authoritative_intent_execution_binding(binding)
    }

    pub fn execute_next_effect_write_intent<T>(
        &mut self,
        effect: &ForgeQueryEffectHandle<T>,
        strategy_version: impl Into<String>,
        input_contract: impl Into<String>,
    ) -> Result<ForgeQueryEffectIntentReceipt, ForgeQueryRuntimeError> {
        let (pending_delivery, handoff) = self.admit_next_effect_write_intent_for_execution(
            effect.name(),
            strategy_version.into(),
            input_contract.into(),
        )?;
        let binding = self.prepare_effect_intent_execution_binding(handoff, &pending_delivery);
        self.execute_effect_intent_execution_binding(binding)
    }

    pub fn admit_authoritative_intent_for_execution(
        &self,
        declaration: ForgeQueryIntentDeclaration,
    ) -> Result<ForgeQueryAuthoritativeIntentExecutionHandoff, ForgeQueryRuntimeError> {
        let review = self.review_authoritative_runtime_intent(declaration)?;
        self.resolve_reviewed_admitted_authoritative_intent_handoff(review)
    }

    pub fn admit_next_effect_write_intent_for_execution(
        &self,
        effect_name: &str,
        strategy_version: impl Into<String>,
        input_contract: impl Into<String>,
    ) -> Result<
        (
            ForgeQueryEffectDelivery,
            ForgeQueryEffectTriggeredIntentExecutionHandoff,
        ),
        ForgeQueryRuntimeError,
    > {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Effect)?;
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Intent)?;
        let pending_delivery = self.pending_effect_write_delivery(effect_name)?.1;
        let request = self.effect_runtime_intent_request(
            &pending_delivery,
            strategy_version.into(),
            input_contract.into(),
        )?;
        let review =
            crate::intent_admission::dx::ForgeQueryRuntimeIntentAdmissionReviewData::from_request(
                request,
            );
        let handoff = self.resolve_reviewed_admitted_effect_intent_handoff(review)?;
        Ok((pending_delivery, handoff))
    }

    pub(crate) fn review_authoritative_runtime_intent(
        &self,
        declaration: ForgeQueryIntentDeclaration,
    ) -> Result<
        crate::intent_admission::dx::ForgeQueryRuntimeIntentAdmissionReviewData,
        ForgeQueryRuntimeError,
    > {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Intent)?;
        let request = self.authoritative_runtime_intent_request(declaration)?;
        Ok(
            crate::intent_admission::dx::ForgeQueryRuntimeIntentAdmissionReviewData::from_request(
                request,
            ),
        )
    }

    pub(crate) fn review_next_effect_write_runtime_intent(
        &self,
        effect_name: &str,
        strategy_version: String,
        input_contract: String,
    ) -> Result<
        (
            ForgeQueryEffectDelivery,
            crate::intent_admission::dx::ForgeQueryRuntimeIntentAdmissionReviewData,
        ),
        ForgeQueryRuntimeError,
    > {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Effect)?;
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Intent)?;
        let (_, pending_delivery) = self.pending_effect_write_delivery(effect_name)?;
        let request = self.effect_runtime_intent_request(
            &pending_delivery,
            strategy_version,
            input_contract,
        )?;
        Ok((
            pending_delivery,
            crate::intent_admission::dx::ForgeQueryRuntimeIntentAdmissionReviewData::from_request(
                request,
            ),
        ))
    }

    fn authoritative_runtime_intent_request(
        &self,
        declaration: ForgeQueryIntentDeclaration,
    ) -> Result<ForgeQueryRawIntentAdmissionRequest, ForgeQueryRuntimeError> {
        let declaration_for_error = declaration.clone();
        ForgeQueryRawIntentAdmissionRequest::authoritative_runtime_entrypoint(declaration).map_err(
            |violation| {
                self.intent_violation_error(&declaration_for_error, violation, None, None, None)
            },
        )
    }

    fn effect_runtime_intent_request(
        &self,
        delivery: &ForgeQueryEffectDelivery,
        strategy_version: String,
        input_contract: String,
    ) -> Result<ForgeQueryRawIntentAdmissionRequest, ForgeQueryRuntimeError> {
        let declaration = ForgeQueryIntentDeclaration::strategy_commit(
            format!(
                "effect:{}:{}",
                delivery.effect_name(),
                delivery.commit_identity()
            ),
            delivery.target().to_string(),
            strategy_version,
            input_contract,
            delivery.payload().clone(),
        )
        .with_source_lane(ForgeQueryIntentSourceLane::EffectTriggered);
        let declaration_for_error = declaration.clone();
        ForgeQueryRawIntentAdmissionRequest::effect_runtime_entrypoint(declaration).map_err(
            |violation| {
                self.intent_violation_error(&declaration_for_error, violation, None, None, None)
            },
        )
    }

    pub(crate) fn intent_violation_error(
        &self,
        declaration: &ForgeQueryIntentDeclaration,
        violation: ForgeQueryIntentViolationDecision,
        execution: Option<&ForgeQueryIntentExecution>,
        decision_trace_envelope: Option<ForgeQueryIntentDecisionTraceEnvelope>,
        execution_provenance: Option<ForgeQueryIntentExecutionProvenance>,
    ) -> ForgeQueryRuntimeError {
        let denial =
            intent::ForgeQueryIntentAdmissionDenial::new(violation.stage(), violation.message());
        let evidence = ForgeQueryIntentDenialEvidence::new_with_trace(
            declaration,
            &denial,
            execution,
            execution_provenance,
            decision_trace_envelope,
        );
        ForgeQueryRuntimeError::IntentCommitDenied {
            intent_name: declaration.name().to_string(),
            stage: violation.stage(),
            message: violation.message().to_string(),
            evidence,
        }
    }

    pub(crate) fn intent_execution_routing_error(
        &self,
        declaration: &ForgeQueryIntentDeclaration,
        execution: &ForgeQueryIntentExecution,
        execution_provenance: ForgeQueryIntentExecutionProvenance,
        decision_trace_envelope: ForgeQueryIntentDecisionTraceEnvelope,
        source: ForgeQueryRuntimeError,
    ) -> ForgeQueryRuntimeError {
        let stage = "post-execution-routing";
        let message = source.to_string();
        let evidence = intent::ForgeQueryIntentExecutionFailureEvidence::new(
            declaration,
            stage,
            message.clone(),
            execution,
            execution_provenance,
            decision_trace_envelope,
        );
        ForgeQueryRuntimeError::IntentExecutionRoutingFailed {
            intent_name: declaration.name().to_string(),
            stage,
            message,
            evidence,
            source: Box::new(source),
        }
    }

    pub(crate) fn admitted_handoff_violation_error(
        &self,
        handoff: &ForgeQueryAdmittedIntentExecutionHandoff,
        stage: &'static str,
        message: impl Into<String>,
    ) -> ForgeQueryRuntimeError {
        let violation = ForgeQueryIntentViolationDecision::new(
            handoff.family(),
            handoff.entrypoint(),
            stage,
            message,
            handoff.request_digest(),
            handoff.eligibility_digest(),
        );
        let decision_trace_envelope =
            ForgeQueryIntentDecisionTraceEnvelope::for_handoff_violation(handoff, &violation);
        self.intent_violation_error(
            handoff.declaration(),
            violation,
            None,
            Some(decision_trace_envelope),
            None,
        )
    }

    pub(crate) fn pending_effect_write_delivery(
        &self,
        effect_name: &str,
    ) -> Result<(usize, ForgeQueryEffectDelivery), ForgeQueryRuntimeError> {
        let runtime = self
            .effects
            .get(effect_name)
            .ok_or_else(|| ForgeQueryRuntimeError::MissingEffect(effect_name.to_string()))?;
        runtime
            .deliveries
            .iter()
            .enumerate()
            .find(|(_, delivery)| {
                delivery.family() == &ForgeQueryEffectDeliveryFamily::PendingWriteIntent
            })
            .map(|(index, delivery)| (index, delivery.clone()))
            .ok_or_else(|| {
                ForgeQueryRuntimeError::MissingPendingWriteIntent(effect_name.to_string())
            })
    }

    pub(crate) fn remove_pending_effect_delivery(
        &mut self,
        effect_name: &str,
        pending_index: usize,
        pending_delivery: &ForgeQueryEffectDelivery,
    ) {
        if let Some(runtime) = self.effects.get_mut(effect_name) {
            if runtime
                .deliveries
                .get(pending_index)
                .is_some_and(|delivery| {
                    delivery.family() == &ForgeQueryEffectDeliveryFamily::PendingWriteIntent
                        && delivery.effect_name() == pending_delivery.effect_name()
                        && delivery.commit_identity() == pending_delivery.commit_identity()
                })
            {
                runtime.deliveries.remove(pending_index);
            } else if let Some(index) = runtime.deliveries.iter().position(|delivery| {
                delivery.family() == &ForgeQueryEffectDeliveryFamily::PendingWriteIntent
                    && delivery.effect_name() == pending_delivery.effect_name()
                    && delivery.commit_identity() == pending_delivery.commit_identity()
            }) {
                runtime.deliveries.remove(index);
            }
        }
    }
}
