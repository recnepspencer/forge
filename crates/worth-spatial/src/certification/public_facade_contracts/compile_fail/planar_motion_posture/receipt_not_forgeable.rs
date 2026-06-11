use worth_spatial::facade::planar_motion_posture::PlanarMotionPostureReceipt;

fn main() {
    let _receipt = PlanarMotionPostureReceipt {
        basis: fake(),
        declaration_digest: String::new(),
        envelope_digest: String::new(),
        retained_motion_digest: String::new(),
        counters: fake(),
    };
}

fn fake<T>() -> T {
    unimplemented!()
}
