use core::marker::PhantomData;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use worth_ui::facade::{
    app::{WorthUi, WorthUiApp, WorthUiApplicationPreparationDenial},
    intent::{
        UiIntent, UiIntentAcceptedInteractions, UiIntentDefinition, UiIntentExecutionProvider,
        UiIntentExecutionRequest, UiIntentId, UiIntentPayload, UiIntentPayloadFieldSet,
        UiIntentPayloadProjection, UiIntentPayloadProjectionViolation, UiIntentProductOutcome,
        UiIntentProviderStart, UiIntentProviderStop, UiIntentProviderVersion,
        UiIntentRuntimeServiceDestination, UiIntentSchema, UiIntentTransitionDestination,
        UiIntentTransitionOutcome, UiSemanticInteractionFamily,
    },
    rebind::UiChangeProfile,
    source::WorthUiSemanticHandoffPreparationStop,
};
use worth_ui_dsl::{WorthUiRustAuthoredArtifactInput, WorthUiRustAuthoredArtifactInputModule};
use worth_ui_runtime::certification_support::WorthUiIntentExecutionBindingCertificationExt;

struct EmptyPayload;

impl UiIntentPayload for EmptyPayload {
    const SCHEMA: UiIntentSchema = UiIntentSchema::stable("phase4.registration.payload", 1);
    const FIELDS: UiIntentPayloadFieldSet = UiIntentPayloadFieldSet::EMPTY;

    fn project(
        _fields: &mut UiIntentPayloadProjection<Self>,
    ) -> Result<Self, UiIntentPayloadProjectionViolation> {
        Ok(Self)
    }
}

struct EmptyOutcome;

impl UiIntentProductOutcome for EmptyOutcome {
    const SCHEMA: UiIntentSchema = UiIntentSchema::stable("phase4.registration.outcome", 1);
    const CONSEQUENCE_FAMILIES: worth_ui::facade::intent::UiIntentProductConsequenceFamilies =
        worth_ui::facade::intent::UiIntentProductConsequenceFamilies::NONE;

    fn into_consequences(self) -> worth_ui::facade::intent::UiIntentProductConsequences {
        worth_ui::facade::intent::UiIntentProductConsequences::none()
    }
}

impl UiIntentTransitionOutcome for EmptyOutcome {
    fn from_completed_transition(_destination: UiIntentTransitionDestination) -> Self {
        Self
    }
}

struct AlphaIntent;
struct BetaIntent;

macro_rules! intent {
    ($intent:ty, $identity:literal) => {
        impl UiIntent for $intent {
            type Payload = EmptyPayload;
            type ProductOutcome = EmptyOutcome;

            const ID: UiIntentId = UiIntentId::stable($identity);
            const ACCEPTED_INTERACTIONS: UiIntentAcceptedInteractions =
                UiIntentAcceptedInteractions::new(&[UiSemanticInteractionFamily::Activate]);
        }
    };
}

intent!(AlphaIntent, "phase4.registration.alpha");
intent!(BetaIntent, "phase4.registration.beta");

struct VersionedProvider<I: UiIntent, const VERSION: u16>(PhantomData<fn() -> I>);

impl<I: UiIntent, const VERSION: u16> VersionedProvider<I, VERSION> {
    const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<I: UiIntent, const VERSION: u16> UiIntentExecutionProvider<I>
    for VersionedProvider<I, VERSION>
{
    const VERSION: UiIntentProviderVersion = UiIntentProviderVersion::stable(VERSION);

    fn begin(&self, request: UiIntentExecutionRequest<I>) -> UiIntentProviderStart<I> {
        drop(request);
        UiIntentProviderStart::RejectedBeforeEffect(UiIntentProviderStop::stable(
            "phase4.registration.before_effect",
        ))
    }
}

struct DropTrackedProvider {
    drops: Arc<AtomicUsize>,
}

impl Drop for DropTrackedProvider {
    fn drop(&mut self) {
        self.drops.fetch_add(1, Ordering::SeqCst);
    }
}

impl UiIntentExecutionProvider<AlphaIntent> for DropTrackedProvider {
    const VERSION: UiIntentProviderVersion = UiIntentProviderVersion::stable(5);

