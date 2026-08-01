use worth_ui::facade::intent::{
    UiIntent, UiIntentAcceptedInteractions, UiIntentDeclaration, UiIntentId, UiIntentPayload,
    UiIntentPayloadField, UiIntentPayloadFieldSet, UiIntentPayloadSource, UiIntentProductOutcome,
    UiIntentSchema, UiIntentText, UiSemanticInteractionFamily,
};

include!("causal_trace_cannot_become_admission.rs");
include!("raw_and_allocation_cannot_mint_semantic_interaction.rs");
include!("diagnostic_evidence_cannot_confirm_or_complete.rs");
include!("ui_admission_cannot_become_query_publication.rs");

const MESSAGE: UiIntentPayloadField<Payload, UiIntentText> =
    UiIntentPayloadField::text(0, "message", 32);

struct Payload;

impl UiIntentPayload for Payload {
    const SCHEMA: UiIntentSchema = UiIntentSchema::stable("compile.payload", 1);
    const FIELDS: UiIntentPayloadFieldSet = UiIntentPayloadFieldSet::new(&[MESSAGE.descriptor()]);

    fn project(
        _fields: &mut worth_ui::facade::intent::UiIntentPayloadProjection<Self>,
    ) -> Result<Self, worth_ui::facade::intent::UiIntentPayloadProjectionViolation> {
        Ok(Self)
    }
}

struct ForeignPayload;

impl UiIntentPayload for ForeignPayload {
    const SCHEMA: UiIntentSchema = UiIntentSchema::stable("compile.foreign-payload", 1);
    const FIELDS: UiIntentPayloadFieldSet = UiIntentPayloadFieldSet::EMPTY;

    fn project(
        _fields: &mut worth_ui::facade::intent::UiIntentPayloadProjection<Self>,
    ) -> Result<Self, worth_ui::facade::intent::UiIntentPayloadProjectionViolation> {
        Ok(Self)
    }
}

struct Outcome;

impl UiIntentProductOutcome for Outcome {
    const SCHEMA: UiIntentSchema = UiIntentSchema::stable("compile.outcome", 1);
    const CONSEQUENCE_FAMILIES: worth_ui::facade::intent::UiIntentProductConsequenceFamilies =
        worth_ui::facade::intent::UiIntentProductConsequenceFamilies::NONE;

    fn into_consequences(self) -> worth_ui::facade::intent::UiIntentProductConsequences {
        worth_ui::facade::intent::UiIntentProductConsequences::none()
    }
}

struct Intent;
struct ForeignIntent;

impl UiIntent for Intent {
    type Payload = Payload;
    type ProductOutcome = Outcome;

    const ID: UiIntentId = UiIntentId::stable("compile.intent");
    const ACCEPTED_INTERACTIONS: UiIntentAcceptedInteractions =
        UiIntentAcceptedInteractions::new(&[UiSemanticInteractionFamily::Activate]);
}

impl UiIntent for ForeignIntent {
    type Payload = ForeignPayload;
    type ProductOutcome = Outcome;

    const ID: UiIntentId = UiIntentId::stable("compile.foreign-intent");
    const ACCEPTED_INTERACTIONS: UiIntentAcceptedInteractions =
        UiIntentAcceptedInteractions::new(&[UiSemanticInteractionFamily::Activate]);
}

fn main() {
    let _ = UiIntentDeclaration::<ForeignIntent>::activate("compile.foreign.route")
        .unwrap()
        .bind_payload(
            MESSAGE,
            UiIntentPayloadSource::<UiIntentText>::constant("shape-crossing"),
        );
}
