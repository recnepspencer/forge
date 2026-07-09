use super::*;
use crate::intent_admission::WorthQueryIntentEligibilityTraceEvidence;

pub(crate) struct WorthQueryWriteAdmissionExecutionRecord {
    pub family: WorthQueryIntentAdmissionFamily,
    pub entrypoint: WorthQueryIntentAdmissionCoveredEntrypoint,
    pub execution_seam: WorthQueryIntentAdmissionExecutionSeam,
    pub request_detail: String,
    pub request_digest: String,
    pub eligibility_trace: WorthQueryIntentEligibilityTraceEvidence,
    pub decision_digest: String,
    pub handoff_digest: String,
    pub binding_digest: String,
    pub obligation_dispatch: Option<WorthQueryAuthoritativeMutationObligationDispatch>,
}

impl WorthQueryRuntime {
    pub(crate) fn build_authoritative_mutation_intent_seed(
        &self,
        command: WorthQueryWriteCommand,
    ) -> crate::intent_admission::WorthQueryAuthoritativeMutationIntentSeed {
        use crate::intent_admission::WorthQueryAuthoritativeMutationPreflight as Preflight;

        let preflight = if let Some(reference) = command.symbolic_target_reference() {
            Preflight::TargetReferenceDenied(WorthQuerySymbolicTargetReferenceDenial::new(
                reference,
                WorthQuerySymbolicTargetReferenceDenialKind::RequiresBatchContext,
                "same-batch symbolic target references require batch execution",
            ))
        } else if let Some(reference) = command.symbolic_aspect_references().first() {
            Preflight::TargetReferenceDenied(WorthQuerySymbolicTargetReferenceDenial::new(
                reference.reference(),
                WorthQuerySymbolicTargetReferenceDenialKind::RequiresBatchContext,
                "same-batch symbolic aspect references require batch execution",
            ))
        } else if let Some(binding) = command.existing_truth_binding() {
            match self.backend.admit_existing_truth_binding(binding) {
                Err(denial) => Preflight::BindingDenied(denial),
                Ok(()) => self.scalar_mutation_post_binding_preflight(&command),
            }
        } else {
            self.scalar_mutation_post_binding_preflight(&command)
        };

        crate::intent_admission::WorthQueryAuthoritativeMutationIntentSeed::new(command, preflight)
    }

    fn scalar_mutation_post_binding_preflight(
        &self,
        command: &WorthQueryWriteCommand,
    ) -> crate::intent_admission::WorthQueryAuthoritativeMutationPreflight {
        use crate::intent_admission::WorthQueryAuthoritativeMutationPreflight as Preflight;

        match admit_continuity_intent(command) {
            Err(denial) => Preflight::ContinuityDenied(denial),
            Ok(()) => match admit_naming_intent(command) {
                Err(denial) => Preflight::NamingDenied(denial),
                Ok(()) => match self.verified_existing_assertion_for_command(command) {
                    Ok(verified_existing_truth_assertion) => Preflight::Admitted {
                        verified_existing_truth_assertion,
                    },
                    Err(WorthQueryRuntimeError::ExistingTruthAssertionDenied(denial)) => {
                        Preflight::AssertionDenied(denial)
                    }
                    Err(other) => panic!("unexpected scalar mutation preflight error: {other}"),
                },
            },
        }
    }

    pub(super) fn verified_existing_assertion_for_command(
        &self,
        command: &WorthQueryWriteCommand,
    ) -> Result<Option<WorthQueryVerifiedExistingTruthAssertion>, WorthQueryRuntimeError> {
        match command {
            WorthQueryWriteCommand::VerifyExistingAspects {
                binding, aspects, ..
            }
            | WorthQueryWriteCommand::VerifyThenUpdateExistingAspects {
                binding,
                asserted_aspects: aspects,
                ..
            }
            | WorthQueryWriteCommand::VerifyThenDeleteExistingAspects {
                binding,
                asserted_aspects: aspects,
                ..
            } => Ok(Some(
                self.backend
                    .verify_existing_truth_assertion(binding, aspects)
                    .map_err(WorthQueryRuntimeError::ExistingTruthAssertionDenied)?,
            )),
            _ => Ok(None),
        }
    }

