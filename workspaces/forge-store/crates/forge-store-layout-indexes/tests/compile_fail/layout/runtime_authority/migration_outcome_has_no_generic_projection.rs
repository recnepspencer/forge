use forge_store_layout_indexes::evolution::migration::MigrationPlanningOutcome;

fn misuse(outcome: MigrationPlanningOutcome) {
    let _ = outcome.into_transition_outcome();
}

fn main() {}
