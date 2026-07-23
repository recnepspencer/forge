use crate::declaration::{stable_text_digest, UiDeclaredMeasurementConstraintModifier};
use crate::evidence::{
    UiAllocationConstraintSummary, UiAllocationNeighborhood, UiAllocationNeighborhoodMemberRole,
    UiConstraintAvailableSpacePosture, UiConstraintAxisScope,
    UiConstraintEqualShareDistributionPolicy, UiConstraintEqualShareDistributionResult,
    UiConstraintEqualShareGroup, UiConstraintEqualShareMember, UiConstraintEqualSharePosture,
    UiConstraintEqualShareSolveOrder, UiConstraintPropagationDenial,
    UiConstraintPropagationDenialReason, UiConstraintPropagationEdge,
    UiConstraintPropagationEdgeFamily, UiConstraintPropagationEdgePayload,
    UiConstraintSiblingNegotiationFixedPointPolicy, UiConstraintSiblingNegotiationResult,
    UiMeasurementBasis, UiMeasurementValue,
};

pub(super) struct UiAdmittedEqualShareDistribution {
    result: Option<UiConstraintEqualShareDistributionResult>,
    edges: Vec<UiConstraintPropagationEdge>,
}

impl UiAdmittedEqualShareDistribution {
    pub(super) fn empty() -> Self {
        Self {
            result: None,
            edges: Vec::new(),
        }
    }

    pub(super) fn result(&self) -> Option<&UiConstraintEqualShareDistributionResult> {
        self.result.as_ref()
    }

    pub(super) fn into_edges(self) -> Vec<UiConstraintPropagationEdge> {
        self.edges
    }
}

pub(super) fn admit_equal_share_distribution(
    measurement_basis: &UiMeasurementBasis,
    neighborhood: &UiAllocationNeighborhood,
    summary: UiAllocationConstraintSummary,
    sibling_negotiation: Option<&UiConstraintSiblingNegotiationResult>,
    allowed_families: &[UiConstraintPropagationEdgeFamily],
) -> Result<UiAdmittedEqualShareDistribution, UiConstraintPropagationDenial> {
    if !allowed_families.contains(&UiConstraintPropagationEdgeFamily::EqualShareDistribution) {
        return Ok(UiAdmittedEqualShareDistribution::empty());
    }

    let Some(axis_scope) = equal_share_axis_scope(summary.equal_share_group()) else {
        return Ok(UiAdmittedEqualShareDistribution::empty());
    };

    let peers = neighborhood
        .members()
        .iter()
        .filter(|member| {
            !matches!(member.role(), UiAllocationNeighborhoodMemberRole::Root)
                && member.layout_participates()
        })
        .collect::<Vec<_>>();
    let policy = resolved_equal_share_policy(
        summary.equal_share_group(),
        peers.as_slice(),
        sibling_negotiation,
    );
    let posture = posture_for_equal_share(summary, axis_scope, peers.len());
    if denies_non_integral_distribution(
        measurement_basis,
        neighborhood,
        summary,
        axis_scope,
        policy,
        peers.as_slice(),
        sibling_negotiation,
    ) {
        return Err(equal_share_denial(
            UiConstraintPropagationDenialReason::ContradictoryEqualShareRequirements,
            non_integral_requirement_witness(summary, peers.as_slice(), sibling_negotiation),
            neighborhood,
        ));
    }

    let members = peers
        .iter()
        .enumerate()
        .map(|(index, member)| {
            UiConstraintEqualShareMember::new(
                member.identity_digest(),
                remainder_rank_for(policy, peers.len(), index),
                member.measurement_constraint_modifier(),
            )
        })
        .collect::<Vec<_>>();
    let result = UiConstraintEqualShareDistributionResult::new(
        neighborhood.identity().identity_digest(),
        summary.equal_share_group(),
        axis_scope,
        policy,
        UiConstraintEqualShareSolveOrder::AfterSiblingNegotiationBeforeBounds,
        posture,
        members,
    );
    let root_identity_digest = neighborhood
        .members()
        .iter()
        .find(|member| matches!(member.role(), UiAllocationNeighborhoodMemberRole::Root))
        .expect("allocation neighborhood must preserve a root member")
        .identity_digest();
    let edges = result
        .members()
        .iter()
        .map(|member| {
            UiConstraintPropagationEdge::new(
                UiConstraintPropagationEdgeFamily::EqualShareDistribution,
                root_identity_digest,
                member.member_identity_digest(),
                UiConstraintPropagationEdgePayload::EqualShareDistribution {
                    axis_scope,
                    policy,
                    group_identity_digest: result.group_identity_digest(),
                    distribution_identity_digest: result.identity_digest(),
                    solve_order: result.solve_order(),
                    posture: result.posture(),
                },
                crate::evidence::UiConstraintCycleParticipationPosture::Acyclic,
            )
        })
        .collect::<Vec<_>>();

    Ok(UiAdmittedEqualShareDistribution {
        result: Some(result),
        edges,
    })
}

