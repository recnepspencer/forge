use forge_relational::facade::runtime::RelationalRuntime;
use forge_runtime_bridge::facade::RuntimeBridge;

use crate::facade::{
    ForgeQueryMutationDelta, ForgeQueryMutationKind, ForgeQueryMutationReceipt,
    ForgeQueryRuntimeWriteAuthorityAdapter, ForgeQueryWorkspaceError, ForgeQueryWriteCommand,
    WriteAuthorityExecutionReceipt,
};
use crate::memory_workspace::{ForgeQueryCommitIdentity, ForgeQuerySnapshotIdentity};
use crate::runtime::build_bridge_authority_bundle;
use forge_runtime_bridge::facade::RelationalBridgeSnapshotIdentityParts;

pub(super) struct TranscriptWriteAuthority;

impl ForgeQueryRuntimeWriteAuthorityAdapter for TranscriptWriteAuthority {
    fn write(
        &mut self,
        _bridge: &RuntimeBridge,
        _relational_runtime: Option<&mut RelationalRuntime>,
        command: ForgeQueryWriteCommand,
    ) -> Result<WriteAuthorityExecutionReceipt, ForgeQueryWorkspaceError> {
        let (collection, aspect_paths) = match &command {
            ForgeQueryWriteCommand::InsertAspects {
                collection,
                aspects,
                ..
            } => (
                collection.clone(),
                aspects
                    .iter()
                    .map(|aspect| aspect.aspect_path().to_string())
                    .collect(),
            ),
            ForgeQueryWriteCommand::UpdateAspect { aspect_path, .. } => {
                ("TranscriptEntity".to_string(), vec![aspect_path.clone()])
            }
            ForgeQueryWriteCommand::UpdateAspects { aspects, .. } => (
                "TranscriptEntity".to_string(),
                aspects
                    .iter()
                    .map(|aspect| aspect.aspect_path().to_string())
                    .collect(),
            ),
            ForgeQueryWriteCommand::UpdateExistingAspects {
                aspects, binding, ..
            }
            | ForgeQueryWriteCommand::VerifyThenUpdateExistingAspects {
                aspects, binding, ..
            }
            | ForgeQueryWriteCommand::AssertExistingAspects {
                aspects, binding, ..
            }
            | ForgeQueryWriteCommand::VerifyExistingAspects {
                aspects, binding, ..
            } => (
                binding
                    .target_collection()
                    .unwrap_or("TranscriptEntity")
                    .to_string(),
                aspects
                    .iter()
                    .map(|aspect| aspect.aspect_path().to_string())
                    .collect(),
            ),
            ForgeQueryWriteCommand::VerifyThenDeleteExistingAspects {
                binding,
                touched_aspect_paths,
                ..
            } => (
                binding
                    .target_collection()
                    .unwrap_or("TranscriptEntity")
                    .to_string(),
                touched_aspect_paths.clone(),
            ),
            ForgeQueryWriteCommand::UpdateSymbolicAspects {
                aspects, reference, ..
            } => (
                reference
                    .target_collection()
                    .unwrap_or("TranscriptEntity")
                    .to_string(),
                aspects
                    .iter()
                    .map(|aspect| aspect.aspect_path().to_string())
                    .collect(),
            ),
            ForgeQueryWriteCommand::DeleteAspects {
                touched_aspect_paths,
                ..
            } => ("TranscriptEntity".to_string(), touched_aspect_paths.clone()),
            ForgeQueryWriteCommand::DeleteExistingAspects {
                binding,
                touched_aspect_paths,
                ..
            } => (
                binding
                    .target_collection()
                    .unwrap_or("TranscriptEntity")
                    .to_string(),
                touched_aspect_paths.clone(),
            ),
            ForgeQueryWriteCommand::DeleteSymbolicAspects {
                reference,
                touched_aspect_paths,
                ..
            } => (
                reference
                    .target_collection()
                    .unwrap_or("TranscriptEntity")
                    .to_string(),
                touched_aspect_paths.clone(),
            ),
            ForgeQueryWriteCommand::Delete { .. } => ("TranscriptEntity".to_string(), Vec::new()),
        };
        let entity_identity_text = "transcript-entity-1";
        let entity_identity = crate::memory_workspace::admit_authored_entity_label(
            entity_identity_text,
        );
        let snapshot_identity = ForgeQuerySnapshotIdentity::from_relational_snapshot(
            RelationalBridgeSnapshotIdentityParts::new(1, 1),
        );
        let bridge_authority = build_bridge_authority_bundle(
            _bridge,
            &snapshot_identity,
            &command,
            &collection,
            &entity_identity,
            ForgeQueryMutationKind::Updated,
        )?;
        let receipt = ForgeQueryMutationReceipt {
            commit_identity: ForgeQueryCommitIdentity::from_relational_commit_id(1),
            snapshot_identity,
            deltas: vec![ForgeQueryMutationDelta {
                collection,
                entity_identity,
                kind: ForgeQueryMutationKind::Updated,
                aspect_paths,
            }],
            bridge_authority: Some(bridge_authority),
        };
        Ok(self.build_write_authority_execution_receipt(&command, receipt))
    }
}
