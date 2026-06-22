use worth_spatial::facade::planar_boolean_loop_reconstruction::PlanarBooleanContinuationOrderingBasis;

fn main() {
    let _ = PlanarBooleanContinuationOrderingBasis {
        basis_identity: String::from("forged"),
        request_identity: String::from("synthetic request"),
        continuation_index_identity: String::from("synthetic continuation index"),
        ordered_continuation_identities: Vec::new(),
    };
}
