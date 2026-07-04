use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::projection::read_views::domain::TopologyReadRequestFamily;

use super::selected_route::TopologyQueryBackedConsumerFamilyRow;

pub(crate) fn family_row_digest(
    request_family: TopologyReadRequestFamily,
    status: &str,
    reuse_posture: &str,
    compiled_product_identity: &str,
    equivalence_policy_identity: &str,
    selected_equivalence_family: String,
    selected_equivalence_basis: &str,
    selected_compatibility_basis: &str,
    selected_reuse_basis: &str,
    reuse_decision_identity: &str,
    rebuild_denial_identity: &str,
    query_execution_count: usize,
    row_scan_fallback_count: usize,
    whole_view_fallback_count: usize,
    repeated_rediscovery_denied_count: usize,
    closeout_row_digest: &str,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            format!("family:{request_family:?}"),
            format!("status:{status}"),
            format!("reuse:{reuse_posture}"),
            format!("compiled-product:{compiled_product_identity}"),
            format!("equivalence-policy:{equivalence_policy_identity}"),
            format!("selected-equivalence-family:{selected_equivalence_family}"),
            format!("selected-equivalence-basis:{selected_equivalence_basis}"),
            format!("selected-compatibility-basis:{selected_compatibility_basis}"),
            format!("selected-reuse-basis:{selected_reuse_basis}"),
            format!("reuse-decision:{reuse_decision_identity}"),
            format!("rebuild-denial:{rebuild_denial_identity}"),
            format!("query-execution:{query_execution_count}"),
            format!("row-scan-fallback:{row_scan_fallback_count}"),
            format!("whole-view-fallback:{whole_view_fallback_count}"),
            format!("repeated-rediscovery-denied:{repeated_rediscovery_denied_count}"),
            format!("closeout-row:{closeout_row_digest}"),
        ],
    )
}

pub(crate) fn closeout_digest(
    family_rows: &[TopologyQueryBackedConsumerFamilyRow],
    handle_identity_digest: &str,
    support_snapshot_digest: &str,
    operating_context_identity_digest: &str,
    parity_verified_count: usize,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &family_rows
            .iter()
            .map(|row| format!("family-row:{}", row.row_digest()))
            .chain(std::iter::once(format!("handle:{handle_identity_digest}")))
            .chain(std::iter::once(format!(
                "support-snapshot:{support_snapshot_digest}"
            )))
            .chain(std::iter::once(format!(
                "operating-context:{operating_context_identity_digest}"
            )))
            .chain(std::iter::once(format!(
                "parity-verified:{parity_verified_count}"
            )))
            .chain(std::iter::once(
                "worth-topo:query-backed-consumer-cutover:v1".to_string(),
            ))
            .collect::<Vec<_>>(),
    )
}
