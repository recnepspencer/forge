use std::collections::BTreeSet;

use crate::runtime::{
    WorthQueryAspectMutationOperation, WorthQueryAspectTouch, WorthQueryGraphTouchDescriptor,
    WorthQueryMutationTargetCollectionIdentity,
};

use super::{
    WorthQueryGraphObligationCollectionLookupIdentity, WorthQueryGraphObligationTouchLookupKey,
};

pub(in crate::runtime::mutation::graph_composition::obligation::index) fn touch_lookup_keys_for_descriptor(
    descriptor: &WorthQueryGraphTouchDescriptor,
) -> Vec<WorthQueryGraphObligationTouchLookupKey> {
    let mut keys = BTreeSet::new();
    keys.insert(WorthQueryGraphObligationTouchLookupKey::AnyGraphTouch);
    for row in descriptor.rows() {
        insert_row_collection_key(&mut keys, row.declared_collection_identity());
        insert_row_relation_kind_key(&mut keys, row.relation_kind_id());
        insert_row_declared_aspect_operation_keys(&mut keys, row.declared_aspect_operations());
        insert_row_touched_aspect_keys(&mut keys, row.admitted_touched_aspects());
        if let Some(read_verb) = row.read_verb() {
            keys.insert(WorthQueryGraphObligationTouchLookupKey::ReadVerb(read_verb));
        } else {
            keys.insert(WorthQueryGraphObligationTouchLookupKey::MutationFamily(
                row.mutation_family(),
            ));
        }
        if let Some(lifecycle_family) = row.lifecycle_family() {
            keys.insert(WorthQueryGraphObligationTouchLookupKey::LifecycleFamily(
                lifecycle_family,
            ));
        }
    }
    keys.into_iter().collect()
}

fn insert_row_collection_key(
    keys: &mut BTreeSet<WorthQueryGraphObligationTouchLookupKey>,
    collection: Option<&WorthQueryMutationTargetCollectionIdentity>,
) {
    if let Some(collection) = collection {
        keys.insert(WorthQueryGraphObligationTouchLookupKey::Collection(
            WorthQueryGraphObligationCollectionLookupIdentity::from_collection_identity(collection),
        ));
    }
}

fn insert_row_relation_kind_key(
    keys: &mut BTreeSet<WorthQueryGraphObligationTouchLookupKey>,
    relation_kind_id: Option<worth_relational::facade::identity::KindId>,
) {
    if let Some(relation_kind_id) = relation_kind_id {
        keys.insert(WorthQueryGraphObligationTouchLookupKey::RelationKindId(
            relation_kind_id,
        ));
    }
}

fn insert_row_declared_aspect_operation_keys(
    keys: &mut BTreeSet<WorthQueryGraphObligationTouchLookupKey>,
    declared_aspect_operations: &[WorthQueryAspectMutationOperation],
) {
    for operation in declared_aspect_operations {
        keys.insert(
            WorthQueryGraphObligationTouchLookupKey::DeclaredAspectOperation(operation.clone()),
        );
        keys.insert(WorthQueryGraphObligationTouchLookupKey::AspectTouch(
            operation.aspect_touch().clone(),
        ));
    }
}

fn insert_row_touched_aspect_keys(
    keys: &mut BTreeSet<WorthQueryGraphObligationTouchLookupKey>,
    touched_aspects: &[WorthQueryAspectTouch],
) {
    for aspect_touch in touched_aspects {
        keys.insert(WorthQueryGraphObligationTouchLookupKey::AspectTouch(
            aspect_touch.clone(),
        ));
    }
}
