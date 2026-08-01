use worth_ui::facade::{observation::UiObservationFamily, rebind::UiAffectedScopeCost};

use super::model::Cause;

mod application;
mod cause_order;
mod interaction_evidence;
mod publication;
mod scenario;

pub(super) struct OrderingVerdict {
    pub(super) families: [UiObservationFamily; 3],
    pub(super) cost: UiAffectedScopeCost,
    pub(super) admitted_order: [Cause; 4],
    pub(super) interaction_position: usize,
    pub(super) cause_publications: usize,
    pub(super) queued_observations: usize,
    pub(super) target_preserved: bool,
    pub(super) provider_counts: [usize; 7],
}

pub(super) fn run(order: [Cause; 4], run: usize) -> OrderingVerdict {
    let mut scenario = scenario::ReadyOrderingScenario::launch();
    let causes = cause_order::produce_and_admit(&mut scenario, order, run);
    let families = causes.families();
    let admitted_order = causes.admitted_order();
    let interaction_position = causes.interaction_position();
    let target_preserved = causes.target_preserved();
    let (settled, publication) = publication::publish_successor(scenario, causes, run);
    let provider_counts = settled.finish();

    OrderingVerdict {
        families,
        cost: publication.cost(),
        admitted_order,
        interaction_position,
        cause_publications: publication.count(),
        queued_observations: publication.queued_observations(),
        target_preserved,
        provider_counts,
    }
}
