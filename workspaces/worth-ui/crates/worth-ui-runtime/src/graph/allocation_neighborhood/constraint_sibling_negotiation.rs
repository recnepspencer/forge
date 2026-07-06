use std::collections::BTreeMap;

use crate::declaration::stable_text_digest;
use crate::declaration::UiDeclaredMeasurementConstraintModifier;
use crate::evidence::{
    UiAllocationConstraintSummary, UiAllocationNeighborhood, UiAllocationNeighborhoodMemberRole,
    UiConstraintAxisScope, UiConstraintChildIntrinsicContribution,
    UiConstraintCycleParticipationPosture, UiConstraintPropagationDenial,
    UiConstraintPropagationDenialReason, UiConstraintPropagationEdge,
    UiConstraintPropagationEdgeFamily, UiConstraintPropagationEdgePayload,
    UiConstraintResizeInputPosture, UiConstraintSiblingNegotiationFixedPointPolicy,
    UiConstraintSiblingNegotiationGroup, UiConstraintSiblingNegotiationMember,
    UiConstraintSiblingNegotiationMode, UiConstraintSiblingNegotiationResult,
    UiConstraintSiblingNegotiationSolveOrder, UiLayoutOperatorCrossAxis,
    UiLayoutOperatorPlanningContract,
};

use super::constraint_summary::axis_scope_for_sibling_mode;

pub(super) struct UiAdmittedSiblingNegotiation {
    result: Option<UiConstraintSiblingNegotiationResult>,
    edges: Vec<UiConstraintPropagationEdge>,
}

impl UiAdmittedSiblingNegotiation {
    pub(super) fn empty() -> Self {
        Self {
            result: None,
            edges: Vec::new(),
        }
    }

    pub(super) fn result(&self) -> Option<&UiConstraintSiblingNegotiationResult> {
        self.result.as_ref()
    }

    pub(super) fn into_edges(self) -> Vec<UiConstraintPropagationEdge> {
        self.edges
    }
}

pub(super) fn admit_sibling_negotiation(
    neighborhood: &UiAllocationNeighborhood,
    summary: UiAllocationConstraintSummary,
    admitted_edges: &[UiConstraintPropagationEdge],
    allowed_families: &[UiConstraintPropagationEdgeFamily],
) -> Result<UiAdmittedSiblingNegotiation, UiConstraintPropagationDenial> {
    if !allowed_families.contains(&UiConstraintPropagationEdgeFamily::SiblingNegotiation) {
        return Ok(UiAdmittedSiblingNegotiation::empty());
    }

    let Some(axis_scope) = axis_scope_for_sibling_mode(summary.sibling_negotiation_mode()) else {
        return Ok(UiAdmittedSiblingNegotiation::empty());
    };

    let peers = neighborhood
        .members()
        .iter()
        .filter(|member| !matches!(member.role(), UiAllocationNeighborhoodMemberRole::Root))
        .collect::<Vec<_>>();
    if peers.len() < 2 {
        return Ok(UiAdmittedSiblingNegotiation::empty());
    }

    let group = UiConstraintSiblingNegotiationGroup::new(
        neighborhood.identity().identity_digest(),
        summary.sibling_negotiation_mode(),
        axis_scope,
        peers
            .iter()
            .map(|member| member.identity_digest())
            .collect(),
    );
    let intrinsic_by_member =
        admitted_intrinsic_by_member(neighborhood, admitted_edges, axis_scope)?;
    let peer_contracts = admit_peer_contracts(
        neighborhood,
        peers.as_slice(),
        &intrinsic_by_member,
        axis_scope,
    )?;
    let fixed_point_policy = fixed_point_policy(
        neighborhood,
        summary.sibling_negotiation_mode(),
        peer_contracts.as_slice(),
        admitted_edges,
        axis_scope,
    )?;
    let normalization = shared_normalization(neighborhood, intrinsic_by_member.values().copied())?;
    let members = peers
        .iter()
        .map(|member| {
            let intrinsic = intrinsic_by_member.get(&member.identity_digest()).copied();
            UiConstraintSiblingNegotiationMember::new(
                member.identity_digest(),
                member.measurement_constraint_modifier(),
                intrinsic.map(|contribution| contribution.identity_digest()),
                intrinsic.map(|contribution| contribution.source_posture()),
            )
        })
        .collect::<Vec<_>>();
    let result = UiConstraintSiblingNegotiationResult::new(
        group,
        fixed_point_policy,
        UiConstraintSiblingNegotiationSolveOrder::BeforeEqualShareAndBounds,
        normalization.map(|value| value.0),
        normalization.map(|value| value.1),
        normalization.map(|value| value.2),
        members,
    );
    let peer_digests = result.group().member_identity_digests();
    let mut edges = Vec::new();
    for left_index in 0..peer_digests.len() {
        for right_index in (left_index + 1)..peer_digests.len() {
            edges.push(UiConstraintPropagationEdge::new(
                UiConstraintPropagationEdgeFamily::SiblingNegotiation,
                peer_digests[left_index],
                peer_digests[right_index],
                UiConstraintPropagationEdgePayload::SiblingNegotiation {
                    axis_scope,
                    group_identity_digest: result.group().identity_digest(),
                    negotiation_identity_digest: result.identity_digest(),
                    fixed_point_policy: result.fixed_point_policy(),
                    solve_order: result.solve_order(),
                },
                UiConstraintCycleParticipationPosture::Acyclic,
            ));
        }
    }

    Ok(UiAdmittedSiblingNegotiation {
        result: Some(result),
        edges,
    })
}

