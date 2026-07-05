use worth_spatial::facade::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapIslandSet;

fn overlap_islands() -> PlanarBooleanOverlapIslandSet {
    panic!("compile-fail fixture should not execute")
}

fn main() {
    let overlap_islands = overlap_islands();
    let _ = overlap_islands.classify_boundary_contact_components();
}
