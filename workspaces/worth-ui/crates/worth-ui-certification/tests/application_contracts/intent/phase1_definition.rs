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

fn frozen_digest(definition: UiIntentDefinition<AdvanceStatus>) -> u64 {
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