    pub(super) fn lower_backend_write_command(
        command: WorthQueryWriteCommand,
    ) -> WorthQueryWriteCommand {
        match command {
            WorthQueryWriteCommand::VerifyThenUpdateExistingAspects {
                binding,
                aspects,
                metadata,
                naming_intent,
                continuity_intent,
                ..
            } => WorthQueryWriteCommand::UpdateExistingAspects {
                binding,
                aspects,
                metadata,
                naming_intent,
                continuity_intent,
            },
            WorthQueryWriteCommand::VerifyThenDeleteExistingAspects {
                binding,
                touched_aspects,
                metadata,
                naming_intent,
                ..
            } => WorthQueryWriteCommand::DeleteExistingAspects {
                binding,
                touched_aspects,
                metadata,
                naming_intent,
            },
            other => other,
        }
    }

    pub fn probe_existing(
        &self,
        request: WorthQueryExistingTruthProbeRequest,
    ) -> Result<WorthQueryExistingTruthProbe, WorthQueryRuntimeError> {
        Ok(self
            .probe_existing_intent(request)
            .execute()?
            .probe()
            .clone())
    }

    pub(crate) fn execute_authoritative_write_command_direct(
        &mut self,
        command: WorthQueryWriteCommand,
        verified_existing_truth_assertion: Option<WorthQueryVerifiedExistingTruthAssertion>,
        shared_admission: Option<WorthQueryWriteAdmissionExecutionRecord>,
    ) -> Result<WorthQueryWriteReceipt, WorthQueryRuntimeError> {
        let mutation_family = command.mutation_family();
        let declared_collection_identity = command.declared_collection_identity();
        let declared_entity_identity = command.declared_entity_identity();
        let existing_truth_binding = command.existing_truth_binding().cloned();
        let symbolic_target_reference = None;
        let symbolic_aspect_resolution_evidence = Vec::new();
        let naming_intent = command.naming_intent().cloned();
        let continuity_intent = command.continuity_intent().cloned();
        let declared_aspect_operations = command.declared_aspect_operations();
        let declared_aspect_value_digest = command_declared_aspect_value_identity(&command);
        let mutation_metadata = command.mutation_metadata();
        let receipt = self
            .execute_backend_or_synthetic_write(command, declared_aspect_value_digest.as_ref())?;
        let receipt = self.attach_optional_mutation_bundles(
            receipt,
            existing_truth_binding.as_ref(),
            continuity_intent.as_ref(),
            naming_intent.as_ref(),
        );
        let execution_provenance =
            self.shared_write_execution_provenance(shared_admission.as_ref(), &receipt);
        let decision_trace_envelope = self.shared_write_decision_trace_envelope(
            shared_admission.as_ref(),
            mutation_family,
            &receipt,
        );
        let receipt = self.route_authoritative_mutation_receipt(
            receipt,
            mutation_family,
            declared_collection_identity,
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
            decision_trace_envelope,
            execution_provenance,
            shared_admission.and_then(|record| record.obligation_dispatch),
        )?;
        self.journal_replay.record_write_receipt(&receipt);
        Ok(receipt)
    }

    fn execute_backend_or_synthetic_write(
        &mut self,
        command: WorthQueryWriteCommand,
        declared_aspect_value_digest: Option<&crate::evidence_identity::WorthQueryEvidenceIdentity>,
    ) -> Result<WorthQueryMutationReceipt, WorthQueryRuntimeError> {
        match &command {
            WorthQueryWriteCommand::AssertExistingAspects { binding, .. }
            | WorthQueryWriteCommand::VerifyExistingAspects { binding, .. } => {
                Ok(synthetic_existing_assertion_receipt(
                    binding,
                    &self.current_snapshot_identity(),
                    declared_aspect_value_digest,
                ))
            }
            _ => self
                .backend
                .write(WorthQueryBackendAdmissibleMutation::from_admitted_command(
                    Self::lower_backend_write_command(command),
                ))
                .map_err(Into::into),
        }
    }

    fn attach_optional_mutation_bundles(
        &self,
        mut receipt: WorthQueryMutationReceipt,
        existing_truth_binding: Option<&WorthQueryExistingTruthTargetBinding>,
        continuity_intent: Option<&WorthQueryContinuityMutationIntent>,
        naming_intent: Option<&WorthQueryNamingMutationIntent>,
    ) -> WorthQueryMutationReceipt {
        receipt = self.attach_optional_continuity_bundle(
            receipt,
            existing_truth_binding,
            continuity_intent,
        );
        self.attach_optional_naming_bundle(receipt, existing_truth_binding, naming_intent)
    }

