use super::ApplicationQueryResultShape;

pub(super) fn portable_identity_is_valid(identity: &str) -> bool {
    !identity.is_empty()
        && identity.trim() == identity
        && !identity
            .chars()
            .any(|character| character.is_whitespace() || character.is_control())
}

pub(super) fn shape_portable_identities_are_valid(shape: &ApplicationQueryResultShape) -> bool {
    portable_identity_is_valid(shape.query_type())
        && portable_identity_is_valid(shape.result_type())
        && shape.fields().iter().all(|field| {
            portable_identity_is_valid(field.query_type())
                && portable_identity_is_valid(field.slot_type())
                && portable_identity_is_valid(field.value_type())
        })
        && shape.relations().iter().all(|relation| {
            portable_identity_is_valid(relation.query_type())
                && portable_identity_is_valid(relation.slot_type())
                && shape_portable_identities_are_valid(relation.nested_shape())
        })
}