fn fixed_point_policy(
    neighborhood: &UiAllocationNeighborhood,
    mode: UiConstraintSiblingNegotiationMode,
    peer_contracts: &[UiSiblingNegotiationPeerContract],
    admitted_edges: &[UiConstraintPropagationEdge],
    axis_scope: UiConstraintAxisScope,
) -> Result<UiConstraintSiblingNegotiationFixedPointPolicy, UiConstraintPropagationDenial> {
    match mode {
        UiConstraintSiblingNegotiationMode::None => {
            Ok(UiConstraintSiblingNegotiationFixedPointPolicy::NotRequired)
        }
        UiConstraintSiblingNegotiationMode::StablePeerPrimaryAxis => {
            let requires_mutual = peer_contracts.iter().any(|contract| {
                contract.bounded_axis_scope == Some(UiConstraintAxisScope::Primary)
            });
            if !requires_mutual {
                return Ok(UiConstraintSiblingNegotiationFixedPointPolicy::NotRequired);
            }
            if let Some(contract) = peer_contracts
                .iter()
                .find(|contract| !contract.supports_primary_axis_mutual())
            {
                return Err(sibling_denial(
                    UiConstraintPropagationDenialReason::ContradictorySiblingRequirements,
                    contract.identity_digest
                        ^ stable_text_digest(
                            "worth-ui.constraint-sibling-negotiation.primary-contradiction",
                        ),
                    neighborhood,
                ));
            }
            if !has_admitted_durable_resize_support(admitted_edges, axis_scope) {
                return Err(sibling_denial(
                    UiConstraintPropagationDenialReason::UnsupportedSiblingFixedPoint,
                    stable_text_digest(
                        "worth-ui.constraint-sibling-negotiation.primary-fixed-point-unsupported",
                    ),
                    neighborhood,
                ));
            }
            Ok(UiConstraintSiblingNegotiationFixedPointPolicy::AdmittedStablePeerMutual)
        }
        UiConstraintSiblingNegotiationMode::StablePeerTwoDimensional => {
            let enters_bidirectional = peer_contracts.iter().any(|contract| {
                contract.bounded_axis_scope == Some(UiConstraintAxisScope::Both)
                    || contract.intrinsic_axis_scope == Some(UiConstraintAxisScope::Both)
            });
            if !enters_bidirectional {
                return Ok(UiConstraintSiblingNegotiationFixedPointPolicy::NotRequired);
            }
            if let Some(contract) = peer_contracts
                .iter()
                .find(|contract| contract.requires_bidirectional_intrinsic())
            {
                return Err(sibling_denial(
                    UiConstraintPropagationDenialReason::ContradictorySiblingRequirements,
                    contract.identity_digest
                        ^ stable_text_digest(
                            "worth-ui.constraint-sibling-negotiation.bidirectional-contradiction",
                        ),
                    neighborhood,
                ));
            }
            if peer_contracts
                .iter()
                .any(|contract| contract.intrinsic_axis_scope != Some(UiConstraintAxisScope::Both))
            {
                return Err(sibling_denial(
                    UiConstraintPropagationDenialReason::UnsupportedSiblingFixedPoint,
                    stable_text_digest(
                        "worth-ui.constraint-sibling-negotiation.bidirectional-fixed-point-unsupported",
                    ),
                    neighborhood,
                ));
            }
            if !has_admitted_durable_resize_support(admitted_edges, axis_scope) {
                return Err(sibling_denial(
                    UiConstraintPropagationDenialReason::UnsupportedSiblingFixedPoint,
                    stable_text_digest(
                        "worth-ui.constraint-sibling-negotiation.bidirectional-durable-support-unsupported",
                    ),
                    neighborhood,
                ));
            }
            Ok(UiConstraintSiblingNegotiationFixedPointPolicy::AdmittedStablePeerMutual)
        }
    }
}

