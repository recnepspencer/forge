use super::*;
use crate::declarative_live::DeclarativeLiveQueryRequest;
use crate::runtime::{
    ForgeQueryEffectPolicy, ForgeQueryRuntimeEvidenceAuthority,
    ForgeQueryRuntimeInspectionEvidence, ForgeQueryWriteCommand,
};
use crate::schema_view::QuerySchemaView;

impl crate::runtime::ForgeQueryRuntimeBackend for ForgeQueryMemoryApp {
    fn support_profile(&self) -> crate::runtime::ForgeQueryRuntimeSupportProfile {
        crate::runtime::ForgeQueryRuntimeSupportProfile::compatibility_backend()
    }

    fn admit_live_view_declaration(
        &self,
        _name: &str,
        request: &DeclarativeLiveQueryRequest,
        _schema_view: &QuerySchemaView,
    ) -> Result<(), ForgeQueryWorkspaceError> {
        if self.collections.contains_key(request.target()) {
            Ok(())
        } else {
            Err(ForgeQueryWorkspaceError::new(format!(
                "unknown live view collection `{}`",
                request.target()
            )))
        }
    }

    fn declare_live_view(
        &mut self,
        name: String,
        request: DeclarativeLiveQueryRequest,
        schema_view: QuerySchemaView,
    ) -> Result<ForgeQueryLiveViewHandle, ForgeQueryWorkspaceError> {
        Self::declare_live_view(self, name, request, schema_view)
    }

    #[allow(deprecated)]
    fn write(
        &mut self,
        command: ForgeQueryWriteCommand,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        match command {
            ForgeQueryWriteCommand::Insert { collection, payload } => {
                self.insert(&collection, payload)
            }
            ForgeQueryWriteCommand::InsertAspects {
                collection, aspects, ..
            } => self.insert_aspects(&collection, aspects),
            ForgeQueryWriteCommand::UpdateAspect {
                entity_identity,
                aspect_path,
                value,
            } => self.update_aspect(&entity_identity, &aspect_path, value),
            ForgeQueryWriteCommand::UpdateAspects {
                entity_identity,
                aspects,
                ..
            } => self.update_aspects(&entity_identity, aspects),
            ForgeQueryWriteCommand::UpdateExistingAspects {
                binding, aspects, ..
            } => self.update_aspects_existing(binding, aspects),
            ForgeQueryWriteCommand::VerifyThenUpdateExistingAspects { .. }
            | ForgeQueryWriteCommand::VerifyThenDeleteExistingAspects { .. } => {
                Err(ForgeQueryWorkspaceError::new(
                    "backend-verified existing-truth mutations are lowered in the runtime before backend write execution",
                ))
            }
            ForgeQueryWriteCommand::AssertExistingAspects { .. }
            | ForgeQueryWriteCommand::VerifyExistingAspects { .. } => Err(
                ForgeQueryWorkspaceError::new(
                    "existing-truth assertions are synthesized in the runtime before backend write execution",
                ),
            ),
            ForgeQueryWriteCommand::UpdateSymbolicAspects { reference, .. } => {
                Err(ForgeQueryWorkspaceError::new(format!(
                    "same-batch symbolic target `{}` must be resolved before backend write execution",
                    reference.symbol()
                )))
            }
            ForgeQueryWriteCommand::DeleteAspects {
                entity_identity,
                touched_aspect_paths,
                ..
            } => self.delete_with(&entity_identity, touched_aspect_paths),
            ForgeQueryWriteCommand::DeleteExistingAspects {
                binding,
                touched_aspect_paths,
                ..
            } => self.delete_existing_with(binding, touched_aspect_paths),
            ForgeQueryWriteCommand::DeleteSymbolicAspects { reference, .. } => {
                Err(ForgeQueryWorkspaceError::new(format!(
                    "same-batch symbolic target `{}` must be resolved before backend delete execution",
                    reference.symbol()
                )))
            }
            ForgeQueryWriteCommand::Delete { entity_identity } => self.delete(&entity_identity),
        }
    }

    fn write_batch(
        &mut self,
        commands: Vec<ForgeQueryWriteCommand>,
    ) -> Result<Vec<ForgeQueryMutationReceipt>, ForgeQueryWorkspaceError> {
        let mut receipts = Vec::with_capacity(commands.len());
        for command in commands {
            receipts.push(self.write(command)?);
        }
        Ok(receipts)
    }

