use forge_relational::facade::runtime::RelationalRuntime;
use forge_runtime_bridge::facade::RuntimeBridge;

use crate::facade::{
    ForgeQueryMutationDelta, ForgeQueryMutationKind, ForgeQueryMutationReceipt,
    ForgeQueryRuntimeWriteAuthorityAdapter, ForgeQueryWorkspaceError, ForgeQueryWriteCommand,
};

pub(super) struct TranscriptWriteAuthority;

impl ForgeQueryRuntimeWriteAuthorityAdapter for TranscriptWriteAuthority {
    fn write(
        &mut self,
        _bridge: &RuntimeBridge,
        _relational_runtime: Option<&mut RelationalRuntime>,
        command: ForgeQueryWriteCommand,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        let (collection, aspect_paths) = match command {
            ForgeQueryWriteCommand::InsertAspects {
                collection,
                aspects,
                ..
            } => (
                collection,
                aspects
                    .iter()
                    .map(|aspect| aspect.aspect_path().to_string())
                    .collect(),
            ),
            ForgeQueryWriteCommand::UpdateAspect { aspect_path, .. } => {
                ("TranscriptEntity".to_string(), vec![aspect_path])
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
                touched_aspect_paths,
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
            } => ("TranscriptEntity".to_string(), touched_aspect_paths),
            ForgeQueryWriteCommand::DeleteExistingAspects {
                binding,
                touched_aspect_paths,
                ..
            } => (
                binding
                    .target_collection()
                    .unwrap_or("TranscriptEntity")
                    .to_string(),
                touched_aspect_paths,
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
                touched_aspect_paths,
            ),
            ForgeQueryWriteCommand::Delete { .. } => ("TranscriptEntity".to_string(), Vec::new()),
        };
        Ok(ForgeQueryMutationReceipt {
            commit_identity: format!("transcript-commit:{collection}"),
            snapshot_token: format!("transcript-snapshot:{collection}"),
            deltas: vec![ForgeQueryMutationDelta {
                collection,
                entity_identity: "transcript-entity-1".to_string(),
                kind: ForgeQueryMutationKind::Updated,
                aspect_paths,
            }],
            bridge_authority: None,
        })
    }
}
