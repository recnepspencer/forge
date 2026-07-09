use super::{
    WorthQueryGraphReadOrderingFieldAuthority, WorthQueryGraphReadPredicateFieldAuthority,
    WorthQueryGraphReadRelationAuthority,
};
use crate::runtime::{WorthQueryBooleanSelectivityShape, WorthQueryGraphReadAccessShape};

pub(crate) fn relation_authority(
    schema_basis_digest: &str,
    relation_name: &str,
) -> WorthQueryGraphReadRelationAuthority {
    WorthQueryGraphReadRelationAuthority::new(schema_basis_digest, relation_name)
}

pub(crate) fn predicate_field_authorities(
    access_shape: &WorthQueryGraphReadAccessShape,
    selectivity_shape: &WorthQueryBooleanSelectivityShape,
) -> Vec<WorthQueryGraphReadPredicateFieldAuthority> {
    let schema_basis_digest = access_shape
        .operation_resolution()
        .references()
        .schema_basis_digest();
    selectivity_shape
        .predicate_rows()
        .iter()
        .map(|row| {
            WorthQueryGraphReadPredicateFieldAuthority::new(
                schema_basis_digest,
                row.native_aspect_key().clone(),
                row.native_field_key().clone(),
                row.field_kind().as_str(),
            )
        })
        .collect()
}

pub(crate) fn ordering_field_authorities(
    access_shape: &WorthQueryGraphReadAccessShape,
) -> Vec<WorthQueryGraphReadOrderingFieldAuthority> {
    let references = access_shape.operation_resolution().references();
    references
        .orderings()
        .iter()
        .map(|row| {
            WorthQueryGraphReadOrderingFieldAuthority::new(
                references.schema_basis_digest(),
                row.native_aspect_key().clone(),
                row.native_field_key().clone(),
                row.direction(),
                row.kind().as_str(),
            )
        })
        .collect()
}
