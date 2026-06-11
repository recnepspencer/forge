use worth_kernel::workload_composition::{OperatorWorkload, WorkloadStageRequirement};

struct RawSpatialFixture {
    points: Vec<[f64; 2]>,
}

fn main() {
    let raw_fixture = RawSpatialFixture {
        points: vec![[0.0, 0.0], [1.0, 0.0]],
    };

    let _ = OperatorWorkload::requiring(WorkloadStageRequirement::Projection)
        .accept(&raw_fixture)
        .unwrap();
}
