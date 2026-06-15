use worth_kernel::workload_composition::{
    PlanarBooleanCommonPlaneAdmittedOperandScope, PlanarBooleanCommonPlaneScopeAdmittedRequest,
};

fn main() {
    let reduction_request = unreachable_value();
    let _ = PlanarBooleanCommonPlaneScopeAdmittedRequest {
        reduction_request,
        admitted_scope: PlanarBooleanCommonPlaneAdmittedOperandScope::ClosedPlanarBodyPair,
        scope_admission_identity: String::new(),
    };
}

fn unreachable_value<T>() -> T {
    panic!("compile-fail fixture should never execute")
}
