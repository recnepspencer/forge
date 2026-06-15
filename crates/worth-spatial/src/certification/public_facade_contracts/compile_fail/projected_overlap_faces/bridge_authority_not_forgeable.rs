use worth_spatial::facade::projected_overlap_faces::CertifiedProjectedOverlapBridgeAuthority;

fn main() {
    let _authority = CertifiedProjectedOverlapBridgeAuthority {
        authority_digest: "fake".to_string(),
        context_identity: "fake".to_string(),
        projection_stage_identity: "fake".to_string(),
        movement_rotation_posture_identity: "fake".to_string(),
        certified_faces: panic!("cannot provide certified faces"),
        extraction_bundle: panic!("cannot provide extraction bundle"),
    };
}