    fn attach_optional_continuity_bundle(
        &self,
        receipt: WorthQueryMutationReceipt,
        existing_truth_binding: Option<&WorthQueryExistingTruthTargetBinding>,
        continuity_intent: Option<&WorthQueryContinuityMutationIntent>,
    ) -> WorthQueryMutationReceipt {
        let Some(intent) = continuity_intent else {
            return receipt;
        };
        let (_, target_collection, target_entity_identity) =
            classify_receipt_mutation_summary(&receipt);
        let basis_binding_digest = existing_truth_binding.map(|binding| binding.binding_digest());
        match bridge_continuity_mutation_bundle(
            intent,
            basis_binding_digest.as_deref(),
            target_entity_identity.as_ref(),
            target_collection.as_ref(),
        ) {
            Some(bundle) => attach_continuity_mutation_to_receipt(receipt, bundle),
            None => receipt,
        }
    }

    fn attach_optional_naming_bundle(
        &self,
        receipt: WorthQueryMutationReceipt,
        existing_truth_binding: Option<&WorthQueryExistingTruthTargetBinding>,
        naming_intent: Option<&WorthQueryNamingMutationIntent>,
    ) -> WorthQueryMutationReceipt {
        let Some(intent) = naming_intent else {
            return receipt;
        };
        let (_, mut target_collection, mut target_entity_identity) =
            classify_receipt_mutation_summary(&receipt);
        if let Some(binding) = existing_truth_binding {
            target_collection = binding.target_collection_identity().cloned();
            target_entity_identity = Some(binding.resolved_entity_artifact_identity());
        }
        match bridge_naming_mutation_bundle(
            intent,
            target_entity_identity.as_ref(),
            target_collection.as_ref(),
        ) {
            Some(bundle) => attach_naming_mutation_to_receipt(receipt, bundle),
            None => receipt,
        }
    }

    fn shared_write_execution_provenance(
        &self,
        shared_admission: Option<&WorthQueryWriteAdmissionExecutionRecord>,
        receipt: &WorthQueryMutationReceipt,
    ) -> Option<WorthQueryIntentExecutionProvenance> {
        shared_admission.map(|record| {
            let commit_label = receipt
                .commit_identity
                .evidence_identity()
                .reporting_projection()
                .to_string();
            let snapshot_evidence_identity = receipt.snapshot_identity.evidence_identity();
            WorthQueryIntentExecutionProvenance::for_shared_execution_typed_parts(
                record.family,
                record.entrypoint,
                record.execution_seam,
                &record.decision_digest,
                &record.handoff_digest,
                &record.binding_digest,
                &commit_label,
                &snapshot_evidence_identity,
            )
        })
    }

    fn shared_write_decision_trace_envelope(
        &self,
        shared_admission: Option<&WorthQueryWriteAdmissionExecutionRecord>,
        mutation_family: WorthQueryMutationFamily,
        receipt: &WorthQueryMutationReceipt,
    ) -> Option<WorthQueryIntentDecisionTraceEnvelope> {
        shared_admission.map(|record| {
            let commit_label = receipt
                .commit_identity
                .evidence_identity()
                .reporting_projection()
                .to_string();
            let obligation_dispatch_envelope_digest = record
                .obligation_dispatch
                .as_ref()
                .and_then(WorthQueryAuthoritativeMutationObligationDispatch::envelope_digest);
            WorthQueryIntentDecisionTraceEnvelope::for_admitted_execution_parts_with_obligation_dispatch(
                record.family,
                record.entrypoint,
                &record.request_detail,
                &record.request_digest,
                record.eligibility_trace.clone(),
                &record.decision_digest,
                &record.handoff_digest,
                record.execution_seam,
                obligation_dispatch_envelope_digest,
                mutation_family.as_str(),
                &commit_label,
                "mutation-write",
            )
        })
    }

    pub fn write(
        &mut self,
        command: WorthQueryWriteCommand,
    ) -> Result<WorthQueryWriteReceipt, WorthQueryRuntimeError> {
        self.write_intent(command).execute()
    }

    pub fn write_with_policy_context(
        &mut self,
        command: WorthQueryWriteCommand,
        policy_context: crate::policy_basis::AdmittedPolicyTenantContext,
    ) -> Result<WorthQueryWriteReceipt, WorthQueryRuntimeError> {
        let review = self.review_authoritative_runtime_write(command)?;
        let handoff = self
            .resolve_reviewed_admitted_authoritative_write_handoff_with_policy_context(
                review,
                &policy_context,
            )?;
        let binding = self.prepare_authoritative_mutation_execution_binding(handoff);
        self.execute_authoritative_mutation_execution_binding(binding)
    }
}
