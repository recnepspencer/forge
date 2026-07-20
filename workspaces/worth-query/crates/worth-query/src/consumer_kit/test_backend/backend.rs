use std::collections::BTreeMap;

use crate::declarative_live::DeclarativeLiveQueryRequest;
use crate::memory_workspace::{
    WorthQueryEntity, WorthQueryLivePatch, WorthQueryLiveViewHandle, WorthQueryMemoryWorkspace,
    WorthQueryMutationReceipt, WorthQuerySnapshotIdentity, WorthQueryWorkspaceError,
    WorthQueryWorkspaceErrorKind,
};
use crate::runtime::{
    runtime_subscription_support_evidence_identity, LiveViewDeclarationAdmissionBoundaryReceipt,
    LiveViewDeclarationAdmissionReceipt, SubscriptionActivationReceipt,
    WorthQueryBackendAdmissibleMutation, WorthQueryBasisAdmissionEvidenceRow,
    WorthQueryEffectPolicy, WorthQueryIntentDeclaration, WorthQueryIntentExecution,
    WorthQueryLiveArtifactTarget, WorthQueryMutationFamily,
    WorthQueryMutationTargetCollectionIdentity, WorthQueryPreviewBasisAdmission,
    WorthQueryRuntimeBackend, WorthQueryRuntimeError, WorthQueryRuntimeEvidenceAuthority,
    WorthQueryRuntimeInspectionEvidence, WorthQueryRuntimeSupportProfile, WorthQueryWriteCommand,
    WorthQueryWriteReceipt,
};
use crate::schema_view::QuerySchemaView;
use crate::session_label::WorthQuerySessionLabel;
use crate::subscription::SubscriptionActivationInput;

use super::support_profile::in_memory_test_backend_support_profile;

pub(super) struct WorthQueryInMemoryTestBackend {
    workspace: WorthQueryMemoryWorkspace,
    support_profile: WorthQueryRuntimeSupportProfile,
    live_views: BTreeMap<WorthQueryLiveArtifactTarget, WorthQueryMutationTargetCollectionIdentity>,
}

impl WorthQueryInMemoryTestBackend {
    pub(super) fn new(workspace: WorthQueryMemoryWorkspace) -> Self {
        Self::with_support_profile(workspace, in_memory_test_backend_support_profile())
    }

    pub(super) fn with_support_profile(
        workspace: WorthQueryMemoryWorkspace,
        support_profile: WorthQueryRuntimeSupportProfile,
    ) -> Self {
        Self {
            workspace,
            support_profile,
            live_views: BTreeMap::new(),
        }
    }

    fn ensure_declared_collection(
        &self,
        collection: &crate::runtime::WorthQueryMutationTargetCollectionIdentity,
    ) -> Result<(), WorthQueryWorkspaceError> {
        let collection_label = collection.as_str();
        if collection_label != self.workspace.kind_name() {
            let message = format!(
                "in-memory test backend only supports collection `{}`; command declared `{collection_label}`",
                self.workspace.kind_name()
            );
            return Err(WorthQueryWorkspaceError::with_kind(
                WorthQueryWorkspaceErrorKind::UnsupportedCollection,
                message,
            ));
        }
        Ok(())
    }

    fn ensure_live_request_target(
        &self,
        request: &DeclarativeLiveQueryRequest,
    ) -> Result<(), WorthQueryWorkspaceError> {
        self.ensure_declared_collection(
            &crate::runtime::WorthQueryMutationTargetCollectionIdentity::new(
                "in-memory-live-request",
                request.target(),
            ),
        )
    }

