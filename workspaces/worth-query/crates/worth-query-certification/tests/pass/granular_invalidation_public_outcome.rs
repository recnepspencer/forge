use worth_query::facade::domain::WorthQueryPrimaryGranularMaintenanceOutcome;

fn observe_query_owned_outcome(outcome: WorthQueryPrimaryGranularMaintenanceOutcome) {
    match outcome {
        WorthQueryPrimaryGranularMaintenanceOutcome::Performed(performed) => {
            let _effect = performed.deliveries()[0].effect();
            assert_eq!(performed.shared_execution_count(), 1);
        }
        WorthQueryPrimaryGranularMaintenanceOutcome::NoRelevantChange(no_change) => {
            assert_eq!(no_change.lower_truth_delivery_count(), 0);
        }
    }
}

fn main() {
    let _ = observe_query_owned_outcome;
}
