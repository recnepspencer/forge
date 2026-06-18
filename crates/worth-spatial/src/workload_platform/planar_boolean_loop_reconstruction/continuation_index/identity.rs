use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::workload_platform::planar_boolean_events::PlanarBooleanSourceIntervalSense;

use super::ordering::PlanarBooleanContinuationOrderingBasis;
use super::row::{
    PlanarBooleanFragmentContinuationEndpointRole, PlanarBooleanFragmentContinuationRow,
};

pub(crate) fn continuation_neighborhood_identity(
    request_identity: &str,
    split_vertex_identity: &str,
    source_loop_identity: &str,
    source_sense: PlanarBooleanSourceIntervalSense,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-fragment-continuation-neighborhood".to_string(),
            format!("request:{request_identity}"),
            format!("vertex:{split_vertex_identity}"),
            format!("source-loop:{source_loop_identity}"),
            format!("source-sense:{}", source_sense_name(source_sense)),
        ],
    )
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn continuation_identity(
    request_identity: &str,
    split_vertex_identity: &str,
    fragment_identity: &str,
    source_loop_identity: &str,
    source_face_identity: &str,
    source_edge_identity: &str,
    carrier_identity: &str,
    source_loop_carrier_identity: &str,
    fragment_endpoint_role: PlanarBooleanFragmentContinuationEndpointRole,
    source_sense: PlanarBooleanSourceIntervalSense,
    endpoint_parameter_bits: u64,
    fragment_parameter_range_bits: [u64; 2],
    local_frame_identity: &str,
    precision_basis_identity: &str,
    event_group_identities: &[String],
    boundary_roles: &[crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanOverlapChainBoundaryRole],
) -> String {
    let mut parts = vec![
        "planar-boolean-fragment-continuation".to_string(),
        format!("request:{request_identity}"),
        format!("vertex:{split_vertex_identity}"),
        format!("fragment:{fragment_identity}"),
        format!("source-loop:{source_loop_identity}"),
        format!("source-face:{source_face_identity}"),
        format!("source-edge:{source_edge_identity}"),
        format!("carrier:{carrier_identity}"),
        format!("source-loop-carrier:{source_loop_carrier_identity}"),
        format!(
            "endpoint-role:{}",
            endpoint_role_name(fragment_endpoint_role)
        ),
        format!("source-sense:{}", source_sense_name(source_sense)),
        format!("endpoint-parameter-bits:{endpoint_parameter_bits}"),
        format!("fragment-start-bits:{}", fragment_parameter_range_bits[0]),
        format!("fragment-end-bits:{}", fragment_parameter_range_bits[1]),
        format!("local-frame:{local_frame_identity}"),
        format!("precision-basis:{precision_basis_identity}"),
    ];
    parts.extend(
        event_group_identities
            .iter()
            .map(|identity| format!("event-group:{identity}")),
    );
    parts.extend(
        boundary_roles
            .iter()
            .map(|role| format!("boundary-role:{}", boundary_role_name(*role))),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(crate) fn continuation_index_identity(
    request_identity: &str,
    provenance_bundle_identity: &str,
    rows: &[PlanarBooleanFragmentContinuationRow],
) -> String {
    let mut parts = vec![
        "planar-boolean-fragment-continuation-index".to_string(),
        format!("request:{request_identity}"),
        format!("source-provenance:{provenance_bundle_identity}"),
    ];
    parts.extend(
        rows.iter()
            .map(|row| format!("continuation:{}", row.continuation_identity())),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(crate) fn continuation_ordering_basis_identity(
    request_identity: &str,
    continuation_index_identity: &str,
    ordered_continuation_identities: &[String],
) -> String {
    let mut parts = vec![
        "planar-boolean-continuation-ordering-basis".to_string(),
        format!("request:{request_identity}"),
        format!("continuation-index:{continuation_index_identity}"),
    ];
    parts.extend(
        ordered_continuation_identities
            .iter()
            .map(|identity| format!("ordered:{identity}")),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(crate) fn ordering_basis_from_identities(
    request_identity: &str,
    continuation_index_identity: &str,
    ordered_continuation_identities: Vec<String>,
) -> PlanarBooleanContinuationOrderingBasis {
    let basis_identity = continuation_ordering_basis_identity(
        request_identity,
        continuation_index_identity,
        &ordered_continuation_identities,
    );
    PlanarBooleanContinuationOrderingBasis::new(
        basis_identity,
        request_identity.to_string(),
        continuation_index_identity.to_string(),
        ordered_continuation_identities,
    )
}

fn endpoint_role_name(role: PlanarBooleanFragmentContinuationEndpointRole) -> &'static str {
    match role {
        PlanarBooleanFragmentContinuationEndpointRole::Start => "start",
        PlanarBooleanFragmentContinuationEndpointRole::End => "end",
    }
}

fn source_sense_name(sense: PlanarBooleanSourceIntervalSense) -> &'static str {
    match sense {
        PlanarBooleanSourceIntervalSense::Forward => "forward",
        PlanarBooleanSourceIntervalSense::Reversed => "reversed",
    }
}

fn boundary_role_name(
    role: crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanOverlapChainBoundaryRole,
) -> &'static str {
    match role {
        crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanOverlapChainBoundaryRole::FullOverlapSpan => "full-overlap-span",
        crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanOverlapChainBoundaryRole::OverlapStartBoundary => "overlap-start-boundary",
        crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanOverlapChainBoundaryRole::OverlapInteriorFragment => "overlap-interior-fragment",
        crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanOverlapChainBoundaryRole::OverlapEndBoundary => "overlap-end-boundary",
    }
}
