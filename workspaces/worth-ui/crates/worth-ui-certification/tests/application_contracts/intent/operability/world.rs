use worth_ui::facade::intent::{
    UiIntentOperabilityOutcome, UiIntentRouteResolution, UiIntentRouteSource,
    UiPreparedIntentPayload,
};
use worth_ui::facade::interaction::{
    UiHostInteractionIngressOutcome, UiInteractionTransition, UiSemanticInteraction,
};
use worth_ui::facade::observation_report::UiHostPointerButtonTransition;
use worth_ui_runtime::facade::mounted::UiHostSurfacePresentationMode;

use super::super::super::filesystem_mounted_world::{
    component_graph_nodes, launch_mounted_components,
};
use super::super::interaction_world::InteractionWorld;
use super::topology::build_scoped;
pub(super) use super::topology::OccupancyLayout;
use super::OperabilityFacts;

pub(super) const PRIMARY_POINT: [i64; 2] = [10, 20];
pub(super) const PEER_POINT: [i64; 2] = [70, 20];

pub(super) struct OperabilityWorld {
    pub(super) interaction: InteractionWorld,
    facts: OperabilityFacts,
    next_pointer: u64,
}

impl OperabilityWorld {
    pub(super) fn scoped(layout: OccupancyLayout) -> Self {
        let (app, facts) = build_scoped(layout);
        Self::launch(app, facts)
    }

    fn launch(app: worth_ui::facade::app::WorthUiApp, facts: OperabilityFacts) -> Self {
        let nodes = component_graph_nodes(&app);
        let session =
            launch_mounted_components(app, nodes, UiHostSurfacePresentationMode::RecordOnly);
        Self {
            interaction: InteractionWorld::from_session(session),
            facts,
            next_pointer: 1,
        }
    }

    pub(super) fn set_axes(
        &mut self,
        writable: bool,
        ready: bool,
        policy: bool,
        confirmation: bool,
    ) {
        for (fact, value) in [
            (&self.facts.mutability, writable),
            (&self.facts.readiness, ready),
            (&self.facts.policy, policy),
            (&self.facts.confirmation, confirmation),
        ] {
            self.interaction
                .session
                .update_intent_boolean_fact(fact, value)
                .expect("operability fact update remains within its registered owner");
        }
    }

    pub(super) fn prepare(&mut self, point: [i64; 2]) -> UiPreparedIntentPayload {
        let pointer = self.next_pointer;
        self.next_pointer += 1;
        let interaction = activation(&mut self.interaction, pointer, point);
        self.prepare_interaction(interaction)
    }

    pub(super) fn prepare_interaction(
        &mut self,
        interaction: UiSemanticInteraction,
    ) -> UiPreparedIntentPayload {
        let route = product_route(&mut self.interaction, interaction);
        self.interaction
            .session
            .prepare_intent_payload(route)
            .expect("empty payload and operability share one current basis")
    }

    pub(super) fn evaluate(&mut self, point: [i64; 2]) -> UiIntentOperabilityOutcome {
        let candidate = self.prepare(point);
        self.interaction
            .session
            .evaluate_intent_operability(candidate)
    }
}

fn activation(
    world: &mut InteractionWorld,
    pointer: u64,
    point: [i64; 2],
) -> UiSemanticInteraction {
    let _ = world.button(pointer, 1, UiHostPointerButtonTransition::Pressed, point);
    let released = world.button(pointer, 1, UiHostPointerButtonTransition::Released, point);
    let UiHostInteractionIngressOutcome::Applied(receipt) = released else {
        panic!("release must reach the semantic interaction owner: {released:?}")
    };
    receipt
        .into_transitions()
        .into_vec()
        .into_iter()
        .find_map(|transition| match transition {
            UiInteractionTransition::Semantic(interaction) => Some(interaction),
            _ => None,
        })
        .expect("one complete activation mints one semantic interaction")
}

fn product_route(
    world: &mut InteractionWorld,
    interaction: UiSemanticInteraction,
) -> worth_ui::facade::intent::UiResolvedProductIntentRoute {
    match world
        .session
        .resolve_intent_route(UiIntentRouteSource::mounted_interaction(interaction))
        .expect("mounted activation resolves its exact route")
    {
        UiIntentRouteResolution::Product(route) => route,
        UiIntentRouteResolution::Confirmation(_) => {
            panic!("product activation cannot cross into confirmation routing")
        }
    }
}
