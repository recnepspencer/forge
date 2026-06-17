use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::group::PlanarBooleanEventGroupKind;

pub(crate) struct EventGroupIdentityBasis<'a> {
    pub(crate) kind: PlanarBooleanEventGroupKind,
    pub(crate) canonical_group_key: &'a str,
    pub(crate) point_event_identities: &'a [String],
    pub(crate) interval_event_identities: &'a [String],
    pub(crate) segment_pair_identities: &'a [String],
    pub(crate) participating_carrier_identities: &'a [String],
    pub(crate) source_endpoint_identities: &'a [String],
    pub(crate) source_interval_identities: &'a [String],
}

pub(crate) fn event_group_identity(basis: EventGroupIdentityBasis<'_>) -> String {
    let mut parts = vec![
        "planar-boolean-event-group".to_string(),
        format!("kind:{}", basis.kind.as_str()),
        format!("key:{}", basis.canonical_group_key),
    ];
    parts.extend(
        basis
            .point_event_identities
            .iter()
            .map(|identity| format!("point-event:{identity}")),
    );
    parts.extend(
        basis
            .interval_event_identities
            .iter()
            .map(|identity| format!("interval-event:{identity}")),
    );
    parts.extend(
        basis
            .segment_pair_identities
            .iter()
            .map(|identity| format!("segment-pair:{identity}")),
    );
    parts.extend(
        basis
            .participating_carrier_identities
            .iter()
            .map(|identity| format!("carrier:{identity}")),
    );
    parts.extend(
        basis
            .source_endpoint_identities
            .iter()
            .map(|identity| format!("source-endpoint:{identity}")),
    );
    parts.extend(
        basis
            .source_interval_identities
            .iter()
            .map(|identity| format!("source-interval:{identity}")),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}
