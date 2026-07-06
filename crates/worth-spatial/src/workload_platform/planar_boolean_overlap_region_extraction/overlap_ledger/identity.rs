use super::rows::PlanarBooleanOverlapRegionDecisionKind;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

fn digest(parts: &[String]) -> String {
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, parts)
}

pub(super) fn decision_row_identity(
    request_identity: &str,
    kind: PlanarBooleanOverlapRegionDecisionKind,
    focal_identity: &str,
    related_identities: &[String],
) -> String {
    let mut parts = vec![
        request_identity.to_string(),
        format!("{kind:?}"),
        focal_identity.to_string(),
    ];
    parts.extend(related_identities.iter().cloned());
    format!("overlap-decision-row:{}", digest(&parts))
}

pub(super) fn decision_log_identity(request_identity: &str, row_identities: &[String]) -> String {
    format!(
        "overlap-decision-log:{request_identity}:{}",
        digest(row_identities)
    )
}

pub(super) fn ledger_row_identity(
    region_identity: &str,
    canonical_winding_identity: &str,
    signature_identity: &str,
) -> String {
    format!(
        "overlap-ledger-row:{}",
        digest(&[
            region_identity.to_string(),
            canonical_winding_identity.to_string(),
            signature_identity.to_string(),
        ])
    )
}

pub(super) fn ledger_identity(request_identity: &str, row_identities: &[String]) -> String {
    format!(
        "overlap-ledger:{request_identity}:{}",
        digest(row_identities)
    )
}

pub(super) fn receipt_identity(
    request_identity: &str,
    decision_log_identity: &str,
    ledger_identity: &str,
) -> String {
    format!(
        "overlap-ledger-receipt:{}",
        digest(&[
            request_identity.to_string(),
            decision_log_identity.to_string(),
            ledger_identity.to_string(),
        ])
    )
}

pub(super) fn bundle_identity(
    request_identity: &str,
    decision_log_identity: &str,
    ledger_identity: &str,
    receipt_identity: &str,
) -> String {
    format!(
        "overlap-ledger-assembly:{}",
        digest(&[
            request_identity.to_string(),
            decision_log_identity.to_string(),
            ledger_identity.to_string(),
            receipt_identity.to_string(),
        ])
    )
}
