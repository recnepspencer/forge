use worth_spatial::facade::planar_boolean_overlap_region_extraction::{
    PlanarBooleanBoundaryOnlyOverlapOutcomeSet, PlanarBooleanPostAdmissionNormalizationBundle,
};

fn boundary_only_overlap_outcomes() -> PlanarBooleanBoundaryOnlyOverlapOutcomeSet {
    panic!("compile-fail fixture should not execute")
}

fn main() {
    let boundary_only_overlap_outcomes = boundary_only_overlap_outcomes();
    let _: Option<PlanarBooleanPostAdmissionNormalizationBundle> = None;
    let _ = boundary_only_overlap_outcomes.normalize_post_admission_canonical_winding();
}
