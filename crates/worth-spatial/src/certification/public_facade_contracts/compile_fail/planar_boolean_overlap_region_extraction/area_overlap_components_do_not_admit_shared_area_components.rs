use worth_spatial::facade::planar_boolean_overlap_region_extraction::{
    PlanarBooleanAreaOverlapComponentSet, PlanarBooleanOverlapCellContainmentMap,
    PlanarBooleanOverlapCellWindingField,
};

fn area_overlap_components() -> PlanarBooleanAreaOverlapComponentSet {
    panic!("compile-fail fixture should not execute")
}

fn containment_map() -> PlanarBooleanOverlapCellContainmentMap {
    panic!("compile-fail fixture should not execute")
}

fn winding_field() -> PlanarBooleanOverlapCellWindingField {
    panic!("compile-fail fixture should not execute")
}

fn main() {
    let area_overlap_components = area_overlap_components();
    let _ = area_overlap_components
        .admit_shared_area_components(&containment_map(), &winding_field());
}
