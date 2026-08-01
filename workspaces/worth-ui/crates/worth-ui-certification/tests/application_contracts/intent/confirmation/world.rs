use worth_ui::facade::intent::{
    UiAdmittedIntent, UiIntentAdmissionDecision, UiIntentApplicationFact, UiIntentBoolean,
    UiIntentConfirmationIssueOutcome, UiIntentDefinition, UiIntentOperabilityOutcome,
    UiIntentRouteResolution, UiIntentRouteSource, UiIntentUnsigned64, UiPendingIntentConfirmation,
    UiResolvedConfirmationIntentRoute,
};
use worth_ui::facade::interaction::{
    UiHostInteractionIngressOutcome, UiInteractionTransition, UiSemanticInteraction,
};
use worth_ui::facade::observation::UiChangeClassificationOutcome;
use worth_ui::facade::observation_report::{
    UiHostObservationTimeBasis, UiHostPointerButtonTransition,
};
use worth_ui::facade::rebind::{
    UiRebindExecutionPolicy, UiRebindExecutionRequest, UiRebindOutcome,
};
use worth_ui::facade::source::{
    WorthUiSourceIngressExt, WorthUiSourceProvider, WorthUiWatcherEvent,
};
use worth_ui_runtime::facade::mounted::UiHostSurfacePresentationMode;

use super::super::super::filesystem_mounted_world::{
    component_graph_nodes, launch_mounted_components,
};
use super::super::interaction_world::InteractionWorld;

mod application;

pub(super) const PRODUCT_POINT: [i64; 2] = [10, 20];
pub(super) const CONFIRMATION_POINT: [i64; 2] = [70, 20];

const DECLARATION: &str = "phase3.confirmation.declaration";
const MUTABILITY: &str = "phase3.confirmation.writable";
const READINESS: &str = "phase3.confirmation.ready";
const POLICY: &str = "phase3.confirmation.policy";
const CONFIRMATION: &str = "phase3.confirmation.required";
const REVISION: &str = "phase3.confirmation.revision";
const OPERABILITY: &str = "phase3.confirmation.operability";
const CONFIRMATION_POLICY: &str = "phase3.confirmation.policy.confirm";
const HIT_ONLY: &str = "visual.identity.component.hit_only";
const PAINT_AND_HIT: &str = "visual.identity.component.paint_and_hit";
const PAINT_ONLY: &str = "visual.identity.component.paint_only";
const NEITHER: &str = "visual.identity.component.neither";
const SURFACE: &str = "visual.identity.surface.main";
const PAINT_ONLY_TOKEN: &str = "theme.visual_identity.paint_only";
const PAINT_AND_HIT_TOKEN: &str = "theme.visual_identity.paint_and_hit";

pub(in crate::intent) struct ConfirmationWorld {
    pub(in crate::intent) interaction: InteractionWorld,
    facts: ConfirmationFacts,
    next_pointer: u64,
}

struct ConfirmationFacts {
    mutability: UiIntentApplicationFact<UiIntentBoolean>,
    readiness: UiIntentApplicationFact<UiIntentBoolean>,
    policy: UiIntentApplicationFact<UiIntentBoolean>,
    confirmation: UiIntentApplicationFact<UiIntentBoolean>,
    revision: UiIntentApplicationFact<UiIntentUnsigned64>,
}

pub(in crate::intent) struct IssuedChallenge {
    pub(super) pending: UiPendingIntentConfirmation,
    pub(super) product_instance: worth_ui_runtime::facade::mounted::UiMountedInstanceIdentity,
}

impl ConfirmationWorld {
    pub(in crate::intent) fn launch() -> Self {
        let facts = ConfirmationFacts::new();
        let app = application::build(
            &facts,
            worth_ui_certification::WorthUiCertificationBeforeEffectProvider::<
                super::types::ConfirmationIntent,
            >::new(),
        );
        Self::launch_application(app, facts)
    }

    pub(in crate::intent) fn launch_with_provider_observation() -> (
        Self,
        worth_ui_certification::WorthUiCertificationProviderObservation,
    ) {
        let facts = ConfirmationFacts::new();
        let (provider, observation) =
            worth_ui_certification::WorthUiCertificationBeforeEffectProvider::<
                super::types::ConfirmationIntent,
            >::with_observation();
        let app = application::build(&facts, provider);
        (Self::launch_application(app, facts), observation)
    }

    fn launch_application(
        app: worth_ui::facade::app::WorthUiApp,
        facts: ConfirmationFacts,
    ) -> Self {
        let nodes = component_graph_nodes(&app);
        let session =
            launch_mounted_components(app, nodes, UiHostSurfacePresentationMode::RecordOnly);
        Self {
            interaction: InteractionWorld::from_session(session),
            facts,
            next_pointer: 1,
        }
    }

