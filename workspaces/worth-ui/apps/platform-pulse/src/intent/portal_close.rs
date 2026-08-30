use worth_ui::facade::intent::{
    UiIntent, UiIntentAcceptedInteractions, UiIntentApplicationFact, UiIntentBoolean,
    UiIntentDefinition, UiIntentId, UiIntentPayload, UiIntentPayloadFieldSet,
    UiIntentPayloadProjection, UiIntentPayloadProjectionViolation,
    UiIntentProductConsequenceFamilies, UiIntentProductConsequences, UiIntentProductOutcome,
    UiIntentRuntimeServiceDestination, UiIntentSchema, UiIntentTransitionDestination,
    UiIntentTransitionOutcome, UiRuntimeServiceDefinitionDestination, UiSemanticInteractionFamily,
};

pub const PLATFORM_PULSE_CLOSE_PORTAL_DEFINITION: &str = "platform.pulse.portal.close";
pub const PLATFORM_PULSE_CLOSE_PORTAL_DECLARATION: &str = "platform.pulse.portal.close.route";

const MUTABILITY_FACT: &str = "platform.pulse.portal.dismiss.mutable";
const READINESS_FACT: &str = "platform.pulse.portal.dismiss.ready";
const POLICY_FACT: &str = "platform.pulse.portal.dismiss.policy-allowed";
const CONFIRMATION_FACT: &str = "platform.pulse.portal.dismiss.confirmation-required";

pub struct PlatformPulseClosePortalPayload;
pub struct PlatformPulseClosePortalOutcome;
pub struct PlatformPulseClosePortal;

pub const fn platform_pulse_close_portal_definition(
) -> UiIntentDefinition<PlatformPulseClosePortal, UiRuntimeServiceDefinitionDestination> {
    UiIntentDefinition::runtime_service(UiIntentRuntimeServiceDestination::ClosePortal)
}

pub fn platform_pulse_close_portal_mutability_fact() -> UiIntentApplicationFact<UiIntentBoolean> {
    boolean_fact(MUTABILITY_FACT)
}

pub fn platform_pulse_close_portal_readiness_fact() -> UiIntentApplicationFact<UiIntentBoolean> {
    boolean_fact(READINESS_FACT)
}

pub fn platform_pulse_close_portal_policy_fact() -> UiIntentApplicationFact<UiIntentBoolean> {
    boolean_fact(POLICY_FACT)
}

pub fn platform_pulse_close_portal_confirmation_fact() -> UiIntentApplicationFact<UiIntentBoolean> {
    boolean_fact(CONFIRMATION_FACT)
}

fn boolean_fact(identity: &str) -> UiIntentApplicationFact<UiIntentBoolean> {
    UiIntentApplicationFact::boolean(identity)
        .expect("the static Pulse portal dismissal fact identity is valid")
}

impl UiIntentPayload for PlatformPulseClosePortalPayload {
    const SCHEMA: UiIntentSchema = UiIntentSchema::stable("platform.pulse.portal.close.payload", 1);
    const FIELDS: UiIntentPayloadFieldSet = UiIntentPayloadFieldSet::EMPTY;

    fn project(
        _fields: &mut UiIntentPayloadProjection<Self>,
    ) -> Result<Self, UiIntentPayloadProjectionViolation> {
        Ok(Self)
    }
}

impl UiIntentProductOutcome for PlatformPulseClosePortalOutcome {
    const SCHEMA: UiIntentSchema = UiIntentSchema::stable("platform.pulse.portal.close.outcome", 1);
    const CONSEQUENCE_FAMILIES: UiIntentProductConsequenceFamilies =
        UiIntentProductConsequenceFamilies::NONE;

    fn into_consequences(self) -> UiIntentProductConsequences {
        UiIntentProductConsequences::none()
    }
}

impl UiIntentTransitionOutcome for PlatformPulseClosePortalOutcome {
    fn from_completed_transition(_destination: UiIntentTransitionDestination) -> Self {
        Self
    }
}

impl UiIntent for PlatformPulseClosePortal {
    type Payload = PlatformPulseClosePortalPayload;
    type ProductOutcome = PlatformPulseClosePortalOutcome;

    const ID: UiIntentId = UiIntentId::stable(PLATFORM_PULSE_CLOSE_PORTAL_DEFINITION);
    const ACCEPTED_INTERACTIONS: UiIntentAcceptedInteractions =
        UiIntentAcceptedInteractions::new(&[
            UiSemanticInteractionFamily::Activate,
            UiSemanticInteractionFamily::Submit,
        ]);
}
