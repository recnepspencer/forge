use std::cmp::Ordering;

use crate::declarative_live::DeclarativeOrderingField;
use crate::memory_workspace::{WorthQueryEntity, WorthQueryEntityIdentity};
use worth_foundational::facade::AspectValue;

use super::row_field_value;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthQueryCanonicalOrderingKey {
    fields: Vec<WorthQueryDirectedOrderingValue>,
    identity: WorthQueryEntityIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum WorthQueryDirectedOrderingValue {
    Ascending(Option<AspectValue>),
    Descending(Option<AspectValue>),
}

impl Ord for WorthQueryDirectedOrderingValue {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Ascending(left), Self::Ascending(right))
            | (Self::Descending(left), Self::Descending(right)) => {
                let ordering = left.cmp(right);
                if matches!(self, Self::Descending(_)) {
                    ordering.reverse()
                } else {
                    ordering
                }
            }
            (Self::Ascending(_), Self::Descending(_)) => Ordering::Less,
            (Self::Descending(_), Self::Ascending(_)) => Ordering::Greater,
        }
    }
}

impl PartialOrd for WorthQueryDirectedOrderingValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for WorthQueryCanonicalOrderingKey {
    fn cmp(&self, other: &Self) -> Ordering {
        self.fields
            .cmp(&other.fields)
            .then_with(|| self.identity.cmp(&other.identity))
    }
}

impl PartialOrd for WorthQueryCanonicalOrderingKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub(crate) fn canonical_ordering_key(
    row: &WorthQueryEntity,
    ordering: &[DeclarativeOrderingField],
) -> WorthQueryCanonicalOrderingKey {
    let fields = ordering
        .iter()
        .map(|entry| {
            let value = row_field_value(row, entry.source_field_key()).cloned();
            match entry.direction() {
                crate::authoring::OrderingDirection::Ascending => {
                    WorthQueryDirectedOrderingValue::Ascending(value)
                }
                crate::authoring::OrderingDirection::Descending => {
                    WorthQueryDirectedOrderingValue::Descending(value)
                }
            }
        })
        .collect();
    WorthQueryCanonicalOrderingKey {
        fields,
        identity: row.identity().clone(),
    }
}
