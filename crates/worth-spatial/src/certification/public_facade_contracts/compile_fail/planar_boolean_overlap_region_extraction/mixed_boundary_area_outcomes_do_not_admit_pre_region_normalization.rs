use worth_spatial::facade::planar_boolean_overlap_region_extraction::{
    PlanarBooleanMixedBoundaryAreaOutcomeSet, PlanarBooleanOverlapChainRegionLineageMap,
};

fn mixed_boundary_area_outcomes() -> PlanarBooleanMixedBoundaryAreaOutcomeSet {
    panic!("compile-fail fixture should not execute")
}

fn chain_lineage_map() -> PlanarBooleanOverlapChainRegionLineageMap {
    panic!("compile-fail fixture should not execute")
}

fn main() {
    let mixed_boundary_area_outcomes = mixed_boundary_area_outcomes();
    let _ = mixed_boundary_area_outcomes.normalize_pre_region_coincidence(&chain_lineage_map());
}
