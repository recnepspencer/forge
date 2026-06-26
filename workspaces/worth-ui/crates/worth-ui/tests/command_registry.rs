use forge_query::facade::ForgeQueryDeclarationEntryReadinessStatus;
use worth_ui::facade::{
    CapabilityDiagnosticCode, CommandCategory, CommandDescriptor, CommandId, CommandProjectionId,
    CommandReadinessBinding, CommandRuntimeIntentBinding, WorthUi,
};

#[test]
fn equivalent_command_descriptors_produce_equivalent_indexes() {
    let first = WorthUi::app()
        .register_command(command_descriptor("workspace.open", "Open Workspace"))
        .register_command(command_descriptor("workspace.close", "Close Workspace"))
        .freeze();
    let second = WorthUi::app()
        .register_command(command_descriptor("workspace.close", "Close Workspace"))
        .register_command(command_descriptor("workspace.open", "Open Workspace"))
        .freeze();

    assert_eq!(
        first.capabilities().commands(),
        second.capabilities().commands()
    );
    assert_eq!(
        first.capabilities().digest(),
        second.capabilities().digest()
    );
    assert_eq!(first.capabilities().commands().len(), 2);
    assert_eq!(
        first.capabilities().commands().descriptors()[0]
            .id()
            .as_str(),
        "workspace.close"
    );
}

#[test]
fn duplicate_command_id_rejected_before_snapshot_freeze() {
    let report = WorthUi::app()
        .register_command(command_descriptor("workspace.open", "Open Workspace"))
        .register_command(command_descriptor("workspace.open", "Open Again"))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().commands().is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[
            CapabilityDiagnosticCode::DuplicateCapabilityId,
            CapabilityDiagnosticCode::DuplicateCapabilityId,
        ],
    );
}

#[test]
fn duplicate_command_id_rejects_only_the_duplicate_identity() {
    let report = WorthUi::app()
        .register_command(command_descriptor("workspace.valid", "Valid Command"))
        .register_command(command_descriptor("workspace.open", "Open Workspace"))
        .register_command(command_descriptor("workspace.open", "Open Again"))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_eq!(report.accepted_snapshot().commands().len(), 1);
    assert_registered_command_ids(report.accepted_snapshot().commands(), &["workspace.valid"]);
    assert_diagnostic_codes_and_identities(
        report.registration_diagnostics(),
        &[
            (
                CapabilityDiagnosticCode::DuplicateCapabilityId,
                "workspace.open",
            ),
            (
                CapabilityDiagnosticCode::DuplicateCapabilityId,
                "workspace.open",
            ),
        ],
    );
}

#[test]
fn command_projection_references_unknown_projection_surface_rejected() {
    let report = WorthUi::app()
        .register_command(
            command_descriptor("workspace.open", "Open Workspace").with_projection_eligibility(
                CommandProjectionId::new("command_palette").expect("valid projection id"),
            ),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().commands().is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::MissingDependency],
    );
}

#[test]
fn command_with_missing_projection_does_not_poison_valid_command() {
    let report = WorthUi::app()
        .register_command(command_descriptor("workspace.valid", "Valid Command"))
        .register_command(
            command_descriptor("workspace.open", "Open Workspace").with_projection_eligibility(
                CommandProjectionId::new("command_palette").expect("valid projection id"),
            ),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_eq!(report.accepted_snapshot().commands().len(), 1);
    assert_registered_command_ids(report.accepted_snapshot().commands(), &["workspace.valid"]);
    assert_diagnostic_codes_and_identities(
        report.registration_diagnostics(),
        &[(
            CapabilityDiagnosticCode::MissingDependency,
            "workspace.open",
        )],
    );
}

#[test]
fn command_readiness_binding_preserves_structured_query_status() {
    let readiness = CommandReadinessBinding::from_query_readiness_status(
        ForgeQueryDeclarationEntryReadinessStatus::Deferred,
    );
    let app = WorthUi::app()
        .register_command(
            command_descriptor("workspace.rebuild", "Rebuild Workspace").with_readiness(readiness),
        )
        .freeze();

    let descriptor = app
        .capabilities()
        .commands()
        .get(&CommandId::new("workspace.rebuild").expect("valid command id"))
        .expect("registered command");
    assert_eq!(
        descriptor.readiness().strongest_status(),
        ForgeQueryDeclarationEntryReadinessStatus::Deferred
    );
}

#[test]
fn different_command_descriptor_meaning_produces_different_snapshot_digest() {
    let plain = WorthUi::app()
        .register_command(command_descriptor("workspace.open", "Open Workspace"))
        .freeze();
    let richer = WorthUi::app()
        .register_command(
            command_descriptor("workspace.open", "Open Workspace")
                .with_description("Open an existing workspace")
                .with_default_shortcut_reference("primary-open")
                .with_runtime_intent_binding(CommandRuntimeIntentBinding::named(
                    "workspace.open.intent",
                )),
        )
        .freeze();

    assert_ne!(
        plain.capabilities().commands(),
        richer.capabilities().commands()
    );
    assert_ne!(
        plain.capabilities().digest(),
        richer.capabilities().digest()
    );
}

#[test]
fn runtime_intent_binding_is_typed_placeholder_metadata_only() {
    let app = WorthUi::app()
        .register_command(
            command_descriptor("workspace.open", "Open Workspace").with_runtime_intent_binding(
                CommandRuntimeIntentBinding::named("workspace.open.intent"),
            ),
        )
        .freeze();

    let descriptor = app
        .capabilities()
        .commands()
        .get(&CommandId::new("workspace.open").expect("valid command id"))
        .expect("registered command");
    assert_eq!(
        descriptor
            .runtime_intent_binding()
            .expect("runtime intent metadata")
            .intent_key(),
        "workspace.open.intent"
    );
}

fn command_descriptor(id: &str, label: &str) -> CommandDescriptor {
    CommandDescriptor::new(CommandId::new(id).expect("valid command id"), label)
        .with_category(CommandCategory::Workspace)
}

fn assert_diagnostic_codes(
    diagnostics: &[worth_ui::facade::CapabilityRegistrationDiagnostic],
    expected_codes: &[CapabilityDiagnosticCode],
) {
    let actual_codes = diagnostics
        .iter()
        .map(|diagnostic| diagnostic.code())
        .collect::<Vec<_>>();
    assert_eq!(actual_codes, expected_codes);
}

fn assert_diagnostic_codes_and_identities(
    diagnostics: &[worth_ui::facade::CapabilityRegistrationDiagnostic],
    expected_codes_and_identities: &[(CapabilityDiagnosticCode, &str)],
) {
    let actual_codes_and_identities = diagnostics
        .iter()
        .map(|diagnostic| {
            (
                diagnostic.code(),
                diagnostic
                    .identity_text()
                    .expect("diagnostic should identify command registration"),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(actual_codes_and_identities, expected_codes_and_identities);
}

fn assert_registered_command_ids(
    commands: &worth_ui::facade::FrozenCommandCapabilities,
    expected_command_ids: &[&str],
) {
    let actual_command_ids = commands
        .descriptors()
        .iter()
        .map(|descriptor| descriptor.id().as_str())
        .collect::<Vec<_>>();
    assert_eq!(actual_command_ids, expected_command_ids);
}
