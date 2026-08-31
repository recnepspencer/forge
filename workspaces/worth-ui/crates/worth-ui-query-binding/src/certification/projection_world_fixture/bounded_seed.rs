use std::collections::BTreeMap;

use worth_foundational::facade::{AspectValue, CanonicalFieldPath, FieldKey};
use worth_query::facade::live::{
    current, declare, AspectFieldSelector, AspectName, AuthoredResultShapeField, FieldName,
    QuerySchemaView, ScalarAspectType, SchemaFieldView, WorthQueryLiveOpenOutcome,
    WorthQueryManagedLiveCloseOutcome,
};
use worth_query::facade::runtime::{
    WorthQueryAspectTouch, WorthQueryAuthoredAspectValue, WorthQueryMutationBatchBuilder,
};

use super::WorthUiCollectionProjectionSeedPosture;

pub(super) const MAX_ATOMIC_SEED_ROWS: usize = 1_024;

pub(super) fn collection_projection_workspace(
    rows: Vec<(String, String)>,
    posture: WorthUiCollectionProjectionSeedPosture,
) -> (
    worth_query::facade::runtime::WorthQueryWorkspace,
    Vec<worth_query::facade::foundation::WorthQueryEntityIdentity>,
) {
    let mut workspace = super::empty_collection_projection_workspace(posture);
    let mut receipt_entities = BTreeMap::new();
    for (batch_index, batch_rows) in rows.chunks(MAX_ATOMIC_SEED_ROWS).enumerate() {
        let receipt = workspace
            .submissions()
            .expect("certification seed submission lane")
            .submit_batch_builder(seed_batch(batch_rows, batch_index))
            .expect("bounded certification projection seed batch");
        assert_eq!(
            receipt.write_count(),
            batch_rows.len(),
            "Query must return one component receipt per seed row"
        );
        for write in receipt.write_receipts() {
            let entity = write
                .target_entity_identity()
                .expect("every admitted seed insertion has a Query target entity")
                .clone();
            assert!(
                receipt_entities
                    .insert(entity.evidence_identity().operational_key(), entity)
                    .is_none(),
                "Query seed receipts must carry unique target entities"
            );
        }
    }
    let entities = correlate_receipt_entities(&mut workspace, rows.len(), receipt_entities);
    (workspace, entities)
}

fn seed_batch(rows: &[(String, String)], batch_index: usize) -> WorthQueryMutationBatchBuilder {
    let first_item_key = batch_index * MAX_ATOMIC_SEED_ROWS + 1;
    rows.iter().enumerate().fold(
        WorthQueryMutationBatchBuilder::new(),
        |batch, (row_index, (identity, status))| {
            batch.insert("WorthUiProjectionText", |entity| {
                entity
                    .set_aspect(
                        touch("identity.id"),
                        WorthQueryAuthoredAspectValue::string(identity.clone()),
                    )
                    .set_aspect(
                        touch("query_text.status"),
                        WorthQueryAuthoredAspectValue::string(status.clone()),
                    )
                    .set_aspect(
                        touch("collection_item.status"),
                        WorthQueryAuthoredAspectValue::string(status.clone()),
                    )
                    .set_aspect(
                        touch("collection_item.key"),
                        WorthQueryAuthoredAspectValue::native(AspectValue::UInt64(
                            (first_item_key + row_index) as u64,
                        )),
                    )
            })
        },
    )
}

