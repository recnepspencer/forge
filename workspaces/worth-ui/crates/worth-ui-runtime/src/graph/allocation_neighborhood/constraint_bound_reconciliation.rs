use crate::evidence::{
    UiAllocationConstraintSummary, UiAllocationNeighborhood, UiAllocationNeighborhoodMemberRole,
    UiBoundReconciliationPosture, UiBoundReconciliationSolveOrder,
    UiConstraintAvailableSpacePosture, UiConstraintAxisScope,
    UiConstraintBoundReconciliationMember, UiConstraintBoundReconciliationResult,
    UiConstraintBoundedMinMaxRequirement, UiConstraintCycleParticipationPosture,
    UiConstraintEqualShareDistributionResult, UiConstraintPropagationEdge,
    UiConstraintPropagationEdgeFamily, UiConstraintPropagationEdgePayload,
    UiConstraintSiblingNegotiationFixedPointPolicy, UiConstraintSiblingNegotiationResult,
    UiMeasurementBasis, UiMeasurementRoundingPosture, UiMeasurementUnitPosture,
};

pub(super) struct UiAdmittedBoundReconciliation {
    result: Option<UiConstraintBoundReconciliationResult>,
    edges: Vec<UiConstraintPropagationEdge>,
}

impl UiAdmittedBoundReconciliation {
    pub(super) fn empty() -> Self {
        Self {
            result: None,
            edges: Vec::new(),
        }
    }

    pub(super) fn result(&self) -> Option<&UiConstraintBoundReconciliationResult> {
        self.result.as_ref()
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        Option<UiConstraintBoundReconciliationResult>,
        Vec<UiConstraintPropagationEdge>,
    ) {
        (self.result, self.edges)
    }
}

pub(super) fn admit_bound_reconciliation(
    measurement_basis: &UiMeasurementBasis,
    neighborhood: &UiAllocationNeighborhood,
    summary: UiAllocationConstraintSummary,
    sibling_negotiation: Option<&UiConstraintSiblingNegotiationResult>,
    equal_share_distribution: Option<&UiConstraintEqualShareDistributionResult>,
    downward_bounded_targets: &[(u64, UiConstraintAxisScope)],
    allowed_families: &[UiConstraintPropagationEdgeFamily],
) -> UiAdmittedBoundReconciliation {
    if !allowed_families.contains(&UiConstraintPropagationEdgeFamily::BoundedReconciliation) {
        return UiAdmittedBoundReconciliation::empty();
    }

    let special_only = has_unsupported_special_input(summary);
    let Some(axis_scope) = bounded_axis_scope(summary.bounded_min_max_requirements())
        .or_else(|| special_only.then(|| summary.incoming_available_space().unwrap_or(UiConstraintAxisScope::Both)))
    else {
        return UiAdmittedBoundReconciliation::empty();
    };

    let members = bounded_members(neighborhood, summary.bounded_min_max_requirements());
    let mixed_bounded_participation = has_mixed_bounded_participation(
        neighborhood,
        summary.bounded_min_max_requirements(),
        members.len(),
    );
    let posture = resolve_bound_posture(
        measurement_basis,
        summary,
        sibling_negotiation,
        equal_share_distribution,
        downward_bounded_targets,
        members.len(),
        axis_scope,
        mixed_bounded_participation,
    );
    let result = UiConstraintBoundReconciliationResult::new(
        neighborhood.identity().identity_digest(),
        axis_scope,
        summary.bounded_min_max_requirements(),
        UiBoundReconciliationSolveOrder::AfterEqualShareBeforePlanCloseout,
        posture,
        summary.incoming_available_space_posture(),
        summary.viewport_requirement(),
        summary.scroll_owner_requirement(),
        summary.portal_anchor_requirement(),
        summary.unit_posture(),
        summary.coordinate_space(),
        summary.rounding_posture(),
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
                UiConstraintPropagationEdgeFamily::BoundedReconciliation,
                root_identity_digest,
                member.member_identity_digest(),
                UiConstraintPropagationEdgePayload::BoundedReconciliation {
                    axis_scope,
                    reconciliation_identity_digest: result.identity_digest(),
                    solve_order: result.solve_order(),
                    posture: result.posture(),
                },
                UiConstraintCycleParticipationPosture::Acyclic,
            )
        })
        .collect();

    UiAdmittedBoundReconciliation {
        result: Some(result),
        edges,
    }
}

fn bounded_members(
    neighborhood: &UiAllocationNeighborhood,
    bounded_requirement: UiConstraintBoundedMinMaxRequirement,
) -> Vec<UiConstraintBoundReconciliationMember> {
    neighborhood
        .members()
        .iter()
        .filter(|member| {
            !matches!(member.role(), UiAllocationNeighborhoodMemberRole::Root)
                && member.layout_participates()
                && member.measurement_constraint_modifier().is_some()
        })
        .map(|member| {
            UiConstraintBoundReconciliationMember::new(
                member.identity_digest(),
                bounded_requirement,
                member.measurement_constraint_modifier(),
            )
        })
        .collect()
}

