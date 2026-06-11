use worth_spatial::facade::projection_fact_parity::{
    ProjectionFactParityCase, ProjectionFactParityCounters, ProjectionFactParityReceipt,
};

fn main() {
    let _ = ProjectionFactParityReceipt::new(
        ProjectionFactParityCase::AdmittedAcrossAllLanes,
        String::new(),
        String::new(),
        String::new(),
        Vec::new(),
        ProjectionFactParityCounters::new(0, 0, 0, 0),
    );
}
