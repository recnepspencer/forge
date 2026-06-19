use worth_spatial::facade::planar_boolean_loop_reconstruction::{
    PlanarBooleanLoopContainmentEvidencePosture, PlanarBooleanLoopContainmentEvidencePostureSet,
};

fn bogus<T>() -> T {
    panic!("compile-fail fixture")
}

fn main() {
    let _ = PlanarBooleanLoopContainmentEvidencePostureSet {
        set_identity: String::new(),
        request_identity: String::new(),
        rows: vec![bogus::<PlanarBooleanLoopContainmentEvidencePosture>()],
    };
}
