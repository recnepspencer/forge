use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::ordered_events::PlanarBooleanOrderedEventSet;

pub(crate) struct EventLedgerIdentityBasis<'a> {
    pub(crate) reduced_pair_identity: &'a str,
    pub(crate) event_extraction_request_identity: &'a str,
    pub(crate) segment_carrier_set_identity: &'a str,
    pub(crate) segment_pair_enumeration_identity: &'a str,
    pub(crate) predicate_binding_identity: &'a str,
    pub(crate) point_event_extraction_identity: &'a str,
    pub(crate) collinear_relation_receipt_identity: &'a str,
    pub(crate) interval_event_extraction_identity: &'a str,
    pub(crate) ordered_events: &'a PlanarBooleanOrderedEventSet,
}

pub(crate) fn event_ledger_identity(basis: EventLedgerIdentityBasis<'_>) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &ledger_identity_parts(basis),
    )
}

pub(crate) fn downstream_consumption_identity(
    ledger_identity: &str,
    ordered_events: &PlanarBooleanOrderedEventSet,
) -> String {
    let mut parts = vec![
        "planar-boolean-event-ledger-downstream-consumption".to_string(),
        format!("ledger:{ledger_identity}"),
    ];
    parts.extend(
        ordered_events
            .event_group_identities()
            .iter()
            .map(|identity| format!("event-group:{identity}")),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

fn ledger_identity_parts(basis: EventLedgerIdentityBasis<'_>) -> Vec<String> {
    let mut parts = vec![
        "planar-boolean-event-ledger".to_string(),
        format!("reduced-pair:{}", basis.reduced_pair_identity),
        format!(
            "event-extraction-request:{}",
            basis.event_extraction_request_identity
        ),
        format!("segment-carrier-set:{}", basis.segment_carrier_set_identity),
        format!(
            "segment-pair-enumeration:{}",
            basis.segment_pair_enumeration_identity
        ),
        format!("predicate-binding:{}", basis.predicate_binding_identity),
        format!(
            "point-event-extraction:{}",
            basis.point_event_extraction_identity
        ),
        format!(
            "collinear-relation-receipt:{}",
            basis.collinear_relation_receipt_identity
        ),
        format!(
            "interval-event-extraction:{}",
            basis.interval_event_extraction_identity
        ),
    ];
    parts.extend(
        basis
            .ordered_events
            .point_event_identities()
            .iter()
            .map(|identity| format!("point-event:{identity}")),
    );
    parts.extend(
        basis
            .ordered_events
            .interval_event_identities()
            .iter()
            .map(|identity| format!("interval-event:{identity}")),
    );
    parts.extend(
        basis
            .ordered_events
            .event_group_identities()
            .iter()
            .map(|identity| format!("event-group:{identity}")),
    );
    parts.extend(
        basis
            .ordered_events
            .relation_diagnostic_identities()
            .iter()
            .map(|identity| format!("relation-diagnostic:{identity}")),
    );
    parts
}