    fn begin(
        &self,
        request: UiIntentExecutionRequest<AlphaIntent>,
    ) -> UiIntentProviderStart<AlphaIntent> {
        drop(request);
        UiIntentProviderStart::RejectedBeforeEffect(UiIntentProviderStop::stable(
            "phase4.registration.before_effect",
        ))
    }
}

#[test]
fn provider_version_changes_application_generation_identity() {
    let version_one = single_application::<AlphaIntent, 1>();
    let version_two = single_application::<AlphaIntent, 2>();
    assert_ne!(
        version_one.generation_identity(),
        version_two.generation_identity()
    );
}

#[test]
fn registration_order_canonicalizes_definition_and_provider_slots() {
    let alpha_then_beta = two_applications(false);
    let beta_then_alpha = two_applications(true);
    assert_eq!(
        alpha_then_beta.generation_identity(),
        beta_then_alpha.generation_identity()
    );
    assert_registration_metrics(&alpha_then_beta, 2);
    assert_registration_metrics(&beta_then_alpha, 2);
}

#[test]
fn destination_specific_registration_freezes_one_binding_per_definition() {
    let transition = base_builder()
        .register_intent_transition_definition(UiIntentDefinition::<AlphaIntent>::ui_transition(
            UiIntentTransitionDestination::NavigatePage,
        ))
        .unwrap()
        .freeze()
        .unwrap();
    let unsupported = base_builder()
        .register_unsupported_intent_definition(UiIntentDefinition::<AlphaIntent>::runtime_service(
            UiIntentRuntimeServiceDestination::InvokeCommand,
        ))
        .unwrap()
        .freeze()
        .unwrap();
    assert_registration_metrics(&transition, 1);
    assert_registration_metrics(&unsupported, 1);
    assert_ne!(
        transition.generation_identity(),
        unsupported.generation_identity()
    );
}

#[test]
fn successful_and_rejected_preparation_retire_each_provider_exactly_once() {
    let prepared_drops = Arc::new(AtomicUsize::new(0));
    let application = application_with_tracked_provider(Arc::clone(&prepared_drops))
        .freeze()
        .unwrap();
    assert_eq!(prepared_drops.load(Ordering::SeqCst), 0);
    drop(application);
    assert_eq!(prepared_drops.load(Ordering::SeqCst), 1);

    let rejected_drops = Arc::new(AtomicUsize::new(0));
    let invalid_input = WorthUiRustAuthoredArtifactInput::from_modules([
        WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
            .with_component("phase4.registration.unregistered"),
    ]);
    let denial = match application_with_tracked_provider(Arc::clone(&rejected_drops))
        .with_rust_authored_input(invalid_input)
        .freeze()
    {
        Ok(_) => panic!("an unregistered component must stop candidate preparation"),
        Err(denial) => denial,
    };
    assert!(matches!(
        denial,
        WorthUiApplicationPreparationDenial::RuntimePreparation(ref cause)
            if cause.stop() == WorthUiSemanticHandoffPreparationStop::CapabilityResolution
    ));
    assert_eq!(rejected_drops.load(Ordering::SeqCst), 1);
}

fn single_application<I: UiIntent, const VERSION: u16>() -> WorthUiApp {
    base_builder()
        .register_intent_definition(UiIntentDefinition::<I>::application_effect())
        .unwrap()
        .register_intent_provider(VersionedProvider::<I, VERSION>::new())
        .unwrap()
        .freeze()
        .unwrap()
}

fn application_with_tracked_provider(
    drops: Arc<AtomicUsize>,
) -> worth_ui::facade::app::WorthUiApplicationBuilder {
    base_builder()
        .register_intent_definition(UiIntentDefinition::<AlphaIntent>::application_effect())
        .unwrap()
        .register_intent_provider(DropTrackedProvider { drops })
        .unwrap()
}

fn two_applications(reverse: bool) -> WorthUiApp {
    let builder = base_builder();
    let builder = if reverse {
        register_beta(register_alpha(builder))
    } else {
        register_alpha(register_beta(builder))
    };
    builder.freeze().unwrap()
}

fn register_alpha(
    builder: worth_ui::facade::app::WorthUiApplicationBuilder,
) -> worth_ui::facade::app::WorthUiApplicationBuilder {
    builder
        .register_intent_definition(UiIntentDefinition::<AlphaIntent>::application_effect())
        .unwrap()
        .register_intent_provider(VersionedProvider::<AlphaIntent, 3>::new())
        .unwrap()
}

fn register_beta(
    builder: worth_ui::facade::app::WorthUiApplicationBuilder,
) -> worth_ui::facade::app::WorthUiApplicationBuilder {
    builder
        .register_intent_definition(UiIntentDefinition::<BetaIntent>::application_effect())
        .unwrap()
        .register_intent_provider(VersionedProvider::<BetaIntent, 4>::new())
        .unwrap()
}

fn base_builder() -> worth_ui::facade::app::WorthUiApplicationBuilder {
    WorthUi::app().with_change_profile(UiChangeProfile::platform_pulse())
}

fn assert_registration_metrics(application: &WorthUiApp, expected: usize) {
    let metrics = application.intent_execution_binding_registration_metrics_for_certification();
    assert_eq!(metrics.definitions(), expected);
    assert_eq!(metrics.bindings(), expected);
}
