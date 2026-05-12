use super::runtime_batching::{should_use_backend_atomic_batch, BatchCommandSummary};
use super::*;

#[path = "runtime_batch_write_symbolics.rs"]
mod runtime_batch_write_symbolics;

use runtime_batch_write_symbolics::{
    admit_atomic_batch_symbolic_references, record_planned_same_batch_symbolic_target,
    symbolic_aspect_resolution_evidence_for_command,
    symbolic_aspect_resolution_evidence_for_references,
};

impl ForgeQueryRuntime {
    pub fn write_batch(
        &mut self,
        commands: Vec<ForgeQueryWriteCommand>,
    ) -> Result<ForgeQueryBatchWriteReceipt, ForgeQueryRuntimeError> {
        self.write_batch_with_graph_artifacts(
            commands,
            ForgeQueryGraphCompositionBreadth::empty(),
            ForgeQueryGraphCompositionProgram::empty(),
        )
    }

    pub(crate) fn write_graph_batch(
        &mut self,
        commands: Vec<ForgeQueryWriteCommand>,
        graph_composition_breadth: ForgeQueryGraphCompositionBreadth,
        graph_composition_program: ForgeQueryGraphCompositionProgram,
    ) -> Result<ForgeQueryBatchWriteReceipt, ForgeQueryRuntimeError> {
        self.write_batch_with_graph_artifacts(
            commands,
            graph_composition_breadth,
            graph_composition_program,
        )
    }

