use worth_spatial::facade::planar_boolean_overlap_region_extraction::{
    PlanarBooleanBoundaryContactClassificationBundle, PlanarBooleanOverlapIslandSet,
};

fn overlap_islands() -> PlanarBooleanOverlapIslandSet {
    panic!("fixture should not run")
}

fn main() {
    let _ = PlanarBooleanBoundaryContactClassificationBundle::admit(overlap_islands());
}
