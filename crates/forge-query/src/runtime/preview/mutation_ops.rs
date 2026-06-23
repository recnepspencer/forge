use super::*;
use crate::memory_workspace::ForgeQueryEntityIdentity;
use crate::runtime::mutation::admit_naming_intent;
use crate::runtime::{ForgeQueryExistingEntityTarget, ForgeQueryExistingRelationTarget};

impl<'a> ForgeQueryPreviewSession<'a> {
    pub fn bind_existing_entity(
        &self,
        target: ForgeQueryExistingEntityTarget,
    ) -> Result<ForgeQueryExistingTruthTargetBinding, ForgeQueryRuntimeError> {
        Ok(ForgeQueryExistingTruthTargetBinding::from_entity_target(
            target,
        )?)
    }

    pub fn bind_existing_relation(
        &self,
        target: ForgeQueryExistingRelationTarget,
    ) -> Result<ForgeQueryExistingTruthTargetBinding, ForgeQueryRuntimeError> {
        Ok(ForgeQueryExistingTruthTargetBinding::from_relation_target(
            target,
        )?)
    }

    pub fn write(
        &mut self,
        command: ForgeQueryWriteCommand,
    ) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
        if let Some(reference) = command.symbolic_target_reference() {
            return Err(ForgeQueryRuntimeError::MutationTargetReferenceDenied(
                ForgeQuerySymbolicTargetReferenceDenial::new(
                    reference,
                    ForgeQuerySymbolicTargetReferenceDenialKind::RequiresBatchContext,
                    "same-batch symbolic target references require batch execution",
                ),
            ));
        }
        deny_preview_assertion(&command)?;
        deny_preview_continuity(&command)?;
        admit_naming_intent(&command).map_err(ForgeQueryRuntimeError::MutationNamingDenied)?;
        let obligation_dispatch = self
            .runtime
            .preview_mutation_obligation_dispatch(&command)?;
        self.runtime.backend.admit_preview_write_command(&command)?;
        let receipt = ForgeQueryWriteReceipt::preview(
            &self.label,
            self.pending_commands.len() + 1,
            &command,
            self.runtime.current_snapshot_identity(),
        )
        .with_obligation_dispatch(obligation_dispatch);
        self.pending_commands.push(command);
        self.writes.push(receipt.clone());
        self.route_preview_execution(&receipt);
        Ok(receipt)
    }

    pub fn insert(
        &mut self,
        collection: impl Into<String>,
        declaration: impl FnOnce(ForgeQueryAspectMutationBuilder) -> ForgeQueryAspectMutationBuilder,
    ) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
        let command =
            declaration(ForgeQueryAspectMutationBuilder::new()).build_insert(collection)?;
        self.write(command)
    }

    pub fn update(
        &mut self,
        entity_identity: ForgeQueryEntityIdentity,
        declaration: impl FnOnce(ForgeQueryAspectMutationBuilder) -> ForgeQueryAspectMutationBuilder,
    ) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
        let command =
            declaration(ForgeQueryAspectMutationBuilder::new()).build_update(entity_identity)?;
        self.write(command)
    }

    pub fn update_existing(
        &mut self,
        binding: ForgeQueryExistingTruthTargetBinding,
        declaration: impl FnOnce(ForgeQueryAspectMutationBuilder) -> ForgeQueryAspectMutationBuilder,
    ) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
        let command =
            declaration(ForgeQueryAspectMutationBuilder::new()).build_update_existing(binding)?;
        self.write(command)
    }

    pub fn assert_existing(
        &mut self,
        binding: ForgeQueryExistingTruthTargetBinding,
        declaration: impl FnOnce(ForgeQueryAspectMutationBuilder) -> ForgeQueryAspectMutationBuilder,
    ) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
        let command =
            declaration(ForgeQueryAspectMutationBuilder::new()).build_assert_existing(binding)?;
        self.write(command)
    }

    pub fn verify_existing(
        &mut self,
        binding: ForgeQueryExistingTruthTargetBinding,
        declaration: impl FnOnce(ForgeQueryAspectMutationBuilder) -> ForgeQueryAspectMutationBuilder,
    ) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
        let command =
            declaration(ForgeQueryAspectMutationBuilder::new()).build_verify_existing(binding)?;
        self.write(command)
    }

    pub fn update_existing_verified(
        &mut self,
        binding: ForgeQueryExistingTruthTargetBinding,
        verify: impl FnOnce(ForgeQueryAspectMutationBuilder) -> ForgeQueryAspectMutationBuilder,
        update: impl FnOnce(ForgeQueryAspectMutationBuilder) -> ForgeQueryAspectMutationBuilder,
    ) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
        let asserted_aspects = verify(ForgeQueryAspectMutationBuilder::new())
            .finish_existing_truth_verification_aspects("backend-verified existing-truth update")?;
        let command = update(ForgeQueryAspectMutationBuilder::new())
            .build_update_existing_verified(binding, asserted_aspects)?;
        self.write(command)
    }

    pub fn delete(
        &mut self,
        entity_identity: impl Into<String>,
    ) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
        self.write(ForgeQueryWriteCommand::Delete {
            entity_identity: crate::memory_workspace::admit_authored_entity_label(
                entity_identity.into(),
            ),
        })
    }

    pub fn delete_with(
        &mut self,
        entity_identity: ForgeQueryEntityIdentity,
        declaration: impl FnOnce(ForgeQueryDeleteMutationBuilder) -> ForgeQueryDeleteMutationBuilder,
    ) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
        let command =
            declaration(ForgeQueryDeleteMutationBuilder::new()).build_delete(entity_identity)?;
        self.write(command)
    }

    pub fn delete_existing(
        &mut self,
        binding: ForgeQueryExistingTruthTargetBinding,
    ) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
        self.write(ForgeQueryWriteCommand::DeleteExistingAspects {
            binding,
            touched_aspects: Vec::new(),
            metadata: ForgeQueryMutationMetadata::default(),
            naming_intent: None,
        })
    }

    pub fn delete_existing_with(
        &mut self,
        binding: ForgeQueryExistingTruthTargetBinding,
        declaration: impl FnOnce(ForgeQueryDeleteMutationBuilder) -> ForgeQueryDeleteMutationBuilder,
    ) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
        let command =
            declaration(ForgeQueryDeleteMutationBuilder::new()).build_delete_existing(binding)?;
        self.write(command)
    }

    pub fn delete_existing_verified(
        &mut self,
        binding: ForgeQueryExistingTruthTargetBinding,
        verify: impl FnOnce(ForgeQueryAspectMutationBuilder) -> ForgeQueryAspectMutationBuilder,
        delete: impl FnOnce(ForgeQueryDeleteMutationBuilder) -> ForgeQueryDeleteMutationBuilder,
    ) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
        let asserted_aspects = verify(ForgeQueryAspectMutationBuilder::new())
            .finish_existing_truth_verification_aspects("backend-verified existing-truth delete")?;
        let command = delete(ForgeQueryDeleteMutationBuilder::new())
            .build_delete_existing_verified(binding, asserted_aspects)?;
        self.write(command)
    }

    pub fn batch(
        &mut self,
        declaration: impl FnOnce(ForgeQueryMutationBatchBuilder) -> ForgeQueryMutationBatchBuilder,
    ) -> Result<ForgeQueryBatchWriteReceipt, ForgeQueryRuntimeError> {
        let commands = declaration(ForgeQueryMutationBatchBuilder::new()).finish()?;
        for command in &commands {
            deny_preview_assertion(command)?;
            deny_preview_continuity(command)?;
            if command.symbolic_target_reference().is_none() {
                admit_naming_intent(command)
                    .map_err(ForgeQueryRuntimeError::MutationNamingDenied)?;
            }
        }
        admit_preview_batch_symbolic_references(&commands)?;
        let obligation_dispatch = self
            .runtime
            .preview_mutation_batch_obligation_dispatch(&commands)?;
        for command in &commands {
            self.runtime.backend.admit_preview_write_command(command)?;
        }
        let mut symbolic_targets = BTreeMap::<
            ForgeQuerySameBatchSymbolicTargetKey,
            ForgeQuerySameBatchSymbolicTarget,
        >::new();
        let mut receipts = Vec::with_capacity(commands.len());
        for command in commands {
            let symbolic_target_reference = command.symbolic_target_reference().cloned();
            let receipt = match &command {
                ForgeQueryWriteCommand::UpdateSymbolicAspects {
                    reference,
                    aspects,
                    metadata,
                    naming_intent,
                    continuity_intent,
                } => {
                    let resolved_target =
                        resolve_preview_symbolic_target(&symbolic_targets, reference)?;
                    let concrete = ForgeQueryWriteCommand::UpdateAspects {
                        entity_identity: resolved_target.entity_identity().clone(),
                        aspects: aspects.clone(),
                        metadata: metadata.clone(),
                        naming_intent: naming_intent.clone(),
                        continuity_intent: continuity_intent.clone(),
                    };
                    ForgeQueryWriteReceipt::preview(
                        &self.label,
                        self.pending_commands.len() + 1,
                        &concrete,
                        self.runtime.current_snapshot_identity(),
                    )
                    .with_symbolic_target_reference(
                        reference,
                        resolved_target.entity_identity().clone(),
                        resolved_target.target_collection_identity().cloned(),
                    )
                }
                ForgeQueryWriteCommand::DeleteSymbolicAspects {
                    reference,
                    touched_aspects,
                    metadata,
                    naming_intent,
                } => {
                    let resolved_target =
                        resolve_preview_symbolic_target(&symbolic_targets, reference)?;
                    let concrete = ForgeQueryWriteCommand::DeleteAspects {
                        entity_identity: resolved_target.entity_identity().clone(),
                        declared_collection: resolved_target.target_collection_identity().cloned(),
                        touched_aspects: touched_aspects.clone(),
                        metadata: metadata.clone(),
                        naming_intent: naming_intent.clone(),
                    };
                    ForgeQueryWriteReceipt::preview(
                        &self.label,
                        self.pending_commands.len() + 1,
                        &concrete,
                        self.runtime.current_snapshot_identity(),
                    )
                    .with_symbolic_target_reference(
                        reference,
                        resolved_target.entity_identity().clone(),
                        resolved_target.target_collection_identity().cloned(),
                    )
                }
                _ => ForgeQueryWriteReceipt::preview(
                    &self.label,
                    self.pending_commands.len() + 1,
                    &command,
                    self.runtime.current_snapshot_identity(),
                ),
            };
            if let Some(reference) = symbolic_target_reference.as_ref() {
                record_preview_symbolic_target(&mut symbolic_targets, reference, &receipt);
            }
            self.pending_commands.push(command);
            self.writes.push(receipt.clone());
            self.route_preview_execution(&receipt);
            receipts.push(receipt);
        }
        ForgeQueryBatchWriteReceipt::from_write_receipts(receipts)
            .map(|receipt| receipt.with_obligation_dispatch(obligation_dispatch))
    }
}