    fn write_batch_with_graph_artifacts(
        &mut self,
        commands: Vec<ForgeQueryWriteCommand>,
        graph_composition_breadth: ForgeQueryGraphCompositionBreadth,
        graph_composition_program: ForgeQueryGraphCompositionProgram,
    ) -> Result<ForgeQueryBatchWriteReceipt, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Write)?;
        if commands.is_empty() {
            return Err(ForgeQueryRuntimeError::Workspace(
                ForgeQueryWorkspaceError::new("mutation batch must declare at least one operation"),
            ));
        }
        let use_backend_atomic_batch =
            should_use_backend_atomic_batch(&self.backend.support_profile(), &commands);
        let mut command_summaries: Vec<BatchCommandSummary> = Vec::with_capacity(commands.len());
        let mut resolved_receipts = Vec::with_capacity(commands.len());
        let mut symbolic_targets = BTreeMap::<String, (String, Option<String>)>::new();
        let mut planned_symbolic_targets = BTreeMap::<String, (String, Option<String>)>::new();
        let mut deferred_backend_commands = Vec::new();
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
            let symbolic_aspect_resolution_evidence = if use_backend_atomic_batch {
                admit_atomic_batch_symbolic_references(&planned_symbolic_targets, &command)?;
                Vec::new()
            } else {
                symbolic_aspect_resolution_evidence_for_command(&symbolic_targets, &command)?
            };
            let summary = BatchCommandSummary::new(
                mutation_family,
                declared_collection,
                declared_entity_identity,
                existing_truth_binding,
                verified_existing_truth_assertion,
                symbolic_target_reference,
                naming_intent,
                continuity_intent,
                declared_aspect_operations,
                declared_aspect_value_digest.clone(),
                command.symbolic_aspect_references().to_vec(),
                symbolic_aspect_resolution_evidence,
                mutation_metadata,
            );
            if use_backend_atomic_batch
                && !matches!(
                    command,
                    ForgeQueryWriteCommand::AssertExistingAspects { .. }
                        | ForgeQueryWriteCommand::VerifyExistingAspects { .. }
                )
            {
                record_planned_same_batch_symbolic_target(&mut planned_symbolic_targets, &command);
                deferred_backend_commands.push(command);
                command_summaries.push(summary);
                resolved_receipts.push(None);
                continue;
            }
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
                ForgeQueryWriteCommand::InsertAspects {
                    collection,
                    aspects,
                    symbolic_aspect_references,
                    metadata,
                    naming_intent,
                    continuity_intent,
                    symbolic_target_reference,
                } => self.backend.write(ForgeQueryWriteCommand::InsertAspects {
                    collection,
                    aspects: resolve_symbolic_aspect_references(
                        &symbolic_targets,
                        aspects,
                        &symbolic_aspect_references,
                    )?,
                    symbolic_aspect_references: Vec::new(),
                    metadata,
                    naming_intent,
                    continuity_intent,
                    symbolic_target_reference,
                })?,
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
                ForgeQueryWriteCommand::VerifyThenUpdateExistingAspects {
                    binding,
                    asserted_aspects,
                    aspects,
                    symbolic_aspect_references,
                    metadata,
                    naming_intent,
                    continuity_intent,
                } => self.backend.write(Self::lower_backend_write_command(
                    ForgeQueryWriteCommand::VerifyThenUpdateExistingAspects {
                        binding,
                        asserted_aspects,
                        aspects: resolve_symbolic_aspect_references(
                            &symbolic_targets,
                            aspects,
                            &symbolic_aspect_references,
                        )?,
                        symbolic_aspect_references: Vec::new(),
                        metadata,
                        naming_intent,
                        continuity_intent,
                    },
                ))?,
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
            if let Some(intent) = summary.continuity_intent().as_ref() {
                let (_, target_collection, target_entity_identity) =
                    classify_receipt_mutation_summary(&receipt);
                let existing_truth_binding = summary.existing_truth_binding();
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
            if let Some(intent) = summary.naming_intent().as_ref() {
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
                summary.symbolic_target_reference().as_ref(),
                summary.declared_collection().as_deref(),
                &receipt,
            );
            command_summaries.push(summary);
            resolved_receipts.push(Some(receipt));
        }
        if use_backend_atomic_batch {
            let deferred_receipts = self
                .backend
                .write_batch(deferred_backend_commands)
                .map_err(ForgeQueryRuntimeError::Workspace)?;
            let deferred_slot_count = resolved_receipts
                .iter()
                .filter(|receipt| receipt.is_none())
                .count();
            if deferred_receipts.len() != deferred_slot_count {
                return Err(ForgeQueryRuntimeError::Workspace(
                    ForgeQueryWorkspaceError::new(format!(
                        "backend atomic batch returned {} receipts for {} commands",
                        deferred_receipts.len(),
                        deferred_slot_count
                    )),
                ));
            }
            let mut deferred_iter = deferred_receipts.into_iter();
            for receipt in &mut resolved_receipts {
                if receipt.is_none() {
                    *receipt = Some(
                        deferred_iter
                            .next()
                            .expect("deferred backend receipt should exist for each empty slot"),
                    );
                }
            }
            let mut deferred_symbolic_targets = BTreeMap::<String, (String, Option<String>)>::new();
            let mut rebuilt_summaries = Vec::with_capacity(command_summaries.len());
            for (summary, receipt) in command_summaries.into_iter().zip(resolved_receipts.iter()) {
                let receipt = receipt
                    .as_ref()
                    .expect("deferred backend receipts should fill every empty slot");
                let symbolic_aspect_references = summary.symbolic_aspect_references();
                let rebuilt_summary = summary.with_symbolic_aspect_resolution_evidence(
                    symbolic_aspect_resolution_evidence_for_references(
                        &deferred_symbolic_targets,
                        &symbolic_aspect_references,
                    )?,
                );
                record_same_batch_symbolic_target(
                    &mut deferred_symbolic_targets,
                    rebuilt_summary.symbolic_target_reference().as_ref(),
                    rebuilt_summary.declared_collection().as_deref(),
                    receipt,
                );
                rebuilt_summaries.push(rebuilt_summary);
            }
            command_summaries = rebuilt_summaries;
        }
        let receipts = resolved_receipts
            .into_iter()
            .map(|receipt| {
                receipt.expect("every batch command should resolve to one concrete receipt")
            })
            .collect::<Vec<_>>();
        let combined_receipt = combined_batch_mutation_receipt(&receipts)?;
        let summary = self.route_authoritative_mutation_summary(
            &combined_receipt,
            &ForgeQueryMutationMetadata::default(),
        )?;
        let write_receipts = receipts
            .into_iter()
            .zip(command_summaries)
            .map(|(receipt, summary)| {
                let mutation_family = summary.mutation_family();
                let declared_collection = summary.declared_collection();
                let declared_entity_identity = summary.declared_entity_identity();
                let existing_truth_binding = summary.existing_truth_binding();
                let verified_existing_truth_assertion = summary.verified_existing_truth_assertion();
                let symbolic_target_reference = summary.symbolic_target_reference();
                let naming_intent = summary.naming_intent();
                let continuity_intent = summary.continuity_intent();
                let declared_aspect_operations = summary.declared_aspect_operations();
                let declared_aspect_value_digest = summary.declared_aspect_value_digest();
                let symbolic_aspect_resolution_evidence =
                    summary.symbolic_aspect_resolution_evidence();
                let mutation_metadata = summary.mutation_metadata();
                let affected_live_view_ids = self.backend.affected_live_view_ids(&receipt);
                let (_, mut target_collection, mut target_entity_identity) =
                    classify_receipt_mutation_summary(&receipt);
                if let Some(binding) = existing_truth_binding.as_ref() {
                    target_collection = binding.target_collection().map(str::to_string);
                    target_entity_identity = Some(binding.resolved_target_identity().to_string());
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
                    symbolic_aspect_resolution_evidence,
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
            graph_composition_breadth,
            graph_composition_program,
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
