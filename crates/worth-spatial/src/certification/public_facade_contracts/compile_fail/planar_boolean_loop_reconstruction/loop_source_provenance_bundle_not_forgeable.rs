use worth_spatial::facade::planar_boolean_loop_reconstruction::{
    PlanarBooleanFragmentMembershipMap, PlanarBooleanLoopOverlapChainLineageMap,
    PlanarBooleanLoopSourceCarrierSet, PlanarBooleanLoopSourceProvenanceBundle,
};

fn bogus<T>() -> T {
    panic!("compile-fail fixture")
}

fn main() {
    let _ = PlanarBooleanLoopSourceProvenanceBundle {
        bundle_identity: String::new(),
        request_identity: String::new(),
        split_ledger_receipt_identity: String::new(),
        source_loop_carriers: bogus::<PlanarBooleanLoopSourceCarrierSet>(),
        fragment_membership_map: bogus::<PlanarBooleanFragmentMembershipMap>(),
        overlap_chain_lineage_map: bogus::<PlanarBooleanLoopOverlapChainLineageMap>(),
        counters: Default::default(),
    };
}
