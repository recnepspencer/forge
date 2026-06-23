use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::row::PlanarBooleanLoopReconstructionLedgerRow;

pub(crate) fn ledger_row_identity(
    canonical_loop_identity: &str,
    tracked_loop_identity: &str,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-loop-reconstruction-ledger-row".to_string(),
            format!("canonical:{canonical_loop_identity}"),
            format!("tracked:{tracked_loop_identity}"),
        ],
    )
}

pub(crate) fn ledger_identity(
    request_identity: &str,
    decision_log_identity: &str,
    rows: &[PlanarBooleanLoopReconstructionLedgerRow],
) -> String {
    let mut parts = vec![
        "planar-boolean-loop-reconstruction-ledger".to_string(),
        format!("request:{request_identity}"),
        format!("decision-log:{decision_log_identity}"),
    ];
    parts.extend(
        rows.iter()
            .map(|row| format!("row:{}", row.ledger_row_identity())),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(crate) fn receipt_identity(ledger_identity: &str, consumed_identities: &[String]) -> String {
    let mut parts = vec![
        "planar-boolean-loop-reconstruction-ledger-receipt".to_string(),
        format!("ledger:{ledger_identity}"),
    ];
    parts.extend(
        consumed_identities
            .iter()
            .map(|identity| format!("consumed:{identity}")),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(crate) fn downstream_consumption_identity(receipt_identity: &str) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-loop-reconstruction-ledger-downstream".to_string(),
            format!("receipt:{receipt_identity}"),
        ],
    )
}
