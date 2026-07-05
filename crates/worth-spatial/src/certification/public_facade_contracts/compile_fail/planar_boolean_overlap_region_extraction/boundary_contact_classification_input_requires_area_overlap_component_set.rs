use worth_spatial::facade::planar_boolean_overlap_region_extraction::{
    PlanarBooleanBoundaryContactClassificationInput, PlanarBooleanBoundaryContactComponentSet,
    PlanarBooleanOverlapIslandSet,
};

fn overlap_islands() -> PlanarBooleanOverlapIslandSet {
    panic!("fixture should not run")
}

fn boundary_contact_components() -> PlanarBooleanBoundaryContactComponentSet {
    panic!("fixture should not run")
}

fn main() {
    let _ = PlanarBooleanBoundaryContactClassificationInput::new(
        &overlap_islands(),
        &boundary_contact_components(),
        &boundary_contact_components(),
    );
}
