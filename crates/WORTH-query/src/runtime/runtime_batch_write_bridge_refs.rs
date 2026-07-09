use crate::memory_workspace::WorthQueryEntityIdentity;
use crate::runtime::{
    WorthQueryMutationTargetCollectionIdentity, WorthQueryRuntimeError,
    WorthQuerySymbolicTargetReference, WorthQueryWorkspaceError,
};
use worth_runtime_bridge::facade::{
    BridgeSymbolicTargetCollection, BridgeSymbolicTargetReferenceBundle,
    BridgeSymbolicTargetResolvedEntityIdentity, BridgeSymbolicTargetSymbolIdentity,
};

pub(super) fn bridge_symbolic_target_reference(
    reference: &WorthQuerySymbolicTargetReference,
    resolved_entity_identity: &WorthQueryEntityIdentity,
    resolved_collection: Option<&WorthQueryMutationTargetCollectionIdentity>,
) -> Result<BridgeSymbolicTargetReferenceBundle, WorthQueryRuntimeError> {
    let resolved_entity_identity = resolved_entity_identity
        .relational_record_parts()
        .map(BridgeSymbolicTargetResolvedEntityIdentity::from_relational_record)
        .ok_or_else(|| {
            WorthQueryRuntimeError::Workspace(WorthQueryWorkspaceError::new(
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
