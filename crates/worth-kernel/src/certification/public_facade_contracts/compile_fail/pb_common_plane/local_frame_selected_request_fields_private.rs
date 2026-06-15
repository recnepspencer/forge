use worth_kernel::workload_composition::PlanarBooleanCommonPlaneLocalFrameSelectedRequest;

fn main() {
    let _ = PlanarBooleanCommonPlaneLocalFrameSelectedRequest {
        shared_plane_identified_request: fake(),
        selection_receipt: fake(),
        local_frame_selection_identity: String::new(),
    };
}

fn fake<T>() -> T {
    unimplemented!()
}
