use worth_ui::facade::observation::{UiAdmittedObservationSet, UiObservationFamily};
use worth_ui_query_binding::UiProjectionObservation;

use super::{
    application,
    interaction_evidence::{
        prepare_competing_interaction, start_effecting_intent, validated_viewport,
        CompetingInteraction,
    },
    scenario::ReadyOrderingScenario,
};
use crate::intent::ordering::model::{Cause, CANONICAL_OBSERVATIONS};

pub(super) struct OrderedCauseEvidence {
    mixed: UiAdmittedObservationSet,
    families: [UiObservationFamily; 3],
    admitted_order: [Cause; 4],
    interaction_position: usize,
    target_preserved: bool,
}

struct ProducedCauses {
    competing: CompetingInteraction,
    source: worth_ui::facade::source::WorthUiWatchedCandidateSubmission,
    query: worth_ui_query_binding::UiCollectionProjectionObservation,
    viewport: worth_ui::facade::observation_report::UiValidatedHostObservationBatch,
    interaction_position: usize,
    target_preserved: bool,
}

impl OrderedCauseEvidence {
    pub(super) const fn families(&self) -> [UiObservationFamily; 3] {
        self.families
    }

    pub(super) const fn interaction_position(&self) -> usize {
        self.interaction_position
    }

    pub(super) const fn admitted_order(&self) -> [Cause; 4] {
        self.admitted_order
    }

    pub(super) const fn target_preserved(&self) -> bool {
        self.target_preserved
    }

    pub(super) fn into_observations(self) -> UiAdmittedObservationSet {
        self.mixed
    }
}

pub(super) fn produce_and_admit(
    scenario: &mut ReadyOrderingScenario,
    order: [Cause; 4],
    run: usize,
) -> OrderedCauseEvidence {
    let produced = produce_causes(scenario, order, run);
    admit_causes(scenario, order, produced)
}

fn produce_causes(
    scenario: &mut ReadyOrderingScenario,
    order: [Cause; 4],
    run: usize,
) -> ProducedCauses {
    let effect_target = start_effecting_intent(&mut scenario.interaction);
    let interaction_position = order
        .iter()
        .position(|cause| *cause == Cause::Interaction)
        .expect("every model order contains the interaction cause");
    let mut competing = None;
    let mut query = None;
    let mut source = None;

    for cause in order {
        match cause {
            Cause::Interaction => {
                competing = Some(prepare_competing_interaction(&mut scenario.interaction));
            }
            Cause::Source => {
                source = Some(application::successor_candidate(
                    &scenario.interaction.session,
                    &scenario.facts,
                    run,
                ));
            }
            Cause::Query => {
                worth_ui_query_binding::certification::update_projection_status(
                    &mut scenario.workspace,
                    scenario.entities[1].clone(),
                    "Bravo IA-09",
                );
                query = Some(refresh_query(scenario));
            }
            Cause::Viewport => {}
        }
    }

    let competing = competing.expect("every order produces the native interaction cause");
    assert_eq!(competing.target, effect_target);
    let viewport = issue_viewport_after_native_sequence(scenario, effect_target);
    ProducedCauses {
        target_preserved: competing.target == effect_target,
        competing,
        source: source.expect("every order produces the authored source cause"),
        query: query.expect("every order produces the Query cause"),
        viewport,
        interaction_position,
    }
}

fn admit_causes(
    scenario: &mut ReadyOrderingScenario,
    order: [Cause; 4],
    produced: ProducedCauses,
) -> OrderedCauseEvidence {
    let ProducedCauses {
        competing,
        source,
        query,
        viewport,
        interaction_position,
        target_preserved,
    } = produced;
    let mut source = Some(source);
    let mut query = Some(query);
    let mut viewport = Some(viewport);
    let mut admitted_order = Vec::with_capacity(4);
    let mut turn = scenario
        .interaction
        .session
        .begin_observation_turn()
        .unwrap();
    for cause in order {
        match cause {
            Cause::Interaction => assert!(competing.occupied),
            Cause::Source => turn
                .admit_source(source.take().unwrap())
                .map(|_| ())
                .unwrap(),
            Cause::Query => turn
                .admit_projection_query(UiProjectionObservation::Collection(query.take().unwrap()))
                .map(|_| ())
                .unwrap(),
            Cause::Viewport => turn
                .admit_host(viewport.take().unwrap())
                .map(|_| ())
                .unwrap(),
        }
        admitted_order.push(cause);
    }
    let mixed = turn.seal().unwrap();
    let families = <[UiObservationFamily; 3]>::try_from(mixed.summary().families())
        .expect("the mixed turn has exactly three observation families");
    assert_eq!(families, CANONICAL_OBSERVATIONS);
    OrderedCauseEvidence {
        mixed,
        families,
        admitted_order: <[Cause; 4]>::try_from(admitted_order)
            .expect("the model admits every cause exactly once"),
        interaction_position,
        target_preserved,
    }
}

fn issue_viewport_after_native_sequence(
    scenario: &mut ReadyOrderingScenario,
    target: worth_ui_runtime::facade::mounted::UiMountedInstanceIdentity,
) -> worth_ui::facade::observation_report::UiValidatedHostObservationBatch {
    validated_viewport(&mut scenario.interaction, target)
}

fn refresh_query(
    scenario: &mut ReadyOrderingScenario,
) -> worth_ui_query_binding::UiCollectionProjectionObservation {
    match scenario.live.refresh(&mut scenario.workspace).unwrap() {
        worth_ui_query_binding::UiCollectionProjectionRefreshOutcome::Applied(receipt) => {
            receipt.into_fact().into_observation()
        }
        worth_ui_query_binding::UiCollectionProjectionRefreshOutcome::NoSemanticDelivery => {
            panic!("the IA-09 Query change must produce one owner observation")
        }
    }
}
