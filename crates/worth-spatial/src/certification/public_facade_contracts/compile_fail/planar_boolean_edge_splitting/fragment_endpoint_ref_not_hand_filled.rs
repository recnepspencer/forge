use worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanSplitEdgeFragmentEndpointRef;

fn main() {
    let _ = PlanarBooleanSplitEdgeFragmentEndpointRef::split_vertex(
        "split vertex",
        "source edge",
        "carrier",
        0.5f64.to_bits(),
        "frame",
        "precision",
    );
}
