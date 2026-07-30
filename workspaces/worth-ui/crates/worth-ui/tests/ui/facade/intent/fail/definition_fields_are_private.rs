use core::marker::PhantomData;
use worth_ui::facade::intent::{
    UiIntent, UiIntentDefinition, UiIntentExecutionDestination, UiIntentId, UiIntentPayload,
    UiIntentProductOutcome, UiIntentSchema, UiSemanticInteractionFamily,
};

struct Payload;

impl UiIntentPayload for Payload {
    const SCHEMA: UiIntentSchema = UiIntentSchema::stable("compile.payload", 1);
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
    const ACCEPTED_INTERACTIONS: &'static [UiSemanticInteractionFamily] =
        &[UiSemanticInteractionFamily::Activate];
}

fn main() {
    let _ = UiIntentDefinition::<Intent> {
        destination: UiIntentExecutionDestination::ApplicationEffect,
        intent: PhantomData,
    };
}
