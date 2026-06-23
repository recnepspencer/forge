use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::carrier::PlanarBooleanSegmentCarrier;

pub(crate) fn segment_carrier_set_identity(
    left: &[PlanarBooleanSegmentCarrier],
    right: &[PlanarBooleanSegmentCarrier],
) -> String {
    let mut parts = vec!["planar-boolean-segment-carrier-set".to_string()];
    parts.extend(
        left.iter()
            .map(|carrier| format!("left-carrier:{}", carrier.carrier_identity())),
    );
    parts.extend(
        right
            .iter()
            .map(|carrier| format!("right-carrier:{}", carrier.carrier_identity())),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(crate) fn segment_carrier_identity(carrier: &PlanarBooleanSegmentCarrier) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-segment-carrier".to_string(),
            format!("side:{}", carrier.operand_side().query_key()),
            format!("face:{}", carrier.source_face_identity()),
            format!("loop:{}", carrier.source_loop_identity()),
            format!("edge:{}", carrier.source_edge_identity()),
            format!("loop-role:{}", carrier.loop_role().query_key()),
            format!(
                "start:{}",
                carrier.start().projected_endpoint_fact_identity()
            ),
            format!("end:{}", carrier.end().projected_endpoint_fact_identity()),
            format!("local-frame:{}", carrier.local_frame_identity()),
            format!("projection-stage:{}", carrier.projection_stage_identity()),
            format!("precision-basis:{}", carrier.precision_basis_identity()),
        ],
    )
}
