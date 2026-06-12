use forge_relational::facade::runtime::RelationalRuntime;
use forge_runtime_bridge::facade::RuntimeBridge;

use crate::facade::{
    ForgeQueryMutationDelta, ForgeQueryMutationKind, ForgeQueryMutationReceipt,
    ForgeQueryRuntimeWriteAuthorityAdapter, ForgeQueryWorkspaceError, ForgeQueryWriteCommand,
    WriteAuthorityExecutionReceipt,
};
use crate::runtime::build_bridge_authority_bundle;
use forge_runtime_bridge::facade::RelationalBridgeSnapshotIdentityParts;

use super::certification_entity_identity;

pub(super) struct CertificationWriteAuthority;

impl ForgeQueryRuntimeWriteAuthorityAdapter for CertificationWriteAuthority {
    fn write(
        &mut self,
        bridge: &RuntimeBridge,
        _relational_runtime: Option<&mut RelationalRuntime>,
        command: ForgeQueryWriteCommand,
    ) -> Result<WriteAuthorityExecutionReceipt, ForgeQueryWorkspaceError> {
        let (collection, aspect_paths) = write_receipt_shape(&command);
        let entity_identity = "certification-entity-1";
        let commit_identity =
            crate::memory_workspace::ForgeQueryCommitIdentity::from_relational_commit_id(1);
        let snapshot_identity =
            crate::memory_workspace::ForgeQuerySnapshotIdentity::from_relational_snapshot(
                RelationalBridgeSnapshotIdentityParts::new(1, 1),
            );
        let entity_identity_handle = certification_entity_identity(entity_identity);
        let bridge_authority = build_bridge_authority_bundle(
            bridge,
            &snapshot_identity,
            &command,
            &collection,
            &entity_identity_handle,
            ForgeQueryMutationKind::Updated,
        )?;
        let receipt = ForgeQueryMutationReceipt {
            commit_identity,
            snapshot_identity,
            deltas: vec![ForgeQueryMutationDelta {
                collection,
                entity_identity: entity_identity_handle,
                kind: ForgeQueryMutationKind::Updated,
                aspect_paths,
            }],
            bridge_authority: Some(bridge_authority),
        };
        Ok(self.build_write_authority_execution_receipt(&command, receipt))
    }
}

fn write_receipt_shape(command: &ForgeQueryWriteCommand) -> (String, Vec<String>) {
    match command {
        ForgeQueryWriteCommand::UpdateAspect { aspect_path, .. } => {
            ("Task".to_string(), vec![aspect_path.clone()])
        }
        ForgeQueryWriteCommand::UpdateAspects { aspects, .. } => (
            "Task".to_string(),
            aspects
                .iter()
                .map(|aspect| aspect.aspect_path().to_string())
                .collect(),
        ),
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
            binding.target_collection().unwrap_or("Task").to_string(),
            aspects
                .iter()
                .map(|aspect| aspect.aspect_path().to_string())
                .collect(),
        ),
        ForgeQueryWriteCommand::VerifyThenDeleteExistingAspects {
            binding,
            touched_aspect_paths,
            ..
        }
        | ForgeQueryWriteCommand::DeleteExistingAspects {
            binding,
            touched_aspect_paths,
            ..
        } => (
            binding.target_collection().unwrap_or("Task").to_string(),
            touched_aspect_paths.clone(),
        ),
        ForgeQueryWriteCommand::UpdateSymbolicAspects {
            aspects, reference, ..
        } => (
            reference.target_collection().unwrap_or("Task").to_string(),
            aspects
                .iter()
                .map(|aspect| aspect.aspect_path().to_string())
                .collect(),
        ),
        ForgeQueryWriteCommand::DeleteAspects {
            touched_aspect_paths,
            ..
        }
        | ForgeQueryWriteCommand::DeleteSymbolicAspects {
            touched_aspect_paths,
            ..
        } => ("Task".to_string(), touched_aspect_paths.clone()),
        ForgeQueryWriteCommand::Delete { .. } => ("Task".to_string(), Vec::new()),
    }
}
