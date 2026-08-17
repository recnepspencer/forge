use worth_ui::facade::{
    app::{WorthUi, WorthUiHostNeutralApp},
    declaration::{
        CommandCategory, CommandDescriptor, CommandId, ComponentChildPolicy, ComponentDescriptor,
        ComponentId, ComponentPropSchema, ComponentStateOwnership, SettingDefaultPosture,
        SettingDefaultValue, SettingDescriptor, SettingEditorHint, SettingId,
        SettingMigrationPosture, SettingOwnershipMetadata, SettingScope, SettingValidationPosture,
        SettingValueSchema, TaskPresentationCancellationPosture, TaskPresentationDescriptor,
        TaskPresentationFailurePosture, TaskPresentationFamily, TaskPresentationId,
        TaskPresentationLifecyclePosture, TaskPresentationProjectionEligibility,
        TaskPresentationRuntimeAuthorityPosture,
    },
    diagnostics::CapabilityRegistrationReport,
};

pub(crate) fn empty_app() -> WorthUiHostNeutralApp {
    WorthUi::app()
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
        .freeze()
        .expect("application preparation should succeed")
}

pub(crate) fn single_command_app() -> WorthUiHostNeutralApp {
    WorthUi::app()
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
        .register_command(command_descriptor())
        .freeze()
        .expect("application preparation should succeed")
}

pub(crate) fn duplicate_representative_family_registration_report() -> CapabilityRegistrationReport
{
    WorthUi::app()
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
        .register_command(command_descriptor())
        .register_command(command_descriptor())
        .register_component(component_descriptor())
        .register_component(component_descriptor())
        .register_setting(setting_descriptor())
        .register_setting(setting_descriptor())
        .register_task_presentation(task_presentation_descriptor())
        .register_task_presentation(task_presentation_descriptor())
        .freeze_with_registration_report()
}

fn command_descriptor() -> CommandDescriptor {
    CommandDescriptor::new(command_id(), "Save").with_category(CommandCategory::Workspace)
}

fn command_id() -> CommandId {
    CommandId::new("registry_extension.command.save").expect("valid command id")
}

fn component_descriptor() -> ComponentDescriptor {
    ComponentDescriptor::new(
        component_id(),
        ComponentPropSchema::named("registry_extension.component.props"),
        ComponentChildPolicy::no_children(),
        ComponentStateOwnership::runtime_owned(),
    )
}

fn component_id() -> ComponentId {
    ComponentId::new("registry_extension.component.panel").expect("valid component id")
}

fn setting_descriptor() -> SettingDescriptor {
    SettingDescriptor::typed(
        setting_id(),
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

fn setting_id() -> SettingId {
    SettingId::new("registry_extension.setting.enabled").expect("valid setting id")
}

fn task_presentation_descriptor() -> TaskPresentationDescriptor {
    TaskPresentationDescriptor::new(task_presentation_id(), TaskPresentationFamily::progress())
        .with_lifecycle_posture(TaskPresentationLifecyclePosture::runtime_owned())
        .with_cancellation_posture(TaskPresentationCancellationPosture::not_cancellable())
        .with_failure_posture(TaskPresentationFailurePosture::runtime_reported())
        .with_projection_eligibility(TaskPresentationProjectionEligibility::progress_indicator())
        .with_runtime_authority_posture(TaskPresentationRuntimeAuthorityPosture::presentation_only())
}

fn task_presentation_id() -> TaskPresentationId {
    TaskPresentationId::new("registry_extension.task.progress").expect("valid task presentation id")
}
