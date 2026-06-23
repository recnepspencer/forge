use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::request::PlanarBooleanEdgeSplitRequest;

pub(crate) fn edge_split_request_identity(request: &PlanarBooleanEdgeSplitRequest) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-edge-split-request".to_string(),
            format!("event-ledger:{}", request.event_ledger_identity()),
            format!(
                "downstream-consumption:{}",
                request.downstream_consumption_identity()
            ),
            format!("reduced-pair:{}", request.reduced_pair_identity()),
            format!(
                "event-extraction-request:{}",
                request.event_extraction_request_identity()
            ),
            format!(
                "segment-carriers:{}",
                request.segment_carrier_set_identity()
            ),
            format!(
                "segment-pair-enumeration:{}",
                request.segment_pair_enumeration_identity()
            ),
            format!(
                "candidate-index-gate:{}",
                request.candidate_index_consumption_gate_identity()
            ),
            format!(
                "candidate-index-product:{}",
                request.candidate_index_product_identity()
            ),
            format!("query-plan:{}", request.query_index_plan_digest()),
            format!(
                "retained-replay:{}",
                request
                    .retained_replay_stage_identity()
                    .unwrap_or("no-retained-replay-stage")
            ),
            format!(
                "segment-carrier-count:{}",
                request.counters().segment_carrier_count()
            ),
            format!("point-events:{}", request.counters().point_event_count()),
            format!(
                "interval-events:{}",
                request.counters().interval_event_count()
            ),
            format!("event-groups:{}", request.counters().event_group_count()),
        ],
    )
}
