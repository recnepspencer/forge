use worth_spatial::facade::planar_boolean_overlap_region_extraction::{
    PlanarBooleanBoundaryContactClassificationBundle, PlanarBooleanOverlapIslandSet,
};

fn overlap_islands() -> PlanarBooleanOverlapIslandSet {
    panic!("compile-fail fixture should never execute");
}

fn main() {
    let _ =
        PlanarBooleanBoundaryContactClassificationBundle::from_island_component_bundle(
            &overlap_islands(),
        );
}