fn admit_preview_batch_symbolic_references(
    commands: &[ForgeQueryWriteCommand],
) -> Result<(), ForgeQueryRuntimeError> {
    let mut planned_symbolic_targets =
        BTreeMap::<ForgeQuerySameBatchSymbolicTargetKey, ForgeQuerySameBatchSymbolicTarget>::new();
    for command in commands {
        if let Some(reference) = command.symbolic_target_reference() {
            if command.mutation_family() != crate::runtime::ForgeQueryMutationFamily::Insert {
                resolve_preview_symbolic_target(&planned_symbolic_targets, reference)?;
            }
        }
        for reference in command.symbolic_aspect_references() {
            resolve_preview_symbolic_target(&planned_symbolic_targets, reference.reference())?;
        }
        record_planned_preview_symbolic_target(&mut planned_symbolic_targets, command);
    }
    Ok(())
}

fn record_planned_preview_symbolic_target(
    planned_symbolic_targets: &mut BTreeMap<
        ForgeQuerySameBatchSymbolicTargetKey,
        ForgeQuerySameBatchSymbolicTarget,
    >,
    command: &ForgeQueryWriteCommand,
) {
    if command.mutation_family() != crate::runtime::ForgeQueryMutationFamily::Insert {
        return;
    }
    let Some(reference) = command.symbolic_target_reference() else {
        return;
    };
    let planned_identity = crate::memory_workspace::admit_authored_entity_label(format!(
        "planned-preview-symbolic:{}",
        reference.symbol()
    ));
    planned_symbolic_targets.insert(
        ForgeQuerySameBatchSymbolicTargetKey::from_reference(reference),
        ForgeQuerySameBatchSymbolicTarget::new(
            planned_identity,
            command.declared_collection_identity(),
        ),
    );
}

