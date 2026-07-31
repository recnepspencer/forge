use worth_ui::facade::{
    app::WorthUi,
    intent::{
        UiIntent, UiIntentAcceptedInteractions, UiIntentDefinition, UiIntentId, UiIntentPayload,
        UiIntentProductOutcome, UiIntentSchema, UiSemanticInteractionFamily,
    },
    rebind::UiChangeProfile,
};

struct Payload;

impl UiIntentPayload for Payload {
    const SCHEMA: UiIntentSchema = UiIntentSchema::stable("compile.payload", 1);
    const FIELDS: worth_ui::facade::intent::UiIntentPayloadFieldSet =
        worth_ui::facade::intent::UiIntentPayloadFieldSet::EMPTY;

    fn project(
        _fields: &mut worth_ui::facade::intent::UiIntentPayloadProjection<Self>,
    ) -> Result<Self, worth_ui::facade::intent::UiIntentPayloadProjectionViolation> {
        Ok(Self)
    }
}

struct Outcome;

impl UiIntentProductOutcome for Outcome {
    const SCHEMA: UiIntentSchema = UiIntentSchema::stable("compile.outcome", 1);
}

struct Intent;

impl UiIntent for Intent {
    type Payload = Payload;
    type ProductOutcome = Outcome;

    const ID: UiIntentId = UiIntentId::stable("compile.intent");
    const ACCEPTED_INTERACTIONS: UiIntentAcceptedInteractions =
        UiIntentAcceptedInteractions::new(&[UiSemanticInteractionFamily::Activate]);
}

fn main() {
    let app = WorthUi::app()
        .with_change_profile(UiChangeProfile::platform_pulse())
        .register_intent_definition(UiIntentDefinition::<Intent>::application_effect())
        .expect("typed definition should register")
        .freeze()
        .expect("typed definition should prepare");
    assert!(app.capabilities().intent_definitions().get(&Intent::ID).is_some());
}
