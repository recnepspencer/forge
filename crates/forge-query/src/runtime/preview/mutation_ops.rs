use super::*;
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
        let receipt = ForgeQueryWriteReceipt::preview(
            &self.label,
            self.pending_commands.len() + 1,
            &command,
            self.runtime.snapshot_token(),
        );
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
        entity_identity: impl Into<String>,
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
            entity_identity: entity_identity.into(),
        })
    }

    pub fn delete_with(
        &mut self,
        entity_identity: impl Into<String>,
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
            touched_aspect_paths: Vec::new(),
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
        let mut symbolic_targets = BTreeMap::<String, (String, Option<String>)>::new();
        let mut receipts = Vec::with_capacity(commands.len());
        for command in commands {
            deny_preview_assertion(&command)?;
            deny_preview_continuity(&command)?;
            let symbolic_target_reference = command.symbolic_target_reference().cloned();
            let receipt = match &command {
                ForgeQueryWriteCommand::UpdateSymbolicAspects {
                    reference,
                    aspects,
                    metadata,
                    naming_intent,
                    continuity_intent,
                } => {
                    let (resolved_entity_identity, resolved_collection) =
                        resolve_preview_symbolic_target(&symbolic_targets, reference)?;
                    let concrete = ForgeQueryWriteCommand::UpdateAspects {
                        entity_identity: resolved_entity_identity.clone(),
                        aspects: aspects.clone(),
                        metadata: metadata.clone(),
                        naming_intent: naming_intent.clone(),
                        continuity_intent: continuity_intent.clone(),
                    };
                    ForgeQueryWriteReceipt::preview(
                        &self.label,
                        self.pending_commands.len() + 1,
                        &concrete,
                        self.runtime.snapshot_token(),
                    )
                    .with_symbolic_target_reference(
                        reference,
                        resolved_entity_identity,
                        resolved_collection,
                    )
                }
                ForgeQueryWriteCommand::DeleteSymbolicAspects {
                    reference,
                    touched_aspect_paths,
                    metadata,
                    naming_intent,
                } => {
                    let (resolved_entity_identity, resolved_collection) =
                        resolve_preview_symbolic_target(&symbolic_targets, reference)?;
                    let concrete = ForgeQueryWriteCommand::DeleteAspects {
                        entity_identity: resolved_entity_identity.clone(),
                        declared_collection: resolved_collection.clone(),
                        touched_aspect_paths: touched_aspect_paths.clone(),
                        metadata: metadata.clone(),
                        naming_intent: naming_intent.clone(),
                    };
                    ForgeQueryWriteReceipt::preview(
                        &self.label,
                        self.pending_commands.len() + 1,
                        &concrete,
                        self.runtime.snapshot_token(),
                    )
                    .with_symbolic_target_reference(
                        reference,
                        resolved_entity_identity,
                        resolved_collection,
                    )
                }
                _ => {
                    admit_naming_intent(&command)
                        .map_err(ForgeQueryRuntimeError::MutationNamingDenied)?;
                    ForgeQueryWriteReceipt::preview(
                        &self.label,
                        self.pending_commands.len() + 1,
                        &command,
                        self.runtime.snapshot_token(),
                    )
                }
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
    }
}

fn record_preview_symbolic_target(
    symbolic_targets: &mut BTreeMap<String, (String, Option<String>)>,
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
        reference.symbol().to_string(),
        (
            target_entity_identity.to_string(),
            receipt.target_collection().map(str::to_string),
        ),
    );
}

fn resolve_preview_symbolic_target(
    symbolic_targets: &BTreeMap<String, (String, Option<String>)>,
    reference: &ForgeQuerySymbolicTargetReference,
) -> Result<(String, Option<String>), ForgeQueryRuntimeError> {
    let Some((resolved_entity_identity, resolved_collection)) =
        symbolic_targets.get(reference.symbol())
    else {
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
    if let Some(expected_collection) = reference.target_collection() {
        if resolved_collection.as_deref() != Some(expected_collection) {
            return Err(ForgeQueryRuntimeError::MutationTargetReferenceDenied(
                ForgeQuerySymbolicTargetReferenceDenial::new(
                    reference,
                    ForgeQuerySymbolicTargetReferenceDenialKind::CollectionMismatch,
                    format!(
                        "same-batch symbolic target `{}` resolved to collection `{}`, not `{expected_collection}`",
                        reference.symbol(),
                        resolved_collection.as_deref().unwrap_or("unknown"),
                    ),
                ),
            ));
        }
    }
    Ok((
        resolved_entity_identity.clone(),
        resolved_collection.clone(),
    ))
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
        return Err(ForgeQueryRuntimeError::UnsupportedAuthority(
            "existing-truth assertion currently requires the authoritative lane".to_string(),
        ));
    }
    Ok(())
}
