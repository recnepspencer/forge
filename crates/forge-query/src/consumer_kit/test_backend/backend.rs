use std::collections::BTreeMap;

use crate::declarative_live::DeclarativeLiveQueryRequest;
use crate::memory_workspace::{
    ForgeQueryEntity, ForgeQueryLivePatch, ForgeQueryLiveViewHandle, ForgeQueryMemoryWorkspace,
    ForgeQueryMutationReceipt, ForgeQuerySnapshotIdentity, ForgeQueryWorkspaceError,
    ForgeQueryWorkspaceErrorKind,
};
use crate::runtime::{
    runtime_subscription_support_evidence_identity, ForgeQueryAspectValue,
    ForgeQueryBasisAdmissionEvidenceRow, ForgeQueryEffectPolicy, ForgeQueryIntentDeclaration,
    ForgeQueryIntentExecution, ForgeQueryPreviewBasisAdmission, ForgeQueryRuntimeBackend,
    ForgeQueryRuntimeError, ForgeQueryRuntimeEvidenceAuthority,
    ForgeQueryRuntimeInspectionEvidence, ForgeQueryRuntimeSupportProfile, ForgeQueryWriteCommand,
    ForgeQueryWriteReceipt, LiveViewDeclarationAdmissionBoundaryReceipt,
    LiveViewDeclarationAdmissionReceipt, SubscriptionActivationReceipt,
};
use crate::schema_view::QuerySchemaView;
use crate::session_label::ForgeQuerySessionLabel;
use crate::subscription::SubscriptionActivationInput;

use super::support_profile::in_memory_test_backend_support_profile;

pub(super) struct ForgeQueryInMemoryTestBackend {
    workspace: ForgeQueryMemoryWorkspace,
    support_profile: ForgeQueryRuntimeSupportProfile,
    live_views: BTreeMap<String, String>,
}

impl ForgeQueryInMemoryTestBackend {
    pub(super) fn new(workspace: ForgeQueryMemoryWorkspace) -> Self {
        Self {
            workspace,
            support_profile: in_memory_test_backend_support_profile(),
            live_views: BTreeMap::new(),
        }
    }

    fn ensure_declared_collection(&self, collection: &str) -> Result<(), ForgeQueryWorkspaceError> {
        if collection != self.workspace.kind_name() {
            let message = format!(
                "in-memory test backend only supports collection `{}`; command declared `{collection}`",
                self.workspace.kind_name()
            );
            return Err(ForgeQueryWorkspaceError::with_kind(
                ForgeQueryWorkspaceErrorKind::UnsupportedCollection,
                message,
            ));
        }
        Ok(())
    }

    fn ensure_live_request_target(
        &self,
        request: &DeclarativeLiveQueryRequest,
    ) -> Result<(), ForgeQueryWorkspaceError> {
        self.ensure_declared_collection(request.target())
    }

    fn admit_write_command(
        &self,
        command: &ForgeQueryWriteCommand,
    ) -> Result<(), ForgeQueryWorkspaceError> {
        if let Some(collection) = command.declared_collection_ref() {
            self.ensure_declared_collection(collection)?;
        }
        match command {
            ForgeQueryWriteCommand::InsertAspects { .. }
            | ForgeQueryWriteCommand::UpdateAspect { .. }
            | ForgeQueryWriteCommand::UpdateAspects { .. }
            | ForgeQueryWriteCommand::DeleteAspects { .. }
            | ForgeQueryWriteCommand::Delete { .. } => Ok(()),
            unsupported => Err(ForgeQueryWorkspaceError::with_kind(
                ForgeQueryWorkspaceErrorKind::UnsupportedWriteFamily,
                format!(
                    "in-memory test backend does not support `{}` write commands",
                    unsupported.mutation_family().as_str()
                ),
            )),
        }
    }

    fn apply_write_command(
        &mut self,
        command: ForgeQueryWriteCommand,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        self.admit_write_command(&command)?;
        match command {
            ForgeQueryWriteCommand::InsertAspects {
                collection,
                aspects,
                ..
            } => {
                self.ensure_declared_collection(&collection)?;
                self.workspace.insert_aspects(aspects)
            }
            ForgeQueryWriteCommand::UpdateAspect {
                entity_identity,
                aspect_path,
                value,
            } => self.workspace.update_aspects(
                entity_identity,
                vec![ForgeQueryAspectValue::new(aspect_path, value)?],
            ),
            ForgeQueryWriteCommand::UpdateAspects {
                entity_identity,
                aspects,
                ..
            } => self.workspace.update_aspects(entity_identity, aspects),
            ForgeQueryWriteCommand::DeleteAspects {
                entity_identity, ..
            }
            | ForgeQueryWriteCommand::Delete { entity_identity } => {
                self.workspace.delete(entity_identity)
            }
            unsupported => unreachable!(
                "write command `{}` should be rejected by admission before execution",
                unsupported.mutation_family().as_str()
            ),
        }
    }

