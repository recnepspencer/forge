use crate::declarative_live::DeclarativeProjectionField;
use crate::runtime::WorthQueryAspectTouch;
use worth_foundational::facade::{AspectKey, CanonicalFieldPath, FieldKey};

use super::writes::native_external_field_path_for_touch;

pub(super) fn identity_aspect_key() -> AspectKey {
    AspectKey::new("identity").expect("identity aspect key must admit")
}

pub(super) fn native_external_field_path_for_projection_field(
    field: &DeclarativeProjectionField,
) -> Result<CanonicalFieldPath, crate::memory_workspace::WorthQueryWorkspaceError> {
    native_external_field_path_for_touch(&WorthQueryAspectTouch::aspect_field_path(
        field.source_field_key().native_aspect_key(),
        CanonicalFieldPath::single(field.source_field_key().native_field_key()),
    ))
}

pub(super) fn native_external_field_path_for_grouping_aspect(
    grouping_aspect: &AspectKey,
) -> Result<CanonicalFieldPath, crate::memory_workspace::WorthQueryWorkspaceError> {
    native_external_field_path_for_touch(&WorthQueryAspectTouch::aspect_field_path(
        grouping_aspect.clone(),
        CanonicalFieldPath::single(FieldKey::new("value").expect("value field key must admit")),
    ))
}

pub(super) fn native_external_field_path_for_aspect_field(
    aspect: &str,
    field: &str,
) -> Result<CanonicalFieldPath, crate::memory_workspace::WorthQueryWorkspaceError> {
    native_external_field_path_for_touch(&WorthQueryAspectTouch::aspect_field_path(
        AspectKey::new(aspect).expect("stateful bridge fixture aspect key must admit"),
        CanonicalFieldPath::single(
            FieldKey::new(field).expect("stateful bridge fixture field key must admit"),
        ),
    ))
}
