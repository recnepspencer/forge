use worth_ui::facade::{
    app::WorthUi,
    declaration::{
        CommandDescriptor, CommandId, MosaicChildRule, MosaicClippingPosture, MosaicFocusScopeKind,
        MosaicHitTestPosture, MosaicRegionKindDescriptor, MosaicRegionKindId,
        MosaicRegionPersistence, MosaicRegionRole, MosaicScrollOwnership, MosaicSizingBehavior,
        SurfacePlacementClass,
    },
    intent::{
        UiIntent, UiIntentAcceptedInteractions, UiIntentDefinition, UiIntentId, UiIntentPayload,
        UiIntentPayloadFieldSet, UiIntentPayloadProjection, UiIntentPayloadProjectionViolation,
        UiIntentProductConsequenceFamilies, UiIntentProductConsequences, UiIntentProductOutcome,
        UiIntentRuntimeServiceDestination, UiIntentSchema, UiSemanticInteractionFamily,
    },
    rebind::UiChangeProfile,
    service::{
        UiCommandKeyCode, UiCommandModifierSet, UiCommandRoutingPolicy, UiCommandShortcutSequence,
        UiCommandShortcutStroke, UiFocusPolicy, UiMotionPolicy, UiPortalPolicy, UiScrollPolicy,
        UiSelectionPolicy,
    },
};
use worth_ui_runtime::certification_support::WorthUiRuntimeServiceInstallationCertificationExt;

struct CommandPayload;

impl UiIntentPayload for CommandPayload {
    const SCHEMA: UiIntentSchema = UiIntentSchema::stable("service.policy.command.payload", 1);
    const FIELDS: UiIntentPayloadFieldSet = UiIntentPayloadFieldSet::EMPTY;

    fn project(
        _fields: &mut UiIntentPayloadProjection<Self>,
    ) -> Result<Self, UiIntentPayloadProjectionViolation> {
        Ok(Self)
    }
}

struct CommandOutcome;

impl UiIntentProductOutcome for CommandOutcome {
    const SCHEMA: UiIntentSchema = UiIntentSchema::stable("service.policy.command.outcome", 1);
    const CONSEQUENCE_FAMILIES: UiIntentProductConsequenceFamilies =
        UiIntentProductConsequenceFamilies::NONE;

    fn into_consequences(self) -> UiIntentProductConsequences {
        UiIntentProductConsequences::none()
    }
}

struct CommandIntent;

impl UiIntent for CommandIntent {
    type Payload = CommandPayload;
    type ProductOutcome = CommandOutcome;

    const ID: UiIntentId = UiIntentId::stable("service.policy.command");
    const ACCEPTED_INTERACTIONS: UiIntentAcceptedInteractions =
        UiIntentAcceptedInteractions::new(&[UiSemanticInteractionFamily::Activate]);
}

#[test]
fn unused_policy_defaults_install_no_service_family() {
    let app = WorthUi::app()
        .with_change_profile(UiChangeProfile::platform_pulse())
        .with_portal_policy_defaults(UiPortalPolicy::modal_dialog())
        .with_focus_policy_defaults(UiFocusPolicy::workbench())
        .with_motion_policy_defaults(UiMotionPolicy::system_respecting())
        .with_command_routing_policy_defaults(UiCommandRoutingPolicy::desktop())
        .with_scroll_policy_defaults(UiScrollPolicy::nested_region())
        .with_selection_policy_defaults(UiSelectionPolicy::multiple())
        .freeze()
        .expect("policy defaults alone do not demand runtime owners");

    assert_eq!(app.service_policy_plan().installed_family_count(), 0);
    let session = launch_headless(app);
    assert_eq!(
        session
            .inspect_runtime_service_installation_for_certification()
            .installed_family_count(),
        0
    );
    drop(session.shutdown());
}

#[test]
fn installed_command_family_exposes_only_its_normalized_policy() {
    let custom = UiCommandRoutingPolicy::desktop().with_repeat_suppression(false);
    let shortcut = UiCommandShortcutSequence::single(UiCommandShortcutStroke::logical(
        UiCommandKeyCode::S,
        UiCommandModifierSet::none().with_primary(),
    ));
    let app = WorthUi::app()
        .with_change_profile(UiChangeProfile::platform_pulse())
        .with_portal_policy_defaults(UiPortalPolicy::modal_dialog())
        .with_command_routing_policy_defaults(custom)
        .register_command(
            CommandDescriptor::new(
                CommandId::new("service.policy.command").expect("fixture command ID"),
                "Run command",
            )
            .with_default_shortcut(shortcut)
            .with_intent_destination::<CommandIntent>(),
        )
        .register_runtime_service_intent_definition(worth_ui::facade::intent::UiIntentDefinition::<
            CommandIntent,
        >::runtime_service(
            UiIntentRuntimeServiceDestination::InvokeCommand,
        ))
        .expect("command runtime service registers")
        .freeze()
        .expect("installed command policy normalizes");
    let plan = app.service_policy_plan();

    assert_eq!(plan.command_routing(), Some(custom));
    assert_eq!(plan.installed_family_count(), 1);
    assert_eq!(plan.portal(), None);
    let session = launch_headless(app);
    let installed = session.inspect_runtime_service_installation_for_certification();
    assert_eq!(installed.installed_family_count(), 1);
    assert!(installed.command_routing());
    drop(session.shutdown());
}

