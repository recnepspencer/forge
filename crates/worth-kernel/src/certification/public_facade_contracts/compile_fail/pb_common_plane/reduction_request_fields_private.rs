use worth_kernel::workload_composition::{
    PlanarBooleanCommonPlaneReductionRequest,
};

fn main() {
    let pair = unreachable_value();
    let construction_receipt = unreachable_value();
    let _ = PlanarBooleanCommonPlaneReductionRequest {
        pair,
        request_identity: String::new(),
        construction_receipt,
    };
}

fn unreachable_value<T>() -> T {
    panic!("compile-fail fixture should never execute")
}
