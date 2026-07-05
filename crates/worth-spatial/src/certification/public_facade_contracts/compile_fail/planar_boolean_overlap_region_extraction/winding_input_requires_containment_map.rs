use worth_spatial::facade::planar_boolean_overlap_region_extraction::{
    PlanarBooleanCoplanarOverlapArrangementGraph, PlanarBooleanOverlapCellWindingFieldInput,
};

fn arrangement_graph() -> PlanarBooleanCoplanarOverlapArrangementGraph {
    unreachable!()
}

fn main() {
    let _ = PlanarBooleanOverlapCellWindingFieldInput::from_arrangement(&arrangement_graph());
}
