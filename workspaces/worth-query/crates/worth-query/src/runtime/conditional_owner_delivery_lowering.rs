use super::{WorthQueryMutationTargetCollectionIdentity, WorthQueryRuntimeError};

pub(super) fn owner_mutation_receipt(
    target_collection: WorthQueryMutationTargetCollectionIdentity,
    receipt: &worth_runtime_bridge::facade::BridgeCorrespondenceDeliveryReceipt,
) -> Result<crate::memory_workspace::WorthQueryMutationReceipt, WorthQueryRuntimeError> {
    let snapshot =
        crate::memory_workspace::WorthQuerySnapshotIdentity::from_admitted_bridge_snapshot_identity(
            receipt.change_set().admitted_snapshot_identity(),
        )
        .ok_or_else(|| owner_routing_error("invalid relational snapshot identity"))?;
    let deltas = receipt
        .change_set()
        .changes()
        .iter()
        .map(|change| owner_mutation_delta(target_collection.clone(), change))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(
        crate::memory_workspace::WorthQueryMutationReceipt::from_authoritative_parts(
            crate::memory_workspace::WorthQueryCommitIdentity::from_admitted_bridge_commit_identity(
                receipt.change_set().admitted_commit_identity(),
            ),
            snapshot,
            deltas,
        ),
    )
}

fn owner_mutation_delta(
    target_collection: WorthQueryMutationTargetCollectionIdentity,
    change: &worth_runtime_bridge::facade::BridgeDeliveredCorrespondenceChange,
) -> Result<crate::memory_workspace::WorthQueryMutationDelta, WorthQueryRuntimeError> {
    let record = change.admitted_record_identity().ok_or_else(|| {
        owner_routing_error("owner change lacks an admitted relational record identity")
    })?;
    let (kind, touches) = classify_owner_change(change)?;
    Ok(
        crate::memory_workspace::WorthQueryMutationDelta::from_collection_identity(
            target_collection,
            crate::memory_workspace::WorthQueryEntityIdentity::from_admitted_bridge_record_identity(
                &record,
            ),
            kind,
            touches,
        ),
    )
}

fn classify_owner_change(
    change: &worth_runtime_bridge::facade::BridgeDeliveredCorrespondenceChange,
) -> Result<
    (
        crate::memory_workspace::WorthQueryMutationKind,
        Vec<super::WorthQueryAspectTouch>,
    ),
    WorthQueryRuntimeError,
> {
    if let Some(semantic) = change.semantic_change() {
        let touch = match semantic.field_path() {
            Some(path) => super::WorthQueryAspectTouch::aspect_field_path(
                semantic.aspect_key().clone(),
                path.clone(),
            ),
            None => super::WorthQueryAspectTouch::whole_aspect(semantic.aspect_key().clone()),
        };
        return Ok((
            crate::memory_workspace::WorthQueryMutationKind::Updated,
            vec![touch],
        ));
    }
    let structural = change
        .structural_change()
        .ok_or_else(|| owner_routing_error("owner change has no admitted semantic meaning"))?;
    let kind = classify_structural_change(structural.kind())?;
    Ok((kind, Vec::new()))
}

fn classify_structural_change(
    kind: worth_runtime_bridge::facade::BridgeCommittedRecordChangeKind,
) -> Result<crate::memory_workspace::WorthQueryMutationKind, WorthQueryRuntimeError> {
    use worth_runtime_bridge::facade::BridgeCommittedRecordChangeKind as StructuralKind;
    match kind {
        StructuralKind::Created => Ok(crate::memory_workspace::WorthQueryMutationKind::Created),
        StructuralKind::Updated => Ok(crate::memory_workspace::WorthQueryMutationKind::Updated),
        StructuralKind::Deleted => Ok(crate::memory_workspace::WorthQueryMutationKind::Deleted),
        StructuralKind::RetainedForAudit => Err(owner_routing_error(
            "retained-for-audit owner change has no live patch equivalent",
        )),
    }
}

fn owner_routing_error(message: &str) -> WorthQueryRuntimeError {
    WorthQueryRuntimeError::LiveSubscriptionInstallation {
        view_name: "installed-live".to_string(),
        stage: "classified-owner-delivery",
        message: message.to_string(),
    }
}
