use worth_spatial::facade::planar_boolean_overlap_region_extraction::{
    PlanarBooleanCoplanarOverlapArrangementGraph, PlanarBooleanOverlapIslandCandidateInput,
};

fn arrangement_graph() -> PlanarBooleanCoplanarOverlapArrangementGraph {
    unreachable!()
}

fn main() {
    let _ = PlanarBooleanOverlapIslandCandidateInput::from_cell_classification(
        &arrangement_graph(),
    );
}
