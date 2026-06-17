use super::*;
use crate::evidence_identity::ForgeQueryEvidenceIdentity;
use crate::memory_workspace::ForgeQueryEntityIdentity;

impl ForgeQueryRuntime {
    pub(super) fn route_authoritative_mutation_receipt(
        &mut self,
        receipt: ForgeQueryMutationReceipt,
        mutation_family: ForgeQueryMutationFamily,
        declared_collection: Option<String>,
        declared_entity_identity: Option<ForgeQueryEntityIdentity>,
        existing_truth_binding: Option<ForgeQueryExistingTruthTargetBinding>,
        existing_truth_assertion: Option<ForgeQueryVerifiedExistingTruthAssertion>,
        symbolic_target_reference: Option<ForgeQuerySymbolicTargetReference>,
        symbolic_aspect_resolution_evidence: Vec<ForgeQuerySymbolicAspectResolutionEvidence>,
        naming_intent: Option<ForgeQueryNamingMutationIntent>,
        continuity_intent: Option<ForgeQueryContinuityMutationIntent>,
        declared_aspect_operations: Vec<crate::runtime::ForgeQueryAspectMutationOperation>,
        declared_aspect_value_digest: Option<ForgeQueryEvidenceIdentity>,
        mutation_metadata: ForgeQueryMutationMetadata,
        decision_trace_envelope: Option<ForgeQueryIntentDecisionTraceEnvelope>,
        execution_provenance: Option<ForgeQueryIntentExecutionProvenance>,
    ) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
        let (_, mut target_collection, mut target_entity_identity) =
            classify_receipt_mutation_summary(&receipt);
        if let Some(binding) = existing_truth_binding.as_ref() {
            target_collection = binding.target_collection().map(str::to_string);
            target_entity_identity = Some(binding.resolved_entity_artifact_identity());
        }
        let summary = self.route_authoritative_mutation_summary(&receipt, &mutation_metadata)?;
        self.capture_shared_read_generation(receipt.snapshot_identity.clone());
        Ok(ForgeQueryWriteReceipt::from_mutation_receipt(
            receipt,
            mutation_family,
            declared_collection,
            declared_entity_identity,
            existing_truth_binding,
            existing_truth_assertion,
            symbolic_target_reference,
            symbolic_aspect_resolution_evidence,
            naming_intent,
            continuity_intent,
            target_collection,
            target_entity_identity,
            declared_aspect_operations,
            declared_aspect_value_digest,
            mutation_metadata,
            summary.affected_live_view_ids,
            summary.affected_derived_view_ids,
            summary.considered_computed_view_count,
            summary.considered_effect_count,
            summary.delivered_effect_count,
            summary.pending_write_intent_count,
            summary.suppressed_effect_count,
            summary.meaningful_effect_suppression_count,
            summary.effect_expression_failure_count,
            summary.refresh_fallback,
            decision_trace_envelope,
            execution_provenance,
        ))
    }

    pub(super) fn route_authoritative_mutation_summary(
        &mut self,
        receipt: &ForgeQueryMutationReceipt,
        mutation_metadata: &ForgeQueryMutationMetadata,
    ) -> Result<ForgeQueryRoutedMutationSummary, ForgeQueryRuntimeError> {
        let affected_live_view_ids = route_live_subscription_delivery(
            &mut self.active_subscriptions,
            &mut self.live_subscriptions,
            &self.live_subscription_index,
            receipt,
        )?;
        let computed_candidate_live_views = self.computed_candidate_live_views(receipt);
        let retained_live_view_names = retained_live_view_names_for_candidates(
            &self.derived_views,
            &self.derived_dependency_index,
            computed_candidate_live_views.iter().cloned(),
        );
        let retained_live_rows = retained_live_view_names
            .into_iter()
            .map(|view_name| {
                let rows = self.backend.live_entities(&view_name);
                (view_name, rows)
            })
            .collect::<BTreeMap<_, _>>();
        let computed_result = route_derived_view_patches(
            &mut self.derived_views,
            &self.derived_dependency_index,
            computed_candidate_live_views,
            &retained_live_rows,
            receipt,
            mutation_metadata,
        );
        let refresh_fallback = computed_result.refresh_fallback();
        let considered_computed_view_count = computed_result.considered_view_count();
        let affected_derived_view_ids = computed_result.affected_view_ids();
        let live_view_targets = self.live_view_targets();
        let effect_result = route_effect_deliveries(
            &mut self.effects,
            &self.effect_index,
            &self.derived_views,
            &live_view_targets,
            receipt,
            &affected_live_view_ids,
            &affected_derived_view_ids,
        );
        Ok(ForgeQueryRoutedMutationSummary {
            affected_live_view_ids,
            affected_derived_view_ids,
            considered_computed_view_count,
            considered_effect_count: effect_result.considered_effect_count(),
            delivered_effect_count: effect_result.delivered_effect_count(),
            pending_write_intent_count: effect_result.pending_write_intent_count(),
            suppressed_effect_count: effect_result.suppressed_effect_count(),
            meaningful_effect_suppression_count: effect_result.meaningful_suppression_count(),
            effect_expression_failure_count: effect_result.expression_failure_count(),
            refresh_fallback,
        })
    }
}
