use worth_spatial::facade::boolean_readiness_workload::{
    PlanarBooleanReadinessWorkloadCounters, PlanarBooleanReadinessWorkloadReceipt,
};

fn main() {
    let _receipt = PlanarBooleanReadinessWorkloadReceipt::new(
        panic!("cannot construct M7 readiness receipt"),
        "digest".to_string(),
        "declaration".to_string(),
        PlanarBooleanReadinessWorkloadCounters::certified(0, 0, 0, 0, 0),
    );
}