    pub(in crate::intent) fn issue(&mut self) -> IssuedChallenge {
        let interaction = self.activation(PRODUCT_POINT);
        let product_instance = interaction.target().mounted_instance();
        let route = match self
            .interaction
            .session
            .resolve_intent_route(UiIntentRouteSource::mounted_interaction(interaction))
            .expect("product activation resolves")
        {
            UiIntentRouteResolution::Product(route) => route,
            UiIntentRouteResolution::Confirmation(_) => {
                panic!("product activation cannot resolve as confirmation")
            }
        };
        let candidate = self
            .interaction
            .session
            .prepare_intent_payload(route)
            .expect("confirmation payload is coherent");
        let UiIntentOperabilityOutcome::Inoperable(candidate) = self
            .interaction
            .session
            .evaluate_intent_operability(candidate)
        else {
            panic!("required confirmation is the only inoperable cause")
        };
        let UiIntentConfirmationIssueOutcome::Pending(pending) = self
            .interaction
            .session
            .issue_intent_confirmation(candidate)
        else {
            panic!("exclusively confirmable candidate must occupy one slot")
        };
        IssuedChallenge {
            pending,
            product_instance,
        }
    }

    pub(super) fn operable_proof(&mut self) -> worth_ui::facade::intent::UiIntentOperabilityProof {
        let proof = self.prepare_operable_proof();
        self.set_confirmation(true);
        proof
    }

    pub(in crate::intent) fn admit_operable(
        &mut self,
    ) -> UiAdmittedIntent<super::types::ConfirmationIntent> {
        let proof = self.prepare_operable_proof();
        match self.interaction.session.admit_intent(
            UiIntentDefinition::<super::types::ConfirmationIntent>::application_effect(),
            UiIntentOperabilityOutcome::Operable(proof),
        ) {
            UiIntentAdmissionDecision::Admitted(admitted) => admitted,
            UiIntentAdmissionDecision::ConfirmationRequired(_) => {
                panic!("confirmation-disabled candidate cannot require confirmation")
            }
            UiIntentAdmissionDecision::Stopped(stop) => {
                panic!("current operable candidate must admit: {:?}", stop.reason())
            }
        }
    }

    fn prepare_operable_proof(&mut self) -> worth_ui::facade::intent::UiIntentOperabilityProof {
        self.set_confirmation(false);
        let interaction = self.activation(PRODUCT_POINT);
        let route = match self
            .interaction
            .session
            .resolve_intent_route(UiIntentRouteSource::mounted_interaction(interaction))
            .expect("operable product activation resolves")
        {
            UiIntentRouteResolution::Product(route) => route,
            UiIntentRouteResolution::Confirmation(_) => unreachable!(),
        };
        let candidate = self
            .interaction
            .session
            .prepare_intent_payload(route)
            .expect("operable payload prepares");
        let UiIntentOperabilityOutcome::Operable(proof) = self
            .interaction
            .session
            .evaluate_intent_operability(candidate)
        else {
            panic!("confirmation-disabled candidate is operable")
        };
        proof
    }

    pub(super) fn confirmation_route(
        &mut self,
        press_time: UiHostObservationTimeBasis,
        release_time: UiHostObservationTimeBasis,
    ) -> UiResolvedConfirmationIntentRoute {
        let interaction = self.activation_with_time(CONFIRMATION_POINT, press_time, release_time);
        match self
            .interaction
            .session
            .resolve_intent_route(UiIntentRouteSource::mounted_interaction(interaction))
            .expect("confirmation activation resolves")
        {
            UiIntentRouteResolution::Confirmation(route) => route,
            UiIntentRouteResolution::Product(_) => {
                panic!("confirmation activation cannot resolve as product")
            }
        }
    }

    pub(super) fn route_at_confirmation_control(&mut self) -> UiIntentRouteResolution {
        let interaction = self.activation(CONFIRMATION_POINT);
        self.interaction
            .session
            .resolve_intent_route(UiIntentRouteSource::mounted_interaction(interaction))
            .expect("replacement confirmation control resolves through the active catalog")
    }

    pub(in crate::intent) fn publish_successor(&mut self) {
        self.interaction.publish_successor();
    }

