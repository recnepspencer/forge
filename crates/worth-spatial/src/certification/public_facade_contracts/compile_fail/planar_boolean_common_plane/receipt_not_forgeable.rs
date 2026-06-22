use worth_spatial::facade::planar_boolean_common_plane::PlanarBooleanCommonPlaneAgreementReceipt;

fn main() {
    let _receipt = PlanarBooleanCommonPlaneAgreementReceipt {
        agreement_identity: String::new(),
        shared_plane_identity: String::new(),
        left_surface_support_identity: String::new(),
        right_surface_support_identity: String::new(),
        left_witness: fake(),
        right_witness: fake(),
    };
}

fn fake<T>() -> T {
    unimplemented!()
}
