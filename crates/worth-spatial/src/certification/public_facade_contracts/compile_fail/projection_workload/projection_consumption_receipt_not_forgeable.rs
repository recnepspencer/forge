use worth_spatial::facade::projection_workload::ProjectionConsumedWorkloadReceipt;

fn main() {
    let _ = ProjectionConsumedWorkloadReceipt::new(unconstructible(), "local-basis", 3);
}

fn unconstructible<T>() -> T {
    panic!("compile-fail input is never executed")
}
