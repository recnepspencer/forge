use worth_spatial::facade::planar_boolean_overlap_region_extraction::{
    PlanarBooleanOverlapCellContainmentMap, PlanarBooleanOverlapCellWindingField,
    PlanarBooleanPureBoundaryOnlyOutcomeSet,
};

fn pure_boundary_only_outcomes() -> PlanarBooleanPureBoundaryOnlyOutcomeSet {
    panic!("compile-fail fixture should not execute")
}

fn containment_map() -> PlanarBooleanOverlapCellContainmentMap {
    panic!("compile-fail fixture should not execute")
}

fn winding_field() -> PlanarBooleanOverlapCellWindingField {
    panic!("compile-fail fixture should not execute")
}

fn main() {
    let pure_boundary_only_outcomes = pure_boundary_only_outcomes();
    let _ = pure_boundary_only_outcomes
        .admit_shared_area_components(&containment_map(), &winding_field());
}
