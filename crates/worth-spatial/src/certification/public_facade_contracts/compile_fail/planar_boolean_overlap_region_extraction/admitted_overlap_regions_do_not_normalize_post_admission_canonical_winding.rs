use worth_spatial::facade::planar_boolean_overlap_region_extraction::{
    PlanarBooleanAdmittedOverlapRegionSet, PlanarBooleanPostAdmissionNormalizationBundle,
};

fn admitted_overlap_regions() -> PlanarBooleanAdmittedOverlapRegionSet {
    panic!("compile-fail fixture should not execute")
}

fn main() {
    let admitted_overlap_regions = admitted_overlap_regions();
    let _: Option<PlanarBooleanPostAdmissionNormalizationBundle> = None;
    let _ = admitted_overlap_regions.normalize_post_admission_canonical_winding();
}
