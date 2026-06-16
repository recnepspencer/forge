use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::relation_kind::PlanarBooleanCollinearRelationKind;

pub(crate) fn collinear_relation_identity(
    predicate_bound_pair_identity: &str,
    segment_contract_fact_digest: &str,
    kind: PlanarBooleanCollinearRelationKind,
    interval_basis_identity: Option<&str>,
    touch_point_identity: Option<&str>,
) -> String {
    let mut parts = vec![
        "planar-boolean-collinear-relation".to_string(),
        format!("bound-pair:{predicate_bound_pair_identity}"),
        format!("segment-contract:{segment_contract_fact_digest}"),
        format!("kind:{}", kind.as_str()),
    ];
    if let Some(interval_basis_identity) = interval_basis_identity {
        parts.push(format!("interval-basis:{interval_basis_identity}"));
    }
    if let Some(touch_point_identity) = touch_point_identity {
        parts.push(format!("touch-point:{touch_point_identity}"));
    }
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(crate) fn interval_basis_identity(
    left_parameter_range: [f64; 2],
    right_parameter_range: [f64; 2],
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-collinear-interval-basis".to_string(),
            format!("left-start-bits:{}", left_parameter_range[0].to_bits()),
            format!("left-end-bits:{}", left_parameter_range[1].to_bits()),
            format!("right-start-bits:{}", right_parameter_range[0].to_bits()),
            format!("right-end-bits:{}", right_parameter_range[1].to_bits()),
        ],
    )
}

pub(crate) fn receipt_identity(
    predicate_binding_identity: &str,
    relation_identities: impl IntoIterator<Item = impl AsRef<str>>,
) -> String {
    let mut parts = vec![
        "planar-boolean-collinear-relation-receipt".to_string(),
        format!("predicate-binding:{predicate_binding_identity}"),
    ];
    parts.extend(
        relation_identities
            .into_iter()
            .map(|identity| format!("relation:{}", identity.as_ref())),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}
