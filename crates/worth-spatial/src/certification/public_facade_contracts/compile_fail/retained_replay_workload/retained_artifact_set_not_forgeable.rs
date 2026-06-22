use worth_spatial::facade::retained_replay_workload::RetainedArtifactSet;

fn main() {
    let _ = RetainedArtifactSet {
        retained_planar_facts: unconstructible(),
        projection_consumed_facts: None,
    };
}

fn unconstructible<T>() -> T {
    panic!("compile-fail input is never executed")
}
