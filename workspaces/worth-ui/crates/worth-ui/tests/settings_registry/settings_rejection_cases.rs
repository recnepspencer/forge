use worth_ui::facade::{
    app::WorthUi,
    declaration::{
        SettingDefaultPosture, SettingDefaultValue, SettingDescriptor, SettingEditorHint,
        SettingMigrationPosture, SettingOwnershipMetadata, SettingScope, SettingValidationPosture,
        SettingValueSchema,
    },
    diagnostics::CapabilityDiagnosticCode,
    support::ArbitraryKeyValueSettingBag,
};

use super::settings_assertions::{assert_diagnostic_codes, assert_registered_setting_ids};
use super::settings_fixtures::{boolean_workspace_setting, setting_id};

#[test]
fn setting_without_scope_rejected() {
    let report = WorthUi::app()
        .register_setting(
            SettingDescriptor::missing_scope_for_diagnostics(
                setting_id("workspace.setting.no_scope"),
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
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().settings().is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::MissingSettingScope],
    );
}

#[test]
fn duplicate_setting_id_rejected_before_snapshot_freeze() {
    let report = WorthUi::app()
        .register_setting(boolean_workspace_setting("workspace.setting.duplicate"))
        .register_setting(boolean_workspace_setting("workspace.setting.duplicate"))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().settings().is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[
            CapabilityDiagnosticCode::DuplicateCapabilityId,
            CapabilityDiagnosticCode::DuplicateCapabilityId,
        ],
    );
}

#[test]
fn setting_without_value_schema_rejected() {
    let report = WorthUi::app()
        .register_setting(
            SettingDescriptor::missing_value_schema_for_diagnostics(
                setting_id("workspace.setting.no_schema"),
                SettingScope::workspace(),
            )
            .with_default_posture(SettingDefaultPosture::runtime_computed())
            .with_validation_posture(SettingValidationPosture::schema_checked())
            .with_migration_posture(SettingMigrationPosture::not_runtime_migrated())
            .with_editor_hint(SettingEditorHint::toggle())
            .with_ownership_metadata(SettingOwnershipMetadata::platform_runtime_config()),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::MissingSettingValueSchema],
    );
}

#[test]
fn setting_without_default_posture_rejected() {
    let report = WorthUi::app()
        .register_setting(
            SettingDescriptor::typed(
                setting_id("workspace.setting.no_default"),
                SettingScope::workspace(),
                SettingValueSchema::boolean(),
            )
            .with_validation_posture(SettingValidationPosture::schema_checked())
            .with_migration_posture(SettingMigrationPosture::not_runtime_migrated())
            .with_editor_hint(SettingEditorHint::toggle())
            .with_ownership_metadata(SettingOwnershipMetadata::platform_runtime_config()),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::MissingSettingDefaultPosture],
    );
}

#[test]
fn setting_without_validation_posture_rejected() {
    let report = WorthUi::app()
        .register_setting(
            SettingDescriptor::typed(
                setting_id("workspace.setting.no_validation"),
                SettingScope::workspace(),
                SettingValueSchema::boolean(),
            )
            .with_default_posture(SettingDefaultPosture::schema_default(
                SettingDefaultValue::boolean(false),
            ))
            .with_migration_posture(SettingMigrationPosture::not_runtime_migrated())
            .with_editor_hint(SettingEditorHint::toggle())
            .with_ownership_metadata(SettingOwnershipMetadata::platform_runtime_config()),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::MissingSettingValidationPosture],
    );
}

#[test]
fn arbitrary_key_value_setting_bag_rejected() {
    let report = WorthUi::app()
        .register_setting(SettingDescriptor::arbitrary_key_value_bag_for_diagnostics(
            setting_id("workspace.setting.raw_bag"),
            ArbitraryKeyValueSettingBag::new("HashMap<String, serde_json::Value>"),
        ))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[
            CapabilityDiagnosticCode::MissingSettingScope,
            CapabilityDiagnosticCode::MissingSettingValueSchema,
            CapabilityDiagnosticCode::MissingSettingDefaultPosture,
            CapabilityDiagnosticCode::MissingSettingValidationPosture,
            CapabilityDiagnosticCode::MissingSettingMigrationPosture,
            CapabilityDiagnosticCode::MissingSettingEditorHint,
            CapabilityDiagnosticCode::MissingSettingOwnershipMetadata,
            CapabilityDiagnosticCode::ArbitraryKeyValueSettingBag,
        ],
    );
}

#[test]
fn setting_without_migration_posture_rejected() {
    let report = WorthUi::app()
        .register_setting(
            SettingDescriptor::typed(
                setting_id("workspace.setting.no_migration"),
                SettingScope::workspace(),
                SettingValueSchema::boolean(),
            )
            .with_default_posture(SettingDefaultPosture::schema_default(
                SettingDefaultValue::boolean(false),
            ))
            .with_validation_posture(SettingValidationPosture::schema_checked())
            .with_editor_hint(SettingEditorHint::toggle())
            .with_ownership_metadata(SettingOwnershipMetadata::platform_runtime_config()),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::MissingSettingMigrationPosture],
    );
}

#[test]
fn setting_without_editor_hint_rejected() {
    let report = WorthUi::app()
        .register_setting(
            SettingDescriptor::typed(
                setting_id("workspace.setting.no_editor_hint"),
                SettingScope::workspace(),
                SettingValueSchema::boolean(),
            )
            .with_default_posture(SettingDefaultPosture::schema_default(
                SettingDefaultValue::boolean(false),
            ))
            .with_validation_posture(SettingValidationPosture::schema_checked())
            .with_migration_posture(SettingMigrationPosture::not_runtime_migrated())
            .with_ownership_metadata(SettingOwnershipMetadata::platform_runtime_config()),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::MissingSettingEditorHint],
    );
}

