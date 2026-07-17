use crate::declaration::stable_text_digest;

use crate::evidence::{
    UiConstraintAvailableSpacePosture, UiConstraintAxisScope,
    UiConstraintBoundReconciliationResult, UiConstraintEqualShareDistributionResult,
    UiConstraintPortalAnchorPlanningInputResult, UiConstraintPropagationEdge,
    UiConstraintScrollOwnerPlanningInputResult, UiConstraintSiblingNegotiationResult,
    UiConstraintViewportPlanningInputResult, UiLayoutOperatorContractIdentity,
    UiMeasurementCoordinateSpace, UiMeasurementRoundingPosture, UiMeasurementUnitPosture,
};

use super::digest::{
    available_space_posture_digest, axis_scope_digest, bounded_requirement_digest,
    coordinate_space_digest, equal_share_group_digest, resize_permission_posture_digest,
    rounding_posture_digest, sibling_negotiation_mode_digest, special_input_posture_digest,
    unit_posture_digest,
};
use super::identity::UiAllocationConstraintSetIdentity;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiConstraintSiblingNegotiationMode {
    None,
    StablePeerPrimaryAxis,
    StablePeerTwoDimensional,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiConstraintEqualShareGroup {
    None,
    StablePeerPrimaryAxis,
    StablePeerTwoDimensional,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiConstraintBoundedMinMaxRequirement {
    None,
    PrimaryAxis,
    BothAxes,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiConstraintSpecialInputPosture {
    NotRequired,
    Required,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiConstraintResizePermissionPosture {
    None,
    DurableAuthorityLane,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiAllocationConstraintSummary {
    incoming_available_space: Option<UiConstraintAxisScope>,
    incoming_available_space_posture: Option<UiConstraintAvailableSpacePosture>,
    intrinsic_contribution_requirements: Option<UiConstraintAxisScope>,
    sibling_negotiation_mode: UiConstraintSiblingNegotiationMode,
    equal_share_group: UiConstraintEqualShareGroup,
    bounded_min_max_requirements: UiConstraintBoundedMinMaxRequirement,
    viewport_requirement: UiConstraintSpecialInputPosture,
    scroll_owner_requirement: UiConstraintSpecialInputPosture,
    portal_anchor_requirement: UiConstraintSpecialInputPosture,
    resize_permission_posture: UiConstraintResizePermissionPosture,
    unit_posture: Option<UiMeasurementUnitPosture>,
    coordinate_space: Option<UiMeasurementCoordinateSpace>,
    rounding_posture: Option<UiMeasurementRoundingPosture>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiAllocationConstraintSet {
    identity: UiAllocationConstraintSetIdentity,
    neighborhood_identity_digest: u64,
    layout_operator_contract_identity: UiLayoutOperatorContractIdentity,
    summary: UiAllocationConstraintSummary,
    viewport_planning_input: Option<UiConstraintViewportPlanningInputResult>,
    scroll_owner_planning_input: Option<UiConstraintScrollOwnerPlanningInputResult>,
    portal_anchor_planning_input: Option<UiConstraintPortalAnchorPlanningInputResult>,
    sibling_negotiation: Option<UiConstraintSiblingNegotiationResult>,
    equal_share_distribution: Option<UiConstraintEqualShareDistributionResult>,
    bound_reconciliation: Option<UiConstraintBoundReconciliationResult>,
    propagation_edges: Box<[UiConstraintPropagationEdge]>,
}

pub(crate) struct UiAllocationConstraintSummaryInput {
    pub(crate) incoming_available_space: Option<UiConstraintAxisScope>,
    pub(crate) incoming_available_space_posture: Option<UiConstraintAvailableSpacePosture>,
    pub(crate) intrinsic_contribution_requirements: Option<UiConstraintAxisScope>,
    pub(crate) sibling_negotiation_mode: UiConstraintSiblingNegotiationMode,
    pub(crate) equal_share_group: UiConstraintEqualShareGroup,
    pub(crate) bounded_min_max_requirements: UiConstraintBoundedMinMaxRequirement,
    pub(crate) viewport_requirement: UiConstraintSpecialInputPosture,
    pub(crate) scroll_owner_requirement: UiConstraintSpecialInputPosture,
    pub(crate) portal_anchor_requirement: UiConstraintSpecialInputPosture,
    pub(crate) resize_permission_posture: UiConstraintResizePermissionPosture,
    pub(crate) unit_posture: Option<UiMeasurementUnitPosture>,
    pub(crate) coordinate_space: Option<UiMeasurementCoordinateSpace>,
    pub(crate) rounding_posture: Option<UiMeasurementRoundingPosture>,
}

pub(crate) struct UiAllocationConstraintSetInput {
    pub(crate) neighborhood_identity_digest: u64,
    pub(crate) layout_operator_contract_identity: UiLayoutOperatorContractIdentity,
    pub(crate) summary: UiAllocationConstraintSummary,
    pub(crate) viewport_planning_input: Option<UiConstraintViewportPlanningInputResult>,
    pub(crate) scroll_owner_planning_input: Option<UiConstraintScrollOwnerPlanningInputResult>,
    pub(crate) portal_anchor_planning_input: Option<UiConstraintPortalAnchorPlanningInputResult>,
    pub(crate) sibling_negotiation: Option<UiConstraintSiblingNegotiationResult>,
    pub(crate) equal_share_distribution: Option<UiConstraintEqualShareDistributionResult>,
    pub(crate) bound_reconciliation: Option<UiConstraintBoundReconciliationResult>,
    pub(crate) propagation_edges: Vec<UiConstraintPropagationEdge>,
}

impl UiAllocationConstraintSummary {
    pub(crate) const fn new(input: UiAllocationConstraintSummaryInput) -> Self {
        let UiAllocationConstraintSummaryInput {
            incoming_available_space,
            incoming_available_space_posture,
            intrinsic_contribution_requirements,
            sibling_negotiation_mode,
            equal_share_group,
            bounded_min_max_requirements,
            viewport_requirement,
            scroll_owner_requirement,
            portal_anchor_requirement,
            resize_permission_posture,
            unit_posture,
            coordinate_space,
            rounding_posture,
        } = input;
        Self {
            incoming_available_space,
            incoming_available_space_posture,
            intrinsic_contribution_requirements,
            sibling_negotiation_mode,
            equal_share_group,
            bounded_min_max_requirements,
            viewport_requirement,
            scroll_owner_requirement,
            portal_anchor_requirement,
            resize_permission_posture,
            unit_posture,
            coordinate_space,
            rounding_posture,
        }
    }

    pub fn incoming_available_space(&self) -> Option<UiConstraintAxisScope> {
        self.incoming_available_space
    }
    pub fn incoming_available_space_posture(&self) -> Option<UiConstraintAvailableSpacePosture> {
        self.incoming_available_space_posture
    }
    pub fn intrinsic_contribution_requirements(&self) -> Option<UiConstraintAxisScope> {
        self.intrinsic_contribution_requirements
    }
    pub fn sibling_negotiation_mode(&self) -> UiConstraintSiblingNegotiationMode {
        self.sibling_negotiation_mode
    }
    pub fn equal_share_group(&self) -> UiConstraintEqualShareGroup {
        self.equal_share_group
    }
    pub fn bounded_min_max_requirements(&self) -> UiConstraintBoundedMinMaxRequirement {
        self.bounded_min_max_requirements
    }
    pub fn viewport_requirement(&self) -> UiConstraintSpecialInputPosture {
        self.viewport_requirement
    }
    pub fn scroll_owner_requirement(&self) -> UiConstraintSpecialInputPosture {
        self.scroll_owner_requirement
    }
    pub fn portal_anchor_requirement(&self) -> UiConstraintSpecialInputPosture {
        self.portal_anchor_requirement
    }
    pub fn resize_permission_posture(&self) -> UiConstraintResizePermissionPosture {
        self.resize_permission_posture
    }
    pub fn unit_posture(&self) -> Option<UiMeasurementUnitPosture> {
        self.unit_posture
    }
    pub fn coordinate_space(&self) -> Option<UiMeasurementCoordinateSpace> {
        self.coordinate_space
    }
    pub fn rounding_posture(&self) -> Option<UiMeasurementRoundingPosture> {
        self.rounding_posture
    }

    pub(crate) fn identity_digest(&self) -> u64 {
        stable_text_digest("worth-ui.allocation-constraint-summary")
            ^ axis_scope_digest(self.incoming_available_space).rotate_left(7)
            ^ available_space_posture_digest(self.incoming_available_space_posture).rotate_left(11)
            ^ axis_scope_digest(self.intrinsic_contribution_requirements).rotate_left(13)
            ^ sibling_negotiation_mode_digest(self.sibling_negotiation_mode).rotate_left(17)
            ^ equal_share_group_digest(self.equal_share_group).rotate_left(19)
            ^ bounded_requirement_digest(self.bounded_min_max_requirements).rotate_left(23)
            ^ special_input_posture_digest(self.viewport_requirement).rotate_left(29)
            ^ special_input_posture_digest(self.scroll_owner_requirement).rotate_left(31)
            ^ special_input_posture_digest(self.portal_anchor_requirement).rotate_left(37)
            ^ resize_permission_posture_digest(self.resize_permission_posture).rotate_left(41)
            ^ unit_posture_digest(self.unit_posture).rotate_left(43)
            ^ coordinate_space_digest(self.coordinate_space).rotate_left(47)
            ^ rounding_posture_digest(self.rounding_posture).rotate_left(53)
    }
}

impl UiAllocationConstraintSet {
    #[cfg(test)]
    pub(crate) fn new(
        neighborhood_identity_digest: u64,
        layout_operator_contract_identity: UiLayoutOperatorContractIdentity,
        summary: UiAllocationConstraintSummary,
        propagation_edges: Vec<UiConstraintPropagationEdge>,
    ) -> Self {
        Self::construct(UiAllocationConstraintSetInput {
            neighborhood_identity_digest,
            layout_operator_contract_identity,
            summary,
            viewport_planning_input: None,
            scroll_owner_planning_input: None,
            portal_anchor_planning_input: None,
            sibling_negotiation: None,
            equal_share_distribution: None,
            bound_reconciliation: None,
            propagation_edges,
        })
    }

    pub(crate) fn new_with_sibling_negotiation(
        _: crate::graph::UiGraphConstraintMintAuthority,
        input: UiAllocationConstraintSetInput,
    ) -> Self {
        Self::construct(input)
    }

    fn construct(input: UiAllocationConstraintSetInput) -> Self {
        let UiAllocationConstraintSetInput {
            neighborhood_identity_digest,
            layout_operator_contract_identity,
            summary,
            viewport_planning_input,
            scroll_owner_planning_input,
            portal_anchor_planning_input,
            sibling_negotiation,
            equal_share_distribution,
            bound_reconciliation,
            mut propagation_edges,
        } = input;
        propagation_edges.sort_unstable_by_key(UiConstraintPropagationEdge::canonical_sort_key);
        let identity = UiAllocationConstraintSetIdentity::new(
            propagation_edges.iter().fold(
                stable_text_digest("worth-ui.allocation-constraint-set")
                    ^ neighborhood_identity_digest.rotate_left(7)
                    ^ layout_operator_contract_identity
                        .identity_digest()
                        .rotate_left(13)
                    ^ summary.identity_digest().rotate_left(19)
                    ^ viewport_planning_input
                        .as_ref()
                        .map_or(
                            stable_text_digest("worth-ui.allocation-constraint-set.no-viewport"),
                            UiConstraintViewportPlanningInputResult::identity_digest,
                        )
                        .rotate_left(21)
                    ^ scroll_owner_planning_input
                        .as_ref()
                        .map_or(
                            stable_text_digest(
                                "worth-ui.allocation-constraint-set.no-scroll-owner",
                            ),
                            UiConstraintScrollOwnerPlanningInputResult::identity_digest,
                        )
                        .rotate_left(22)
                    ^ portal_anchor_planning_input
                        .as_ref()
                        .map_or(
                            stable_text_digest(
                                "worth-ui.allocation-constraint-set.no-portal-anchor",
                            ),
                            UiConstraintPortalAnchorPlanningInputResult::identity_digest,
                        )
                        .rotate_left(23)
                    ^ sibling_negotiation
                        .as_ref()
                        .map_or(
                            stable_text_digest("worth-ui.allocation-constraint-set.no-sibling"),
                            UiConstraintSiblingNegotiationResult::identity_digest,
                        )
                        .rotate_left(24)
                    ^ equal_share_distribution
                        .as_ref()
                        .map_or(
                            stable_text_digest("worth-ui.allocation-constraint-set.no-equal-share"),
                            UiConstraintEqualShareDistributionResult::identity_digest,
                        )
                        .rotate_left(30)
                    ^ bound_reconciliation
                        .as_ref()
                        .map_or(
                            stable_text_digest("worth-ui.allocation-constraint-set.no-bounds"),
                            UiConstraintBoundReconciliationResult::identity_digest,
                        )
                        .rotate_left(32),
                |digest, edge| digest.rotate_left(11) ^ edge.identity_digest().rotate_left(17),
            ),
        );
        Self {
            identity,
            neighborhood_identity_digest,
            layout_operator_contract_identity,
            summary,
            viewport_planning_input,
            scroll_owner_planning_input,
            portal_anchor_planning_input,
            sibling_negotiation,
            equal_share_distribution,
            bound_reconciliation,
            propagation_edges: propagation_edges.into_boxed_slice(),
        }
    }

    pub fn identity(&self) -> UiAllocationConstraintSetIdentity {
        self.identity
    }

    pub fn neighborhood_identity_digest(&self) -> u64 {
        self.neighborhood_identity_digest
    }

    pub fn layout_operator_contract_identity(&self) -> UiLayoutOperatorContractIdentity {
        self.layout_operator_contract_identity
    }

    pub fn summary(&self) -> UiAllocationConstraintSummary {
        self.summary
    }

    pub fn sibling_negotiation(&self) -> Option<&UiConstraintSiblingNegotiationResult> {
        self.sibling_negotiation.as_ref()
    }

    pub fn viewport_planning_input(&self) -> Option<&UiConstraintViewportPlanningInputResult> {
        self.viewport_planning_input.as_ref()
    }
    pub fn scroll_owner_planning_input(
        &self,
    ) -> Option<&UiConstraintScrollOwnerPlanningInputResult> {
        self.scroll_owner_planning_input.as_ref()
    }
    pub fn portal_anchor_planning_input(
        &self,
    ) -> Option<&UiConstraintPortalAnchorPlanningInputResult> {
        self.portal_anchor_planning_input.as_ref()
    }

    pub fn equal_share_distribution(&self) -> Option<&UiConstraintEqualShareDistributionResult> {
        self.equal_share_distribution.as_ref()
    }

    pub fn bound_reconciliation(&self) -> Option<&UiConstraintBoundReconciliationResult> {
        self.bound_reconciliation.as_ref()
    }

    pub fn propagation_edges(&self) -> &[UiConstraintPropagationEdge] {
        &self.propagation_edges
    }
}