    fn admit_write_command(
        &self,
        command: &WorthQueryWriteCommand,
    ) -> Result<(), WorthQueryWorkspaceError> {
        if let Some(collection) = command.declared_collection_identity() {
            self.ensure_declared_collection(&collection)?;
        }
        match command {
            WorthQueryWriteCommand::InsertAspects { .. }
            | WorthQueryWriteCommand::UpdateAspect { .. }
            | WorthQueryWriteCommand::UpdateAspects { .. }
            | WorthQueryWriteCommand::DeleteAspects { .. }
            | WorthQueryWriteCommand::Delete { .. } => Ok(()),
            unsupported => Err(WorthQueryWorkspaceError::with_kind(
                WorthQueryWorkspaceErrorKind::UnsupportedWriteFamily,
                format!(
                    "in-memory test backend does not support `{}` write commands",
                    unsupported.mutation_family().as_str()
                ),
            )),
        }
    }

    fn apply_backend_admissible_mutation(
        &mut self,
        mutation: WorthQueryBackendAdmissibleMutation,
    ) -> Result<WorthQueryMutationReceipt, WorthQueryWorkspaceError> {
        match mutation.mutation_family() {
            WorthQueryMutationFamily::Insert => {
                let collection = mutation.declared_collection_identity().ok_or_else(|| {
                    WorthQueryWorkspaceError::new(
                        "insert mutation missing declared collection after admission",
                    )
                })?;
                self.ensure_declared_collection(&collection)?;
                self.workspace
                    .insert_portable_patch(mutation.portable_patch())
            }
            WorthQueryMutationFamily::Update => {
                let entity_identity = mutation.declared_entity_identity().ok_or_else(|| {
                    WorthQueryWorkspaceError::new(
                        "update mutation missing declared entity after admission",
                    )
                })?;
                self.workspace
                    .update_portable_patch(entity_identity, mutation.portable_patch())
            }
            WorthQueryMutationFamily::Delete => {
                let entity_identity = mutation.declared_entity_identity().ok_or_else(|| {
                    WorthQueryWorkspaceError::new(
                        "delete mutation missing declared entity after admission",
                    )
                })?;
                self.workspace.delete(entity_identity)
            }
            unsupported => Err(WorthQueryWorkspaceError::new(format!(
                "write command `{}` should be rejected by admission before execution",
                unsupported.as_str()
            ))),
        }
    }

    fn view_targets_collection(&self, target: &WorthQueryLiveArtifactTarget) -> bool {
        self.live_views
            .get(target)
            .is_some_and(|target| target.as_str() == self.workspace.kind_name())
    }
}

impl WorthQueryRuntimeBackend for WorthQueryInMemoryTestBackend {
    fn support_profile(&self) -> WorthQueryRuntimeSupportProfile {
        self.support_profile.clone()
    }

    fn current_snapshot_identity(&self) -> WorthQuerySnapshotIdentity {
        self.workspace.snapshot_identity()
    }

    fn admit_live_view_declaration(
        &self,
        name: &str,
        request: &DeclarativeLiveQueryRequest,
        _schema_view: &QuerySchemaView,
    ) -> Result<LiveViewDeclarationAdmissionBoundaryReceipt, WorthQueryWorkspaceError> {
        self.ensure_live_request_target(request)?;
        let admission = LiveViewDeclarationAdmissionReceipt::from_request(name, request);
        Ok(LiveViewDeclarationAdmissionBoundaryReceipt::from_request(
            name, request, admission,
        ))
    }

    fn declare_live_view(
        &mut self,
        name: String,
        request: DeclarativeLiveQueryRequest,
        _schema_view: QuerySchemaView,
    ) -> Result<WorthQueryLiveViewHandle, WorthQueryWorkspaceError> {
        self.ensure_live_request_target(&request)?;
        let live_target = WorthQueryLiveArtifactTarget::from_view_name(name.clone());
        self.live_views
            .insert(live_target, request.target_collection_identity());
        Ok(WorthQueryLiveViewHandle::new(name))
    }

    fn close_live_view(&mut self, name: &str) -> Result<(), WorthQueryWorkspaceError> {
        self.live_views
            .remove(&WorthQueryLiveArtifactTarget::from_view_name(name));
        Ok(())
    }