#[test]
fn setting_without_ownership_metadata_rejected() {
    let report = WorthUi::app()
        .register_setting(
            SettingDescriptor::typed(
                setting_id("workspace.setting.no_ownership"),
                SettingScope::workspace(),
                SettingValueSchema::boolean(),
            )
            .with_default_posture(SettingDefaultPosture::schema_default(
                SettingDefaultValue::boolean(false),
            ))
            .with_validation_posture(SettingValidationPosture::schema_checked())
            .with_migration_posture(SettingMigrationPosture::not_runtime_migrated())
            .with_editor_hint(SettingEditorHint::toggle()),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::MissingSettingOwnershipMetadata],
    );
}

#[test]
fn setting_default_value_must_satisfy_schema() {
    let report = WorthUi::app()
        .register_setting(
            SettingDescriptor::typed(
                setting_id("workspace.setting.mismatch"),
                SettingScope::workspace(),
                SettingValueSchema::boolean(),
            )
            .with_default_posture(SettingDefaultPosture::schema_default(
                SettingDefaultValue::text("false"),
            ))
            .with_validation_posture(SettingValidationPosture::schema_checked())
            .with_migration_posture(SettingMigrationPosture::not_runtime_migrated())
            .with_editor_hint(SettingEditorHint::toggle())
            .with_ownership_metadata(SettingOwnershipMetadata::platform_runtime_config()),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::SettingDefaultValueSchemaMismatch],
    );
}

#[test]
fn invalid_enum_setting_schema_rejected() {
    let report = WorthUi::app()
        .register_setting(
            SettingDescriptor::typed(
                setting_id("workspace.setting.empty_enum"),
                SettingScope::theme(),
                SettingValueSchema::enumeration([] as [&str; 0]),
            )
            .with_default_posture(SettingDefaultPosture::runtime_computed())
            .with_validation_posture(SettingValidationPosture::schema_checked())
            .with_migration_posture(SettingMigrationPosture::not_runtime_migrated())
            .with_editor_hint(SettingEditorHint::select())
            .with_ownership_metadata(SettingOwnershipMetadata::platform_runtime_config()),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::InvalidSettingValueSchema],
    );
}

#[test]
fn duplicate_enum_setting_schema_options_rejected() {
    let report = WorthUi::app()
        .register_setting(
            SettingDescriptor::typed(
                setting_id("workspace.setting.duplicate_enum"),
                SettingScope::theme(),
                SettingValueSchema::enumeration(["light", "light"]),
            )
            .with_default_posture(SettingDefaultPosture::schema_default(
                SettingDefaultValue::enumeration("light"),
            ))
            .with_validation_posture(SettingValidationPosture::schema_checked())
            .with_migration_posture(SettingMigrationPosture::not_runtime_migrated())
            .with_editor_hint(SettingEditorHint::select())
            .with_ownership_metadata(SettingOwnershipMetadata::platform_runtime_config()),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::InvalidSettingValueSchema],
    );
}

#[test]
fn malformed_decimal_setting_default_rejected() {
    let report = WorthUi::app()
        .register_setting(
            SettingDescriptor::typed(
                setting_id("workspace.setting.bad_decimal"),
                SettingScope::workspace(),
                SettingValueSchema::decimal(),
            )
            .with_default_posture(SettingDefaultPosture::schema_default(
                SettingDefaultValue::decimal("not-a-decimal"),
            ))
            .with_validation_posture(SettingValidationPosture::schema_checked())
            .with_migration_posture(SettingMigrationPosture::not_runtime_migrated())
            .with_editor_hint(SettingEditorHint::number_input())
            .with_ownership_metadata(SettingOwnershipMetadata::platform_runtime_config()),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::SettingDefaultValueSchemaMismatch],
    );
}

#[test]
fn setting_persistence_cannot_claim_domain_truth() {
    let report = WorthUi::app()
        .register_setting(
            boolean_workspace_setting("workspace.setting.truth_claim")
                .with_migration_posture(
                    SettingMigrationPosture::claims_authoritative_domain_truth_for_diagnostics(),
                )
                .with_ownership_metadata(
                    SettingOwnershipMetadata::claims_authoritative_domain_truth_for_diagnostics(),
                ),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().settings().is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::SettingPersistenceClaimsDomainTruth],
    );
}

#[test]
fn rejected_setting_does_not_poison_valid_setting() {
    let report = WorthUi::app()
        .register_setting(
            SettingDescriptor::typed(
                setting_id("workspace.setting.bad"),
                SettingScope::workspace(),
                SettingValueSchema::boolean(),
            )
            .with_default_posture(SettingDefaultPosture::schema_default(
                SettingDefaultValue::text("false"),
            ))
            .with_validation_posture(SettingValidationPosture::schema_checked())
            .with_migration_posture(SettingMigrationPosture::not_runtime_migrated())
            .with_editor_hint(SettingEditorHint::toggle())
            .with_ownership_metadata(SettingOwnershipMetadata::platform_runtime_config()),
        )
        .register_setting(boolean_workspace_setting("workspace.setting.good"))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_registered_setting_ids(
        report.accepted_snapshot().settings(),
        &["workspace.setting.good"],
    );
}
