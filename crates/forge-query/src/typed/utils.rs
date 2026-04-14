use crate::authoring::{AspectFieldSelector, AuthoredResultShapeField};

use super::traits::TypedProjectableField;

pub(super) fn selector_for<F: TypedProjectableField>() -> AspectFieldSelector {
    AspectFieldSelector::new(F::ASPECT, F::FIELD)
        .expect("typed projection constants must be valid non-empty identifiers")
}

pub(super) fn result_shape_field_for<F: TypedProjectableField>(
    delivered_name: &str,
) -> AuthoredResultShapeField {
    AuthoredResultShapeField::new(F::ASPECT, F::FIELD, delivered_name)
        .expect("typed result-shape constants must be valid non-empty identifiers")
}
