use worth_ui::facade::app::WorthUi;
use worth_ui::facade::intent::{
    UiIntent, UiIntentAcceptedInteractions, UiIntentDefinition,
    UiIntentDefinitionRegistrationError, UiIntentId, UiIntentPayload, UiIntentProductOutcome,
    UiIntentSchema, UiIntentTransitionDestination, UiIntentTransitionOutcome,
    UiSemanticInteractionFamily,
};
use worth_ui_certification::WorthUiCertificationBeforeEffectProvider;

struct AdvancePayload;

impl UiIntentPayload for AdvancePayload {
    const SCHEMA: UiIntentSchema = UiIntentSchema::stable("platform.pulse.advance_payload", 1);
    const FIELDS: worth_ui::facade::intent::UiIntentPayloadFieldSet =
        worth_ui::facade::intent::UiIntentPayloadFieldSet::EMPTY;

    fn project(
        _fields: &mut worth_ui::facade::intent::UiIntentPayloadProjection<Self>,
    ) -> Result<Self, worth_ui::facade::intent::UiIntentPayloadProjectionViolation> {
        Ok(Self)
    }
}

struct AdvanceOutcome;

impl UiIntentProductOutcome for AdvanceOutcome {
    const SCHEMA: UiIntentSchema = UiIntentSchema::stable("platform.pulse.advance_outcome", 1);
    const CONSEQUENCE_FAMILIES: worth_ui::facade::intent::UiIntentProductConsequenceFamilies =
        worth_ui::facade::intent::UiIntentProductConsequenceFamilies::NONE;

    fn into_consequences(self) -> worth_ui::facade::intent::UiIntentProductConsequences {
        worth_ui::facade::intent::UiIntentProductConsequences::none()
    }
}

impl UiIntentTransitionOutcome for AdvanceOutcome {
    fn from_completed_transition(_destination: UiIntentTransitionDestination) -> Self {
        Self
    }
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
    const FIELDS: worth_ui::facade::intent::UiIntentPayloadFieldSet =
        worth_ui::facade::intent::UiIntentPayloadFieldSet::EMPTY;

    fn project(
        _fields: &mut worth_ui::facade::intent::UiIntentPayloadProjection<Self>,
    ) -> Result<Self, worth_ui::facade::intent::UiIntentPayloadProjectionViolation> {
        Ok(Self)
    }
}

struct CollisionPayloadB;

impl UiIntentPayload for CollisionPayloadB {
    const SCHEMA: UiIntentSchema = UiIntentSchema::stable("c", 1);
    const FIELDS: worth_ui::facade::intent::UiIntentPayloadFieldSet =
        worth_ui::facade::intent::UiIntentPayloadFieldSet::EMPTY;

    fn project(
        _fields: &mut worth_ui::facade::intent::UiIntentPayloadProjection<Self>,
    ) -> Result<Self, worth_ui::facade::intent::UiIntentPayloadProjectionViolation> {
        Ok(Self)
    }
}

struct CollisionOutcome;

impl UiIntentProductOutcome for CollisionOutcome {
    const SCHEMA: UiIntentSchema = UiIntentSchema::stable("outcome", 1);
    const CONSEQUENCE_FAMILIES: worth_ui::facade::intent::UiIntentProductConsequenceFamilies =
        worth_ui::facade::intent::UiIntentProductConsequenceFamilies::NONE;

    fn into_consequences(self) -> worth_ui::facade::intent::UiIntentProductConsequences {
        worth_ui::facade::intent::UiIntentProductConsequences::none()
    }
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
        .bind_certification_host_adapter(
            worth_ui_host_contract::UiCertificationHostBindingGrant::for_certification(),
            worth_ui_host_headless::WorthUiHeadlessHost,
        )
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .register_intent_definition(UiIntentDefinition::<AdvanceStatus>::application_effect())
        .expect("one typed definition should register")
        .register_intent_provider(WorthUiCertificationBeforeEffectProvider::<AdvanceStatus>::new())
        .expect("one typed provider should register")
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
        .bind_certification_host_adapter(
            worth_ui_host_contract::UiCertificationHostBindingGrant::for_certification(),
            worth_ui_host_headless::WorthUiHeadlessHost,
        )
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .register_intent_definition(UiIntentDefinition::<AdvanceStatus>::application_effect())
        .expect("first definition should register")
        .register_intent_provider(WorthUiCertificationBeforeEffectProvider::<AdvanceStatus>::new())
        .expect("first provider should register");

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
        frozen_application_digest(UiIntentDefinition::<AdvanceStatus>::application_effect());
    let ui_transition =
        frozen_transition_digest(UiIntentDefinition::<AdvanceStatus>::ui_transition(
            UiIntentTransitionDestination::NavigatePage,
        ));
    assert_ne!(application_effect, ui_transition);
}

#[test]
fn variable_width_fields_and_interaction_families_change_frozen_meaning() {
    let collision_ab =
        frozen_application_digest(UiIntentDefinition::<CollisionIntentAb>::application_effect());
    let collision_b =
        frozen_application_digest(UiIntentDefinition::<CollisionIntentB>::application_effect());
    assert_ne!(
        collision_ab, collision_b,
        "field framing must distinguish `a` + `bc` from `ab` + `c`"
    );

    assert_ne!(
        frozen_application_digest(UiIntentDefinition::<AdvanceStatus>::application_effect()),
        frozen_application_digest(UiIntentDefinition::<SubmitStatus>::application_effect()),
        "accepted interaction meaning participates in the frozen digest"
    );
}

fn frozen_application_digest<I: UiIntent>(definition: UiIntentDefinition<I>) -> u64 {
    WorthUi::app()
        .bind_certification_host_adapter(
            worth_ui_host_contract::UiCertificationHostBindingGrant::for_certification(),
            worth_ui_host_headless::WorthUiHeadlessHost,
        )
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .register_intent_definition(definition)
        .expect("definition should register")
        .register_intent_provider(WorthUiCertificationBeforeEffectProvider::<I>::new())
        .expect("provider should register")
        .freeze()
        .expect("definition should prepare")
        .capabilities()
        .digest()
        .as_u64()
}

fn frozen_transition_digest<I>(
    definition: worth_ui::facade::intent::UiIntentDefinition<
        I,
        worth_ui::facade::intent::UiTransitionDefinitionDestination,
    >,
) -> u64
where
    I: UiIntent,
    I::ProductOutcome: UiIntentTransitionOutcome,
{
    WorthUi::app()
        .bind_certification_host_adapter(
            worth_ui_host_contract::UiCertificationHostBindingGrant::for_certification(),
            worth_ui_host_headless::WorthUiHeadlessHost,
        )
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .register_intent_transition_definition(definition)
        .expect("transition definition should register")
        .freeze()
        .expect("transition definition should prepare")
        .capabilities()
        .digest()
        .as_u64()
}
