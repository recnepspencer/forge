use super::*;

impl ForgeQueryRuntime {
    pub(super) fn verified_existing_assertion_for_command(
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

    pub(super) fn lower_backend_write_command(
        command: ForgeQueryWriteCommand,
    ) -> ForgeQueryWriteCommand {
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
        if let Some(reference) = command.symbolic_aspect_references().first() {
            return Err(ForgeQueryRuntimeError::MutationTargetReferenceDenied(
                ForgeQuerySymbolicTargetReferenceDenial::new(
                    reference.reference(),
                    ForgeQuerySymbolicTargetReferenceDenialKind::RequiresBatchContext,
                    "same-batch symbolic aspect references require batch execution",
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
        let symbolic_aspect_resolution_evidence = Vec::new();
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
            symbolic_aspect_resolution_evidence,
            naming_intent,
            continuity_intent,
            declared_aspect_operations,
            declared_aspect_value_digest,
            mutation_metadata,
        )
    }
}
