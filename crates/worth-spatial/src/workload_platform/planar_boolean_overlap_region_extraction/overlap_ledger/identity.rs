use super::rows::PlanarBooleanOverlapRegionDecisionKind;

fn digest(parts: &[String]) -> String {
    parts.join("|")
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
        "overlap-ledger-row:{region_identity}:{canonical_winding_identity}:{signature_identity}"
    )
}

pub(super) fn ledger_identity(request_identity: &str, row_identities: &[String]) -> String {
    format!("overlap-ledger:{request_identity}:{}", digest(row_identities))
}

pub(super) fn receipt_identity(
    request_identity: &str,
    decision_log_identity: &str,
    ledger_identity: &str,
) -> String {
    format!("overlap-ledger-receipt:{request_identity}:{decision_log_identity}|{ledger_identity}")
}

pub(super) fn bundle_identity(
    request_identity: &str,
    decision_log_identity: &str,
    ledger_identity: &str,
    receipt_identity: &str,
) -> String {
    format!(
        "overlap-ledger-assembly:{request_identity}:{decision_log_identity}|{ledger_identity}|{receipt_identity}"
    )
}
