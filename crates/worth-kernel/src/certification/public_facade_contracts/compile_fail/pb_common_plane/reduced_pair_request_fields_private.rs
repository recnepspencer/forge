use worth_kernel::workload_composition::PlanarBooleanCommonPlaneReducedOperandPairRequest;

fn main() {
    let operand_a_projected_request = unreachable_value();
    let operand_b_projected_request = unreachable_value();
    let reduced_pair_receipt = unreachable_value();
    let _ = PlanarBooleanCommonPlaneReducedOperandPairRequest {
        operand_a_projected_request,
        operand_b_projected_request,
        reduced_pair_receipt,
        source_left_operand_workload_identity: String::new(),
        source_right_operand_workload_identity: String::new(),
        reduced_operand_pair_request_identity: String::new(),
    };
}

fn unreachable_value<T>() -> T {
    panic!("compile-fail fixture should never execute")
}
