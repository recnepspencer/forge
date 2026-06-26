use crate::memory_workspace::ForgeQueryEntityIdentity;
use crate::runtime::{
    ForgeQueryMutationTargetCollectionIdentity, ForgeQueryRuntimeError,
    ForgeQuerySymbolicTargetReference, ForgeQueryWorkspaceError,
};
use forge_runtime_bridge::facade::{
    BridgeSymbolicTargetCollection, BridgeSymbolicTargetReferenceBundle,
    BridgeSymbolicTargetResolvedEntityIdentity, BridgeSymbolicTargetSymbolIdentity,
};

pub(super) fn bridge_symbolic_target_reference(
    reference: &ForgeQuerySymbolicTargetReference,
    resolved_entity_identity: &ForgeQueryEntityIdentity,
    resolved_collection: Option<&ForgeQueryMutationTargetCollectionIdentity>,
) -> Result<BridgeSymbolicTargetReferenceBundle, ForgeQueryRuntimeError> {
    let resolved_entity_identity = resolved_entity_identity
        .relational_record_parts()
        .map(BridgeSymbolicTargetResolvedEntityIdentity::from_relational_record)
        .ok_or_else(|| {
            ForgeQueryRuntimeError::Workspace(ForgeQueryWorkspaceError::new(
                "same-batch symbolic target bridge lowering requires a relational resolved entity identity",
            ))
        })?;
    Ok(BridgeSymbolicTargetReferenceBundle::same_batch_target(
        BridgeSymbolicTargetSymbolIdentity::from_external_symbol_evidence(reference.symbol()),
        resolved_entity_identity,
        resolved_collection
            .map(|collection| BridgeSymbolicTargetCollection::new(collection.as_str())),
    ))
}
