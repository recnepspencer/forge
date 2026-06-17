use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::carrier_event_row::PlanarBooleanSplitEventParticipationRow;

pub(crate) fn participation_index_identity(
    event_ledger_identity: &str,
    recovered_carrier_set_identity: &str,
    rows: &[PlanarBooleanSplitEventParticipationRow],
) -> String {
    let mut parts = vec![
        "planar-boolean-split-event-participation-index".to_string(),
        format!("event-ledger:{event_ledger_identity}"),
        format!("recovered-carrier-set:{recovered_carrier_set_identity}"),
    ];
    parts.extend(
        rows.iter()
            .map(|row| format!("row:{}", row.participation_row_identity())),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(crate) fn participation_row_identity(
    event_ledger_identity: &str,
    carrier_identity: &str,
    source_edge_identity: &str,
    start_source_endpoint_identity: &str,
    start_projected_endpoint_fact_identity: &str,
    end_source_endpoint_identity: &str,
    end_projected_endpoint_fact_identity: &str,
    point_event_identities: &[String],
    interval_event_identities: &[String],
    event_group_identities: &[String],
) -> String {
    let mut parts = vec![
        "planar-boolean-split-event-participation-row".to_string(),
        format!("event-ledger:{event_ledger_identity}"),
        format!("carrier:{carrier_identity}"),
        format!("source-edge:{source_edge_identity}"),
        format!("start-source-endpoint:{start_source_endpoint_identity}"),
        format!("start-projected-endpoint:{start_projected_endpoint_fact_identity}"),
        format!("end-source-endpoint:{end_source_endpoint_identity}"),
        format!("end-projected-endpoint:{end_projected_endpoint_fact_identity}"),
    ];
    parts.extend(
        point_event_identities
            .iter()
            .map(|value| format!("point:{value}")),
    );
    parts.extend(
        interval_event_identities
            .iter()
            .map(|value| format!("interval:{value}")),
    );
    parts.extend(
        event_group_identities
            .iter()
            .map(|value| format!("group:{value}")),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}
