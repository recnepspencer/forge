use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::endpoint_ref::{
    PlanarBooleanSplitEdgeFragmentEndpointKind, PlanarBooleanSplitEdgeFragmentEndpointRef,
};
use super::fragment_row::PlanarBooleanSplitEdgeFragment;
use super::fragment_set::PlanarBooleanSplitEdgeFragmentSchedule;

#[allow(clippy::too_many_arguments)]
pub(super) fn split_edge_fragment_identity(
    source_edge_identity: &str,
    carrier_identity: &str,
    start_endpoint: &PlanarBooleanSplitEdgeFragmentEndpointRef,
    end_endpoint: &PlanarBooleanSplitEdgeFragmentEndpointRef,
    parameter_range_bits: [u64; 2],
    local_frame_identity: &str,
    precision_basis_identity: &str,
    cause_identities: &[String],
) -> String {
    let mut parts = vec![
        "planar-boolean-split-edge-fragment".to_string(),
        format!("source-edge:{source_edge_identity}"),
        format!("carrier:{carrier_identity}"),
        format!("start:{}", endpoint_identity_part(start_endpoint)),
        format!("end:{}", endpoint_identity_part(end_endpoint)),
        format!("range-start-bits:{}", parameter_range_bits[0]),
        format!("range-end-bits:{}", parameter_range_bits[1]),
        format!("frame:{local_frame_identity}"),
        format!("precision:{precision_basis_identity}"),
    ];
    parts.extend(
        cause_identities
            .iter()
            .map(|identity| format!("cause:{identity}")),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(super) fn split_edge_fragment_schedule_identity(
    interval_subdivision_schedule_identity: &str,
    split_vertex_identity_schedule_identity: &str,
    fragments: &[PlanarBooleanSplitEdgeFragment],
) -> String {
    let mut parts = vec![
        "planar-boolean-split-edge-fragment-schedule".to_string(),
        format!("interval-subdivision-schedule:{interval_subdivision_schedule_identity}"),
        format!("split-vertex-schedule:{split_vertex_identity_schedule_identity}"),
    ];
    parts.extend(
        fragments
            .iter()
            .map(|fragment| format!("fragment:{}", fragment.fragment_identity())),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(super) fn split_edge_fragment_set_identity(
    interval_subdivision_schedule_set_identity: &str,
    split_vertex_identity_set_identity: &str,
    schedules: &[PlanarBooleanSplitEdgeFragmentSchedule],
) -> String {
    let mut parts = vec![
        "planar-boolean-split-edge-fragment-set".to_string(),
        format!("interval-subdivision-schedule-set:{interval_subdivision_schedule_set_identity}"),
        format!("split-vertex-set:{split_vertex_identity_set_identity}"),
    ];
    parts.extend(
        schedules
            .iter()
            .map(|schedule| format!("schedule:{}", schedule.schedule_identity())),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

fn endpoint_identity_part(endpoint: &PlanarBooleanSplitEdgeFragmentEndpointRef) -> String {
    format!(
        "{}:{}:{}",
        endpoint_kind_name(endpoint.endpoint_kind()),
        endpoint.endpoint_identity(),
        endpoint.parameter_bits()
    )
}

fn endpoint_kind_name(kind: PlanarBooleanSplitEdgeFragmentEndpointKind) -> &'static str {
    match kind {
        PlanarBooleanSplitEdgeFragmentEndpointKind::OriginalSourceStart => "original-start",
        PlanarBooleanSplitEdgeFragmentEndpointKind::SplitVertex => "split-vertex",
        PlanarBooleanSplitEdgeFragmentEndpointKind::OriginalSourceEnd => "original-end",
    }
}
