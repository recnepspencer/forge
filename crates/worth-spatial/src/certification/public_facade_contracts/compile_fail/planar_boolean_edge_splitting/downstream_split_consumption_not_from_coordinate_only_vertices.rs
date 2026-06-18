use worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanDownstreamSplitConsumptionInput;

fn reject_coordinate_only_vertices(vertices: Vec<(f64, f64)>) {
    let _ =
        PlanarBooleanDownstreamSplitConsumptionInput::from_coordinate_only_vertices(vertices);
}

fn main() {}
