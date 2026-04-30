use super::*;

impl ForgeQueryRuntime {
    pub fn execute_intent(
        &mut self,
        declaration: ForgeQueryIntentDeclaration,
    ) -> Result<ForgeQueryIntentReceipt, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Intent)?;
        intent::admit_authoritative_intent_declaration(&declaration).map_err(|denial| {
            let evidence = ForgeQueryIntentDenialEvidence::new(&declaration, &denial, None);
            ForgeQueryRuntimeError::IntentCommitDenied {
                intent_name: declaration.name().to_string(),
                stage: denial.stage(),
                message: denial.message().to_string(),
                evidence,
            }
        })?;
        let execution = self.backend.execute_intent(&declaration)?;
        intent::admit_authoritative_intent_execution(&declaration, &execution).map_err(
            |denial| {
                let evidence =
                    ForgeQueryIntentDenialEvidence::new(&declaration, &denial, Some(&execution));
                ForgeQueryRuntimeError::IntentCommitDenied {
                    intent_name: declaration.name().to_string(),
                    stage: denial.stage(),
                    message: denial.message().to_string(),
                    evidence,
                }
            },
        )?;
        let summary = classify_receipt_mutation_summary(execution.mutation_receipt());
        let write_receipt = self.route_authoritative_mutation_receipt(
            execution.mutation_receipt().clone(),
            summary.0,
            summary.1,
            summary.2,
            None,
            None,
            None,
            None,
            None,
            Vec::new(),
            None,
            ForgeQueryMutationMetadata::default(),
        )?;
        Ok(ForgeQueryIntentReceipt::new(
            &declaration,
            execution,
            &write_receipt,
        ))
    }

    pub fn execute_next_effect_write_intent<T>(
        &mut self,
        effect: &ForgeQueryEffectHandle<T>,
        strategy_version: impl Into<String>,
        input_contract: impl Into<String>,
    ) -> Result<ForgeQueryEffectIntentReceipt, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Effect)?;
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Intent)?;
        let strategy_version = strategy_version.into();
        let input_contract = input_contract.into();
        let (pending_index, pending_delivery) = {
            let runtime = self
                .effects
                .get(effect.name())
                .ok_or_else(|| ForgeQueryRuntimeError::MissingEffect(effect.name().to_string()))?;
            runtime
                .deliveries
                .iter()
                .enumerate()
                .find(|(_, delivery)| {
                    delivery.family() == &ForgeQueryEffectDeliveryFamily::PendingWriteIntent
                })
                .map(|(index, delivery)| (index, delivery.clone()))
                .ok_or_else(|| {
                    ForgeQueryRuntimeError::MissingPendingWriteIntent(effect.name().to_string())
                })?
        };
        let declaration = ForgeQueryIntentDeclaration::strategy_commit(
            format!(
                "effect:{}:{}",
                pending_delivery.effect_name(),
                pending_delivery.commit_identity()
            ),
            pending_delivery.target().to_string(),
            strategy_version,
            input_contract,
            pending_delivery.payload().clone(),
        )
        .with_source_lane(ForgeQueryIntentSourceLane::EffectTriggered);
        intent::admit_effect_triggered_intent_declaration(&declaration).map_err(|denial| {
            let evidence = ForgeQueryIntentDenialEvidence::new(&declaration, &denial, None);
            ForgeQueryRuntimeError::IntentCommitDenied {
                intent_name: declaration.name().to_string(),
                stage: denial.stage(),
                message: denial.message().to_string(),
                evidence,
            }
        })?;
        let execution = self.backend.execute_intent(&declaration)?;
        intent::admit_authoritative_intent_execution(&declaration, &execution).map_err(
            |denial| {
                let evidence =
                    ForgeQueryIntentDenialEvidence::new(&declaration, &denial, Some(&execution));
                ForgeQueryRuntimeError::IntentCommitDenied {
                    intent_name: declaration.name().to_string(),
                    stage: denial.stage(),
                    message: denial.message().to_string(),
                    evidence,
                }
            },
        )?;
        let summary = classify_receipt_mutation_summary(execution.mutation_receipt());
        let write_receipt = self.route_authoritative_mutation_receipt(
            execution.mutation_receipt().clone(),
            summary.0,
            summary.1,
            summary.2,
            None,
            None,
            None,
            None,
            None,
            Vec::new(),
            None,
            ForgeQueryMutationMetadata::default(),
        )?;
        let intent_receipt = ForgeQueryIntentReceipt::new(&declaration, execution, &write_receipt);
        if let Some(runtime) = self.effects.get_mut(effect.name()) {
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
        Ok(ForgeQueryEffectIntentReceipt::new(
            &pending_delivery,
            intent_receipt,
        ))
    }

    pub(super) fn route_authoritative_mutation_receipt(
        &mut self,
        receipt: ForgeQueryMutationReceipt,
        mutation_family: ForgeQueryMutationFamily,
        declared_collection: Option<String>,
        declared_entity_identity: Option<String>,
        existing_truth_binding: Option<ForgeQueryExistingTruthTargetBinding>,
        existing_truth_assertion: Option<ForgeQueryVerifiedExistingTruthAssertion>,
        symbolic_target_reference: Option<ForgeQuerySymbolicTargetReference>,
        naming_intent: Option<ForgeQueryNamingMutationIntent>,
        continuity_intent: Option<ForgeQueryContinuityMutationIntent>,
        declared_aspect_operations: Vec<crate::runtime::ForgeQueryAspectMutationOperation>,
        declared_aspect_value_digest: Option<String>,
        mutation_metadata: ForgeQueryMutationMetadata,
    ) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
        let (_, mut target_collection, mut target_entity_identity) =
            classify_receipt_mutation_summary(&receipt);
        if target_collection.is_none() {
            target_collection = existing_truth_binding
                .as_ref()
                .and_then(|binding| binding.target_collection().map(str::to_string));
        }
        if target_entity_identity.is_none() {
            target_entity_identity = existing_truth_binding
                .as_ref()
                .map(|binding| binding.resolved_target_identity().to_string());
        }
        let summary = self.route_authoritative_mutation_summary(&receipt, &mutation_metadata)?;
        Ok(ForgeQueryWriteReceipt::from_mutation_receipt(
            receipt,
            mutation_family,
            declared_collection,
            declared_entity_identity,
            existing_truth_binding,
            existing_truth_assertion,
            symbolic_target_reference,
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