#[test]
fn default_focus_reveal_does_not_install_undeclared_scroll() {
    let app = WorthUi::app()
        .with_change_profile(UiChangeProfile::platform_pulse())
        .register_runtime_service_intent_definition(
            UiIntentDefinition::<CommandIntent>::runtime_service(
                UiIntentRuntimeServiceDestination::OpenPortal,
            ),
        )
        .expect("portal runtime service registers")
        .freeze()
        .expect("portal service policy normalizes");
    let plan = app.service_policy_plan();

    assert_eq!(plan.installed_family_count(), 3);
    assert!(plan.portal().is_some());
    assert!(plan.focus().is_some());
    assert!(plan.motion().is_some());
    assert_eq!(plan.scroll(), None);
    let session = launch_headless(app);
    let installed = session.inspect_runtime_service_installation_for_certification();
    assert_eq!(installed.installed_family_count(), 3);
    assert!(installed.portal());
    assert!(installed.focus());
    assert!(installed.motion());
    assert!(!installed.scroll());
    drop(session.shutdown());
}

#[test]
fn mosaic_scroll_ownership_demands_scroll_and_preserves_public_policy_defaults() {
    let custom = UiScrollPolicy::nested_region().with_remainder_bubbling(false);
    let app = WorthUi::app()
        .with_change_profile(UiChangeProfile::platform_pulse())
        .with_scroll_policy_defaults(custom)
        .register_mosaic_region_kind(scroll_region(MosaicScrollOwnership::viewport_owned()))
        .freeze()
        .expect("Mosaic scroll ownership demands the Scroll owner");

    assert_eq!(app.service_policy_plan().scroll(), Some(custom));
    let session = launch_headless(app);
    let installed = session.inspect_runtime_service_installation_for_certification();
    assert_eq!(installed.installed_family_count(), 1);
    assert!(installed.scroll());
    drop(session.shutdown());
}

#[test]
fn non_scrolling_mosaic_does_not_demand_scroll() {
    let app = WorthUi::app()
        .with_change_profile(UiChangeProfile::platform_pulse())
        .with_scroll_policy_defaults(UiScrollPolicy::nested_region())
        .register_mosaic_region_kind(scroll_region(MosaicScrollOwnership::no_scrolling()))
        .freeze()
        .expect("non-scrolling Mosaic remains owner-free");

    assert_eq!(app.service_policy_plan().scroll(), None);
    let session = launch_headless(app);
    assert!(!session
        .inspect_runtime_service_installation_for_certification()
        .scroll());
    drop(session.shutdown());
}

#[test]
fn shortcut_macro_produces_the_constructor_owned_typed_value() {
    let constructed = UiCommandShortcutSequence::single(UiCommandShortcutStroke::logical(
        UiCommandKeyCode::S,
        UiCommandModifierSet::none().with_primary().with_shift(),
    ));
    assert_eq!(worth_ui::shortcut!(Primary + Shift + S), constructed);

    let sequence = worth_ui::shortcut!((Primary + K), (Primary + C));
    assert_eq!(sequence.len(), 2);
}

fn launch_headless(
    app: worth_ui::facade::app::WorthUiHostNeutralApp,
) -> worth_ui::facade::app::WorthUiActiveApplicationSession {
    worth_ui_runtime::facade::entry::WorthUiCertificationApplicationTransition::activate_headless(
        app,
    )
    .launch()
    .expect("normalized service policy launches through the production composition root")
}

fn scroll_region(ownership: MosaicScrollOwnership) -> MosaicRegionKindDescriptor {
    MosaicRegionKindDescriptor::new(
        MosaicRegionKindId::new("service.policy.scroll.region").expect("fixture region ID"),
        MosaicRegionRole::primary(),
    )
    .with_sizing_behavior(MosaicSizingBehavior::fills_available_space())
    .with_scroll_ownership(ownership)
    .with_focus_scope(MosaicFocusScopeKind::active_surface_scope())
    .with_child_rule(MosaicChildRule::accepts_surfaces())
    .with_allowed_surface_class(SurfacePlacementClass::primary_region())
    .with_persistence(MosaicRegionPersistence::restorable())
    .with_clipping(MosaicClippingPosture::clip_to_region())
    .with_hit_test(MosaicHitTestPosture::participates())
}
