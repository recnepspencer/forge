use std::collections::BTreeSet;

use crate::runtime::ForgeQueryGraphTouchDescriptor;

use super::ForgeQueryGraphObligationTouchLookupKey;

pub(in crate::runtime::mutation::graph_composition::obligation::index) fn touch_lookup_keys_for_descriptor(
    descriptor: &ForgeQueryGraphTouchDescriptor,
) -> Vec<ForgeQueryGraphObligationTouchLookupKey> {
    let mut keys = BTreeSet::new();
    keys.insert(ForgeQueryGraphObligationTouchLookupKey::AnyGraphTouch);
    for row in descriptor.rows() {
        insert_row_collection_key(&mut keys, row.declared_collection());
        insert_row_relation_kind_key(&mut keys, row.relation_kind_id());
        insert_row_declared_aspect_operation_keys(&mut keys, row.declared_aspect_operations());
        insert_row_touched_aspect_path_keys(&mut keys, row.touched_aspect_paths());
        if let Some(read_verb) = row.read_verb() {
            keys.insert(ForgeQueryGraphObligationTouchLookupKey::ReadVerb(read_verb));
        } else {
            keys.insert(ForgeQueryGraphObligationTouchLookupKey::MutationFamily(
                row.mutation_family(),
            ));
        }
        if let Some(lifecycle_family) = row.lifecycle_family() {
            keys.insert(ForgeQueryGraphObligationTouchLookupKey::LifecycleFamily(
                lifecycle_family,
            ));
        }
    }
    keys.into_iter().collect()
}

fn insert_row_collection_key(
    keys: &mut BTreeSet<ForgeQueryGraphObligationTouchLookupKey>,
    collection: Option<&str>,
) {
    if let Some(collection) = collection {
        keys.insert(ForgeQueryGraphObligationTouchLookupKey::Collection(
            collection.to_string(),
        ));
    }
}

fn insert_row_relation_kind_key(
    keys: &mut BTreeSet<ForgeQueryGraphObligationTouchLookupKey>,
    relation_kind_id: Option<forge_relational::facade::identity::KindId>,
) {
    if let Some(relation_kind_id) = relation_kind_id {
        keys.insert(ForgeQueryGraphObligationTouchLookupKey::RelationKindId(
            relation_kind_id,
        ));
    }
}

fn insert_row_declared_aspect_operation_keys(
    keys: &mut BTreeSet<ForgeQueryGraphObligationTouchLookupKey>,
    declared_aspect_operations: &[String],
) {
    for operation in declared_aspect_operations {
        keys.insert(
            ForgeQueryGraphObligationTouchLookupKey::DeclaredAspectOperation(operation.clone()),
        );
        if let Some((_, aspect_path)) = operation.split_once(':') {
            keys.insert(ForgeQueryGraphObligationTouchLookupKey::AspectPath(
                aspect_path.to_string(),
            ));
        }
    }
}

fn insert_row_touched_aspect_path_keys(
    keys: &mut BTreeSet<ForgeQueryGraphObligationTouchLookupKey>,
    touched_aspect_paths: &[String],
) {
    for aspect_path in touched_aspect_paths {
        keys.insert(ForgeQueryGraphObligationTouchLookupKey::AspectPath(
            aspect_path.clone(),
        ));
    }
}
