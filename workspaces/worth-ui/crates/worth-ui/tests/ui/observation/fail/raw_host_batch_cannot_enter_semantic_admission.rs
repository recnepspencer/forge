use worth_ui::facade::observation::UiObservationTurn;
use worth_ui::facade::observation_report::UiHostObservationBatch;

fn raw_host_batch_cannot_enter(
    turn: &mut UiObservationTurn<'_>,
    raw: UiHostObservationBatch,
) {
    let _ = turn.admit_host(raw);
}

fn main() {}
