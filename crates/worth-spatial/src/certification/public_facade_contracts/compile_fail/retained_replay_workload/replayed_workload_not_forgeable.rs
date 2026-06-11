use worth_spatial::facade::retained_replay_workload::ReplayedWorkload;

fn main() {
    let _ = ReplayedWorkload::new(unconstructible(), unconstructible(), unconstructible());
}

fn unconstructible<T>() -> T {
    panic!("compile-fail input is never executed")
}
