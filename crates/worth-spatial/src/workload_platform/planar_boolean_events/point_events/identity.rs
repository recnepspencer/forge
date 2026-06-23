use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::event_kind::PlanarBooleanPointEventKind;

pub(crate) struct PointEventIdentityBasis<'a> {
    pub(crate) segment_pair_identity: &'a str,
    pub(crate) left_segment_identity: &'a str,
    pub(crate) right_segment_identity: &'a str,
    pub(crate) left_carrier_identity: &'a str,
    pub(crate) right_carrier_identity: &'a str,
    pub(crate) local_frame_identity: &'a str,
    pub(crate) precision_basis_identity: &'a str,
    pub(crate) coordinate_fact_identity: &'a str,
    pub(crate) kind: PlanarBooleanPointEventKind,
}

pub(crate) fn point_event_identity(basis: PointEventIdentityBasis<'_>) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-point-event".to_string(),
            format!("segment-pair:{}", basis.segment_pair_identity),
            format!("left-segment:{}", basis.left_segment_identity),
            format!("right-segment:{}", basis.right_segment_identity),
            format!("left-carrier:{}", basis.left_carrier_identity),
            format!("right-carrier:{}", basis.right_carrier_identity),
            format!("local-frame:{}", basis.local_frame_identity),
            format!("precision-basis:{}", basis.precision_basis_identity),
            format!("coordinate-fact:{}", basis.coordinate_fact_identity),
            format!("kind:{}", basis.kind.as_str()),
        ],
    )
}

pub(crate) fn deduplicated_point_event_identity(
    kind: PlanarBooleanPointEventKind,
    coordinate_fact_identity: &str,
    participating_carrier_identities: &[String],
    endpoint_source_identities: &[String],
    endpoint_projection_fact_digests: &[String],
) -> String {
    let mut parts = vec![
        "planar-boolean-deduplicated-point-event".to_string(),
        format!("kind:{}", kind.as_str()),
        format!("coordinate-fact:{coordinate_fact_identity}"),
    ];
    parts.extend(
        participating_carrier_identities
            .iter()
            .map(|identity| format!("carrier:{identity}")),
    );
    parts.extend(
        endpoint_source_identities
            .iter()
            .map(|identity| format!("source-endpoint:{identity}")),
    );
    parts.extend(
        endpoint_projection_fact_digests
            .iter()
            .map(|digest| format!("endpoint-projection:{digest}")),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(crate) fn coordinate_fact_identity(
    point_2d: [f64; 2],
    local_frame_identity: &str,
    precision_basis_identity: &str,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-point-event-coordinate".to_string(),
            format!("x-bits:{}", point_2d[0].to_bits()),
            format!("y-bits:{}", point_2d[1].to_bits()),
            format!("local-frame:{local_frame_identity}"),
            format!("precision-basis:{precision_basis_identity}"),
        ],
    )
}

pub(crate) fn segment_parameter_identity(
    segment_identity: &str,
    carrier_identity: &str,
    parameter: f64,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-point-event-segment-parameter".to_string(),
            format!("segment:{segment_identity}"),
            format!("carrier:{carrier_identity}"),
            format!("parameter-bits:{}", parameter.to_bits()),
        ],
    )
}

pub(crate) fn extraction_identity(
    predicate_binding_identity: &str,
    point_event_identities: impl IntoIterator<Item = impl AsRef<str>>,
) -> String {
    let mut parts = vec![
        "planar-boolean-point-event-extraction".to_string(),
        format!("predicate-binding:{predicate_binding_identity}"),
    ];
    parts.extend(
        point_event_identities
            .into_iter()
            .map(|identity| format!("point-event:{}", identity.as_ref())),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}
