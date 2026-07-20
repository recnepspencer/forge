use std::collections::{BTreeMap, BTreeSet};

use crate::declarative_live::DeclarativeLiveQueryRequest;
use crate::memory_workspace::WorthQueryEntity;
use worth_foundational::facade::{AspectValue, CanonicalFieldPath, FieldKey};

pub(in crate::runtime) fn project_rows_to_request(
    rows: Vec<WorthQueryEntity>,
    request: &DeclarativeLiveQueryRequest,
) -> Vec<WorthQueryEntity> {
    let selected_fields = request
        .query_projection()
        .iter()
        .map(|field| canonical_field_path(field.source_field_key()))
        .collect::<BTreeSet<_>>();
    rows.into_iter()
        .map(|row| project_row(row, &selected_fields))
        .collect()
}

fn project_row(
    row: WorthQueryEntity,
    selected_fields: &BTreeSet<CanonicalFieldPath>,
) -> WorthQueryEntity {
    let field_values = row
        .native_field_values()
        .filter(|(field, _)| selected_fields.contains(*field))
        .map(|(field, value)| (field.clone(), value.clone()))
        .collect::<BTreeMap<_, AspectValue>>();
    WorthQueryEntity::from_native_field_values(row.identity().clone(), field_values)
}

fn canonical_field_path(field: &crate::authoring::AspectFieldKey) -> CanonicalFieldPath {
    CanonicalFieldPath::new([
        FieldKey::new(field.native_aspect_key().as_str())
            .expect("an admitted aspect key must remain a valid path segment"),
        field.native_field_key().clone(),
    ])
    .expect("an aspect-field pair must form a non-empty field path")
}
