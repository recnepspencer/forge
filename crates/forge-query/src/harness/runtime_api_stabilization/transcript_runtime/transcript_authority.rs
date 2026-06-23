use forge_relational::facade::runtime::RelationalRuntime;
use forge_runtime_bridge::facade::RuntimeBridge;

use crate::facade::{
    ForgeQueryAspectTouch, ForgeQueryBackendAdmissibleMutation, ForgeQueryMutationDelta,
    ForgeQueryMutationKind, ForgeQueryMutationReceipt, ForgeQueryRuntimeWriteAuthorityAdapter,
    ForgeQueryWorkspaceError, WriteAuthorityExecutionReceipt,
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
        mutation: ForgeQueryBackendAdmissibleMutation,
    ) -> Result<WriteAuthorityExecutionReceipt, ForgeQueryWorkspaceError> {
        let collection = mutation
            .declared_collection_identity()
            .map(|collection| collection.as_str().to_string())
            .unwrap_or("TranscriptEntity".to_string());
        let aspect_touches: Vec<ForgeQueryAspectTouch> = mutation.declared_aspect_touches();
        let entity_identity_text = "transcript-entity-1";
        let entity_identity =
            crate::memory_workspace::admit_authored_entity_label(entity_identity_text);
        let snapshot_identity = ForgeQuerySnapshotIdentity::from_relational_snapshot(
            RelationalBridgeSnapshotIdentityParts::new(1, 1),
        );
        let bridge_authority = build_bridge_authority_bundle(
            _bridge,
            &snapshot_identity,
            &mutation,
            &collection,
            &entity_identity,
            ForgeQueryMutationKind::Updated,
        )?;
        let receipt = ForgeQueryMutationReceipt::from_bridge_authoritative_parts(
            ForgeQueryCommitIdentity::from_relational_commit_id(1),
            snapshot_identity,
            vec![ForgeQueryMutationDelta::from_touched_aspects(
                collection.clone(),
                entity_identity,
                ForgeQueryMutationKind::Updated,
                aspect_touches,
            )],
            bridge_authority,
        );
        Ok(self.build_write_authority_execution_receipt(&mutation, receipt))
    }
}
