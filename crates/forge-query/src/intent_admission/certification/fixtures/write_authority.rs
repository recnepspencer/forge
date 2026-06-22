use forge_relational::facade::runtime::RelationalRuntime;
use forge_runtime_bridge::facade::RuntimeBridge;

use crate::facade::{
    ForgeQueryAspectTouch, ForgeQueryBackendAdmissibleMutation, ForgeQueryMutationDelta,
    ForgeQueryMutationKind, ForgeQueryMutationReceipt, ForgeQueryRuntimeWriteAuthorityAdapter,
    ForgeQueryWorkspaceError, WriteAuthorityExecutionReceipt,
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
        mutation: ForgeQueryBackendAdmissibleMutation,
    ) -> Result<WriteAuthorityExecutionReceipt, ForgeQueryWorkspaceError> {
        let (collection, aspect_touches) = write_receipt_shape(&mutation);
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
            &mutation,
            &collection,
            &entity_identity_handle,
            ForgeQueryMutationKind::Updated,
        )?;
        let receipt = ForgeQueryMutationReceipt::from_bridge_authoritative_parts(
            commit_identity,
            snapshot_identity,
            vec![ForgeQueryMutationDelta::from_touched_aspects(
                collection,
                entity_identity_handle,
                ForgeQueryMutationKind::Updated,
                aspect_touches,
            )],
            bridge_authority,
        );
        Ok(self.build_write_authority_execution_receipt(&mutation, receipt))
    }
}

fn write_receipt_shape(
    mutation: &ForgeQueryBackendAdmissibleMutation,
) -> (String, Vec<ForgeQueryAspectTouch>) {
    (
        mutation
            .declared_collection_identity()
            .map(|collection| collection.as_str().to_string())
            .unwrap_or("Task".to_string()),
        mutation.declared_aspect_touches(),
    )
}