fn equal_share_axis_scope(group: UiConstraintEqualShareGroup) -> Option<UiConstraintAxisScope> {
    match group {
        UiConstraintEqualShareGroup::None => None,
        UiConstraintEqualShareGroup::StablePeerPrimaryAxis => Some(UiConstraintAxisScope::Primary),
        UiConstraintEqualShareGroup::StablePeerTwoDimensional => Some(UiConstraintAxisScope::Both),
    }
}

fn resolved_equal_share_policy(
    group: UiConstraintEqualShareGroup,
    peers: &[&crate::evidence::UiAllocationNeighborhoodMember],
    sibling_negotiation: Option<&UiConstraintSiblingNegotiationResult>,
) -> UiConstraintEqualShareDistributionPolicy {
    match group {
        UiConstraintEqualShareGroup::None => UiConstraintEqualShareDistributionPolicy::ExactFractional,
        UiConstraintEqualShareGroup::StablePeerTwoDimensional => {
            UiConstraintEqualShareDistributionPolicy::DeterministicRemainderLeftToRightByStablePeerIdentity
        }
        UiConstraintEqualShareGroup::StablePeerPrimaryAxis => {
            if integral_distribution_required(peers, sibling_negotiation) {
                UiConstraintEqualShareDistributionPolicy::DenyIfNonIntegralRequired
            } else {
                UiConstraintEqualShareDistributionPolicy::ExactFractional
            }
        }
    }
}

fn posture_for_equal_share(
    summary: UiAllocationConstraintSummary,
    axis_scope: UiConstraintAxisScope,
    peer_count: usize,
) -> UiConstraintEqualSharePosture {
    if peer_count == 0 {
        return UiConstraintEqualSharePosture::ZeroShare;
    }
    if peer_count == 1 {
        return UiConstraintEqualSharePosture::SingleSurvivingPeer;
    }
    if !available_space_covers(summary.incoming_available_space(), axis_scope) {
        return UiConstraintEqualSharePosture::NoAdmittedAvailableSpace;
    }
    if summary.incoming_available_space_posture()
        == Some(UiConstraintAvailableSpacePosture::AdmittedZeroExtent)
    {
        return UiConstraintEqualSharePosture::ZeroAvailableSpace;
    }
    match summary.equal_share_group() {
        UiConstraintEqualShareGroup::StablePeerPrimaryAxis => {
            UiConstraintEqualSharePosture::ExactFractional
        }
        UiConstraintEqualShareGroup::StablePeerTwoDimensional => {
            UiConstraintEqualSharePosture::DeterministicRemainderApplied
        }
        UiConstraintEqualShareGroup::None => UiConstraintEqualSharePosture::ZeroShare,
    }
}

fn available_space_covers(
    admitted_scope: Option<UiConstraintAxisScope>,
    required_scope: UiConstraintAxisScope,
) -> bool {
    matches!(
        (admitted_scope, required_scope),
        (Some(UiConstraintAxisScope::Both), _)
            | (
                Some(UiConstraintAxisScope::Primary),
                UiConstraintAxisScope::Primary
            )
            | (
                Some(UiConstraintAxisScope::Cross),
                UiConstraintAxisScope::Cross
            )
    )
}