fn record_preview_symbolic_target(
    symbolic_targets: &mut BTreeMap<
        ForgeQuerySameBatchSymbolicTargetKey,
        ForgeQuerySameBatchSymbolicTarget,
    >,
    reference: &ForgeQuerySymbolicTargetReference,
    receipt: &ForgeQueryWriteReceipt,
) {
    if receipt.mutation_family() != crate::runtime::ForgeQueryMutationFamily::Insert {
        return;
    }
    let Some(target_entity_identity) = receipt.target_entity_identity() else {
        return;
    };
    symbolic_targets.insert(
        ForgeQuerySameBatchSymbolicTargetKey::from_reference(reference),
        ForgeQuerySameBatchSymbolicTarget::new(
            target_entity_identity.clone(),
            receipt.target_collection_identity().cloned(),
        ),
    );
}

fn resolve_preview_symbolic_target(
    symbolic_targets: &BTreeMap<
        ForgeQuerySameBatchSymbolicTargetKey,
        ForgeQuerySameBatchSymbolicTarget,
    >,
    reference: &ForgeQuerySymbolicTargetReference,
) -> Result<ForgeQuerySameBatchSymbolicTarget, ForgeQueryRuntimeError> {
    let target_key = ForgeQuerySameBatchSymbolicTargetKey::from_reference(reference);
    let Some(resolved_target) = symbolic_targets.get(&target_key) else {
        return Err(ForgeQueryRuntimeError::MutationTargetReferenceDenied(
            ForgeQuerySymbolicTargetReferenceDenial::new(
                reference,
                ForgeQuerySymbolicTargetReferenceDenialKind::UnresolvedSameBatchTarget,
                format!(
                    "same-batch symbolic target `{}` was not declared earlier in this preview batch",
                    reference.symbol()
                ),
            ),
        ));
    };
    if let Some(expected_collection) = reference.target_collection_identity() {
        if resolved_target
            .target_collection_identity()
            .is_none_or(|resolved_collection| {
                !resolved_collection.same_target_collection_as(expected_collection)
            })
        {
            return Err(ForgeQueryRuntimeError::MutationTargetReferenceDenied(
                ForgeQuerySymbolicTargetReferenceDenial::new(
                    reference,
                    ForgeQuerySymbolicTargetReferenceDenialKind::CollectionMismatch,
                    format!(
                        "same-batch symbolic target `{}` resolved to collection `{}`, not `{expected_collection}`",
                        reference.symbol(),
                        resolved_target
                            .target_collection_identity()
                            .map(|collection| collection.as_str())
                            .unwrap_or("unknown"),
                        expected_collection = expected_collection.as_str(),
                    ),
                ),
            ));
        }
    }
    Ok(resolved_target.clone())
}

