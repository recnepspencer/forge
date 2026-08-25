use worth_ui::facade::query_binding::{
    UiProjectionFieldRequirement, UiProjectionLifecycleRequirement, UiScalarSchemaRequirement,
};
use worth_ui::facade::rebind::UiProjectionSchemaRequirement;

use super::{
    scalar_field, PlatformPulseLifecycleObservationProjectionDenial,
    PlatformPulseProjectionSchemaField,
};

#[test]
fn typed_pulse_schema_fields_project_without_reinterpreting_leaf_names() {
    assert_eq!(
        scalar_field(&scalar(UiProjectionFieldRequirement::query_text_status())),
        Ok(PlatformPulseProjectionSchemaField::Status)
    );
    assert_eq!(
        scalar_field(&scalar(UiProjectionFieldRequirement::query_revision())),
        Ok(PlatformPulseProjectionSchemaField::Revision)
    );
}

#[test]
fn diagnostic_field_name_cannot_impersonate_typed_revision_authority() {
    let diagnostic = UiProjectionFieldRequirement::declared("revision")
        .expect("fixture diagnostic field name is syntactically valid");
    assert!(matches!(
        scalar_field(&scalar(diagnostic)),
        Err(PlatformPulseLifecycleObservationProjectionDenial::UnsupportedSchemaTransitionField)
    ));
}

fn scalar(field: UiProjectionFieldRequirement) -> UiProjectionSchemaRequirement {
    UiProjectionSchemaRequirement::Scalar(UiScalarSchemaRequirement::text(
        field,
        UiProjectionLifecycleRequirement::Live,
    ))
}
