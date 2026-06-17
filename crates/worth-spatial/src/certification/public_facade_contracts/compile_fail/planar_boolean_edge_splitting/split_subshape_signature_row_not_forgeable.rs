use worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanSplitSubshapeSignatureRow;

fn main() {
    let _ = PlanarBooleanSplitSubshapeSignatureRow {
        row_identity: "forged".to_string(),
        artifact_identity: "artifact".to_string(),
        signature_basis_identity: "signature".to_string(),
        correspondence_only: false,
    };
}
