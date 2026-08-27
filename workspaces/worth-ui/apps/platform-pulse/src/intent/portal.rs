use worth_ui::facade::intent::{
    UiIntent, UiIntentAcceptedInteractions, UiIntentDefinition, UiIntentId, UiIntentPayload,
    UiIntentPayloadFieldSet, UiIntentPayloadProjection, UiIntentPayloadProjectionViolation,
    UiIntentProductConsequenceFamilies, UiIntentProductConsequences, UiIntentProductOutcome,
    UiIntentRuntimeServiceDestination, UiIntentSchema, UiIntentTransitionDestination,
    UiIntentTransitionOutcome, UiRuntimeServiceDefinitionDestination, UiSemanticInteractionFamily,
};

pub const PLATFORM_PULSE_OPEN_PORTAL_DEFINITION: &str = "platform.pulse.portal.open";
pub const PLATFORM_PULSE_OPEN_PORTAL_DECLARATION: &str = "platform.pulse.portal.open.route";

pub struct PlatformPulseOpenPortalPayload;
pub struct PlatformPulseOpenPortalOutcome;
pub struct PlatformPulseOpenPortal;

pub const fn platform_pulse_open_portal_definition(
) -> UiIntentDefinition<PlatformPulseOpenPortal, UiRuntimeServiceDefinitionDestination> {
    UiIntentDefinition::runtime_service(UiIntentRuntimeServiceDestination::OpenPortal)
}

impl UiIntentPayload for PlatformPulseOpenPortalPayload {
    const SCHEMA: UiIntentSchema = UiIntentSchema::stable("platform.pulse.portal.open.payload", 1);
    const FIELDS: UiIntentPayloadFieldSet = UiIntentPayloadFieldSet::EMPTY;

    fn project(
        _fields: &mut UiIntentPayloadProjection<Self>,
    ) -> Result<Self, UiIntentPayloadProjectionViolation> {
        Ok(Self)
    }
}

impl UiIntentProductOutcome for PlatformPulseOpenPortalOutcome {
    const SCHEMA: UiIntentSchema = UiIntentSchema::stable("platform.pulse.portal.open.outcome", 1);
    const CONSEQUENCE_FAMILIES: UiIntentProductConsequenceFamilies =
        UiIntentProductConsequenceFamilies::NONE;

    fn into_consequences(self) -> UiIntentProductConsequences {
        UiIntentProductConsequences::none()
    }
}

impl UiIntentTransitionOutcome for PlatformPulseOpenPortalOutcome {
    fn from_completed_transition(_destination: UiIntentTransitionDestination) -> Self {
        Self
    }
}

impl UiIntent for PlatformPulseOpenPortal {
    type Payload = PlatformPulseOpenPortalPayload;
    type ProductOutcome = PlatformPulseOpenPortalOutcome;

    const ID: UiIntentId = UiIntentId::stable(PLATFORM_PULSE_OPEN_PORTAL_DEFINITION);
    const ACCEPTED_INTERACTIONS: UiIntentAcceptedInteractions =
        UiIntentAcceptedInteractions::new(&[UiSemanticInteractionFamily::Activate]);
}