    fn write(
        &mut self,
        mutation: WorthQueryBackendAdmissibleMutation,
    ) -> Result<WorthQueryMutationReceipt, WorthQueryWorkspaceError> {
        self.apply_backend_admissible_mutation(mutation)
    }

    fn write_batch(
        &mut self,
        mutations: Vec<WorthQueryBackendAdmissibleMutation>,
    ) -> Result<Vec<WorthQueryMutationReceipt>, WorthQueryWorkspaceError> {
        if mutations.len() > 1 {
            return Err(WorthQueryWorkspaceError::with_kind(
                WorthQueryWorkspaceErrorKind::BatchAtomicityUnsupported,
                "in-memory test backend denies multi-command batches before execution because scaffold batch atomicity is not supported",
            ));
        }
        let mut receipts = Vec::with_capacity(mutations.len());
        for mutation in mutations {
            receipts.push(self.apply_backend_admissible_mutation(mutation)?);
        }
        Ok(receipts)
    }

    fn execute_intent(
        &mut self,
        _declaration: &WorthQueryIntentDeclaration,
    ) -> Result<WorthQueryIntentExecution, WorthQueryRuntimeError> {
        Err(WorthQueryRuntimeError::MissingIntentAuthority)
    }

    fn live_entities_for_target(
        &self,
        target: &WorthQueryLiveArtifactTarget,
    ) -> Vec<WorthQueryEntity> {
        if self.view_targets_collection(target) {
            return self.workspace.entities();
        }
        Vec::new()
    }

    fn drain_live_patches_for_target(
        &mut self,
        _target: &WorthQueryLiveArtifactTarget,
    ) -> Vec<WorthQueryLivePatch> {
        Vec::new()
    }

    fn affected_live_view_targets(
        &self,
        receipt: &WorthQueryMutationReceipt,
    ) -> Vec<WorthQueryLiveArtifactTarget> {
        self.live_views
            .iter()
            .filter(|(_, target)| {
                receipt.deltas().iter().any(|delta| {
                    delta
                        .target_collection_identity()
                        .same_target_collection_as(target)
                })
            })
            .map(|(target, _)| target.clone())
            .collect()
    }

    fn install_live_subscription(
        &mut self,
        view_name: &str,
        activation: &SubscriptionActivationInput,
    ) -> Result<SubscriptionActivationReceipt, WorthQueryWorkspaceError> {
        Ok(SubscriptionActivationReceipt::from_activation(
            view_name,
            activation,
            runtime_subscription_support_evidence_identity("consumer-kit-in-memory-test-backend"),
            None,
        ))
    }

    fn admit_preview_basis(
        &self,
        label: &WorthQuerySessionLabel,
        effect_policy: WorthQueryEffectPolicy,
        authority: &WorthQueryRuntimeEvidenceAuthority,
    ) -> Result<WorthQueryPreviewBasisAdmission, WorthQueryWorkspaceError> {
        Ok(WorthQueryPreviewBasisAdmission::new(
            authority,
            label.clone(),
            effect_policy,
            [WorthQueryBasisAdmissionEvidenceRow::tagged(
                "preview-basis-admission",
                "consumer-kit-in-memory-test-backend",
            )],
        ))
    }

    fn inspect_write_receipt(
        &self,
        receipt: &WorthQueryWriteReceipt,
        authority: &WorthQueryRuntimeEvidenceAuthority,
    ) -> Result<WorthQueryRuntimeInspectionEvidence, WorthQueryWorkspaceError> {
        Ok(WorthQueryRuntimeInspectionEvidence::new(
            authority,
            "consumer-kit-in-memory-test-write-receipt",
            receipt.authority_lane(),
            ["consumer-kit-in-memory-test-inspection"],
        ))
    }

    fn admit_preview_write_command(
        &self,
        command: &WorthQueryWriteCommand,
    ) -> Result<(), WorthQueryWorkspaceError> {
        self.admit_write_command(command)
    }
}
