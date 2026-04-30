use super::*;

impl ForgeQueryRuntime {
    fn verified_existing_assertion_for_command(
        &self,
        command: &ForgeQueryWriteCommand,
    ) -> Result<Option<ForgeQueryVerifiedExistingTruthAssertion>, ForgeQueryRuntimeError> {
        match command {
            ForgeQueryWriteCommand::VerifyExistingAspects {
                binding, aspects, ..
            }
            | ForgeQueryWriteCommand::VerifyThenUpdateExistingAspects {
                binding,
                asserted_aspects: aspects,
                ..
            }
            | ForgeQueryWriteCommand::VerifyThenDeleteExistingAspects {
                binding,
                asserted_aspects: aspects,
                ..
            } => Ok(Some(
                self.backend
                    .verify_existing_truth_assertion(binding, aspects)
                    .map_err(ForgeQueryRuntimeError::ExistingTruthAssertionDenied)?,
            )),
            _ => Ok(None),
        }
    }

    fn lower_backend_write_command(command: ForgeQueryWriteCommand) -> ForgeQueryWriteCommand {
        match command {
            ForgeQueryWriteCommand::VerifyThenUpdateExistingAspects {
                binding,
                aspects,
                metadata,
                naming_intent,
                continuity_intent,
                ..
            } => ForgeQueryWriteCommand::UpdateExistingAspects {
                binding,
                aspects,
                metadata,
                naming_intent,
                continuity_intent,
            },
            ForgeQueryWriteCommand::VerifyThenDeleteExistingAspects {
                binding,
                touched_aspect_paths,
                metadata,
                naming_intent,
                ..
            } => ForgeQueryWriteCommand::DeleteExistingAspects {
                binding,
                touched_aspect_paths,
                metadata,
                naming_intent,
            },
            other => other,
        }
    }

    pub fn probe_existing(
        &self,
        request: ForgeQueryExistingTruthProbeRequest,
    ) -> Result<ForgeQueryExistingTruthProbe, ForgeQueryRuntimeError> {
        self.backend
            .admit_existing_truth_binding(request.binding())
            .map_err(ForgeQueryRuntimeError::MutationBindingDenied)?;
        self.backend
            .probe_existing_truth(&request)
            .map_err(ForgeQueryRuntimeError::ExistingTruthProbeDenied)
    }

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
        let verified_existing_truth_assertion =
            self.verified_existing_assertion_for_command(&command)?;
        let symbolic_target_reference = None;
        let naming_intent = command.naming_intent().cloned();
        let continuity_intent = command.continuity_intent().cloned();
        let declared_aspect_operations = command.declared_aspect_operations();
        let declared_aspect_value_digest = command_declared_aspect_value_digest(&command);
        let mutation_metadata = command.mutation_metadata();
        let mut receipt = match &command {
            ForgeQueryWriteCommand::AssertExistingAspects { binding, .. } => {
                synthetic_existing_assertion_receipt(
                    binding,
                    &self.backend.snapshot_token(),
                    declared_aspect_value_digest.as_deref(),
                )
            }
            ForgeQueryWriteCommand::VerifyExistingAspects { binding, .. } => {
                synthetic_existing_assertion_receipt(
                    binding,
                    &self.backend.snapshot_token(),
                    declared_aspect_value_digest.as_deref(),
                )
            }
            _ => self
                .backend
                .write(Self::lower_backend_write_command(command))?,
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
        self.route_authoritative_mutation_receipt(
            receipt,
            mutation_family,
            declared_collection,
            declared_entity_identity,
            existing_truth_binding,
            verified_existing_truth_assertion,
            symbolic_target_reference,
            naming_intent,
            continuity_intent,
            declared_aspect_operations,
            declared_aspect_value_digest,
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
            let verified_existing_truth_assertion =
                self.verified_existing_assertion_for_command(&command)?;
            let declared_aspect_operations = command.declared_aspect_operations();
            let declared_aspect_value_digest = command_declared_aspect_value_digest(&command);
            let mutation_metadata = command.mutation_metadata();
            let symbolic_target_reference = command.symbolic_target_reference().cloned();
            let naming_intent = command.naming_intent().cloned();
            let continuity_intent = command.continuity_intent().cloned();
            let mut receipt = match command {
                ForgeQueryWriteCommand::AssertExistingAspects { binding, .. } => {
                    synthetic_existing_assertion_receipt(
                        &binding,
                        &self.backend.snapshot_token(),
                        declared_aspect_value_digest.as_deref(),
                    )
                }
                ForgeQueryWriteCommand::VerifyExistingAspects { binding, .. } => {
                    synthetic_existing_assertion_receipt(
                        &binding,
                        &self.backend.snapshot_token(),
                        declared_aspect_value_digest.as_deref(),
                    )
                }
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
                other => self
                    .backend
                    .write(Self::lower_backend_write_command(other))?,
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
                verified_existing_truth_assertion,
                symbolic_target_reference,
                naming_intent,
                continuity_intent,
                declared_aspect_operations,
                declared_aspect_value_digest,
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
                let (
                    mutation_family,
                    declared_collection,
                    declared_entity_identity,
                    existing_truth_binding,
                    verified_existing_truth_assertion,
                    symbolic_target_reference,
                    naming_intent,
                    continuity_intent,
                    declared_aspect_operations,
                    declared_aspect_value_digest,
                    mutation_metadata,
                ) = summary;
                let affected_live_view_ids = self.backend.affected_live_view_ids(&receipt);
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
                ForgeQueryWriteReceipt::batch_component(
                    receipt,
                    mutation_family,
                    ForgeQueryAuthorityLane::AuthoritativeTruth,
                    declared_collection,
                    declared_entity_identity,
                    existing_truth_binding,
                    verified_existing_truth_assertion,
                    symbolic_target_reference,
                    naming_intent,
                    continuity_intent,
                    target_collection,
                    target_entity_identity,
                    declared_aspect_operations,
                    declared_aspect_value_digest,
                    mutation_metadata,
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
