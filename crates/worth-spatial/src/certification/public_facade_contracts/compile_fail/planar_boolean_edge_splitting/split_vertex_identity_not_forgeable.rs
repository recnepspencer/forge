use worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanSplitVertexIdentityRow;

fn main() {
    let _ = PlanarBooleanSplitVertexIdentityRow::new(
        "vertex".to_string(),
        "source edge".to_string(),
        "carrier".to_string(),
        0.5,
        0.5f64.to_bits(),
        "frame".to_string(),
        "precision".to_string(),
        vec!["point cut".to_string()],
        vec!["parameter fact".to_string()],
        vec![],
        vec![],
        vec!["coordinate fact".to_string()],
        vec!["event provenance".to_string()],
        vec!["event group".to_string()],
    );
}
