use worth_query::facade::read::WorthQueryManagedLiveLifecycleObservation;

fn author_lifecycle(observation: &mut WorthQueryManagedLiveLifecycleObservation) {
    observation.advance_maintenance();
}

fn main() {}
