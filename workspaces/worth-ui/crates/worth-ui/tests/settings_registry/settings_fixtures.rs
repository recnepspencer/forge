use worth_ui::facade::registry::{
    SettingDefaultPosture, SettingDefaultValue, SettingDescriptor, SettingEditorHint, SettingId,
    SettingMigrationPosture, SettingOwnershipMetadata, SettingScope, SettingValidationPosture,
    SettingValueSchema,
};

pub(crate) fn setting_id(raw_text: &str) -> SettingId {
    SettingId::new(raw_text).expect("valid setting id")
}

pub(crate) fn boolean_workspace_setting(id: &str) -> SettingDescriptor {
    SettingDescriptor::typed(
        setting_id(id),
        SettingScope::workspace(),
        SettingValueSchema::boolean(),
    )
    .with_default_posture(SettingDefaultPosture::schema_default(
        SettingDefaultValue::boolean(false),
    ))
    .with_validation_posture(SettingValidationPosture::schema_checked())
    .with_migration_posture(SettingMigrationPosture::not_runtime_migrated())
    .with_editor_hint(SettingEditorHint::toggle())
    .with_ownership_metadata(SettingOwnershipMetadata::platform_runtime_config())
}

pub(crate) fn enum_theme_setting(id: &str) -> SettingDescriptor {
    SettingDescriptor::typed(
        setting_id(id),
        SettingScope::theme(),
        SettingValueSchema::enumeration(["light", "dark"]),
    )
    .with_default_posture(SettingDefaultPosture::schema_default(
        SettingDefaultValue::enumeration("light"),
    ))
    .with_validation_posture(SettingValidationPosture::schema_checked())
    .with_migration_posture(SettingMigrationPosture::migration_artifact_deferred())
    .with_editor_hint(SettingEditorHint::select())
    .with_ownership_metadata(SettingOwnershipMetadata::application_runtime_config())
}
