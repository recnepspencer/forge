use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::canonical_segment::PlanarBooleanCanonicalSegment;

pub(crate) fn canonical_segment_identity(segment: &PlanarBooleanCanonicalSegment) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-canonical-segment".to_string(),
            format!("side:{}", segment.operand_side().query_key()),
            format!("face:{}", segment.source_face_identity()),
            format!("loop:{}", segment.source_loop_identity()),
            format!("edge:{}", segment.source_edge_identity()),
            format!("loop-role:{}", segment.loop_role().query_key()),
            format!(
                "low-endpoint:{}",
                segment
                    .normalized_endpoints()
                    .low()
                    .projected_endpoint_fact_identity()
            ),
            format!(
                "high-endpoint:{}",
                segment
                    .normalized_endpoints()
                    .high()
                    .projected_endpoint_fact_identity()
            ),
            format!("local-frame:{}", segment.local_frame_identity()),
            format!("projection-stage:{}", segment.projection_stage_identity()),
            format!("precision-basis:{}", segment.precision_basis_identity()),
        ],
    )
}