fn denies_non_integral_distribution(
    measurement_basis: &UiMeasurementBasis,
    neighborhood: &UiAllocationNeighborhood,
    summary: UiAllocationConstraintSummary,
    axis_scope: UiConstraintAxisScope,
    policy: UiConstraintEqualShareDistributionPolicy,
    peers: &[&crate::evidence::UiAllocationNeighborhoodMember],
    sibling_negotiation: Option<&UiConstraintSiblingNegotiationResult>,
) -> bool {
    if policy != UiConstraintEqualShareDistributionPolicy::DenyIfNonIntegralRequired
        || peers.len() < 2
        || !integral_distribution_required(peers, sibling_negotiation)
    {
        return false;
    }
    match summary.incoming_available_space_posture() {
        Some(UiConstraintAvailableSpacePosture::AdmittedPositiveExtent) => {
            fractional_remainder_required(
                measurement_basis,
                axis_scope,
                neighborhood
                    .layout_operator_planning_contract()
                    .semantics()
                    .primary_axis(),
                peers.len(),
            )
        }
        Some(UiConstraintAvailableSpacePosture::AdmittedZeroExtent) => false,
        Some(UiConstraintAvailableSpacePosture::DeclaredExtentUnknown) | None => true,
    }
}

fn remainder_rank_for(
    policy: UiConstraintEqualShareDistributionPolicy,
    peer_count: usize,
    index: usize,
) -> Option<u16> {
    match policy {
        UiConstraintEqualShareDistributionPolicy::DeterministicRemainderLeftToRightByStablePeerIdentity => {
            Some(index as u16)
        }
        UiConstraintEqualShareDistributionPolicy::DeterministicRemainderCenterOutByStablePeerIdentity => {
            Some(center_out_rank(peer_count, index))
        }
        UiConstraintEqualShareDistributionPolicy::ExactFractional
        | UiConstraintEqualShareDistributionPolicy::DenyIfNonIntegralRequired => None,
    }
}

fn equal_share_denial(
    reason: UiConstraintPropagationDenialReason,
    witness_digest: u64,
    neighborhood: &UiAllocationNeighborhood,
) -> UiConstraintPropagationDenial {
    UiConstraintPropagationDenial::new(
        reason,
        neighborhood.identity().identity_digest(),
        neighborhood.layout_operator_contract_identity_digest(),
        Some(UiConstraintPropagationEdgeFamily::EqualShareDistribution),
        witness_digest,
    )
}

fn integral_distribution_required(
    peers: &[&crate::evidence::UiAllocationNeighborhoodMember],
    sibling_negotiation: Option<&UiConstraintSiblingNegotiationResult>,
) -> bool {
    sibling_negotiation.is_some_and(|result| {
        result.fixed_point_policy()
            == UiConstraintSiblingNegotiationFixedPointPolicy::AdmittedStablePeerMutual
    }) && peers.iter().any(|member| {
        member.measurement_constraint_modifier()
            == Some(UiDeclaredMeasurementConstraintModifier::Bounded)
    })
}

fn fractional_remainder_required(
    measurement_basis: &UiMeasurementBasis,
    axis_scope: UiConstraintAxisScope,
    primary_axis: crate::evidence::UiLayoutOperatorPrimaryAxis,
    peer_count: usize,
) -> bool {
    if peer_count == 0 {
        return false;
    }
    measurement_basis
        .evidence_inputs()
        .iter()
        .filter_map(|input| input.as_host_measurement_result())
        .find_map(|result| extent_units_for(result.value(), axis_scope, primary_axis))
        .is_none_or(|units| units % peer_count != 0)
}

