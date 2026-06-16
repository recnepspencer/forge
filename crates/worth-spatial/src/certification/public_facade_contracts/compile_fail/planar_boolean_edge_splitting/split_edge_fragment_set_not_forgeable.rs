use worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanSplitEdgeFragmentSet;

fn main() {
    let _ = PlanarBooleanSplitEdgeFragmentSet::new(
        "fragment set".to_string(),
        "interval subdivision set".to_string(),
        "split vertex set".to_string(),
        vec![],
        Default::default(),
    );
}
