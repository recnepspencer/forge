use std::collections::BTreeMap;

use worth_query::facade::installed::collection::{
    WorthQueryCollectionPatchApplicationReceipt, WorthQueryCollectionPatchOperation,
    WorthQueryCollectionRowHandle,
};

use super::{
    derive_collection_projection, UiCollectionDerivationContext, UiCollectionProjectionChange,
    UiCollectionProjectionFactReceipt, UiCollectionProjectionRowReference,
};

pub(crate) fn derive_applied_collection_projection(
    context: UiCollectionDerivationContext<'_>,
    receipt: &WorthQueryCollectionPatchApplicationReceipt,
) -> UiCollectionProjectionFactReceipt {
    let mut changed_rows = BTreeMap::new();
    let mut changes = Vec::new();
    for operation in receipt.operations() {
        translate_operation(operation, &mut changed_rows, &mut changes);
    }
    let rows = changed_rows.values().copied().collect::<Vec<_>>();
    derive_collection_projection(context, &rows, changes.into_boxed_slice())
}

fn translate_operation<'a>(
    operation: &'a WorthQueryCollectionPatchOperation,
    changed_rows: &mut BTreeMap<
        worth_query::facade::foundation::WorthQueryEntityIdentity,
        &'a WorthQueryCollectionRowHandle,
    >,
    changes: &mut Vec<UiCollectionProjectionChange>,
) {
    match operation {
        WorthQueryCollectionPatchOperation::Insert { row, at } => {
            changed_rows.insert(row.entity_identity().clone(), row);
            changes.push(UiCollectionProjectionChange::Insert {
                row: row_reference(row.entity_identity()),
                at: *at,
            });
        }
        WorthQueryCollectionPatchOperation::Remove { entity, from } => {
            changes.push(UiCollectionProjectionChange::Remove {
                row: row_reference(entity),
                from: *from,
            });
        }
        WorthQueryCollectionPatchOperation::Move { row, from, to } => {
            changes.push(UiCollectionProjectionChange::Move {
                row: row_reference(row.entity_identity()),
                from: *from,
                to: *to,
            });
        }
        WorthQueryCollectionPatchOperation::Update { row } => {
            changed_rows.insert(row.entity_identity().clone(), row);
            changes.push(UiCollectionProjectionChange::Update {
                row: row_reference(row.entity_identity()),
            });
        }
        WorthQueryCollectionPatchOperation::WindowShift { .. } => {
            changes.push(UiCollectionProjectionChange::WindowShift);
        }
        WorthQueryCollectionPatchOperation::ResetRequired { reason, .. } => {
            changes.push(UiCollectionProjectionChange::ResetRequired {
                reason: crate::collection_delivery::map_reset_reason(*reason),
            });
        }
        WorthQueryCollectionPatchOperation::ResultState { .. }
        | WorthQueryCollectionPatchOperation::Warnings { .. }
        | WorthQueryCollectionPatchOperation::Continuation { .. } => {}
    }
}

fn row_reference(
    identity: &worth_query::facade::foundation::WorthQueryEntityIdentity,
) -> UiCollectionProjectionRowReference {
    UiCollectionProjectionRowReference::query_issued(identity.evidence_identity())
}
