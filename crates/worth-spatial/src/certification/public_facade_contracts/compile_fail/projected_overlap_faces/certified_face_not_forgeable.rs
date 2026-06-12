use worth_spatial::facade::projected_overlap_faces::CertifiedProjectedOverlapFace;

fn fake<T>() -> T {
    panic!("compile-fail only")
}

fn main() {
    let _forged = CertifiedProjectedOverlapFace {
        projection_stage_identity: String::new(),
        face_identity: String::new(),
        projected_face_identity: String::new(),
        loop_identity: String::new(),
        projected_loop_identity: String::new(),
        source_geometry: fake(),
        certified_face: fake(),
    };
}
