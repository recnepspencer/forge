use std::collections::BTreeMap;

use crate::declarative_live::DeclarativeLiveQueryRequest;
use crate::memory_workspace::WorthQueryEntity;
use worth_foundational::facade::{
    AspectKey, AspectValue, CanonicalFieldPath, FieldKey, StructAspectValue,
};

pub(in crate::runtime) fn project_rows_to_request(
    rows: Vec<WorthQueryEntity>,
    request: &DeclarativeLiveQueryRequest,
) -> Vec<WorthQueryEntity> {
    rows.into_iter()
        .map(|row| project_row(row, request.query_projection()))
        .collect()
}

fn project_row(
    row: WorthQueryEntity,
    selected_fields: &[crate::declarative_live::DeclarativeProjectionField],
) -> WorthQueryEntity {
    let mut aspect_values = BTreeMap::new();
    let mut struct_fields = BTreeMap::<AspectKey, Vec<(FieldKey, AspectValue)>>::new();
    let mut delivered_field_values = BTreeMap::new();

    for selected in selected_fields {
        let source = selected.source_field_key();
        let aspect = source.native_aspect_key();
        let field = source.native_field_key();
        if let Some(value) = row
            .struct_aspect_value(&aspect)
            .and_then(|value| value.get(&field))
        {
            retain_delivered_field(&mut delivered_field_values, &aspect, &field, value.clone());
            struct_fields
                .entry(aspect)
                .or_default()
                .push((field, value.clone()));
            continue;
        }
        if let Some(value) = row.aspect_value(&aspect) {
            retain_delivered_field(&mut delivered_field_values, &aspect, &field, value.clone());
            aspect_values.insert(aspect, value.clone());
            continue;
        }
        if let Some(path) = delivered_field_path(&aspect, &field) {
            if let Some(value) = row.scalar_value_at(&path) {
                delivered_field_values.insert(path, value.clone());
            }
        }
    }

    let struct_aspect_values = struct_fields
        .into_iter()
        .map(|(aspect, fields)| {
            (
                aspect,
                StructAspectValue::new(fields)
                    .expect("one canonical query projection visits each field once"),
            )
        })
        .collect();
    WorthQueryEntity::from_aspect_projection(
        row.identity().clone(),
        aspect_values,
        struct_aspect_values,
        delivered_field_values,
    )
}

fn retain_delivered_field(
    delivered: &mut BTreeMap<CanonicalFieldPath, AspectValue>,
    aspect: &AspectKey,
    field: &FieldKey,
    value: AspectValue,
) {
    if let Some(path) = delivered_field_path(aspect, field) {
        delivered.insert(path, value);
    }
}

fn delivered_field_path(aspect: &AspectKey, field: &FieldKey) -> Option<CanonicalFieldPath> {
    CanonicalFieldPath::new([FieldKey::new(aspect.as_str().to_string())?, field.clone()])
}
