use worth_spatial::facade::planar_clean_fail_boundary::{
    PlanarCleanFailBoundaryCounters, PlanarCleanFailBoundaryReceipt,
};

fn main() {
    let _receipt = PlanarCleanFailBoundaryReceipt::new(
        panic!("basis constructor is not public"),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        PlanarCleanFailBoundaryCounters::certified(0, 0, 0, 0),
    );
}
