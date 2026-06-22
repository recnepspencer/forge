use worth_spatial::facade::planar_boolean_loop_reconstruction::{
    PlanarBooleanLoopIslandPartition, PlanarBooleanLoopIslandPartitionCounters,
    PlanarBooleanLoopIslandPartitionRow,
};

fn bogus<T>() -> T {
    panic!("compile-fail fixture")
}

fn main() {
    let _ = PlanarBooleanLoopIslandPartition {
        partition_identity: String::new(),
        request_identity: String::new(),
        rows: vec![bogus::<PlanarBooleanLoopIslandPartitionRow>()],
        counters: PlanarBooleanLoopIslandPartitionCounters::default(),
    };
}
