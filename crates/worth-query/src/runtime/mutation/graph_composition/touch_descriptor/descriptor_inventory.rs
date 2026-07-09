use std::collections::BTreeSet;

use crate::runtime::{
    WorthQueryAspectMutationOperation, WorthQueryAspectTouch, WorthQueryMutationFamily,
    WorthQueryMutationTargetCollectionIdentity,
};
use worth_relational::facade::identity::KindId;

use super::touch_rows::WorthQueryGraphTouchDescriptorRow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct WorthQueryGraphTouchDescriptorInventory {
    declared_collections: Vec<WorthQueryMutationTargetCollectionIdentity>,
    relation_kind_ids: BTreeSet<KindId>,
    declared_aspect_touches: BTreeSet<WorthQueryAspectTouch>,
    declared_aspect_operations: BTreeSet<WorthQueryAspectMutationOperation>,
    touched_aspects: BTreeSet<WorthQueryAspectTouch>,
    insert_command_count: usize,
    update_command_count: usize,
    assertion_command_count: usize,
    delete_command_count: usize,
}

impl WorthQueryGraphTouchDescriptorInventory {
    pub(super) fn from_rows(rows: &[WorthQueryGraphTouchDescriptorRow]) -> Self {
        let declared_collections = collect_declared_collections(rows);
        let relation_kind_ids = collect_relation_kind_ids(rows);
        let declared_aspect_operations = collect_declared_aspect_operations(rows);
        let declared_aspect_touches = collect_declared_aspect_touches(&declared_aspect_operations);
        let touched_aspects = collect_touched_aspects(rows);
        let insert_command_count = count_command_family(rows, WorthQueryMutationFamily::Insert);
        let update_command_count = count_command_family(rows, WorthQueryMutationFamily::Update);
        let assertion_command_count =
            count_command_family(rows, WorthQueryMutationFamily::Assertion);
        let delete_command_count = count_command_family(rows, WorthQueryMutationFamily::Delete);

        Self {
            declared_collections,
            relation_kind_ids,
            declared_aspect_touches,
            declared_aspect_operations,
            touched_aspects,
            insert_command_count,
            update_command_count,
            assertion_command_count,
            delete_command_count,
        }
    }

    pub(super) fn insert_command_count(&self) -> usize {
        self.insert_command_count
    }

    pub(super) fn update_command_count(&self) -> usize {
        self.update_command_count
    }

    pub(super) fn assertion_command_count(&self) -> usize {
        self.assertion_command_count
    }

    pub(super) fn delete_command_count(&self) -> usize {
        self.delete_command_count
    }

    pub(super) fn declared_collection_count(&self) -> usize {
        self.declared_collections.len()
    }

    pub(super) fn relation_kind_count(&self) -> usize {
        self.relation_kind_ids.len()
    }

    pub(super) fn declared_aspect_touch_count(&self) -> usize {
        self.declared_aspect_touches.len()
    }

    pub(super) fn declared_aspect_operation_count(&self) -> usize {
        self.declared_aspect_operations.len()
    }

    pub(super) fn touched_aspect_count(&self) -> usize {
        self.touched_aspects.len()
    }
}

fn collect_declared_collections(
    rows: &[WorthQueryGraphTouchDescriptorRow],
) -> Vec<WorthQueryMutationTargetCollectionIdentity> {
    let mut collections = Vec::new();
    for collection in rows
        .iter()
        .filter_map(WorthQueryGraphTouchDescriptorRow::declared_collection_identity)
    {
        if !collections
            .iter()
            .any(|existing: &WorthQueryMutationTargetCollectionIdentity| {
                existing.same_target_collection_as(collection)
            })
        {
            collections.push(collection.clone());
        }
    }
    collections
}

fn collect_relation_kind_ids(rows: &[WorthQueryGraphTouchDescriptorRow]) -> BTreeSet<KindId> {
    rows.iter()
        .filter_map(WorthQueryGraphTouchDescriptorRow::relation_kind_id)
        .collect()
}

fn collect_declared_aspect_operations(
    rows: &[WorthQueryGraphTouchDescriptorRow],
) -> BTreeSet<WorthQueryAspectMutationOperation> {
    rows.iter()
        .flat_map(WorthQueryGraphTouchDescriptorRow::declared_aspect_operations)
        .cloned()
        .collect()
}

fn collect_declared_aspect_touches(
    declared_aspect_operations: &BTreeSet<WorthQueryAspectMutationOperation>,
) -> BTreeSet<WorthQueryAspectTouch> {
    declared_aspect_operations
        .iter()
        .map(|operation| operation.aspect_touch().clone())
        .collect()
}

fn collect_touched_aspects(
    rows: &[WorthQueryGraphTouchDescriptorRow],
) -> BTreeSet<WorthQueryAspectTouch> {
    rows.iter()
        .flat_map(WorthQueryGraphTouchDescriptorRow::admitted_touched_aspects)
        .cloned()
        .collect()
}

fn count_command_family(
    rows: &[WorthQueryGraphTouchDescriptorRow],
    family: WorthQueryMutationFamily,
) -> usize {
    rows.iter()
        .filter(|row| row.mutation_family() == family)
        .count()
}
