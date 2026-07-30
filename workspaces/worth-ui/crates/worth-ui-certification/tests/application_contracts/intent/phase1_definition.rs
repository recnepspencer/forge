use worth_ui::facade::app::WorthUi;
use worth_ui::facade::intent::{
    UiIntent, UiIntentAcceptedInteractions, UiIntentDefinition,
    UiIntentDefinitionRegistrationError, UiIntentId, UiIntentPayload, UiIntentProductOutcome,
    UiIntentSchema, UiIntentTransitionDestination, UiSemanticInteractionFamily,
};

struct AdvancePayload;

impl UiIntentPayload for AdvancePayload {
    const SCHEMA: UiIntentSchema = UiIntentSchema::stable("platform.pulse.advance_payload", 1);
}

struct AdvanceOutcome;

impl UiIntentProductOutcome for AdvanceOutcome {
    const SCHEMA: UiIntentSchema = UiIntentSchema::stable("platform.pulse.advance_outcome", 1);
}

struct AdvanceStatus;

impl UiIntent for AdvanceStatus {
    type Payload = AdvancePayload;
    type ProductOutcome = AdvanceOutcome;

    const ID: UiIntentId = UiIntentId::stable("platform.pulse.advance");
    const ACCEPTED_INTERACTIONS: UiIntentAcceptedInteractions =
        UiIntentAcceptedInteractions::new(&[UiSemanticInteractionFamily::Activate]);
}

struct CollisionPayloadAb;

impl UiIntentPayload for CollisionPayloadAb {
    const SCHEMA: UiIntentSchema = UiIntentSchema::stable("bc", 1);
}

struct CollisionPayloadB;

impl UiIntentPayload for CollisionPayloadB {
    const SCHEMA: UiIntentSchema = UiIntentSchema::stable("c", 1);
}

struct CollisionOutcome;

impl UiIntentProductOutcome for CollisionOutcome {
    const SCHEMA: UiIntentSchema = UiIntentSchema::stable("outcome", 1);
}

struct CollisionIntentAb;

impl UiIntent for CollisionIntentAb {
    type Payload = CollisionPayloadAb;
    type ProductOutcome = CollisionOutcome;

    const ID: UiIntentId = UiIntentId::stable("a");
    const ACCEPTED_INTERACTIONS: UiIntentAcceptedInteractions =
        UiIntentAcceptedInteractions::new(&[UiSemanticInteractionFamily::Activate]);
}

struct CollisionIntentB;

impl UiIntent for CollisionIntentB {
    type Payload = CollisionPayloadB;
    type ProductOutcome = CollisionOutcome;

    const ID: UiIntentId = UiIntentId::stable("ab");
    const ACCEPTED_INTERACTIONS: UiIntentAcceptedInteractions =
        UiIntentAcceptedInteractions::new(&[UiSemanticInteractionFamily::Activate]);
}

struct SubmitStatus;

impl UiIntent for SubmitStatus {
    type Payload = AdvancePayload;
    type ProductOutcome = AdvanceOutcome;

    const ID: UiIntentId = AdvanceStatus::ID;
    const ACCEPTED_INTERACTIONS: UiIntentAcceptedInteractions =
        UiIntentAcceptedInteractions::new(&[UiSemanticInteractionFamily::Submit]);
}

#[test]
fn typed_definition_freezes_once_into_application_generation_meaning() {
    let app = WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .register_intent_definition(UiIntentDefinition::<AdvanceStatus>::application_effect())
        .expect("one typed definition should register")
        .freeze()
        .expect("typed definition should prepare");

    let definitions = app.capabilities().intent_definitions();
    let frozen = definitions
        .get(&AdvanceStatus::ID)
        .expect("registered definition should freeze");
    assert_eq!(definitions.len(), 1);
    assert_eq!(frozen.id(), AdvanceStatus::ID);
    assert_eq!(frozen.payload_schema(), AdvancePayload::SCHEMA);
    assert_eq!(frozen.product_outcome_schema(), AdvanceOutcome::SCHEMA);
    assert_eq!(
        frozen.accepted_interactions(),
        &[UiSemanticInteractionFamily::Activate]
    );
    assert_eq!(app.capabilities().metrics().registered_family_count(), 1);
    assert_eq!(app.capabilities().metrics().total_width(), 1);
}

#[test]
fn duplicate_definition_identity_stops_before_application_preparation() {
    let builder = WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .register_intent_definition(UiIntentDefinition::<AdvanceStatus>::application_effect())
        .expect("first definition should register");

    let error = match builder
        .register_intent_definition(UiIntentDefinition::<AdvanceStatus>::application_effect())
    {
        Ok(_) => panic!("duplicate definition must stop"),
        Err(error) => error,
    };
    assert_eq!(
        error,
        UiIntentDefinitionRegistrationError::DuplicateIdentity {
            identity: AdvanceStatus::ID,
        }
    );
}

#[test]
fn execution_destination_changes_frozen_application_meaning() {
    let application_effect =
        frozen_digest(UiIntentDefinition::<AdvanceStatus>::application_effect());
    let ui_transition = frozen_digest(UiIntentDefinition::<AdvanceStatus>::ui_transition(
        UiIntentTransitionDestination::NavigatePage,
    ));
    assert_ne!(application_effect, ui_transition);
}

#[test]
fn variable_width_fields_and_interaction_families_change_frozen_meaning() {
    let collision_ab = frozen_digest(UiIntentDefinition::<CollisionIntentAb>::application_effect());
    let collision_b = frozen_digest(UiIntentDefinition::<CollisionIntentB>::application_effect());
    assert_ne!(
        collision_ab, collision_b,
        "field framing must distinguish `a` + `bc` from `ab` + `c`"
    );

    assert_ne!(
        frozen_digest(UiIntentDefinition::<AdvanceStatus>::application_effect()),
        frozen_digest(UiIntentDefinition::<SubmitStatus>::application_effect()),
        "accepted interaction meaning participates in the frozen digest"
    );
}

fn frozen_digest<I: UiIntent>(definition: UiIntentDefinition<I>) -> u64 {
    WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .register_intent_definition(definition)
        .expect("definition should register")
        .freeze()
        .expect("definition should prepare")
        .capabilities()
        .digest()
        .as_u64()
}
