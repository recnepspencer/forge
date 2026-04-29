use super::*;

impl ForgeQueryRuntime {
    pub fn write(
        &mut self,
        command: ForgeQueryWriteCommand,
    ) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Write)?;
        if let Some(reference) = command.symbolic_target_reference() {
            return Err(ForgeQueryRuntimeError::MutationTargetReferenceDenied(
                ForgeQuerySymbolicTargetReferenceDenial::new(
                    reference,
                    ForgeQuerySymbolicTargetReferenceDenialKind::RequiresBatchContext,
                    "same-batch symbolic target references require batch execution",
                ),
            ));
        }
        if let Some(binding) = command.existing_truth_binding() {
            self.backend
                .admit_existing_truth_binding(binding)
                .map_err(ForgeQueryRuntimeError::MutationBindingDenied)?;
        }
        admit_continuity_intent(&command)
            .map_err(ForgeQueryRuntimeError::MutationContinuityDenied)?;
        admit_naming_intent(&command).map_err(ForgeQueryRuntimeError::MutationNamingDenied)?;
        let mutation_family = command.mutation_family();
        let declared_collection = command.declared_collection();
        let declared_entity_identity = command.declared_entity_identity();
        let existing_truth_binding = command.existing_truth_binding().cloned();
        let symbolic_target_reference = None;
        let naming_intent = command.naming_intent().cloned();
        let continuity_intent = command.continuity_intent().cloned();
        let declared_aspect_operations = command.declared_aspect_operations();
        let mutation_metadata = command.mutation_metadata();
        let mut receipt = self.backend.write(command)?;
        if let Some(intent) = continuity_intent.as_ref() {
            let (_, target_collection, target_entity_identity) =
                classify_receipt_mutation_summary(&receipt);
            let basis_binding_digest = existing_truth_binding
                .as_ref()
                .map(|binding| binding.binding_digest());
            if let Some(bundle) = bridge_continuity_mutation_bundle(
                intent,
                basis_binding_digest.as_deref(),
                target_entity_identity.as_deref(),
                target_collection.as_deref(),
            ) {
                receipt = attach_continuity_mutation_to_receipt(receipt, bundle);
            }
        }
        if let Some(intent) = naming_intent.as_ref() {
            let (_, target_collection, target_entity_identity) =
                classify_receipt_mutation_summary(&receipt);
            if let Some(bundle) = bridge_naming_mutation_bundle(
                intent,
                target_entity_identity.as_deref(),
                target_collection.as_deref(),
            ) {
                receipt = attach_naming_mutation_to_receipt(receipt, bundle);
            }
        }
        self.route_authoritative_mutation_receipt(
            receipt,
            mutation_family,
            declared_collection,
            declared_entity_identity,
            existing_truth_binding,
            symbolic_target_reference,
            naming_intent,
            continuity_intent,
            declared_aspect_operations,
            mutation_metadata,
        )
    }

    pub fn write_batch(
        &mut self,
        commands: Vec<ForgeQueryWriteCommand>,
    ) -> Result<ForgeQueryBatchWriteReceipt, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Write)?;
        if commands.is_empty() {
            return Err(ForgeQueryRuntimeError::Workspace(
                ForgeQueryWorkspaceError::new("mutation batch must declare at least one operation"),
            ));
        }
        let mut receipts = Vec::with_capacity(commands.len());
        let mut command_summaries = Vec::with_capacity(commands.len());
        let mut symbolic_targets = BTreeMap::<String, (String, Option<String>)>::new();
        for command in commands {
            if let Some(binding) = command.existing_truth_binding() {
                self.backend
                    .admit_existing_truth_binding(binding)
                    .map_err(ForgeQueryRuntimeError::MutationBindingDenied)?;
            }
            admit_continuity_intent(&command)
                .map_err(ForgeQueryRuntimeError::MutationContinuityDenied)?;
            admit_naming_intent(&command).map_err(ForgeQueryRuntimeError::MutationNamingDenied)?;
            let mutation_family = command.mutation_family();
            let declared_collection = command.declared_collection();
            let declared_entity_identity = command.declared_entity_identity();
            let existing_truth_binding = command.existing_truth_binding().cloned();
            let declared_aspect_operations = command.declared_aspect_operations();
            let mutation_metadata = command.mutation_metadata();
            let symbolic_target_reference = command.symbolic_target_reference().cloned();
            let naming_intent = command.naming_intent().cloned();
            let continuity_intent = command.continuity_intent().cloned();
            let mut receipt = match command {
                ForgeQueryWriteCommand::UpdateSymbolicAspects {
                    reference,
                    aspects,
                    metadata,
                    naming_intent,
                    continuity_intent,
                } => {
                    let (resolved_entity_identity, resolved_collection) =
                        resolve_same_batch_symbolic_target(&symbolic_targets, &reference)?;
                    attach_symbolic_target_reference_to_receipt(
                        self.backend.write(ForgeQueryWriteCommand::UpdateAspects {
                            entity_identity: resolved_entity_identity.clone(),
                            aspects,
                            metadata,
                            naming_intent,
                            continuity_intent,
                        })?,
                        BridgeSymbolicTargetReferenceBundle::same_batch_target(
                            reference.symbol(),
                            resolved_entity_identity,
                            resolved_collection.as_deref(),
                        ),
                    )
                }
                ForgeQueryWriteCommand::DeleteSymbolicAspects {
                    reference,
                    touched_aspect_paths,
                    metadata,
                    naming_intent,
                } => {
                    let (resolved_entity_identity, resolved_collection) =
                        resolve_same_batch_symbolic_target(&symbolic_targets, &reference)?;
                    attach_symbolic_target_reference_to_receipt(
                        self.backend.write(ForgeQueryWriteCommand::DeleteAspects {
                            entity_identity: resolved_entity_identity.clone(),
                            declared_collection: resolved_collection.clone(),
                            touched_aspect_paths,
                            metadata,
                            naming_intent,
                        })?,
                        BridgeSymbolicTargetReferenceBundle::same_batch_target(
                            reference.symbol(),
                            resolved_entity_identity,
                            resolved_collection.as_deref(),
                        ),
                    )
                }
                other => self.backend.write(other)?,
            };
            if let Some(intent) = continuity_intent.as_ref() {
                let (_, target_collection, target_entity_identity) =
                    classify_receipt_mutation_summary(&receipt);
                let basis_binding_digest = existing_truth_binding
                    .as_ref()
                    .map(|binding| binding.binding_digest());
                if let Some(bundle) = bridge_continuity_mutation_bundle(
                    intent,
                    basis_binding_digest.as_deref(),
                    target_entity_identity.as_deref(),
                    target_collection.as_deref(),
                ) {
                    receipt = attach_continuity_mutation_to_receipt(receipt, bundle);
                }
            }
            if let Some(intent) = naming_intent.as_ref() {
                let (_, target_collection, target_entity_identity) =
                    classify_receipt_mutation_summary(&receipt);
                if let Some(bundle) = bridge_naming_mutation_bundle(
                    intent,
                    target_entity_identity.as_deref(),
                    target_collection.as_deref(),
                ) {
                    receipt = attach_naming_mutation_to_receipt(receipt, bundle);
                }
            }
            record_same_batch_symbolic_target(
                &mut symbolic_targets,
                symbolic_target_reference.as_ref(),
                &receipt,
            );
            command_summaries.push((
                mutation_family,
                declared_collection,
                declared_entity_identity,
                existing_truth_binding,
                symbolic_target_reference,
                naming_intent,
                continuity_intent,
                declared_aspect_operations,
                mutation_metadata,
            ));
            receipts.push(receipt);
        }
        let combined_receipt = combined_batch_mutation_receipt(&receipts)?;
        let summary = self.route_authoritative_mutation_summary(
            &combined_receipt,
            &ForgeQueryMutationMetadata::default(),
        )?;
        let write_receipts = receipts
            .into_iter()
            .zip(command_summaries)
            .map(|(receipt, summary)| {
                let affected_live_view_ids = self.backend.affected_live_view_ids(&receipt);
                let (_, target_collection, target_entity_identity) =
                    classify_receipt_mutation_summary(&receipt);
                ForgeQueryWriteReceipt::batch_component(
                    receipt,
                    summary.0,
                    ForgeQueryAuthorityLane::AuthoritativeTruth,
                    summary.1,
                    summary.2,
                    summary.3,
                    summary.4,
                    summary.5,
                    summary.6,
                    target_collection,
                    target_entity_identity,
                    summary.7,
                    summary.8,
                    affected_live_view_ids,
                    ForgeQueryAuthorityLane::AuthoritativeTruth,
                )
            })
            .collect::<Vec<_>>();
        let mut touched_aspect_paths = combined_receipt
            .deltas
            .iter()
            .flat_map(|delta| delta.aspect_paths.iter().cloned())
            .collect::<Vec<_>>();
        touched_aspect_paths.sort();
        touched_aspect_paths.dedup();
        ForgeQueryBatchWriteReceipt::new(
            write_receipts,
            ForgeQueryAuthorityLane::AuthoritativeTruth,
            ForgeQueryAuthorityLane::AuthoritativeTruth,
            touched_aspect_paths,
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
        )
    }
}
