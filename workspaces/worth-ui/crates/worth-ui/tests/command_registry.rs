use worth_ui::facade::{
    app::WorthUi,
    declaration::{
        CommandCategory, CommandDescriptor, CommandId, CommandProjectionId, UiCommandKeyCode,
        UiCommandModifierSet, UiCommandRouteDeclaration, UiCommandRouteDestination,
        UiCommandRouteScope, UiCommandRouteScopeIdentity, UiCommandShortcutSequence,
        UiCommandShortcutStroke,
    },
    diagnostics::CapabilityDiagnosticCode,
};

#[test]
fn equivalent_command_descriptors_produce_equivalent_indexes() {
    let first = WorthUi::app()
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
        .register_command(command_descriptor("workspace.open", "Open Workspace"))
        .register_command(command_descriptor("workspace.close", "Close Workspace"))
        .freeze()
        .expect("application preparation should succeed");
    let second = WorthUi::app()
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
        .register_command(command_descriptor("workspace.close", "Close Workspace"))
        .register_command(command_descriptor("workspace.open", "Open Workspace"))
        .freeze()
        .expect("application preparation should succeed");

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
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
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
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
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
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
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
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
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
fn different_command_descriptor_meaning_produces_different_snapshot_digest() {
    let plain = WorthUi::app()
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
        .register_command(command_descriptor("workspace.open", "Open Workspace"))
        .freeze()
        .expect("application preparation should succeed");
    let richer = WorthUi::app()
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
        .register_command(
            command_descriptor("workspace.open", "Open Workspace")
                .with_description("Open an existing workspace"),
        )
        .freeze()
        .expect("application preparation should succeed");

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
fn shortcut_without_typed_route_destination_is_rejected() {
    let shortcut = UiCommandShortcutSequence::single(UiCommandShortcutStroke::logical(
        UiCommandKeyCode::O,
        UiCommandModifierSet::none().with_primary(),
    ));
    let report = WorthUi::app()
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
        .register_command(
            command_descriptor("workspace.open", "Open Workspace").with_default_shortcut(shortcut),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().commands().is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::MissingCommandRouteDestination],
    );
}

#[test]
fn shortcut_with_primary_and_platform_specific_alias_is_rejected() {
    let shortcut = UiCommandShortcutSequence::single(UiCommandShortcutStroke::logical(
        UiCommandKeyCode::O,
        UiCommandModifierSet::none().with_primary().with_control(),
    ));
    let report = WorthUi::app()
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
        .register_command(
            command_descriptor("workspace.open", "Open Workspace")
                .with_default_shortcut(shortcut)
                .with_intent_destination::<ShortcutIntent>(),
        )
        .register_runtime_service_intent_definition(worth_ui::facade::intent::UiIntentDefinition::<
            ShortcutIntent,
        >::runtime_service(
            worth_ui::facade::intent::UiIntentRuntimeServiceDestination::InvokeCommand,
        ))
        .expect("fixture intent registers")
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().commands().is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::ConflictingCommandShortcutAlias],
    );
}

#[test]
fn active_region_command_scope_is_rejected_until_runtime_region_authority_exists() {
    let report = WorthUi::app()
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
        .register_command(
            command_descriptor("workspace.open", "Open Workspace").with_route(
                UiCommandRouteDeclaration::new(UiCommandRouteDestination::for_intent::<
                    ShortcutIntent,
                >())
                .with_scope(UiCommandRouteScope::ActiveRegion),
            ),
        )
        .register_runtime_service_intent_definition(worth_ui::facade::intent::UiIntentDefinition::<
            ShortcutIntent,
        >::runtime_service(
            worth_ui::facade::intent::UiIntentRuntimeServiceDestination::InvokeCommand,
        ))
        .expect("fixture intent registers")
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().commands().is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::UnsupportedCommandRouteScope],
    );
}

