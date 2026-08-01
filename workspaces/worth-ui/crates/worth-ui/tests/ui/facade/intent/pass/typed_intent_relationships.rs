use std::sync::Arc;

use worth_ui::facade::{
    app::{WorthUi, WorthUiActiveApplicationSession},
    intent::{
        UiAdmittedIntent, UiIntent, UiIntentAcceptedInteractions, UiIntentAdmissionSettlementReceipt,
        UiIntentApplicationFact, UiIntentBoolean, UiIntentConcurrencyScope,
        UiIntentConfirmationContract, UiIntentConsequenceContract, UiIntentDeclaration,
        UiIntentDefinition,
        UiIntentExecutionClockReading, UiIntentExecutionDispatchOutcome, UiIntentId,
        UiIntentMutabilitySource,
        UiIntentOperabilityContract, UiIntentPayload,
        UiIntentPayloadField, UiIntentPayloadFieldSet, UiIntentPayloadProjection,
        UiIntentPayloadProjectionViolation, UiIntentPayloadSource, UiIntentPolicySource,
        UiIntentProductOutcome, UiIntentProviderStart, UiIntentProviderStop,
        UiIntentProviderVersion, UiIntentReadinessSource, UiIntentSchema, UiIntentText,
        UiIntentExecutionProvider, UiIntentExecutionRequest, UiIntentExecutionTransition,
        UiIntentRecoveryHandle, UiIntentRecoveryProgressOutcome, UiSemanticInteractionFamily,
    },
    rebind::UiChangeProfile,
};

const MESSAGE: UiIntentPayloadField<Payload, UiIntentText> =
    UiIntentPayloadField::text(0, "message", 32);

struct Payload {
    _message: Arc<str>,
}

impl UiIntentPayload for Payload {
    const SCHEMA: UiIntentSchema = UiIntentSchema::stable("compile.payload", 1);
    const FIELDS: UiIntentPayloadFieldSet = UiIntentPayloadFieldSet::new(&[MESSAGE.descriptor()]);

    fn project(
        fields: &mut UiIntentPayloadProjection<Self>,
    ) -> Result<Self, UiIntentPayloadProjectionViolation> {
        Ok(Self {
            _message: fields.take(MESSAGE)?,
        })
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

impl UiIntent for Intent {
    type Payload = Payload;
    type ProductOutcome = Outcome;

    const ID: UiIntentId = UiIntentId::stable("compile.intent");
    const ACCEPTED_INTERACTIONS: UiIntentAcceptedInteractions =
        UiIntentAcceptedInteractions::new(&[UiSemanticInteractionFamily::Activate]);
}

struct Provider;

impl UiIntentExecutionProvider<Intent> for Provider {
    const VERSION: UiIntentProviderVersion = UiIntentProviderVersion::stable(1);

    fn begin(&self, request: UiIntentExecutionRequest<Intent>) -> UiIntentProviderStart<Intent> {
        drop(request);
        UiIntentProviderStart::RejectedBeforeEffect(UiIntentProviderStop::stable(
            "compile.before_effect",
        ))
    }
}

fn settle_typed<I: UiIntent>(
    session: &mut WorthUiActiveApplicationSession,
    admitted: UiAdmittedIntent<I>,
) -> UiIntentAdmissionSettlementReceipt {
    session.cancel_admitted_intent(admitted)
}

fn dispatch_once<I: UiIntent>(
    session: &mut WorthUiActiveApplicationSession,
    admitted: UiAdmittedIntent<I>,
) -> UiIntentExecutionDispatchOutcome {
    let deadline = UiIntentExecutionClockReading::at_tick(0)
        .deadline_after_ticks(1)
        .expect("compile fixture deadline fits");
    session.dispatch_admitted_intent(admitted, deadline)
}

fn retry_recovery_once(
    session: &mut WorthUiActiveApplicationSession,
    recovery: UiIntentRecoveryHandle,
) -> UiIntentRecoveryProgressOutcome {
    session.retry_intent_recovery(recovery, UiIntentExecutionClockReading::at_tick(1))
}

fn consume_terminal_once(
    transition: UiIntentExecutionTransition,
) -> Option<UiIntentRecoveryHandle> {
    transition.into_recovery()
}

fn main() {
    let _settle: fn(
        &mut WorthUiActiveApplicationSession,
        UiAdmittedIntent<Intent>,
    ) -> UiIntentAdmissionSettlementReceipt = settle_typed::<Intent>;
    let _dispatch: fn(
        &mut WorthUiActiveApplicationSession,
        UiAdmittedIntent<Intent>,
    ) -> UiIntentExecutionDispatchOutcome = dispatch_once::<Intent>;
    let _retry: fn(
        &mut WorthUiActiveApplicationSession,
        UiIntentRecoveryHandle,
    ) -> UiIntentRecoveryProgressOutcome = retry_recovery_once;
    let _terminal: fn(UiIntentExecutionTransition) -> Option<UiIntentRecoveryHandle> =
        consume_terminal_once;
    let operable =
        UiIntentApplicationFact::<UiIntentBoolean>::boolean("compile.intent.operable").unwrap();
    let _declaration = UiIntentDeclaration::<Intent>::activate("compile.intent.route")
        .expect("the accepted interaction constructs")
        .bind_payload(
            MESSAGE,
            UiIntentPayloadSource::<UiIntentText>::constant("hello"),
        )
        .operability_from(
            UiIntentOperabilityContract::new(
                "compile.intent.operability",
                UiIntentMutabilitySource::application_fact(&operable),
                UiIntentReadinessSource::application_fact(&operable),
                UiIntentPolicySource::application_fact(&operable),
            )
            .unwrap(),
        )
        .confirmation(UiIntentConfirmationContract::not_required("compile.intent.confirm").unwrap())
        .concurrency(UiIntentConcurrencyScope::TargetRouteSingleFlight)
        .consequences(UiIntentConsequenceContract::none())
        .into_dsl_spec();
    let app = WorthUi::app()
        .with_change_profile(UiChangeProfile::platform_pulse())
        .register_intent_boolean_fact(operable, true)
        .expect("typed operability fact should register")
        .register_intent_definition(UiIntentDefinition::<Intent>::application_effect())
        .expect("typed definition should register")
        .register_intent_provider(Provider)
        .expect("the exact typed provider should register")
        .freeze()
        .expect("typed definition should prepare");
    assert!(app
        .capabilities()
        .intent_definitions()
        .get(&Intent::ID)
        .is_some());
}
