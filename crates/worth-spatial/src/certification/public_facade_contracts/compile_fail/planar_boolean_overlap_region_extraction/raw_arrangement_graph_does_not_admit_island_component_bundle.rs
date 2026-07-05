use worth_spatial::facade::planar_boolean_overlap_region_extraction::{
    PlanarBooleanCoplanarOverlapArrangementGraph, PlanarBooleanOverlapIslandComponentBundle,
};

fn arrangement_graph() -> PlanarBooleanCoplanarOverlapArrangementGraph {
    panic!("fixture should not run")
}

fn main() {
    let _ = PlanarBooleanOverlapIslandComponentBundle::admit(arrangement_graph());
}
