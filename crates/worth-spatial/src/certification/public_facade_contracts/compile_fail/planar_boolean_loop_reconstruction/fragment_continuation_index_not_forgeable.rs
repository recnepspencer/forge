use worth_spatial::facade::planar_boolean_loop_reconstruction::{
    PlanarBooleanContinuationOrderingBasis, PlanarBooleanFragmentContinuationCounters,
    PlanarBooleanFragmentContinuationIndex, PlanarBooleanFragmentContinuationRow,
};

fn main() {
    let _ = PlanarBooleanFragmentContinuationIndex {
        continuation_index_identity: String::from("forged"),
        request_identity: String::from("synthetic request"),
        source_provenance_bundle_identity: String::from("synthetic provenance"),
        split_vertex_identity_set_identity: String::from("synthetic vertices"),
        fragment_set_identity: String::from("synthetic fragments"),
        overlap_chain_set_identity: String::from("synthetic overlap"),
        rows: Vec::<PlanarBooleanFragmentContinuationRow>::new(),
        ordering_basis: unavailable_ordering_basis(),
        counters: PlanarBooleanFragmentContinuationCounters::default(),
        neighborhood_offsets: unsafe { std::mem::MaybeUninit::zeroed().assume_init() },
    };
}

fn unavailable_ordering_basis() -> PlanarBooleanContinuationOrderingBasis {
    panic!("compile-fail fixture must never construct ordering basis")
}