#[derive(Clone, Copy)]
struct UiSiblingNegotiationPeerContract {
    identity_digest: u64,
    bounded_axis_scope: Option<UiConstraintAxisScope>,
    intrinsic_axis_scope: Option<UiConstraintAxisScope>,
}

impl UiSiblingNegotiationPeerContract {
    fn supports_primary_axis_mutual(self) -> bool {
        self.bounded_axis_scope != Some(UiConstraintAxisScope::Primary)
            || matches!(
                self.intrinsic_axis_scope,
                Some(UiConstraintAxisScope::Primary | UiConstraintAxisScope::Both)
            )
    }

    fn requires_bidirectional_intrinsic(self) -> bool {
        self.bounded_axis_scope == Some(UiConstraintAxisScope::Both)
            && self.intrinsic_axis_scope != Some(UiConstraintAxisScope::Both)
    }
}

fn admit_peer_contracts(
    neighborhood: &UiAllocationNeighborhood,
    peers: &[&crate::evidence::UiAllocationNeighborhoodMember],
    intrinsic_by_member: &BTreeMap<u64, UiConstraintChildIntrinsicContribution>,
    axis_scope: UiConstraintAxisScope,
) -> Result<Vec<UiSiblingNegotiationPeerContract>, UiConstraintPropagationDenial> {
    let contract = neighborhood.layout_operator_planning_contract();
    let mut peer_contracts = Vec::with_capacity(peers.len());
    for member in peers {
        let intrinsic_axis_scope = intrinsic_by_member
            .get(&member.identity_digest())
            .map(UiConstraintChildIntrinsicContribution::axis_scope);
        let bounded_axis_scope =
            peer_bounded_axis_scope(member.measurement_constraint_modifier(), contract);
        if bounded_axis_scope.is_some() && intrinsic_axis_scope.is_none() {
            return Err(sibling_denial(
                UiConstraintPropagationDenialReason::ContradictorySiblingRequirements,
                member.identity_digest()
                    ^ stable_text_digest(
                        "worth-ui.constraint-sibling-negotiation.missing-bounded-intrinsic",
                    ),
                neighborhood,
            ));
        }
        if let Some(intrinsic_axis_scope) = intrinsic_axis_scope {
            if intrinsic_axis_scope != axis_scope {
                return Err(sibling_denial(
                    UiConstraintPropagationDenialReason::ContradictorySiblingRequirements,
                    member.identity_digest()
                        ^ stable_text_digest(
                            "worth-ui.constraint-sibling-negotiation.intrinsic-axis-contract-mismatch",
                        ),
                    neighborhood,
                ));
            }
        }
        peer_contracts.push(UiSiblingNegotiationPeerContract {
            identity_digest: member.identity_digest(),
            bounded_axis_scope,
            intrinsic_axis_scope,
        });
    }
    Ok(peer_contracts)
}

fn peer_bounded_axis_scope(
    child_constraint_modifier: Option<UiDeclaredMeasurementConstraintModifier>,
    contract: &UiLayoutOperatorPlanningContract,
) -> Option<UiConstraintAxisScope> {
    if child_constraint_modifier != Some(UiDeclaredMeasurementConstraintModifier::Bounded)
        || contract.constraint_modifier() != Some(UiDeclaredMeasurementConstraintModifier::Bounded)
    {
        return None;
    }

    Some(match contract.semantics().cross_axis() {
        UiLayoutOperatorCrossAxis::Horizontal | UiLayoutOperatorCrossAxis::Vertical => {
            UiConstraintAxisScope::Primary
        }
        UiLayoutOperatorCrossAxis::TwoDimensional
        | UiLayoutOperatorCrossAxis::Layered
        | UiLayoutOperatorCrossAxis::None => UiConstraintAxisScope::Both,
    })
}

