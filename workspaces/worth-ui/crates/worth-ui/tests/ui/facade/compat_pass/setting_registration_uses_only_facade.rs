use worth_ui::facade::{
    app::WorthUi,
    registry::{SettingDefaultPosture, SettingDefaultValue, SettingDescriptor, SettingEditorHint, SettingId, SettingMigrationPosture, SettingOwnershipMetadata, SettingScope, SettingValidationPosture, SettingValueSchema},
};

fn main() {
    let app = WorthUi::app()
        .register_setting(
            SettingDescriptor::typed(
                SettingId::new("workspace.setting.facade").unwrap(),
                SettingScope::workspace(),
                SettingValueSchema::boolean(),
            )
            .with_default_posture(SettingDefaultPosture::schema_default(
                SettingDefaultValue::boolean(false),
            ))
            .with_validation_posture(SettingValidationPosture::schema_checked())
            .with_migration_posture(SettingMigrationPosture::not_runtime_migrated())
            .with_editor_hint(SettingEditorHint::toggle())
            .with_ownership_metadata(SettingOwnershipMetadata::platform_runtime_config()),
        )
        .freeze().expect("application preparation should succeed");

    assert_eq!(app.capabilities().settings().len(), 1);
}
