use worth_ui::facade::intent::{
    UiIntent, UiIntentAcceptedInteractions, UiIntentApplicationFact, UiIntentBoolean,
    UiIntentDefinition, UiIntentId, UiIntentPayload, UiIntentPayloadField, UiIntentPayloadFieldSet,
    UiIntentPayloadProjection, UiIntentPayloadProjectionViolation,
    UiIntentProductConsequenceFamilies, UiIntentProductConsequences, UiIntentProductOutcome,
    UiIntentSchema, UiIntentUnsigned64, UiSemanticInteractionFamily,
};
use worth_ui::facade::query_binding::UiProjectionObservation;

pub const PLATFORM_PULSE_ACTION_DEFINITION: &str = "platform.pulse.action";
pub const PLATFORM_PULSE_ACTION_DECLARATION: &str = "platform.pulse.action.route";
pub const PLATFORM_PULSE_ACTION_CONFIRMATION: &str = "platform.pulse.action.confirmation";
pub const PLATFORM_PULSE_ACTION_QUERY_VIEW: &str = "platform.pulse.action.values";

const MUTABILITY_FACT: &str = "platform.pulse.action.mutable";
const READINESS_FACT: &str = "platform.pulse.action.ready";
const POLICY_FACT: &str = "platform.pulse.action.policy-allowed";
const CONFIRMATION_FACT: &str = "platform.pulse.action.confirmation-required";
const REVISION_FACT: &str = "platform.pulse.action.input-revision";
const QUERY_DENIAL_FACT: &str = "platform.pulse.action.query-denial-requested";

pub const PLATFORM_PULSE_ACTION_REVISION_FIELD: UiIntentPayloadField<
    PlatformPulseActionPayload,
    UiIntentUnsigned64,
> = UiIntentPayloadField::unsigned64(0, "action_input_revision");
pub const PLATFORM_PULSE_QUERY_DENIAL_FIELD: UiIntentPayloadField<
    PlatformPulseActionPayload,
    UiIntentBoolean,
> = UiIntentPayloadField::boolean(1, "query_denial_requested");

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlatformPulseActionInputRevision(u64);

pub struct PlatformPulseActionPayload {
    action_input_revision: PlatformPulseActionInputRevision,
    query_denial_requested: bool,
}

pub struct PlatformPulseActionOutcome {
    query: UiProjectionObservation,
}

pub struct PlatformPulseAction;

pub const fn platform_pulse_action_definition() -> UiIntentDefinition<PlatformPulseAction> {
    UiIntentDefinition::application_effect()
}

impl PlatformPulseActionPayload {
    pub const fn action_input_revision(&self) -> PlatformPulseActionInputRevision {
        self.action_input_revision
    }

    pub const fn query_denial_requested(&self) -> bool {
        self.query_denial_requested
    }

    #[cfg(test)]
    pub(super) const fn for_test(action_input_revision: u64) -> Self {
        Self {
            action_input_revision: PlatformPulseActionInputRevision(action_input_revision),
            query_denial_requested: false,
        }
    }
}

impl PlatformPulseActionInputRevision {
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl UiIntentPayload for PlatformPulseActionPayload {
    const SCHEMA: UiIntentSchema = UiIntentSchema::stable("platform.pulse.action.payload", 3);
    const FIELDS: UiIntentPayloadFieldSet = UiIntentPayloadFieldSet::new(&[
        PLATFORM_PULSE_ACTION_REVISION_FIELD.descriptor(),
        PLATFORM_PULSE_QUERY_DENIAL_FIELD.descriptor(),
    ]);

    fn project(
        fields: &mut UiIntentPayloadProjection<Self>,
    ) -> Result<Self, UiIntentPayloadProjectionViolation> {
        Ok(Self {
            action_input_revision: PlatformPulseActionInputRevision(
                fields.take(PLATFORM_PULSE_ACTION_REVISION_FIELD)?,
            ),
            query_denial_requested: fields.take(PLATFORM_PULSE_QUERY_DENIAL_FIELD)?,
        })
    }
}

impl PlatformPulseActionOutcome {
    pub fn query(query: UiProjectionObservation) -> Self {
        Self { query }
    }
}

impl UiIntentProductOutcome for PlatformPulseActionOutcome {
    const SCHEMA: UiIntentSchema = UiIntentSchema::stable("platform.pulse.action.outcome", 1);
    const CONSEQUENCE_FAMILIES: UiIntentProductConsequenceFamilies =
        UiIntentProductConsequenceFamilies::QUERY_PROJECTION;

    fn into_consequences(self) -> UiIntentProductConsequences {
        UiIntentProductConsequences::query_projection(self.query)
    }
}

impl UiIntent for PlatformPulseAction {
    type Payload = PlatformPulseActionPayload;
    type ProductOutcome = PlatformPulseActionOutcome;

    const ID: UiIntentId = UiIntentId::stable(PLATFORM_PULSE_ACTION_DEFINITION);
    const ACCEPTED_INTERACTIONS: UiIntentAcceptedInteractions =
        UiIntentAcceptedInteractions::new(&[UiSemanticInteractionFamily::Activate]);
}

pub fn platform_pulse_action_mutability_fact() -> UiIntentApplicationFact<UiIntentBoolean> {
    boolean_fact(MUTABILITY_FACT)
}

pub fn platform_pulse_action_readiness_fact() -> UiIntentApplicationFact<UiIntentBoolean> {
    boolean_fact(READINESS_FACT)
}

pub fn platform_pulse_action_policy_fact() -> UiIntentApplicationFact<UiIntentBoolean> {
    boolean_fact(POLICY_FACT)
}

pub fn platform_pulse_action_confirmation_fact() -> UiIntentApplicationFact<UiIntentBoolean> {
    boolean_fact(CONFIRMATION_FACT)
}

pub fn platform_pulse_action_revision_fact() -> UiIntentApplicationFact<UiIntentUnsigned64> {
    UiIntentApplicationFact::unsigned64(REVISION_FACT)
        .expect("the static Pulse action revision fact identity is valid")
}

pub fn platform_pulse_query_denial_fact() -> UiIntentApplicationFact<UiIntentBoolean> {
    boolean_fact(QUERY_DENIAL_FACT)
}

fn boolean_fact(identity: &str) -> UiIntentApplicationFact<UiIntentBoolean> {
    UiIntentApplicationFact::boolean(identity)
        .expect("the static Pulse action boolean fact identity is valid")
}
