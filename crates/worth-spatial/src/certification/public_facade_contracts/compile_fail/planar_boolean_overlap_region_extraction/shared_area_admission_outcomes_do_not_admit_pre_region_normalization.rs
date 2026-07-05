use worth_spatial::facade::planar_boolean_overlap_region_extraction::{
    PlanarBooleanOverlapChainRegionLineageMap, PlanarBooleanSharedAreaAdmissionOutcomeSet,
};

fn shared_area_admission_outcomes() -> PlanarBooleanSharedAreaAdmissionOutcomeSet {
    panic!("compile-fail fixture should not execute")
}

fn chain_lineage_map() -> PlanarBooleanOverlapChainRegionLineageMap {
    panic!("compile-fail fixture should not execute")
}

fn main() {
    let shared_area_admission_outcomes = shared_area_admission_outcomes();
    let _ = shared_area_admission_outcomes.normalize_pre_region_coincidence(&chain_lineage_map());
}