    fn view_targets_collection(&self, view_name: &str) -> bool {
        self.live_views
            .get(view_name)
            .is_some_and(|target| target == self.workspace.kind_name())
    }
}

impl ForgeQueryRuntimeBackend for ForgeQueryInMemoryTestBackend {
    fn support_profile(&self) -> ForgeQueryRuntimeSupportProfile {
        self.support_profile.clone()
    }

    fn current_snapshot_identity(&self) -> ForgeQuerySnapshotIdentity {
        self.workspace.snapshot_identity()
    }

    fn admit_live_view_declaration(
        &self,
        name: &str,
        request: &DeclarativeLiveQueryRequest,
        _schema_view: &QuerySchemaView,
    ) -> Result<LiveViewDeclarationAdmissionBoundaryReceipt, ForgeQueryWorkspaceError> {
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
    ) -> Result<ForgeQueryLiveViewHandle, ForgeQueryWorkspaceError> {
        self.ensure_live_request_target(&request)?;
        self.live_views
            .insert(name.clone(), request.target().to_string());
        Ok(ForgeQueryLiveViewHandle::new(name))
    }

    fn write(
        &mut self,
        command: ForgeQueryWriteCommand,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        self.apply_write_command(command)
    }

    fn write_batch(
        &mut self,
        commands: Vec<ForgeQueryWriteCommand>,
    ) -> Result<Vec<ForgeQueryMutationReceipt>, ForgeQueryWorkspaceError> {
        if commands.len() > 1 {
            return Err(ForgeQueryWorkspaceError::with_kind(
                ForgeQueryWorkspaceErrorKind::BatchAtomicityUnsupported,
                "in-memory test backend denies multi-command batches before execution because scaffold batch atomicity is not supported",
            ));
        }
        for command in &commands {
            self.admit_write_command(command)?;
        }
        let mut receipts = Vec::with_capacity(commands.len());
        for command in commands {
            receipts.push(self.apply_write_command(command)?);
        }
        Ok(receipts)
    }

    fn execute_intent(
        &mut self,
        _declaration: &ForgeQueryIntentDeclaration,
    ) -> Result<ForgeQueryIntentExecution, ForgeQueryRuntimeError> {
        Err(ForgeQueryRuntimeError::MissingIntentAuthority)
    }

    fn live_entities(&self, view_name: &str) -> Vec<ForgeQueryEntity> {
        if self.view_targets_collection(view_name) {
            return self.workspace.entities();
        }
        Vec::new()
    }

    fn drain_live_patches(&mut self, _view_name: &str) -> Vec<ForgeQueryLivePatch> {
        Vec::new()
    }

    fn affected_live_view_ids(&self, receipt: &ForgeQueryMutationReceipt) -> Vec<String> {
        self.live_views
            .iter()
            .filter(|(_, target)| {
                receipt
                    .deltas()
                    .iter()
                    .any(|delta| delta.collection() == target.as_str())
            })
            .map(|(name, _)| name.clone())
            .collect()
    }

    fn install_live_subscription(
        &mut self,
        view_name: &str,
        activation: &SubscriptionActivationInput,
    ) -> Result<SubscriptionActivationReceipt, ForgeQueryWorkspaceError> {
        Ok(SubscriptionActivationReceipt::from_activation(
            view_name,
            activation,
            runtime_subscription_support_evidence_identity("consumer-kit-in-memory-test-backend"),
            None,
        ))
    }

    fn admit_preview_basis(
        &self,
        label: &ForgeQuerySessionLabel,
        effect_policy: ForgeQueryEffectPolicy,
        authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<ForgeQueryPreviewBasisAdmission, ForgeQueryWorkspaceError> {
        Ok(ForgeQueryPreviewBasisAdmission::new(
            authority,
            label.clone(),
            effect_policy,
            [ForgeQueryBasisAdmissionEvidenceRow::tagged(
                "preview-basis-admission",
                "consumer-kit-in-memory-test-backend",
            )],
        ))
    }

    fn inspect_write_receipt(
        &self,
        receipt: &ForgeQueryWriteReceipt,
        authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<ForgeQueryRuntimeInspectionEvidence, ForgeQueryWorkspaceError> {
        Ok(ForgeQueryRuntimeInspectionEvidence::new(
            authority,
            "consumer-kit-in-memory-test-write-receipt",
            receipt.authority_lane(),
            ["consumer-kit-in-memory-test-inspection"],
        ))
    }

    fn admit_preview_write_command(
        &self,
        command: &ForgeQueryWriteCommand,
    ) -> Result<(), ForgeQueryWorkspaceError> {
        self.admit_write_command(command)
    }
}
