use worth_kernel::workload_composition::PlanarBooleanCommonPlaneSharedPlaneIdentifiedRequest;

fn main() {
    let posture_agreed_request = unreachable_value();
    let identity_receipt = unreachable_value();
    let _ = PlanarBooleanCommonPlaneSharedPlaneIdentifiedRequest {
        posture_agreed_request,
        identity_receipt,
        shared_plane_identified_request_identity: String::new(),
    };
}

fn unreachable_value<T>() -> T {
    panic!("compile-fail fixture should never execute")
}
