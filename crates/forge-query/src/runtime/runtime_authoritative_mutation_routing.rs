use super::*;
use crate::evidence_identity::ForgeQueryEvidenceIdentity;
use crate::memory_workspace::ForgeQueryEntityIdentity;

impl ForgeQueryRuntime {
    pub(super) fn route_authoritative_mutation_receipt(
        &mut self,
        receipt: ForgeQueryMutationReceipt,
        mutation_family: ForgeQueryMutationFamily,
        declared_collection_identity: Option<ForgeQueryMutationTargetCollectionIdentity>,
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
        obligation_dispatch: Option<ForgeQueryAuthoritativeMutationObligationDispatch>,
    ) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
        let (_, target_collection, mut target_entity_identity) =
            classify_receipt_mutation_summary(&receipt);
        let mut target_collection_identity = target_collection;
        if let Some(binding) = existing_truth_binding.as_ref() {
            target_collection_identity = binding.target_collection_identity().cloned();
            target_entity_identity = Some(binding.resolved_entity_artifact_identity());
        }
        let summary = self.route_authoritative_mutation_summary(&receipt, &mutation_metadata)?;
        self.capture_shared_read_generation(receipt.snapshot_identity.clone());
        Ok(ForgeQueryWriteReceipt::from_mutation_receipt(
            receipt,
            mutation_family,
            declared_collection_identity,
            declared_entity_identity,
            existing_truth_binding,
            existing_truth_assertion,
            symbolic_target_reference,
            symbolic_aspect_resolution_evidence,
            naming_intent,
            continuity_intent,
            target_collection_identity,
            target_entity_identity,
            declared_aspect_operations,
            declared_aspect_value_digest,
            mutation_metadata,
            summary.affected_live_view_targets,
            summary.affected_derived_view_targets,
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
            obligation_dispatch,
        ))
    }

    pub(super) fn route_authoritative_mutation_summary(
        &mut self,
        receipt: &ForgeQueryMutationReceipt,
        mutation_metadata: &ForgeQueryMutationMetadata,
    ) -> Result<ForgeQueryRoutedMutationSummary, ForgeQueryRuntimeError> {
        let affected_live_view_targets = route_live_subscription_delivery(
            &mut self.active_subscriptions,
            &mut self.live_subscriptions,
            &self.live_subscription_index,
            receipt,
        )?;
        let computed_candidate_live_views = self.computed_candidate_live_views(receipt);
        let retained_live_view_targets = retained_live_view_names_for_candidates(
            &self.derived_views,
            &self.derived_dependency_index,
            computed_candidate_live_views.iter().cloned(),
        );
        let retained_live_rows = retained_live_view_targets
            .into_iter()
            .map(|target| {
                let rows = self.backend.live_entities_for_target(&target);
                (target, rows)
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
        let affected_derived_view_targets = computed_result.affected_view_targets();
        let live_artifact_target_collections = self.live_artifact_target_collections();
        let effect_result = route_effect_deliveries(
            &mut self.effects,
            &self.effect_index,
            &self.derived_views,
            &live_artifact_target_collections,
            receipt,
            &affected_live_view_targets,
            &affected_derived_view_targets,
        );
        Ok(ForgeQueryRoutedMutationSummary {
            affected_live_view_targets,
            affected_derived_view_targets,
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
