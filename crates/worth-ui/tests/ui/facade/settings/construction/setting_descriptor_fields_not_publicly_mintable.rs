use worth_ui::facade::{
    SettingDefaultPosture, SettingDescriptor, SettingEditorHint, SettingId,
    SettingMigrationPosture, SettingOwnershipMetadata, SettingScope, SettingValidationPosture,
    SettingValueSchema,
};

fn main() {
    let _descriptor = SettingDescriptor {
        id: SettingId::new("workspace.setting.raw").unwrap(),
        scope: Some(SettingScope::workspace()),
        value_schema: Some(SettingValueSchema::boolean()),
        default_posture: Some(SettingDefaultPosture::runtime_computed()),
        validation_posture: Some(SettingValidationPosture::schema_checked()),
        migration_posture: Some(SettingMigrationPosture::not_runtime_migrated()),
        editor_hint: Some(SettingEditorHint::toggle()),
        ownership_metadata: Some(SettingOwnershipMetadata::platform_runtime_config()),
        arbitrary_key_value_bag: None,
    };
}
