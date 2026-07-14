use crate::declaration::stable_text_digest;
use crate::evidence::{
    UiAllocationConstraintSummary, UiAllocationNeighborhood, UiAllocationNeighborhoodMemberRole,
    UiConstraintAxisScope, UiConstraintCycleParticipationPosture, UiConstraintPropagationDenial,
    UiConstraintPropagationDenialReason, UiConstraintPropagationEdge,
    UiConstraintPropagationEdgeFamily, UiConstraintPropagationEdgePayload,
    UiConstraintSiblingNegotiationFixedPointPolicy, UiConstraintSiblingNegotiationGroup,
    UiConstraintSiblingNegotiationMember, UiConstraintSiblingNegotiationMode,
    UiConstraintSiblingNegotiationResult, UiConstraintSiblingNegotiationSolveOrder,
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
pub(super) struct UiSiblingNegotiationPeerContract {
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

mod helpers;
use helpers::*;
