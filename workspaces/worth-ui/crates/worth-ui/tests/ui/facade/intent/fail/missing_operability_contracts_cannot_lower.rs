use worth_ui::facade::intent::{
    UiIntent, UiIntentAcceptedInteractions, UiIntentDeclaration, UiIntentId, UiIntentPayload,
    UiIntentPayloadFieldSet, UiIntentPayloadProjection, UiIntentPayloadProjectionViolation,
    UiIntentProductOutcome, UiIntentSchema, UiSemanticInteractionFamily,
};

struct Payload;
struct Outcome;
struct Intent;

impl UiIntentPayload for Payload {
    const SCHEMA: UiIntentSchema = UiIntentSchema::stable("compile.payload", 1);
    const FIELDS: UiIntentPayloadFieldSet = UiIntentPayloadFieldSet::EMPTY;

    fn project(
        _fields: &mut UiIntentPayloadProjection<Self>,
    ) -> Result<Self, UiIntentPayloadProjectionViolation> {
        Ok(Self)
    }
}

impl UiIntentProductOutcome for Outcome {
    const SCHEMA: UiIntentSchema = UiIntentSchema::stable("compile.outcome", 1);
    const CONSEQUENCE_FAMILIES: worth_ui::facade::intent::UiIntentProductConsequenceFamilies =
        worth_ui::facade::intent::UiIntentProductConsequenceFamilies::NONE;

    fn into_consequences(self) -> worth_ui::facade::intent::UiIntentProductConsequences {
        worth_ui::facade::intent::UiIntentProductConsequences::none()
    }
}

impl UiIntent for Intent {
    type Payload = Payload;
    type ProductOutcome = Outcome;

    const ID: UiIntentId = UiIntentId::stable("compile.intent");
    const ACCEPTED_INTERACTIONS: UiIntentAcceptedInteractions =
        UiIntentAcceptedInteractions::new(&[UiSemanticInteractionFamily::Activate]);
}

fn main() {
    let _ = UiIntentDeclaration::<Intent>::activate("compile.intent.route")
        .unwrap()
        .into_dsl_spec();
}
