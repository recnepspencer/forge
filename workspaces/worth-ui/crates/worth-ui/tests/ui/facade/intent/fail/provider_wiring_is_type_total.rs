use worth_ui::facade::{
    app::WorthUi,
    intent::{
        UiIntent, UiIntentAcceptedInteractions, UiIntentDefinition, UiIntentExecutionProvider,
        UiIntentExecutionRequest, UiIntentId, UiIntentPayload, UiIntentPayloadFieldSet,
        UiIntentPayloadProjection, UiIntentPayloadProjectionViolation, UiIntentProductOutcome,
        UiIntentProviderStart, UiIntentProviderStop, UiIntentProviderVersion, UiIntentSchema,
        UiSemanticInteractionFamily,
    },
    rebind::UiChangeProfile,
};

struct EmptyPayload;

impl UiIntentPayload for EmptyPayload {
    const SCHEMA: UiIntentSchema = UiIntentSchema::stable("compile.empty.payload", 1);
    const FIELDS: UiIntentPayloadFieldSet = UiIntentPayloadFieldSet::EMPTY;

    fn project(
        _fields: &mut UiIntentPayloadProjection<Self>,
    ) -> Result<Self, UiIntentPayloadProjectionViolation> {
        Ok(Self)
    }
}

struct EmptyOutcome;

impl UiIntentProductOutcome for EmptyOutcome {
    const SCHEMA: UiIntentSchema = UiIntentSchema::stable("compile.empty.outcome", 1);
    const CONSEQUENCE_FAMILIES: worth_ui::facade::intent::UiIntentProductConsequenceFamilies =
        worth_ui::facade::intent::UiIntentProductConsequenceFamilies::NONE;

    fn into_consequences(self) -> worth_ui::facade::intent::UiIntentProductConsequences {
        worth_ui::facade::intent::UiIntentProductConsequences::none()
    }
}

struct FirstIntent;

impl UiIntent for FirstIntent {
    type Payload = EmptyPayload;
    type ProductOutcome = EmptyOutcome;

    const ID: UiIntentId = UiIntentId::stable("compile.first-intent");
    const ACCEPTED_INTERACTIONS: UiIntentAcceptedInteractions =
        UiIntentAcceptedInteractions::new(&[UiSemanticInteractionFamily::Activate]);
}

struct SecondIntent;

impl UiIntent for SecondIntent {
    type Payload = EmptyPayload;
    type ProductOutcome = EmptyOutcome;

    const ID: UiIntentId = UiIntentId::stable("compile.second-intent");
    const ACCEPTED_INTERACTIONS: UiIntentAcceptedInteractions =
        UiIntentAcceptedInteractions::new(&[UiSemanticInteractionFamily::Activate]);
}

struct SecondProvider;

impl UiIntentExecutionProvider<SecondIntent> for SecondProvider {
    const VERSION: UiIntentProviderVersion = UiIntentProviderVersion::stable(1);

    fn begin(
        &self,
        request: UiIntentExecutionRequest<SecondIntent>,
    ) -> UiIntentProviderStart<SecondIntent> {
        drop(request);
        UiIntentProviderStart::RejectedBeforeEffect(UiIntentProviderStop::stable(
            "compile.before_effect",
        ))
    }
}

fn missing_provider_cannot_freeze() {
    let _ = WorthUi::app()
        .with_change_profile(UiChangeProfile::platform_pulse())
        .register_intent_definition(UiIntentDefinition::<FirstIntent>::application_effect())
        .unwrap()
        .freeze();
}

fn cross_intent_provider_cannot_register() {
    let _ = WorthUi::app()
        .with_change_profile(UiChangeProfile::platform_pulse())
        .register_intent_definition(UiIntentDefinition::<FirstIntent>::application_effect())
        .unwrap()
        .register_intent_provider(SecondProvider);
}

fn main() {}
