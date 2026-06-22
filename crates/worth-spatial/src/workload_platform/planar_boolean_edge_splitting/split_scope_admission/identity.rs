use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::scope_admission::PlanarBooleanEdgeSplitScopeAdmission;

pub(crate) fn split_scope_admission_identity(
    admission: &PlanarBooleanEdgeSplitScopeAdmission,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-edge-split-scope-admission".to_string(),
            format!("split-request:{}", admission.split_request_identity()),
            format!("event-ledger:{}", admission.event_ledger_identity()),
            format!("reduced-pair:{}", admission.reduced_pair_identity()),
            format!(
                "segment-carriers:{}",
                admission.segment_carrier_set_identity()
            ),
            format!(
                "candidate-index-product:{}",
                admission.candidate_index_product_identity()
            ),
            format!("query-plan:{}", admission.query_index_plan_digest()),
            format!("scope-class:{}", admission.scope_class().stable_name()),
            format!(
                "degeneracy-policy:{}",
                admission.degeneracy_policy().stable_name()
            ),
            format!(
                "determinism-policy:{}",
                admission.determinism_policy().stable_name()
            ),
            format!(
                "overlap-policy:{}",
                admission.overlap_policy().stable_name()
            ),
            format!(
                "policy-outcome:{}",
                admission.policy_outcome().kind().stable_name()
            ),
            format!(
                "source-carriers:{}",
                admission.counters().source_carrier_count()
            ),
            format!("point-events:{}", admission.counters().point_event_count()),
            format!(
                "interval-events:{}",
                admission.counters().interval_event_count()
            ),
            format!("event-groups:{}", admission.counters().event_group_count()),
        ],
    )
}
