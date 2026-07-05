use worth_spatial::facade::planar_boolean_overlap_region_extraction::{
    PlanarBooleanOverlapArrangementCellSet, PlanarBooleanOverlapCellContainmentInput,
};

fn arrangement_cells() -> PlanarBooleanOverlapArrangementCellSet {
    unreachable!()
}

fn main() {
    let _ = PlanarBooleanOverlapCellContainmentInput::from_cell_set(&arrangement_cells());
}
