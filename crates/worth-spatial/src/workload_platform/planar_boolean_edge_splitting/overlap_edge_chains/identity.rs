use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::workload_platform::planar_boolean_edge_splitting::canonical_parameter::canonical_parameter_bits;
use crate::workload_platform::planar_boolean_events::{
    PlanarBooleanIntervalEventKind, PlanarBooleanSourceIntervalSense,
};

use super::boundary_role::{
    PlanarBooleanOverlapChainBoundaryRole, PlanarBooleanOverlapChainPosture,
};
use super::chain_member::PlanarBooleanOverlapEdgeChainMember;
use super::chain_row::PlanarBooleanOverlapEdgeChain;

pub(super) fn overlap_chain_member_identity(
    interval_event_identity: &str,
    interval_subdivision_identity: &str,
    fragment_identity: &str,
    source_sense: PlanarBooleanSourceIntervalSense,
    role: PlanarBooleanOverlapChainBoundaryRole,
    fragment_range: [f64; 2],
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-overlap-edge-chain-member".to_string(),
            format!("interval-event:{interval_event_identity}"),
            format!("subdivision:{interval_subdivision_identity}"),
            format!("fragment:{fragment_identity}"),
            format!("source-sense:{}", source_sense_name(source_sense)),
            format!("role:{}", boundary_role_name(role)),
            format!(
                "fragment-start:{}",
                canonical_parameter_bits(fragment_range[0])
            ),
            format!(
                "fragment-end:{}",
                canonical_parameter_bits(fragment_range[1])
            ),
        ],
    )
}

pub(super) fn overlap_chain_identity(
    interval_event_identity: &str,
    interval_event_kind: PlanarBooleanIntervalEventKind,
    posture: PlanarBooleanOverlapChainPosture,
    members: &[PlanarBooleanOverlapEdgeChainMember],
) -> String {
    let mut parts = vec![
        "planar-boolean-overlap-edge-chain".to_string(),
        format!("interval-event:{interval_event_identity}"),
        format!("interval-kind:{}", interval_kind_name(interval_event_kind)),
        format!("posture:{}", posture_name(posture)),
    ];
    parts.extend(
        members
            .iter()
            .map(|member| format!("member:{}", member.member_identity())),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(super) fn overlap_chain_set_identity(
    interval_schedule_set_identity: &str,
    fragment_set_identity: &str,
    chains: &[PlanarBooleanOverlapEdgeChain],
) -> String {
    let mut parts = vec![
        "planar-boolean-overlap-edge-chain-set".to_string(),
        format!("interval-schedule-set:{interval_schedule_set_identity}"),
        format!("fragment-set:{fragment_set_identity}"),
    ];
    parts.extend(
        chains
            .iter()
            .map(|chain| format!("chain:{}", chain.chain_identity())),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(super) fn source_sense_name(sense: PlanarBooleanSourceIntervalSense) -> &'static str {
    match sense {
        PlanarBooleanSourceIntervalSense::Forward => "forward",
        PlanarBooleanSourceIntervalSense::Reversed => "reversed",
    }
}

pub(super) fn interval_kind_name(kind: PlanarBooleanIntervalEventKind) -> &'static str {
    match kind {
        PlanarBooleanIntervalEventKind::PartialOverlap => "partial-overlap",
        PlanarBooleanIntervalEventKind::ContainmentOverlap => "containment-overlap",
        PlanarBooleanIntervalEventKind::IdenticalSameDirection => "identical-same-direction",
        PlanarBooleanIntervalEventKind::IdenticalAntiParallel => "identical-antiparallel",
    }
}

fn boundary_role_name(role: PlanarBooleanOverlapChainBoundaryRole) -> &'static str {
    match role {
        PlanarBooleanOverlapChainBoundaryRole::FullOverlapSpan => "full-overlap-span",
        PlanarBooleanOverlapChainBoundaryRole::OverlapStartBoundary => "overlap-start-boundary",
        PlanarBooleanOverlapChainBoundaryRole::OverlapInteriorFragment => {
            "overlap-interior-fragment"
        }
        PlanarBooleanOverlapChainBoundaryRole::OverlapEndBoundary => "overlap-end-boundary",
    }
}

fn posture_name(posture: PlanarBooleanOverlapChainPosture) -> &'static str {
    match posture {
        PlanarBooleanOverlapChainPosture::PartialOverlap => "partial-overlap",
        PlanarBooleanOverlapChainPosture::IdenticalParallel => "identical-parallel",
        PlanarBooleanOverlapChainPosture::IdenticalAntiParallel => "identical-antiparallel",
        PlanarBooleanOverlapChainPosture::DifferentParameterization => "different-parameterization",
    }
}
