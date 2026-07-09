use worth_relational::facade::runtime::RelationalRuntime;
use worth_runtime_bridge::facade::RuntimeBridge;

use crate::facade::{
    WorthQueryAspectTouch, WorthQueryBackendAdmissibleMutation, WorthQueryMutationDelta,
    WorthQueryMutationKind, WorthQueryMutationReceipt, WorthQueryRuntimeWriteAuthorityAdapter,
    WorthQueryWorkspaceError, WriteAuthorityExecutionReceipt,
};
use crate::memory_workspace::{WorthQueryCommitIdentity, WorthQuerySnapshotIdentity};
use crate::runtime::build_bridge_authority_bundle;
use worth_runtime_bridge::facade::RelationalBridgeSnapshotIdentityParts;

pub(super) struct TranscriptWriteAuthority;

impl WorthQueryRuntimeWriteAuthorityAdapter for TranscriptWriteAuthority {
    fn write(
        &mut self,
        _bridge: &RuntimeBridge,
        _relational_runtime: Option<&mut RelationalRuntime>,
        mutation: WorthQueryBackendAdmissibleMutation,
    ) -> Result<WriteAuthorityExecutionReceipt, WorthQueryWorkspaceError> {
        let collection = mutation
            .declared_collection_identity()
            .map(|collection| collection.as_str().to_string())
            .unwrap_or("TranscriptEntity".to_string());
        let aspect_touches: Vec<WorthQueryAspectTouch> = mutation.declared_aspect_touches();
        let entity_identity_text = "transcript-entity-1";
        let entity_identity =
            crate::memory_workspace::admit_authored_entity_label(entity_identity_text);
        let snapshot_identity = WorthQuerySnapshotIdentity::from_relational_snapshot(
            RelationalBridgeSnapshotIdentityParts::new(1, 1),
        );
        let bridge_authority = build_bridge_authority_bundle(
            _bridge,
            &snapshot_identity,
            &mutation,
            &collection,
            &entity_identity,
            WorthQueryMutationKind::Updated,
        )?;
        let receipt = WorthQueryMutationReceipt::from_bridge_authoritative_parts(
            WorthQueryCommitIdentity::from_relational_commit_id(1),
            snapshot_identity,
            vec![WorthQueryMutationDelta::from_touched_aspects(
                collection.clone(),
                entity_identity,
                WorthQueryMutationKind::Updated,
                aspect_touches,
            )],
            bridge_authority,
        );
        Ok(self.build_write_authority_execution_receipt(&mutation, receipt))
    }
}
