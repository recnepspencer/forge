use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::event_kind::PlanarBooleanIntervalEventKind;

pub(crate) fn normalized_interval_identity(
    parameter_range: [f64; 2],
    local_frame_identity: &str,
    precision_basis_identity: &str,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-normalized-interval".to_string(),
            format!("start-bits:{}", parameter_range[0].to_bits()),
            format!("end-bits:{}", parameter_range[1].to_bits()),
            format!("local-frame:{local_frame_identity}"),
            format!("precision-basis:{precision_basis_identity}"),
        ],
    )
}

pub(crate) fn source_interval_identity(
    segment_identity: &str,
    carrier_identity: &str,
    source_parameter_range: [f64; 2],
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-source-interval".to_string(),
            format!("segment:{segment_identity}"),
            format!("carrier:{carrier_identity}"),
            format!("start-bits:{}", source_parameter_range[0].to_bits()),
            format!("end-bits:{}", source_parameter_range[1].to_bits()),
        ],
    )
}

pub(crate) struct IntervalEventIdentityBasis<'a> {
    pub(crate) kind: PlanarBooleanIntervalEventKind,
    pub(crate) collinear_relation_identity: &'a str,
    pub(crate) segment_pair_identity: &'a str,
    pub(crate) left_segment_identity: &'a str,
    pub(crate) right_segment_identity: &'a str,
    pub(crate) left_carrier_identity: &'a str,
    pub(crate) right_carrier_identity: &'a str,
    pub(crate) normalized_interval_identity: &'a str,
    pub(crate) left_source_interval_identity: &'a str,
    pub(crate) right_source_interval_identity: &'a str,
}

pub(crate) fn interval_event_identity(basis: IntervalEventIdentityBasis<'_>) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-interval-event".to_string(),
            format!("kind:{}", basis.kind.as_str()),
            format!("collinear-relation:{}", basis.collinear_relation_identity),
            format!("segment-pair:{}", basis.segment_pair_identity),
            format!("left-segment:{}", basis.left_segment_identity),
            format!("right-segment:{}", basis.right_segment_identity),
            format!("left-carrier:{}", basis.left_carrier_identity),
            format!("right-carrier:{}", basis.right_carrier_identity),
            format!("normalized-interval:{}", basis.normalized_interval_identity),
            format!(
                "left-source-interval:{}",
                basis.left_source_interval_identity
            ),
            format!(
                "right-source-interval:{}",
                basis.right_source_interval_identity
            ),
        ],
    )
}

pub(crate) fn extraction_identity(
    collinear_relation_receipt_identity: &str,
    interval_event_identities: impl IntoIterator<Item = impl AsRef<str>>,
) -> String {
    let mut parts = vec![
        "planar-boolean-interval-event-extraction".to_string(),
        format!("collinear-relation-receipt:{collinear_relation_receipt_identity}"),
    ];
    parts.extend(
        interval_event_identities
            .into_iter()
            .map(|identity| format!("interval-event:{}", identity.as_ref())),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}
