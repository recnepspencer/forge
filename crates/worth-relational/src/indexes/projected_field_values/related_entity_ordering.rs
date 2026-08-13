use std::cmp::Ordering;
use std::collections::BTreeMap;

use crate::identity::data::KindId;
use crate::indexes::data::{
    RelatedEntityEndpoint, RelatedEntityOrderingDirection, RelatedEntityOrderingEntry,
    RelatedEntityOrderingField,
};
use crate::runtime::RelationalRuntime;

use super::{entity_aspect_field_ordering_value, IndexProjectionSource};

pub(in crate::indexes) fn build_related_entity_ordering_index(
    runtime: &RelationalRuntime,
    projection: &IndexProjectionSource<'_, '_>,
    relation_kind: KindId,
    parent_endpoint: RelatedEntityEndpoint,
    child_kind: KindId,
    ordering: &[RelatedEntityOrderingField],
) -> BTreeMap<crate::identity::data::EntityId, Vec<RelatedEntityOrderingEntry>> {
    let mut entries =
        BTreeMap::<crate::identity::data::EntityId, Vec<RelatedEntityOrderingEntry>>::new();
    projection.for_each_relation(relation_kind, |relation| {
        let (parent, child) = match parent_endpoint {
            RelatedEntityEndpoint::SourceParent => (relation.source, relation.target),
            RelatedEntityEndpoint::TargetParent => (relation.target, relation.source),
        };
        let values = projection
            .with_entity(child, |record| {
                if record.kind.kind_id != child_kind {
                    return None;
                }
                ordering
                    .iter()
                    .map(|field| {
                        entity_aspect_field_ordering_value(
                            runtime,
                            projection,
                            child,
                            field.locator(),
                        )
                    })
                    .collect::<Option<Vec<_>>>()
            })
            .flatten();
        if let Some(values) = values {
            entries
                .entry(parent)
                .or_default()
                .push(RelatedEntityOrderingEntry::new(
                    values,
                    child,
                    relation.relation_id,
                ));
        }
    });
    for rows in entries.values_mut() {
        rows.sort_by(|left, right| compare_related_entries(left, right, ordering));
    }
    entries
}

pub(in crate::indexes) fn compare_related_entries(
    left: &RelatedEntityOrderingEntry,
    right: &RelatedEntityOrderingEntry,
    ordering: &[RelatedEntityOrderingField],
) -> Ordering {
    for ((left, right), field) in left
        .ordering_values()
        .iter()
        .zip(right.ordering_values())
        .zip(ordering)
    {
        let comparison = match field.direction() {
            RelatedEntityOrderingDirection::Ascending => left.value().cmp(right.value()),
            RelatedEntityOrderingDirection::Descending => right.value().cmp(left.value()),
        };
        if comparison != Ordering::Equal {
            return comparison;
        }
    }
    left.child_entity_id()
        .cmp(&right.child_entity_id())
        .then_with(|| left.relation_id().cmp(&right.relation_id()))
}