fn resolve_bound_posture(
    measurement_basis: &UiMeasurementBasis,
    summary: UiAllocationConstraintSummary,
    sibling_negotiation: Option<&UiConstraintSiblingNegotiationResult>,
    equal_share_distribution: Option<&UiConstraintEqualShareDistributionResult>,
    downward_bounded_targets: &[(u64, UiConstraintAxisScope)],
    member_count: usize,
    axis_scope: UiConstraintAxisScope,
    mixed_bounded_participation: bool,
) -> UiBoundReconciliationPosture {
    if !measurement_basis.generation_compatibility().is_compatible() {
        return UiBoundReconciliationPosture::StaleInput;
    }
    if has_unsupported_special_input(summary) {
        return UiBoundReconciliationPosture::UnsupportedSpecialInput;
    }
    if unsupported_unit_mix(summary.unit_posture()) {
        return UiBoundReconciliationPosture::UnsupportedUnitMix;
    }
    if unsupported_rounding_mix(summary.rounding_posture()) {
        return UiBoundReconciliationPosture::UnsupportedRoundingMix;
    }
    if mixed_bounded_participation {
        return UiBoundReconciliationPosture::ContradictoryMinMax;
    }
    if member_count == 0 || downward_bounded_targets.is_empty() {
        return UiBoundReconciliationPosture::Underconstrained;
    }
    if has_axis_scope_mismatch(downward_bounded_targets, axis_scope) {
        return UiBoundReconciliationPosture::ContradictoryMinMax;
    }
    if summary.incoming_available_space().is_none()
        || summary.incoming_available_space_posture()
            == Some(UiConstraintAvailableSpacePosture::DeclaredExtentUnknown)
    {
        return UiBoundReconciliationPosture::Underconstrained;
    }
    if sibling_negotiation.is_some_and(|result| {
        result.fixed_point_policy()
            == UiConstraintSiblingNegotiationFixedPointPolicy::AdmittedStablePeerMutual
            && equal_share_distribution.is_none()
    }) {
        return UiBoundReconciliationPosture::Overconstrained;
    }
    if equal_share_distribution.is_some_and(|result| {
        matches!(
            result.posture(),
            crate::evidence::UiConstraintEqualSharePosture::ZeroShare
                | crate::evidence::UiConstraintEqualSharePosture::NoAdmittedAvailableSpace
        )
    }) {
        return UiBoundReconciliationPosture::Overconstrained;
    }
    if summary.incoming_available_space_posture()
        == Some(UiConstraintAvailableSpacePosture::AdmittedZeroExtent)
        && summary.bounded_min_max_requirements() != UiConstraintBoundedMinMaxRequirement::None
    {
        return UiBoundReconciliationPosture::Overconstrained;
    }
    if summary.incoming_available_space_posture()
        == Some(UiConstraintAvailableSpacePosture::AdmittedPositiveExtent)
        && summary.bounded_min_max_requirements() != UiConstraintBoundedMinMaxRequirement::None
    {
        return UiBoundReconciliationPosture::SatisfiedWithDeclaredClamp;
    }
    UiBoundReconciliationPosture::SatisfiedWithoutClamp
}

fn bounded_axis_scope(
    requirement: UiConstraintBoundedMinMaxRequirement,
) -> Option<UiConstraintAxisScope> {
    match requirement {
        UiConstraintBoundedMinMaxRequirement::None => None,
        UiConstraintBoundedMinMaxRequirement::PrimaryAxis => Some(UiConstraintAxisScope::Primary),
        UiConstraintBoundedMinMaxRequirement::BothAxes => Some(UiConstraintAxisScope::Both),
    }
}

fn has_unsupported_special_input(summary: UiAllocationConstraintSummary) -> bool {
    summary.portal_anchor_requirement() == crate::evidence::UiConstraintSpecialInputPosture::Required
        || ((summary.viewport_requirement()
            == crate::evidence::UiConstraintSpecialInputPosture::Required
            || summary.scroll_owner_requirement()
                == crate::evidence::UiConstraintSpecialInputPosture::Required)
            && summary.incoming_available_space().is_none())
}

fn unsupported_unit_mix(unit_posture: Option<UiMeasurementUnitPosture>) -> bool {
    matches!(
        unit_posture,
        Some(UiMeasurementUnitPosture::PhysicalPx | UiMeasurementUnitPosture::UnitlessScale)
    )
}

fn unsupported_rounding_mix(rounding_posture: Option<UiMeasurementRoundingPosture>) -> bool {
    matches!(
        rounding_posture,
        Some(
            UiMeasurementRoundingPosture::HostRounded
                | UiMeasurementRoundingPosture::DeferredToAllocation
        )
    )
}

fn has_axis_scope_mismatch(
    downward_bounded_targets: &[(u64, UiConstraintAxisScope)],
    required_scope: UiConstraintAxisScope,
) -> bool {
    downward_bounded_targets
        .iter()
        .any(|(_, target_scope)| *target_scope != required_scope)
}

fn has_mixed_bounded_participation(
    neighborhood: &UiAllocationNeighborhood,
    bounded_requirement: UiConstraintBoundedMinMaxRequirement,
    bounded_member_count: usize,
) -> bool {
    bounded_requirement != UiConstraintBoundedMinMaxRequirement::None
        && bounded_member_count > 0
        && neighborhood.members().iter().any(|member| {
            !matches!(member.role(), UiAllocationNeighborhoodMemberRole::Root)
                && member.layout_participates()
                && member.measurement_constraint_modifier().is_none()
        })
}
