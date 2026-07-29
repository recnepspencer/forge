use worth_ui::facade::{
    app::WorthUi,
    declaration::{
        SettingDefaultPosture, SettingDefaultValue, SettingDescriptor, SettingEditorHint,
        SettingMigrationPosture, SettingOwnershipMetadata, SettingScope, SettingValidationPosture,
        SettingValueSchema,
    },
};

use super::settings_assertions::assert_registered_setting_ids;
use super::settings_fixtures::{boolean_workspace_setting, enum_theme_setting, setting_id};

#[test]
fn equivalent_setting_descriptors_produce_equivalent_defaults() {
    let left = WorthUi::app()
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
        .register_setting(boolean_workspace_setting("workspace.setting.wrap_lines"))
        .freeze()
        .expect("application preparation should succeed");
    let right = WorthUi::app()
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
        .register_setting(boolean_workspace_setting("workspace.setting.wrap_lines"))
        .freeze()
        .expect("application preparation should succeed");

    let left_entry = &left.capabilities().settings().entries()[0];
    let right_entry = &right.capabilities().settings().entries()[0];

    assert_eq!(left.capabilities().digest(), right.capabilities().digest());
    assert_eq!(
        left_entry.descriptor().default_posture(),
        right_entry.descriptor().default_posture()
    );
    assert_eq!(
        left_entry.key().configuration_basis(),
        right_entry.key().configuration_basis()
    );
}

#[test]
fn accepted_settings_are_canonically_ordered_and_inspectable() {
    let app = WorthUi::app()
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
        .register_setting(enum_theme_setting("workspace.setting.theme"))
        .register_setting(boolean_workspace_setting("workspace.setting.wrap_lines"))
        .freeze()
        .expect("application preparation should succeed");

    assert_registered_setting_ids(
        app.capabilities().settings(),
        &["workspace.setting.theme", "workspace.setting.wrap_lines"],
    );
    assert!(app
        .capabilities()
        .settings()
        .get(&setting_id("workspace.setting.theme"))
        .is_some());
}

#[test]
fn setting_schema_change_changes_snapshot_digest() {
    let boolean_app = WorthUi::app()
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
        .register_setting(boolean_workspace_setting("workspace.setting.value"))
        .freeze()
        .expect("application preparation should succeed");
    let text_app = WorthUi::app()
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
        .register_setting(
            SettingDescriptor::typed(
                setting_id("workspace.setting.value"),
                SettingScope::workspace(),
                SettingValueSchema::text(),
            )
            .with_default_posture(SettingDefaultPosture::schema_default(
                SettingDefaultValue::text("off"),
            ))
            .with_validation_posture(SettingValidationPosture::schema_checked()),
        )
        .freeze()
        .expect("application preparation should succeed");

    assert_ne!(
        boolean_app.capabilities().digest(),
        text_app.capabilities().digest()
    );
}

#[test]
fn setting_default_value_change_changes_snapshot_digest() {
    let off_app = WorthUi::app()
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
        .register_setting(boolean_workspace_setting("workspace.setting.value"))
        .freeze()
        .expect("application preparation should succeed");
    let on_app = WorthUi::app()
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
        .register_setting(
            SettingDescriptor::typed(
                setting_id("workspace.setting.value"),
                SettingScope::workspace(),
                SettingValueSchema::boolean(),
            )
            .with_default_posture(SettingDefaultPosture::schema_default(
                SettingDefaultValue::boolean(true),
            ))
            .with_validation_posture(SettingValidationPosture::schema_checked())
            .with_migration_posture(SettingMigrationPosture::not_runtime_migrated())
            .with_editor_hint(SettingEditorHint::toggle())
            .with_ownership_metadata(SettingOwnershipMetadata::platform_runtime_config()),
        )
        .freeze()
        .expect("application preparation should succeed");

    assert_ne!(
        off_app.capabilities().digest(),
        on_app.capabilities().digest()
    );
}

#[test]
fn setting_surface_metadata_changes_snapshot_digest() {
    let visible_app = WorthUi::app()
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
        .register_setting(boolean_workspace_setting("workspace.setting.metadata"))
        .freeze()
        .expect("application preparation should succeed");
    let hidden_app = WorthUi::app()
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
        .register_setting(
            SettingDescriptor::typed(
                setting_id("workspace.setting.metadata"),
                SettingScope::workspace(),
                SettingValueSchema::boolean(),
            )
            .with_default_posture(SettingDefaultPosture::schema_default(
                SettingDefaultValue::boolean(false),
            ))
            .with_validation_posture(SettingValidationPosture::schema_checked())
            .with_migration_posture(SettingMigrationPosture::migration_artifact_deferred())
            .with_editor_hint(SettingEditorHint::hidden())
            .with_ownership_metadata(SettingOwnershipMetadata::application_runtime_config()),
        )
        .freeze()
        .expect("application preparation should succeed");

    assert_ne!(
        visible_app.capabilities().digest(),
        hidden_app.capabilities().digest()
    );
}
