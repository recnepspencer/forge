use worth_spatial::facade::planar_boolean_overlap_region_extraction::{
    PlanarBooleanMixedBoundaryAreaOutcomeSet, PlanarBooleanOverlapRegionCandidateBoundaryBundle,
    PlanarBooleanPreRegionNormalizationBundle,
};

fn mixed_boundary_area_outcomes() -> PlanarBooleanMixedBoundaryAreaOutcomeSet {
    panic!("compile-fail fixture should not execute")
}

fn pre_region_normalization_bundle() -> PlanarBooleanPreRegionNormalizationBundle {
    panic!("compile-fail fixture should not execute")
}

fn main() {
    let mixed_boundary_area_outcomes = mixed_boundary_area_outcomes();
    let _: Option<PlanarBooleanOverlapRegionCandidateBoundaryBundle> = None;
    let _ = mixed_boundary_area_outcomes
        .promote_overlap_region_candidates(&pre_region_normalization_bundle());
}
