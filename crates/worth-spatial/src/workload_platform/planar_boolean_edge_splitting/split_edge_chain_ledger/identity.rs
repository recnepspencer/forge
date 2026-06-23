use worth_primitives::{truth_digest_parts, TruthDigestScope};

pub(crate) fn declaration_identity(
    split_request_identity: &str,
    split_chain_validation_receipt_identity: &str,
    split_persistent_naming_receipt_identity: &str,
    split_decision_log_receipt_identity: &str,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "split-edge-chain-ledger-query-declaration".to_string(),
            format!("request:{split_request_identity}"),
            format!("validation:{split_chain_validation_receipt_identity}"),
            format!("names:{split_persistent_naming_receipt_identity}"),
            format!("decisions:{split_decision_log_receipt_identity}"),
        ],
    )
}

pub(crate) fn chain_identity(
    declaration_identity: &str,
    source_edge_identity: &str,
    carrier_identity: &str,
    fragment_identities: &[String],
    overlap_chain_identities: &[String],
    persistent_name_row_identities: &[String],
    decision_identities: &[String],
) -> String {
    let mut parts = vec![
        "split-edge-chain-ledger-chain".to_string(),
        format!("declaration:{declaration_identity}"),
        format!("source-edge:{source_edge_identity}"),
        format!("carrier:{carrier_identity}"),
    ];
    append_all(&mut parts, "fragment", fragment_identities);
    append_all(&mut parts, "overlap-chain", overlap_chain_identities);
    append_all(
        &mut parts,
        "persistent-name-row",
        persistent_name_row_identities,
    );
    append_all(&mut parts, "decision", decision_identities);
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(crate) fn ledger_identity(declaration_identity: &str, chain_identities: &[String]) -> String {
    let mut parts = vec![
        "split-edge-chain-ledger".to_string(),
        format!("declaration:{declaration_identity}"),
    ];
    append_all(&mut parts, "chain", chain_identities);
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(crate) fn receipt_identity(ledger_identity: &str, consumed_identities: &[String]) -> String {
    let mut parts = vec![
        "split-edge-chain-ledger-receipt".to_string(),
        format!("ledger:{ledger_identity}"),
    ];
    append_all(&mut parts, "consumed", consumed_identities);
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(crate) fn downstream_consumption_identity(receipt_identity: &str) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "split-edge-chain-ledger-downstream-consumption".to_string(),
            format!("receipt:{receipt_identity}"),
        ],
    )
}

fn append_all(parts: &mut Vec<String>, label: &str, identities: &[String]) {
    for identity in identities {
        parts.push(format!("{label}:{identity}"));
    }
}