    pub(in crate::intent) fn rebind_application(&mut self) {
        const PROVIDER: &str = "phase-3-confirmation-replacement";
        let provider = WorthUiSourceProvider::rust_authored(PROVIDER)
            .with_rust_authored_input(application::replacement_input(&self.facts));
        let mut ingress = self
            .interaction
            .session
            .source_event_ingress(provider)
            .start();
        let settled = ingress
            .ingest([WorthUiWatcherEvent::provider_revision(PROVIDER)])
            .expect("replacement Rust source settles through production ingress");
        let submission = settled
            .attempt_candidate_for_certification(self.interaction.session.capabilities())
            .expect("replacement Rust source lowers through the production compiler");
        let mut turn = self.interaction.session.begin_observation_turn().unwrap();
        turn.admit_source(submission).unwrap();
        let admitted = turn.seal().unwrap();
        let changed = match self
            .interaction
            .session
            .classify_observations(admitted)
            .unwrap()
        {
            UiChangeClassificationOutcome::Changed(changed) => changed,
            UiChangeClassificationOutcome::ObservedNoChange(_) => {
                panic!("authored intent route change cannot classify as no-change")
            }
            UiChangeClassificationOutcome::EvidenceOnly(_) => {
                panic!("authored intent route change is executable meaning, not evidence-only")
            }
        };
        let lifecycle = self
            .interaction
            .session
            .resolve_affected_scope(changed)
            .unwrap()
            .resolve_identity_lifecycle()
            .unwrap();
        let plan = self
            .interaction
            .session
            .compile_rebind_plan(lifecycle, UiRebindExecutionPolicy::ordinary())
            .unwrap();
        let prepared = self
            .interaction
            .session
            .prepare_rebind(plan, UiRebindExecutionRequest::new(30))
            .expect("application replacement prepares through the native rebind owner");
        assert!(matches!(
            prepared.execute(30),
            UiRebindOutcome::Published(_)
        ));
    }

    pub(in crate::intent) fn set_revision(&mut self, value: u64) {
        self.interaction
            .session
            .update_intent_unsigned64_fact(&self.facts.revision, value)
            .expect("payload revision update is owner-issued");
    }

    pub(super) fn set_policy(&mut self, value: bool) {
        self.update_boolean(&self.facts.policy.clone(), value);
    }

    pub(super) fn set_confirmation(&mut self, value: bool) {
        self.update_boolean(&self.facts.confirmation.clone(), value);
    }

    pub(super) fn set_mutability(&mut self, value: bool) {
        self.update_boolean(&self.facts.mutability.clone(), value);
    }

    fn update_boolean(&mut self, fact: &UiIntentApplicationFact<UiIntentBoolean>, value: bool) {
        self.interaction
            .session
            .update_intent_boolean_fact(fact, value)
            .expect("operability update is owner-issued");
    }

    fn activation(&mut self, point: [i64; 2]) -> UiSemanticInteraction {
        let pointer = self.take_pointer();
        let _ = self
            .interaction
            .button(pointer, 1, UiHostPointerButtonTransition::Pressed, point);
        semantic(self.interaction.button(
            pointer,
            1,
            UiHostPointerButtonTransition::Released,
            point,
        ))
    }

    fn activation_with_time(
        &mut self,
        point: [i64; 2],
        press_time: UiHostObservationTimeBasis,
        release_time: UiHostObservationTimeBasis,
    ) -> UiSemanticInteraction {
        let pointer = self.take_pointer();
        let _ = self.interaction.button_with_time_basis(
            pointer,
            1,
            UiHostPointerButtonTransition::Pressed,
            point,
            press_time,
        );
        semantic(self.interaction.button_with_time_basis(
            pointer,
            1,
            UiHostPointerButtonTransition::Released,
            point,
            release_time,
        ))
    }

    fn take_pointer(&mut self) -> u64 {
        let pointer = self.next_pointer;
        self.next_pointer += 1;
        pointer
    }
}

impl ConfirmationFacts {
    fn new() -> Self {
        Self {
            mutability: UiIntentApplicationFact::boolean(MUTABILITY).unwrap(),
            readiness: UiIntentApplicationFact::boolean(READINESS).unwrap(),
            policy: UiIntentApplicationFact::boolean(POLICY).unwrap(),
            confirmation: UiIntentApplicationFact::boolean(CONFIRMATION).unwrap(),
            revision: UiIntentApplicationFact::unsigned64(REVISION).unwrap(),
        }
    }
}

fn semantic(outcome: UiHostInteractionIngressOutcome) -> UiSemanticInteraction {
    let UiHostInteractionIngressOutcome::Applied(receipt) = outcome else {
        panic!("confirmation input reaches the interaction owner: {outcome:?}")
    };
    receipt
        .into_transitions()
        .into_vec()
        .into_iter()
        .find_map(|transition| match transition {
            UiInteractionTransition::Semantic(interaction) => Some(interaction),
            _ => None,
        })
        .expect("complete press/release seals one semantic activation")
}
