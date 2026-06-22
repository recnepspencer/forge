use super::{
    ForgeQueryGraphReadOrderingFieldAuthority, ForgeQueryGraphReadPredicateFieldAuthority,
    ForgeQueryGraphReadRelationAuthority,
};
use crate::runtime::{ForgeQueryBooleanSelectivityShape, ForgeQueryGraphReadAccessShape};

pub(crate) fn relation_authority(
    schema_basis_digest: &str,
    relation_name: &str,
) -> ForgeQueryGraphReadRelationAuthority {
    ForgeQueryGraphReadRelationAuthority::new(schema_basis_digest, relation_name)
}

pub(crate) fn predicate_field_authorities(
    access_shape: &ForgeQueryGraphReadAccessShape,
    selectivity_shape: &ForgeQueryBooleanSelectivityShape,
) -> Vec<ForgeQueryGraphReadPredicateFieldAuthority> {
    let schema_basis_digest = access_shape
        .operation_resolution()
        .references()
        .schema_basis_digest();
    selectivity_shape
        .predicate_rows()
        .iter()
        .map(|row| {
            ForgeQueryGraphReadPredicateFieldAuthority::new(
                schema_basis_digest,
                row.native_aspect_key().clone(),
                row.native_field_key().clone(),
                row.field_kind().as_str(),
            )
        })
        .collect()
}

pub(crate) fn ordering_field_authorities(
    access_shape: &ForgeQueryGraphReadAccessShape,
) -> Vec<ForgeQueryGraphReadOrderingFieldAuthority> {
    let references = access_shape.operation_resolution().references();
    references
        .orderings()
        .iter()
        .map(|row| {
            ForgeQueryGraphReadOrderingFieldAuthority::new(
                references.schema_basis_digest(),
                row.native_aspect_key().clone(),
                row.native_field_key().clone(),
                row.direction(),
                row.kind().as_str(),
            )
        })
        .collect()
}