#[test]
fn focused_and_portal_routes_require_an_exact_authored_scope_identity() {
    for scope in [
        UiCommandRouteScope::FocusedControl,
        UiCommandRouteScope::ActivePortal,
    ] {
        let report = WorthUi::app()
            .with_change_profile(
                worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse(),
            )
            .register_command(
                command_descriptor("workspace.open", "Open Workspace").with_route(
                    UiCommandRouteDeclaration::new(UiCommandRouteDestination::for_intent::<
                        ShortcutIntent,
                    >())
                    .with_scope(scope),
                ),
            )
            .register_runtime_service_intent_definition(
                worth_ui::facade::intent::UiIntentDefinition::<ShortcutIntent>::runtime_service(
                    worth_ui::facade::intent::UiIntentRuntimeServiceDestination::InvokeCommand,
                ),
            )
            .expect("fixture intent registers")
            .freeze_with_registration_report();
        assert_diagnostic_codes(
            report.registration_diagnostics(),
            &[CapabilityDiagnosticCode::MissingCommandRouteScopeIdentity],
        );
    }

    let identity = UiCommandRouteScopeIdentity::for_authored_semantic_name("editor.control");
    let app = WorthUi::app()
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
        .register_command(
            command_descriptor("workspace.open", "Open Workspace").with_route(
                UiCommandRouteDeclaration::new(UiCommandRouteDestination::for_intent::<
                    ShortcutIntent,
                >())
                .for_focused_control(identity),
            ),
        )
        .register_runtime_service_intent_definition(worth_ui::facade::intent::UiIntentDefinition::<
            ShortcutIntent,
        >::runtime_service(
            worth_ui::facade::intent::UiIntentRuntimeServiceDestination::InvokeCommand,
        ))
        .expect("fixture intent registers")
        .freeze()
        .expect("an exact authored scope binding is admitted");
    assert_eq!(app.capabilities().commands().len(), 1);
}

struct ShortcutPayload;

impl worth_ui::facade::intent::UiIntentPayload for ShortcutPayload {
    const SCHEMA: worth_ui::facade::intent::UiIntentSchema =
        worth_ui::facade::intent::UiIntentSchema::stable("command.shortcut.payload", 1);
    const FIELDS: worth_ui::facade::intent::UiIntentPayloadFieldSet =
        worth_ui::facade::intent::UiIntentPayloadFieldSet::EMPTY;

    fn project(
        _fields: &mut worth_ui::facade::intent::UiIntentPayloadProjection<Self>,
    ) -> Result<Self, worth_ui::facade::intent::UiIntentPayloadProjectionViolation> {
        Ok(Self)
    }
}

struct ShortcutOutcome;

impl worth_ui::facade::intent::UiIntentProductOutcome for ShortcutOutcome {
    const SCHEMA: worth_ui::facade::intent::UiIntentSchema =
        worth_ui::facade::intent::UiIntentSchema::stable("command.shortcut.outcome", 1);
    const CONSEQUENCE_FAMILIES: worth_ui::facade::intent::UiIntentProductConsequenceFamilies =
        worth_ui::facade::intent::UiIntentProductConsequenceFamilies::NONE;

    fn into_consequences(self) -> worth_ui::facade::intent::UiIntentProductConsequences {
        worth_ui::facade::intent::UiIntentProductConsequences::none()
    }
}

struct ShortcutIntent;

impl worth_ui::facade::intent::UiIntent for ShortcutIntent {
    type Payload = ShortcutPayload;
    type ProductOutcome = ShortcutOutcome;

    const ID: worth_ui::facade::intent::UiIntentId =
        worth_ui::facade::intent::UiIntentId::stable("command.shortcut.intent");
    const ACCEPTED_INTERACTIONS: worth_ui::facade::intent::UiIntentAcceptedInteractions =
        worth_ui::facade::intent::UiIntentAcceptedInteractions::new(&[
            worth_ui::facade::intent::UiSemanticInteractionFamily::Activate,
        ]);
}

fn command_descriptor(id: &str, label: &str) -> CommandDescriptor {
    CommandDescriptor::new(CommandId::new(id).expect("valid command id"), label)
        .with_category(CommandCategory::Workspace)
}

fn assert_diagnostic_codes(
    diagnostics: &[worth_ui::facade::diagnostics::CapabilityRegistrationDiagnostic],
    expected_codes: &[CapabilityDiagnosticCode],
) {
    let actual_codes = diagnostics
        .iter()
        .map(|diagnostic| diagnostic.code())
        .collect::<Vec<_>>();
    assert_eq!(actual_codes, expected_codes);
}

fn assert_diagnostic_codes_and_identities(
    diagnostics: &[worth_ui::facade::diagnostics::CapabilityRegistrationDiagnostic],
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
    commands: &worth_ui_runtime::facade::registry::snapshot::FrozenCommandCapabilities,
    expected_command_ids: &[&str],
) {
    let actual_command_ids = commands
        .descriptors()
        .iter()
        .map(|descriptor| descriptor.id().as_str())
        .collect::<Vec<_>>();
    assert_eq!(actual_command_ids, expected_command_ids);
}