fn correlate_receipt_entities(
    workspace: &mut worth_query::facade::runtime::WorthQueryWorkspace,
    row_count: usize,
    mut receipt_entities: BTreeMap<
        worth_query::facade::runtime::WorthQueryEvidenceIdentityKey,
        worth_query::facade::foundation::WorthQueryEntityIdentity,
    >,
) -> Vec<worth_query::facade::foundation::WorthQueryEntityIdentity> {
    let opened = declare("worth-ui-certification-seed-correlation", |read| {
        read.local_collection(
            "WorthUiProjectionText",
            QuerySchemaView::new(
                "worth-ui-certification-seed-correlation",
                [
                    SchemaFieldView::new(
                        AspectName::new("identity").expect("seed identity aspect"),
                        FieldName::new("id").expect("seed identity field"),
                        ScalarAspectType::String,
                    ),
                    SchemaFieldView::new(
                        AspectName::new("collection_item").expect("seed item aspect"),
                        FieldName::new("key").expect("seed item key field"),
                        ScalarAspectType::UInt64,
                    ),
                ],
                [],
            ),
            |query| {
                query.project(
                    AspectFieldSelector::new("collection_item", "key")
                        .expect("seed item key selector"),
                )
            },
            |shape| {
                shape.field(
                    AuthoredResultShapeField::new("collection_item", "key", "collection_item.key")
                        .expect("seed item key result field"),
                )
            },
        )
    })
    .expect("seed correlation declaration")
    .using(current())
    .open(workspace);
    let correlation = match opened {
        WorthQueryLiveOpenOutcome::Opened(completion) => completion.into_handle(),
        WorthQueryLiveOpenOutcome::Stopped(stop) => {
            panic!("seed correlation Query view stopped: {:?}", stop.source())
        }
    };
    let item_key_path = field_path("collection_item.key");
    let read = correlation
        .read(workspace)
        .expect("seed correlation Query read");
    let mut entities_by_item_key = BTreeMap::new();
    for row in read.rows() {
        let item_key = match row.scalar_value_at(&item_key_path) {
            Some(AspectValue::UInt64(value)) => *value,
            _ => panic!("seed correlation item key must be one unsigned integer"),
        };
        let entity_key = row.identity().evidence_identity().operational_key();
        let entity = receipt_entities
            .remove(&entity_key)
            .expect("correlation rows must originate in seed receipts");
        assert!(
            entities_by_item_key.insert(item_key, entity).is_none(),
            "seed correlation item keys must be unique"
        );
    }
    assert!(
        receipt_entities.is_empty(),
        "every seed receipt must correlate"
    );
    match correlation.close(workspace) {
        WorthQueryManagedLiveCloseOutcome::Closed(closed) => assert!(closed.lane_terminal()),
        WorthQueryManagedLiveCloseOutcome::Stopped(stop) => {
            panic!("seed correlation Query close stopped: {:?}", stop.error())
        }
    }
    (1..=row_count as u64)
        .map(|item_key| {
            entities_by_item_key
                .remove(&item_key)
                .expect("every authored seed item key must correlate")
        })
        .collect()
}

fn touch(path: &str) -> WorthQueryAspectTouch {
    WorthQueryAspectTouch::from_authoring_ingress_text(path).expect("projection seed touch")
}

fn field_path(path: &str) -> CanonicalFieldPath {
    CanonicalFieldPath::new(
        path.split('.')
            .map(|segment| FieldKey::new(segment).expect("seed field path segment")),
    )
    .expect("seed field path")
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use crate::{
        UiCollectionProjectionBindingAdmission, UiCollectionProjectionBudget,
        UiCollectionProjectionOpenOutcome, UiCollectionProjectionRegistration,
        UiLiveCollectionProjectionCloseOutcome, UiPresentProjection, UiProjectionAvailability,
        UiProjectionFieldRequirement, WorthUiQueryWorkspaceExt,
    };

    #[test]
    fn receipt_entities_correlate_to_authored_order_across_batches() {
        let row_count = MAX_ATOMIC_SEED_ROWS + 1;
        let rows = (0..row_count)
            .map(|index| {
                (
                    format!("receipt-order.{index:05}"),
                    format!("Value {index:05}"),
                )
            })
            .collect();
        let (mut workspace, entities) =
            collection_projection_workspace(rows, WorthUiCollectionProjectionSeedPosture::Complete);
        assert_eq!(entities.len(), row_count);

        let installed = workspace.worth_ui().expect("WORTH UI domain installed");
        let registration = UiCollectionProjectionRegistration::text(
            installed
                .projection_view("certification.collection.qp04")
                .expect("QP04 installed collection view"),
            UiProjectionFieldRequirement::declared("identity.id").expect("row identity field"),
            [UiProjectionFieldRequirement::declared("status").expect("selected field")],
            false,
            true,
        )
        .expect("QP04 collection registration");
        let UiCollectionProjectionBindingAdmission::Ready(binding) = registration.admit(&workspace)
        else {
            panic!("QP04 binding must admit");
        };
        let budget = UiCollectionProjectionBudget::new(row_count as u32, 131_072, 1, 8_388_608)
            .expect("QP04 collection budget");
        let UiCollectionProjectionOpenOutcome::Opened(opened) =
            binding.open(budget, &mut workspace)
        else {
            panic!("QP04 collection projection must open");
        };
        let (live, fact) = opened.into_parts();
        let UiProjectionAvailability::Present(UiPresentProjection::Current(value)) =
            fact.availability()
        else {
            panic!("bounded seed must produce current collection truth");
        };
        let query_entities_by_status = value
            .rows()
            .iter()
            .map(|row| {
                (
                    row.selected_values()[0].as_str().to_owned(),
                    row.row().query_identity().operational_key(),
                )
            })
            .collect::<BTreeMap<_, _>>();
        for (index, entity) in entities.iter().enumerate() {
            assert_eq!(
                query_entities_by_status.get(&format!("Value {index:05}")),
                Some(&entity.evidence_identity().operational_key())
            );
        }
        let UiLiveCollectionProjectionCloseOutcome::Closed(closed) = live.close(&mut workspace)
        else {
            panic!("QP04 live collection owner must close");
        };
        assert!(closed.owner_terminal());
    }
}