fn deny_preview_continuity(command: &ForgeQueryWriteCommand) -> Result<(), ForgeQueryRuntimeError> {
    let Some(intent) = command.continuity_intent() else {
        return Ok(());
    };
    let denial = crate::runtime::ForgeQueryContinuityMutationDenial::new(
        intent,
        command.existing_truth_binding(),
        crate::runtime::ForgeQueryContinuityMutationDenialKind::RequiresAuthoritativeLane,
        "continuity-aware mutation currently requires the authoritative bridge-backed lane",
    );
    Err(ForgeQueryRuntimeError::MutationContinuityDenied(denial))
}

fn deny_preview_assertion(command: &ForgeQueryWriteCommand) -> Result<(), ForgeQueryRuntimeError> {
    if matches!(
        command,
        ForgeQueryWriteCommand::AssertExistingAspects { .. }
            | ForgeQueryWriteCommand::VerifyThenUpdateExistingAspects { .. }
            | ForgeQueryWriteCommand::VerifyThenDeleteExistingAspects { .. }
            | ForgeQueryWriteCommand::VerifyExistingAspects { .. }
    ) {
        return Err(
            ForgeQueryRuntimeError::ExistingTruthAssertionRequiresAuthorityLane {
                required_lane: crate::runtime::ForgeQueryAuthorityLane::AuthoritativeTruth,
            },
        );
    }
    Ok(())
}