    fn admit_existing_truth_binding(
        &self,
        binding: &crate::runtime::ForgeQueryExistingTruthTargetBinding,
    ) -> Result<(), crate::runtime::ForgeQueryExistingTruthBindingDenial> {
        match binding.family() {
            crate::runtime::ForgeQueryExistingTruthBindingFamily::DirectEntityIdentity
            | crate::runtime::ForgeQueryExistingTruthBindingFamily::DirectRelationIdentity => {}
        }
        if let Some(collection) = binding.target_collection() {
            let Some(_rows) = self.collections.get(collection) else {
                return Err(crate::runtime::ForgeQueryExistingTruthBindingDenial::new(
                    binding,
                    crate::runtime::ForgeQueryExistingTruthBindingDenialKind::CollectionMismatch,
                    format!("declared target collection `{collection}` is not installed"),
                ));
            };
        }
        let Some(actual_collection) = self
            .entity_collections
            .get(binding.resolved_target_identity())
        else {
            return Err(crate::runtime::ForgeQueryExistingTruthBindingDenial::new(
                binding,
                crate::runtime::ForgeQueryExistingTruthBindingDenialKind::ResolvedTargetMissing,
                format!(
                    "resolved target `{}` is not present in authoritative truth",
                    binding.resolved_target_identity()
                ),
            ));
        };
        if let Some(expected_collection) = binding.target_collection() {
            if actual_collection != expected_collection {
                return Err(crate::runtime::ForgeQueryExistingTruthBindingDenial::new(
                    binding,
                    crate::runtime::ForgeQueryExistingTruthBindingDenialKind::CollectionMismatch,
                    format!(
                        "resolved target `{}` belongs to collection `{actual_collection}`, not `{expected_collection}`",
                        binding.resolved_target_identity()
                    ),
                ));
            }
        }
        Ok(())
    }

    fn verify_existing_truth_assertion(
        &self,
        binding: &crate::runtime::ForgeQueryExistingTruthTargetBinding,
        aspects: &[crate::runtime::ForgeQueryAspectValue],
    ) -> Result<
        crate::runtime::ForgeQueryVerifiedExistingTruthAssertion,
        crate::runtime::ForgeQueryExistingTruthAssertionDenial,
    > {
        let current_payload =
            super::helpers::parse_entity_identity(binding.resolved_target_identity())
                .ok()
                .and_then(|entity_id| self.latest_entity_payload(entity_id).ok().flatten());
        self.verify_existing_assertion(binding, aspects)
            .map_err(|error: ForgeQueryWorkspaceError| {
                let message = error.to_string();
                let aspect = aspects.iter().find(|aspect| {
                    if aspect.clears_existing_value() {
                        return true;
                    }
                    current_payload
                        .as_ref()
                        .and_then(|payload| {
                            super::helpers::get_json_path(payload, aspect.aspect_path())
                        })
                        .map(|found| found != aspect.value())
                        .unwrap_or(true)
                });
                let kind = if let Some(aspect) = aspect {
                    if aspect.clears_existing_value() {
                        crate::runtime::ForgeQueryExistingTruthAssertionDenialKind::ClearAssertionUnsupported
                    } else if message.contains("is not present") {
                        crate::runtime::ForgeQueryExistingTruthAssertionDenialKind::MissingAssertedAspect
                    } else {
                        crate::runtime::ForgeQueryExistingTruthAssertionDenialKind::AssertedValueMismatch
                    }
                } else {
                    crate::runtime::ForgeQueryExistingTruthAssertionDenialKind::AssertedValueMismatch
                };
                crate::runtime::ForgeQueryExistingTruthAssertionDenial::new(
                    binding,
                    kind,
                    aspect.map(|value| value.aspect_path().to_string()),
                    aspect.and_then(|value| {
                        (!value.clears_existing_value()).then(|| {
                            serde_json::to_string(value.value())
                                .unwrap_or_else(|_| value.value().to_string())
                        })
                    }),
                    if kind
                        == crate::runtime::ForgeQueryExistingTruthAssertionDenialKind::AssertedValueMismatch
                    {
                        aspect.and_then(|value| {
                            current_payload.as_ref().and_then(|payload| {
                                super::helpers::get_json_path(payload, value.aspect_path()).cloned()
                            }).map(|found| {
                                serde_json::to_string(&found).unwrap_or_else(|_| found.to_string())
                            })
                        })
                    } else {
                        None
                    },
                    message,
                )
            })
    }

