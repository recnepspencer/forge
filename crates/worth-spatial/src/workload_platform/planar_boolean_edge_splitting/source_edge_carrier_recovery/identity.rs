use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::carrier_set::PlanarBooleanSplitSourceEdgeCarrierSet;
use super::recovered_carrier::PlanarBooleanSplitSourceEdgeCarrier;

pub(crate) fn recovered_carrier_identity(
    scope_admission_identity: &str,
    event_ledger_identity: &str,
    carrier: &PlanarBooleanSplitSourceEdgeCarrier,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-split-source-edge-carrier".to_string(),
            format!("scope-admission:{scope_admission_identity}"),
            format!("event-ledger:{event_ledger_identity}"),
            format!("operand-side:{}", carrier.operand_side().query_key()),
            format!("source-face:{}", carrier.source_face_identity()),
            format!("source-loop:{}", carrier.source_loop_identity()),
            format!("source-edge:{}", carrier.source_edge_identity()),
            format!("carrier:{}", carrier.carrier_identity()),
            format!(
                "start-source-endpoint:{}",
                carrier.start_source_endpoint_identity()
            ),
            format!(
                "end-source-endpoint:{}",
                carrier.end_source_endpoint_identity()
            ),
            format!("local-frame:{}", carrier.local_frame_identity()),
            format!("projection-stage:{}", carrier.projection_stage_identity()),
            format!("precision-basis:{}", carrier.precision_basis_identity()),
        ],
    )
}

pub(crate) fn recovered_carrier_set_identity(
    set: &PlanarBooleanSplitSourceEdgeCarrierSet,
) -> String {
    let mut parts = vec![
        "planar-boolean-split-source-edge-carrier-set".to_string(),
        format!("scope-admission:{}", set.scope_admission_identity()),
        format!("split-request:{}", set.split_request_identity()),
        format!("event-ledger:{}", set.event_ledger_identity()),
        format!("segment-carrier-set:{}", set.segment_carrier_set_identity()),
        format!(
            "candidate-index-product:{}",
            set.candidate_index_product_identity()
        ),
        format!("query-index-plan:{}", set.query_index_plan_digest()),
    ];
    parts.extend(
        set.carriers()
            .iter()
            .map(|carrier| format!("carrier:{}", carrier.recovered_carrier_identity())),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}
