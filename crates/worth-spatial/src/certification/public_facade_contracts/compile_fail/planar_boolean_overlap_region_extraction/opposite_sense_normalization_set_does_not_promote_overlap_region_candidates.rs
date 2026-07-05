use worth_spatial::facade::planar_boolean_overlap_region_extraction::{
    PlanarBooleanOppositeSenseOverlapNormalizationSet,
    PlanarBooleanOverlapRegionCandidateBoundaryBundle, PlanarBooleanSharedAreaAdmissionBundle,
};

fn normalization_set() -> PlanarBooleanOppositeSenseOverlapNormalizationSet {
    panic!("compile-fail fixture should not execute")
}

fn shared_area_admission_bundle() -> PlanarBooleanSharedAreaAdmissionBundle {
    panic!("compile-fail fixture should not execute")
}

fn main() {
    let normalization_set = normalization_set();
    let _: Option<PlanarBooleanOverlapRegionCandidateBoundaryBundle> = None;
    let _ = normalization_set.promote_overlap_region_candidates(&shared_area_admission_bundle());
}