    fn probe_existing_truth(
        &self,
        request: &crate::runtime::ForgeQueryExistingTruthProbeRequest,
    ) -> Result<
        crate::runtime::ForgeQueryExistingTruthProbe,
        crate::runtime::ForgeQueryExistingTruthProbeDenial,
    > {
        let current_payload =
            super::helpers::parse_entity_identity(request.binding().resolved_target_identity())
                .ok()
                .and_then(|entity_id| self.latest_entity_payload(entity_id).ok().flatten());
        self.probe_existing_truth(request)
            .map_err(|error: ForgeQueryWorkspaceError| {
                let message = error.to_string();
                let kind = if message.contains("is not present") {
                    crate::runtime::ForgeQueryExistingTruthProbeDenialKind::MissingProbedAspect
                } else {
                    crate::runtime::ForgeQueryExistingTruthProbeDenialKind::ResolvedTargetUnavailable
                };
                crate::runtime::ForgeQueryExistingTruthProbeDenial::new(
                    request.binding(),
                    kind,
                    if kind
                        == crate::runtime::ForgeQueryExistingTruthProbeDenialKind::MissingProbedAspect
                    {
                        request.aspect_paths().iter().find_map(|aspect_path| {
                            current_payload
                                .as_ref()
                                .and_then(|payload| {
                                    super::helpers::get_json_path(payload, aspect_path)
                                })
                                .is_none()
                                .then(|| aspect_path.clone())
                        })
                    } else {
                        None
                    },
                    message,
                )
            })
    }

    fn execute_intent(
        &mut self,
        declaration: &crate::runtime::ForgeQueryIntentDeclaration,
    ) -> Result<crate::runtime::ForgeQueryIntentExecution, crate::runtime::ForgeQueryRuntimeError>
    {
        Err(crate::runtime::ForgeQueryRuntimeError::Workspace(
            ForgeQueryWorkspaceError::new(format!(
                "intent `{}` is not supported by the memory compatibility backend",
                declaration.name()
            )),
        ))
    }

    fn live_entities(&self, view_name: &str) -> Vec<ForgeQueryEntity> {
        Self::live_entities(self, view_name)
    }

    fn drain_live_patches(&mut self, view_name: &str) -> Vec<ForgeQueryLivePatch> {
        Self::drain_live_patches(self, view_name)
    }

    fn affected_live_view_ids(&self, receipt: &ForgeQueryMutationReceipt) -> Vec<String> {
        Self::affected_live_view_ids(self, receipt)
    }

    fn snapshot_token(&self) -> String {
        Self::snapshot_token(self)
    }

    fn install_live_subscription(
        &mut self,
        view_name: &str,
        activation: &crate::subscription::SubscriptionActivationInput,
    ) -> Result<String, ForgeQueryWorkspaceError> {
        Ok(format!(
            "memory-live-subscription:{}:{}",
            view_name,
            activation.activation_digest()
        ))
    }

    fn admit_preview_basis(
        &self,
        label: &str,
        effect_policy: ForgeQueryEffectPolicy,
        authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<crate::runtime::ForgeQueryPreviewBasisAdmission, ForgeQueryWorkspaceError> {
        Ok(crate::runtime::ForgeQueryPreviewBasisAdmission::new(
            authority,
            label,
            effect_policy,
            ["memory-preview-basis"],
        ))
    }

    fn inspect_write_receipt(
        &self,
        receipt: &crate::runtime::ForgeQueryWriteReceipt,
        authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<ForgeQueryRuntimeInspectionEvidence, ForgeQueryWorkspaceError> {
        Ok(ForgeQueryRuntimeInspectionEvidence::new(
            authority,
            "write-receipt",
            receipt.authority_lane(),
            ["memory-inspector-evidence"],
        ))
    }

    fn grouped_baseline_members(
        &self,
        request: &DeclarativeLiveQueryRequest,
    ) -> Result<Option<Vec<(String, String)>>, ForgeQueryWorkspaceError> {
        Ok(self.grouped_baseline_members_for_request(request))
    }
}
