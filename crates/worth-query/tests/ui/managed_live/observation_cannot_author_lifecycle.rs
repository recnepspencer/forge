use worth_query::facade::live::WorthQueryManagedLiveLifecycleObservation;

fn author_lifecycle(observation: &mut WorthQueryManagedLiveLifecycleObservation) {
    observation.advance_maintenance();
}

fn main() {}
