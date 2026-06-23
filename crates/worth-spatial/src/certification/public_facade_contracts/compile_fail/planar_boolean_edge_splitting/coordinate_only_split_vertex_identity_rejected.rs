use worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanSplitVertexIdentityRow;

fn main() {
    let _ = PlanarBooleanSplitVertexIdentityRow::new(
        "coordinate-only".to_string(),
        "source edge".to_string(),
        "carrier".to_string(),
        0.5,
        0.5f64.to_bits(),
        "frame".to_string(),
        "precision".to_string(),
        vec![],
        vec![],
        vec![],
        vec![],
        vec!["coordinate fact".to_string()],
        vec![],
        vec![],
    );
}
