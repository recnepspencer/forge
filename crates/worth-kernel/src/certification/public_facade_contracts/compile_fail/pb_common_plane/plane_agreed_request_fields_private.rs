use worth_kernel::workload_composition::PlanarBooleanCommonPlanePlaneAgreedRequest;

fn main() {
    let admitted_request = unreachable_value();
    let agreement_receipt = unreachable_value();
    let _ = PlanarBooleanCommonPlanePlaneAgreedRequest {
        admitted_request,
        agreement_receipt,
        plane_agreement_identity: String::new(),
    };
}

fn unreachable_value<T>() -> T {
    panic!("compile-fail fixture should never execute")
}