fn has_admitted_durable_resize_support(
    admitted_edges: &[UiConstraintPropagationEdge],
    axis_scope: UiConstraintAxisScope,
) -> bool {
    admitted_edges.iter().any(|edge| {
        matches!(
            edge.payload(),
            UiConstraintPropagationEdgePayload::DurableResizeInput {
                durable_identity_digest: _,
                axis_scope: edge_axis_scope,
                posture: UiConstraintResizeInputPosture::DurableAuthorityRequired,
                planning_time_only: true,
            } if edge_axis_scope == axis_scope
        )
    })
}

fn admitted_intrinsic_by_member(
    neighborhood: &UiAllocationNeighborhood,
    intrinsic_edges: &[UiConstraintPropagationEdge],
    axis_scope: UiConstraintAxisScope,
) -> Result<BTreeMap<u64, UiConstraintChildIntrinsicContribution>, UiConstraintPropagationDenial> {
    let mut intrinsic_by_member = BTreeMap::new();
    let mut peer_graph_digests = neighborhood
        .members()
        .iter()
        .filter(|member| !matches!(member.role(), UiAllocationNeighborhoodMemberRole::Root))
        .map(|member| {
            (
                member.identity_digest(),
                member.graph_node_identity().digest(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    for edge in intrinsic_edges {
        let Some(contribution) = edge.payload().child_intrinsic_contribution() else {
            continue;
        };
        if contribution.axis_scope() != axis_scope {
            return Err(sibling_denial(
                UiConstraintPropagationDenialReason::ContradictorySiblingRequirements,
                contribution.identity_digest()
                    ^ stable_text_digest("worth-ui.constraint-sibling-negotiation.axis-scope"),
                neighborhood,
            ));
        }
        let member_identity_digest = edge.source_member_identity_digest();
        let Some(expected_graph_digest) = peer_graph_digests.remove(&member_identity_digest) else {
            return Err(sibling_denial(
                UiConstraintPropagationDenialReason::ContradictorySiblingRequirements,
                contribution.identity_digest()
                    ^ stable_text_digest("worth-ui.constraint-sibling-negotiation.unknown-peer"),
                neighborhood,
            ));
        };
        if contribution.contributor_graph_node_identity().digest() != expected_graph_digest {
            return Err(sibling_denial(
                UiConstraintPropagationDenialReason::ContradictorySiblingRequirements,
                contribution.identity_digest()
                    ^ stable_text_digest("worth-ui.constraint-sibling-negotiation.peer-mismatch"),
                neighborhood,
            ));
        }
        if intrinsic_by_member
            .insert(member_identity_digest, contribution)
            .is_some()
        {
            return Err(sibling_denial(
                UiConstraintPropagationDenialReason::ContradictorySiblingRequirements,
                contribution.identity_digest()
                    ^ stable_text_digest("worth-ui.constraint-sibling-negotiation.duplicate-peer"),
                neighborhood,
            ));
        }
    }
    Ok(intrinsic_by_member)
}

fn shared_normalization(
    neighborhood: &UiAllocationNeighborhood,
    contributions: impl Iterator<Item = UiConstraintChildIntrinsicContribution>,
) -> Result<
    Option<(
        crate::evidence::UiMeasurementUnitPosture,
        crate::evidence::UiMeasurementCoordinateSpace,
        crate::evidence::UiMeasurementRoundingPosture,
    )>,
    UiConstraintPropagationDenial,
> {
    let mut normalization = None;
    for contribution in contributions {
        let current = (
            contribution.unit_posture(),
            contribution.coordinate_space(),
            contribution.rounding_posture(),
        );
        match normalization {
            None => normalization = Some(current),
            Some(existing) if existing == current => {}
            Some(_) => {
                return Err(sibling_denial(
                    UiConstraintPropagationDenialReason::ContradictorySiblingRequirements,
                    contribution.identity_digest()
                        ^ stable_text_digest(
                            "worth-ui.constraint-sibling-negotiation.normalization-mismatch",
                        ),
                    neighborhood,
                ));
            }
        }
    }
    Ok(normalization)
}

fn sibling_denial(
    reason: UiConstraintPropagationDenialReason,
    witness_digest: u64,
    neighborhood: &UiAllocationNeighborhood,
) -> UiConstraintPropagationDenial {
    UiConstraintPropagationDenial::new(
        reason,
        neighborhood.identity().identity_digest(),
        neighborhood.layout_operator_contract_identity_digest(),
        Some(UiConstraintPropagationEdgeFamily::SiblingNegotiation),
        witness_digest,
    )
}
