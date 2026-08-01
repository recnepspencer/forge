use worth_ui::facade::{
    intent::UiIntentExecutionTransitionPosture,
    observation::UiChangeClassificationOutcome,
    rebind::{
        UiAffectedScopeCost, UiProducedFactFamily, UiRebindExecutionRequest, UiRebindOutcome,
    },
};
use worth_ui_runtime::certification_support::WorthUiIntentExecutionReservationCertificationExt;

use super::{
    cause_order::OrderedCauseEvidence,
    interaction_evidence::{advance, only_transition},
    scenario::{ReadyOrderingScenario, SettledOrderingScenario},
};

pub(super) struct PublicationProof {
    cost: UiAffectedScopeCost,
    count: usize,
    queued_observations: usize,
}

impl PublicationProof {
    pub(super) const fn cost(&self) -> UiAffectedScopeCost {
        self.cost
    }

    pub(super) const fn count(&self) -> usize {
        self.count
    }

    pub(super) const fn queued_observations(&self) -> usize {
        self.queued_observations
    }
}

pub(super) fn publish_successor(
    scenario: ReadyOrderingScenario,
    causes: OrderedCauseEvidence,
    run: usize,
) -> (SettledOrderingScenario, PublicationProof) {
    let ReadyOrderingScenario {
        mut interaction,
        workspace,
        live,
        provider_observation,
        predecessor_plan,
        ..
    } = scenario;
    let prepared = interaction
        .session
        .prepare_rebind(
            predecessor_plan,
            UiRebindExecutionRequest::new(40_000 + run as u64),
        )
        .expect("the predecessor Query snapshot prepares while the intent is effecting");
    let mut effecting = prepared
        .begin_effecting()
        .unwrap_or_else(|_| panic!("the production rebind owns the sole effecting queue"));
    let queued = effecting
        .admit_observations(causes.into_observations())
        .unwrap_or_else(|_| panic!("the three owner observations fit the production queue"));
    assert_eq!(queued.admitted_observations(), 3);
    assert_eq!(queued.total_queued_observations(), 3);
    let queued_observations = queued.total_queued_observations();
    let (predecessor_outcome, queued_sets) = effecting.complete(40_000 + run as u64).into_parts();
    let predecessor_frame = match predecessor_outcome {
        UiRebindOutcome::Published(ref receipt) => receipt
            .mounted_publication()
            .expect("the predecessor Query snapshot publishes a mounted frame")
            .frame(),
        _ => panic!("the predecessor Query snapshot publishes before the queued successor"),
    };
    drop(predecessor_outcome);

    let (cost, successor_publications) = publish_mixed_successor(
        &mut interaction,
        queued_sets.into_vec(),
        predecessor_frame,
        run,
    );
    prove_intent_settlement(&mut interaction, run);
    (
        SettledOrderingScenario {
            interaction,
            workspace,
            live,
            provider_observation,
        },
        PublicationProof {
            cost,
            count: successor_publications,
            queued_observations,
        },
    )
}

fn publish_mixed_successor(
    interaction: &mut super::interaction_evidence::OrderingInteractionWorld,
    mut queued_sets: Vec<worth_ui::facade::observation::UiAdmittedObservationSet>,
    predecessor_frame: worth_ui_host_contract::UiMountedFrameIdentity,
    run: usize,
) -> (UiAffectedScopeCost, usize) {
    assert_eq!(queued_sets.len(), 1);
    let changed = match interaction
        .session
        .classify_observations(queued_sets.pop().unwrap())
        .unwrap()
    {
        UiChangeClassificationOutcome::Changed(changed) => changed,
        _ => panic!("the queued source, viewport, and Query evidence changes meaning"),
    };
    assert_eq!(
        changed
            .facts()
            .iter()
            .map(|fact| fact.family())
            .collect::<Vec<_>>(),
        [
            UiProducedFactFamily::AuthoredSource,
            UiProducedFactFamily::HostViewport,
            UiProducedFactFamily::Query,
        ]
    );
    let scope = interaction.session.resolve_affected_scope(changed).unwrap();
    let cost = scope.cost();
    assert_eq!(cost.observations(), 3);
    assert_eq!(cost.changed_facts(), 3);
    assert_eq!(cost.lookup_receipts(), 6);
    assert_eq!(cost.index_probes(), 6);
    let lifecycle = scope.resolve_identity_lifecycle().unwrap();
    let plan = interaction
        .session
        .compile_rebind_plan(
            lifecycle,
            worth_ui::facade::rebind::UiRebindExecutionPolicy::ordinary(),
        )
        .unwrap();
    let prepared = interaction
        .session
        .prepare_rebind(plan, UiRebindExecutionRequest::new(41_000 + run as u64))
        .expect("the queued mixed successor prepares once");
    let successor = match prepared.execute(41_000 + run as u64) {
        UiRebindOutcome::Published(receipt) => receipt,
        _ => panic!("the queued mixed successor publishes atomically"),
    };
    assert!(successor.application_publication().is_some());
    let successor_frame = successor
        .mounted_publication()
        .expect("the mixed successor publishes one mounted frame")
        .frame();
    assert_ne!(successor_frame, predecessor_frame);
    (cost, 1)
}

fn prove_intent_settlement(
    interaction: &mut super::interaction_evidence::OrderingInteractionWorld,
    run: usize,
) {
    let terminal = only_transition(advance(interaction, 50_000 + run as u64));
    assert!(matches!(
        terminal.posture(),
        UiIntentExecutionTransitionPosture::Completed { .. }
    ));
    let metrics = interaction
        .session
        .intent_execution_reservation_metrics_for_certification();
    assert_eq!(metrics.active_attempts(), 0);
    assert_eq!(metrics.active_occupancy(), 0);
    assert_eq!(metrics.recovering_attempts(), 0);
    assert_eq!(metrics.consequence_pending_attempts(), 1);
}