fn non_integral_requirement_witness(
    summary: UiAllocationConstraintSummary,
    peers: &[&crate::evidence::UiAllocationNeighborhoodMember],
    sibling_negotiation: Option<&UiConstraintSiblingNegotiationResult>,
) -> u64 {
    stable_text_digest("worth-ui.constraint-equal-share.non-integral-requirement")
        ^ axis_scope_tag(equal_share_axis_scope(summary.equal_share_group())).rotate_left(7)
        ^ (peers.len() as u64).rotate_left(13)
        ^ summary
            .incoming_available_space_posture()
            .map_or(0, available_space_posture_tag)
            .rotate_left(19)
        ^ sibling_negotiation
            .map_or(0, UiConstraintSiblingNegotiationResult::identity_digest)
            .rotate_left(23)
}

fn available_space_posture_tag(posture: UiConstraintAvailableSpacePosture) -> u64 {
    stable_text_digest(match posture {
        UiConstraintAvailableSpacePosture::DeclaredExtentUnknown => {
            "worth-ui.constraint-equal-share.available-space.unknown"
        }
        UiConstraintAvailableSpacePosture::AdmittedZeroExtent => {
            "worth-ui.constraint-equal-share.available-space.zero"
        }
        UiConstraintAvailableSpacePosture::AdmittedPositiveExtent => {
            "worth-ui.constraint-equal-share.available-space.positive"
        }
    })
}

fn center_out_rank(peer_count: usize, index: usize) -> u16 {
    let center = (peer_count.saturating_sub(1)) as i32 / 2;
    let distance = (index as i32 - center).unsigned_abs();
    ((distance as usize * peer_count) + index) as u16
}

fn extent_units_for(
    value: &UiMeasurementValue,
    axis_scope: UiConstraintAxisScope,
    primary_axis: crate::evidence::UiLayoutOperatorPrimaryAxis,
) -> Option<usize> {
    let (width, height) = match value {
        UiMeasurementValue::ViewportExtent(value) => (value.width, value.height),
        UiMeasurementValue::ScrollContainerViewport(value) => (value.width, value.height),
        _ => return None,
    };
    let units = match axis_scope {
        UiConstraintAxisScope::Primary => primary_extent_units(width, height, primary_axis),
        UiConstraintAxisScope::Cross => cross_extent_units(width, height, primary_axis)?,
        UiConstraintAxisScope::Both => width.min(height) as usize,
    };
    Some(units)
}

fn primary_extent_units(
    width: f32,
    height: f32,
    primary_axis: crate::evidence::UiLayoutOperatorPrimaryAxis,
) -> usize {
    match primary_axis {
        crate::evidence::UiLayoutOperatorPrimaryAxis::Horizontal => width as usize,
        crate::evidence::UiLayoutOperatorPrimaryAxis::Vertical
        | crate::evidence::UiLayoutOperatorPrimaryAxis::TwoDimensional
        | crate::evidence::UiLayoutOperatorPrimaryAxis::Layered
        | crate::evidence::UiLayoutOperatorPrimaryAxis::None => height as usize,
    }
}

fn cross_extent_units(
    width: f32,
    height: f32,
    primary_axis: crate::evidence::UiLayoutOperatorPrimaryAxis,
) -> Option<usize> {
    Some(match primary_axis {
        crate::evidence::UiLayoutOperatorPrimaryAxis::Horizontal => height as usize,
        crate::evidence::UiLayoutOperatorPrimaryAxis::Vertical => width as usize,
        crate::evidence::UiLayoutOperatorPrimaryAxis::TwoDimensional
        | crate::evidence::UiLayoutOperatorPrimaryAxis::Layered => width.min(height) as usize,
        crate::evidence::UiLayoutOperatorPrimaryAxis::None => return None,
    })
}

fn axis_scope_tag(axis_scope: Option<UiConstraintAxisScope>) -> u64 {
    stable_text_digest(match axis_scope {
        Some(UiConstraintAxisScope::Primary) => "worth-ui.constraint-axis.primary",
        Some(UiConstraintAxisScope::Cross) => "worth-ui.constraint-axis.cross",
        Some(UiConstraintAxisScope::Both) => "worth-ui.constraint-axis.both",
        None => "worth-ui.constraint-axis.none",
    })
}

#[cfg(test)]
#[path = "constraint_equal_share_distribution_tests.rs"]
mod tests;
