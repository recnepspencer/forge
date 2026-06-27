use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::counters::EvidenceLookupIndexProductCounters;
use super::lifecycle_posture::EvidenceLookupIndexLifecyclePosture;

pub(crate) fn index_product_digest(
    selected_plan_digest: &str,
    spatial_touch_digest: &str,
    stage_receipt_digest: &str,
    basis_digest: &str,
    topology_support_digest: &str,
    query_support_digest: &str,
    lifecycle_posture: EvidenceLookupIndexLifecyclePosture,
    disposal_posture: super::disposal_posture::EvidenceLookupIndexDisposalPosture,
    counters: &EvidenceLookupIndexProductCounters,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "worth-spatial:evidence-lookup-index-product:v1".to_string(),
            format!("selected-plan:{selected_plan_digest}"),
            format!("spatial-touch:{spatial_touch_digest}"),
            format!("stage-receipt:{stage_receipt_digest}"),
            format!("basis:{basis_digest}"),
            format!("topology-support:{topology_support_digest}"),
            format!("query-support:{query_support_digest}"),
            format!("lifecycle:{:?}", lifecycle_posture.kind()),
            format!("disposal:{:?}", disposal_posture.kind()),
            format!("basis-rows:{}", counters.selected_basis_row_count()),
            format!("resident-bytes:{}", counters.resident_byte_count()),
            format!("reused:{}", counters.reused_index_count()),
            format!("rebuilt:{}", counters.rebuilt_index_count()),
        ],
    )
}
