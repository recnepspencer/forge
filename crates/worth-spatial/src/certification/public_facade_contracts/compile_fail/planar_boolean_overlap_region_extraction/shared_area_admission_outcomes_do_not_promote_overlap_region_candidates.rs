use worth_spatial::facade::planar_boolean_overlap_region_extraction::{
    PlanarBooleanOverlapRegionCandidateBoundaryBundle, PlanarBooleanPreRegionNormalizationBundle,
    PlanarBooleanSharedAreaAdmissionOutcomeSet,
};

fn shared_area_admission_outcomes() -> PlanarBooleanSharedAreaAdmissionOutcomeSet {
    panic!("compile-fail fixture should not execute")
}

fn pre_region_normalization_bundle() -> PlanarBooleanPreRegionNormalizationBundle {
    panic!("compile-fail fixture should not execute")
}

fn main() {
    let shared_area_admission_outcomes = shared_area_admission_outcomes();
    let _: Option<PlanarBooleanOverlapRegionCandidateBoundaryBundle> = None;
    let _ = shared_area_admission_outcomes
        .promote_overlap_region_candidates(&pre_region_normalization_bundle());
}
