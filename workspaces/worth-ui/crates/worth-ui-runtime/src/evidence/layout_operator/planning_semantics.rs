use crate::declaration::{
    stable_text_digest, UiDeclarationPlanningOperatorKind, UiDeclaredMeasurementBasisSource,
    UiDeclaredMeasurementMode, UiDeclaredMeasurementOwnershipPosture,
};
use super::planning_axis::{
    child_participation_rule_name, cross_axis_name, primary_axis_name,
    UiLayoutOperatorChildParticipationRule, UiLayoutOperatorCrossAxis, UiLayoutOperatorPrimaryAxis,
};
use crate::evidence::UiConstraintPropagationEdgeFamily;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiLayoutOperatorSizingMode {
    AvailableSpace,
    IntrinsicContent,
    BoundedContent,
    ViewportBound,
    PortalAnchorBound,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiLayoutOperatorIntrinsicReturnPolicy {
    None,
    ChildrenToParent,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiLayoutOperatorSiblingGroupingRule {
    None,
    StablePeerGroup,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiLayoutOperatorOverflowPolicy {
    None,
    Clip,
    ScrollViewport,
    PortalAnchorBounded,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiLayoutOperatorSpecialInputRequirement {
    ViewportExtent,
    ScrollViewportExtent,
    PortalAnchorRect,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiLayoutOperatorDenialPolicy {
    RejectUnsupportedPropagationAuthority,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiLayoutOperatorPlanningSemantics {
    primary_axis: UiLayoutOperatorPrimaryAxis,
    cross_axis: UiLayoutOperatorCrossAxis,
    child_participation_rule: UiLayoutOperatorChildParticipationRule,
    allowed_propagation_families: Box<[UiConstraintPropagationEdgeFamily]>,
    admitted_cycle_families: Box<[UiConstraintPropagationEdgeFamily]>,
    allowed_sizing_modes: Box<[UiLayoutOperatorSizingMode]>,
    intrinsic_return_policy: UiLayoutOperatorIntrinsicReturnPolicy,
    sibling_grouping_rule: UiLayoutOperatorSiblingGroupingRule,
    overflow_policy: UiLayoutOperatorOverflowPolicy,
    special_input_requirements: Box<[UiLayoutOperatorSpecialInputRequirement]>,
    denial_policy: UiLayoutOperatorDenialPolicy,
}

impl UiLayoutOperatorPlanningSemantics {
    pub(crate) fn for_operator_kind(
        operator_kind: UiDeclarationPlanningOperatorKind,
        measurement_mode: Option<UiDeclaredMeasurementMode>,
        basis_source: Option<UiDeclaredMeasurementBasisSource>,
        ownership_posture: Option<UiDeclaredMeasurementOwnershipPosture>,
    ) -> Self {
        let mut families = vec![UiConstraintPropagationEdgeFamily::ParentAvailableSpace];
        let mut cycle_families = vec![UiConstraintPropagationEdgeFamily::ParentAvailableSpace];
        let mut sizing = vec![UiLayoutOperatorSizingMode::AvailableSpace];
        let mut special = Vec::new();
        let mut primary_axis = UiLayoutOperatorPrimaryAxis::None;
        let mut cross_axis = UiLayoutOperatorCrossAxis::None;
        let mut child_participation_rule = UiLayoutOperatorChildParticipationRule::None;
        let mut intrinsic_return_policy = UiLayoutOperatorIntrinsicReturnPolicy::None;
        let mut sibling_grouping_rule = UiLayoutOperatorSiblingGroupingRule::None;
        let mut overflow_policy = UiLayoutOperatorOverflowPolicy::None;

        if measurement_mode.is_some() {
            sizing.push(UiLayoutOperatorSizingMode::BoundedContent);
            intrinsic_return_policy = UiLayoutOperatorIntrinsicReturnPolicy::ChildrenToParent;
        }
        match operator_kind {
            UiDeclarationPlanningOperatorKind::Stack => {
                primary_axis = UiLayoutOperatorPrimaryAxis::Vertical;
                cross_axis = UiLayoutOperatorCrossAxis::Horizontal;
                child_participation_rule = UiLayoutOperatorChildParticipationRule::VerticalPeerFlow;
                families.push(UiConstraintPropagationEdgeFamily::ChildIntrinsicContribution);
                families.push(UiConstraintPropagationEdgeFamily::SiblingNegotiation);
                families.push(UiConstraintPropagationEdgeFamily::BoundedReconciliation);
                cycle_families.push(UiConstraintPropagationEdgeFamily::ChildIntrinsicContribution);
                cycle_families.push(UiConstraintPropagationEdgeFamily::SiblingNegotiation);
                cycle_families.push(UiConstraintPropagationEdgeFamily::BoundedReconciliation);
                sizing.push(UiLayoutOperatorSizingMode::IntrinsicContent);
                intrinsic_return_policy = UiLayoutOperatorIntrinsicReturnPolicy::ChildrenToParent;
                sibling_grouping_rule = UiLayoutOperatorSiblingGroupingRule::StablePeerGroup;
                overflow_policy = UiLayoutOperatorOverflowPolicy::Clip;
            }
            UiDeclarationPlanningOperatorKind::Row => {
                primary_axis = UiLayoutOperatorPrimaryAxis::Horizontal;
                cross_axis = UiLayoutOperatorCrossAxis::Vertical;
                child_participation_rule =
                    UiLayoutOperatorChildParticipationRule::HorizontalPeerFlow;
                families.push(UiConstraintPropagationEdgeFamily::ChildIntrinsicContribution);
                families.push(UiConstraintPropagationEdgeFamily::SiblingNegotiation);
                families.push(UiConstraintPropagationEdgeFamily::BoundedReconciliation);
                cycle_families.push(UiConstraintPropagationEdgeFamily::ChildIntrinsicContribution);
                cycle_families.push(UiConstraintPropagationEdgeFamily::SiblingNegotiation);
                cycle_families.push(UiConstraintPropagationEdgeFamily::BoundedReconciliation);
                sizing.push(UiLayoutOperatorSizingMode::IntrinsicContent);
                intrinsic_return_policy = UiLayoutOperatorIntrinsicReturnPolicy::ChildrenToParent;
                sibling_grouping_rule = UiLayoutOperatorSiblingGroupingRule::StablePeerGroup;
                overflow_policy = UiLayoutOperatorOverflowPolicy::Clip;
            }
            UiDeclarationPlanningOperatorKind::Grid => {
                primary_axis = UiLayoutOperatorPrimaryAxis::TwoDimensional;
                cross_axis = UiLayoutOperatorCrossAxis::TwoDimensional;
                child_participation_rule = UiLayoutOperatorChildParticipationRule::GridCellPeers;
                families.push(UiConstraintPropagationEdgeFamily::ChildIntrinsicContribution);
                families.push(UiConstraintPropagationEdgeFamily::SiblingNegotiation);
                families.push(UiConstraintPropagationEdgeFamily::EqualShareDistribution);
                families.push(UiConstraintPropagationEdgeFamily::BoundedReconciliation);
                cycle_families.push(UiConstraintPropagationEdgeFamily::ChildIntrinsicContribution);
                cycle_families.push(UiConstraintPropagationEdgeFamily::SiblingNegotiation);
                cycle_families.push(UiConstraintPropagationEdgeFamily::EqualShareDistribution);
                cycle_families.push(UiConstraintPropagationEdgeFamily::BoundedReconciliation);
                sizing.push(UiLayoutOperatorSizingMode::IntrinsicContent);
                intrinsic_return_policy = UiLayoutOperatorIntrinsicReturnPolicy::ChildrenToParent;
                sibling_grouping_rule = UiLayoutOperatorSiblingGroupingRule::StablePeerGroup;
                overflow_policy = UiLayoutOperatorOverflowPolicy::Clip;
            }
            UiDeclarationPlanningOperatorKind::Split => {
                primary_axis = UiLayoutOperatorPrimaryAxis::Horizontal;
                cross_axis = UiLayoutOperatorCrossAxis::Vertical;
                child_participation_rule = UiLayoutOperatorChildParticipationRule::SplitPanels;
                families.push(UiConstraintPropagationEdgeFamily::ChildIntrinsicContribution);
                families.push(UiConstraintPropagationEdgeFamily::SiblingNegotiation);
                families.push(UiConstraintPropagationEdgeFamily::EqualShareDistribution);
                families.push(UiConstraintPropagationEdgeFamily::BoundedReconciliation);
                families.push(UiConstraintPropagationEdgeFamily::DurableResizeInput);
                cycle_families.push(UiConstraintPropagationEdgeFamily::ChildIntrinsicContribution);
                cycle_families.push(UiConstraintPropagationEdgeFamily::SiblingNegotiation);
                cycle_families.push(UiConstraintPropagationEdgeFamily::EqualShareDistribution);
                cycle_families.push(UiConstraintPropagationEdgeFamily::BoundedReconciliation);
                cycle_families.push(UiConstraintPropagationEdgeFamily::DurableResizeInput);
                sizing.push(UiLayoutOperatorSizingMode::IntrinsicContent);
                intrinsic_return_policy = UiLayoutOperatorIntrinsicReturnPolicy::ChildrenToParent;
                sibling_grouping_rule = UiLayoutOperatorSiblingGroupingRule::StablePeerGroup;
                overflow_policy = UiLayoutOperatorOverflowPolicy::Clip;
            }
            UiDeclarationPlanningOperatorKind::Mosaic => {
                primary_axis = UiLayoutOperatorPrimaryAxis::TwoDimensional;
                cross_axis = UiLayoutOperatorCrossAxis::TwoDimensional;
                child_participation_rule = UiLayoutOperatorChildParticipationRule::MosaicTiles;
                families.push(UiConstraintPropagationEdgeFamily::ChildIntrinsicContribution);
                families.push(UiConstraintPropagationEdgeFamily::SiblingNegotiation);
                families.push(UiConstraintPropagationEdgeFamily::EqualShareDistribution);
                families.push(UiConstraintPropagationEdgeFamily::BoundedReconciliation);
                families.push(UiConstraintPropagationEdgeFamily::DurableResizeInput);
                cycle_families.push(UiConstraintPropagationEdgeFamily::ChildIntrinsicContribution);
                cycle_families.push(UiConstraintPropagationEdgeFamily::SiblingNegotiation);
                cycle_families.push(UiConstraintPropagationEdgeFamily::EqualShareDistribution);
                cycle_families.push(UiConstraintPropagationEdgeFamily::BoundedReconciliation);
                cycle_families.push(UiConstraintPropagationEdgeFamily::DurableResizeInput);
                sizing.push(UiLayoutOperatorSizingMode::IntrinsicContent);
                intrinsic_return_policy = UiLayoutOperatorIntrinsicReturnPolicy::ChildrenToParent;
                sibling_grouping_rule = UiLayoutOperatorSiblingGroupingRule::StablePeerGroup;
                overflow_policy = UiLayoutOperatorOverflowPolicy::Clip;
            }
            UiDeclarationPlanningOperatorKind::Overlay => {
                primary_axis = UiLayoutOperatorPrimaryAxis::Layered;
                cross_axis = UiLayoutOperatorCrossAxis::Layered;
                child_participation_rule = UiLayoutOperatorChildParticipationRule::OverlayLayers;
                families.push(UiConstraintPropagationEdgeFamily::ChildIntrinsicContribution);
                families.push(UiConstraintPropagationEdgeFamily::BoundedReconciliation);
                cycle_families.push(UiConstraintPropagationEdgeFamily::ChildIntrinsicContribution);
                cycle_families.push(UiConstraintPropagationEdgeFamily::BoundedReconciliation);
                sizing.push(UiLayoutOperatorSizingMode::IntrinsicContent);
                intrinsic_return_policy = UiLayoutOperatorIntrinsicReturnPolicy::ChildrenToParent;
                overflow_policy = UiLayoutOperatorOverflowPolicy::Clip;
            }
            UiDeclarationPlanningOperatorKind::Scroll => {
                primary_axis = UiLayoutOperatorPrimaryAxis::Vertical;
                cross_axis = UiLayoutOperatorCrossAxis::Horizontal;
                child_participation_rule = UiLayoutOperatorChildParticipationRule::ScrollContent;
                families.push(UiConstraintPropagationEdgeFamily::ChildIntrinsicContribution);
                families.push(UiConstraintPropagationEdgeFamily::BoundedReconciliation);
                families.push(UiConstraintPropagationEdgeFamily::ScrollViewportInput);
                cycle_families.push(UiConstraintPropagationEdgeFamily::ChildIntrinsicContribution);
                cycle_families.push(UiConstraintPropagationEdgeFamily::BoundedReconciliation);
                sizing.push(UiLayoutOperatorSizingMode::ViewportBound);
                intrinsic_return_policy = UiLayoutOperatorIntrinsicReturnPolicy::ChildrenToParent;
                overflow_policy = UiLayoutOperatorOverflowPolicy::ScrollViewport;
                if matches!(
                    basis_source,
                    Some(UiDeclaredMeasurementBasisSource::ScrollViewport)
                ) || matches!(
                    ownership_posture,
                    Some(UiDeclaredMeasurementOwnershipPosture::ScrollContainerBasis)
                ) {
                    special.push(UiLayoutOperatorSpecialInputRequirement::ScrollViewportExtent);
                }
            }
            UiDeclarationPlanningOperatorKind::PortalAnchor => {
                primary_axis = UiLayoutOperatorPrimaryAxis::Layered;
                cross_axis = UiLayoutOperatorCrossAxis::Layered;
                child_participation_rule =
                    UiLayoutOperatorChildParticipationRule::PortalAnchoredContent;
                families.push(UiConstraintPropagationEdgeFamily::BoundedReconciliation);
                families.push(UiConstraintPropagationEdgeFamily::PortalAnchorInput);
                cycle_families.push(UiConstraintPropagationEdgeFamily::BoundedReconciliation);
                sizing.push(UiLayoutOperatorSizingMode::PortalAnchorBound);
                overflow_policy = UiLayoutOperatorOverflowPolicy::PortalAnchorBounded;
                if matches!(
                    basis_source,
                    Some(UiDeclaredMeasurementBasisSource::PortalAnchor)
                ) || matches!(
                    ownership_posture,
                    Some(UiDeclaredMeasurementOwnershipPosture::PortalAnchorBasisRequired)
                ) {
                    special.push(UiLayoutOperatorSpecialInputRequirement::PortalAnchorRect);
                }
            }
            UiDeclarationPlanningOperatorKind::PageRoot => {
                primary_axis = UiLayoutOperatorPrimaryAxis::TwoDimensional;
                cross_axis = UiLayoutOperatorCrossAxis::TwoDimensional;
                child_participation_rule =
                    UiLayoutOperatorChildParticipationRule::RootViewportFrame;
                families.push(UiConstraintPropagationEdgeFamily::BoundedReconciliation);
                families.push(UiConstraintPropagationEdgeFamily::ViewportInput);
                cycle_families.push(UiConstraintPropagationEdgeFamily::BoundedReconciliation);
                sizing.push(UiLayoutOperatorSizingMode::ViewportBound);
                special.push(UiLayoutOperatorSpecialInputRequirement::ViewportExtent);
            }
            UiDeclarationPlanningOperatorKind::PageSet
            | UiDeclarationPlanningOperatorKind::Region
            | UiDeclarationPlanningOperatorKind::LocalComposition
            | UiDeclarationPlanningOperatorKind::Control => {
                primary_axis = UiLayoutOperatorPrimaryAxis::TwoDimensional;
                cross_axis = UiLayoutOperatorCrossAxis::TwoDimensional;
                child_participation_rule =
                    UiLayoutOperatorChildParticipationRule::ContainerChildren;
                families.push(UiConstraintPropagationEdgeFamily::ChildIntrinsicContribution);
                families.push(UiConstraintPropagationEdgeFamily::BoundedReconciliation);
                cycle_families.push(UiConstraintPropagationEdgeFamily::ChildIntrinsicContribution);
                cycle_families.push(UiConstraintPropagationEdgeFamily::BoundedReconciliation);
                sizing.push(UiLayoutOperatorSizingMode::IntrinsicContent);
                intrinsic_return_policy = UiLayoutOperatorIntrinsicReturnPolicy::ChildrenToParent;
            }
            UiDeclarationPlanningOperatorKind::DiagnosticSurface => {}
        }

        families.sort_unstable_by_key(|family| family.rank());
        families.dedup();
        cycle_families.sort_unstable_by_key(|family| family.rank());
        cycle_families.dedup();
        sizing.sort_unstable();
        sizing.dedup();
        special.sort_unstable();
        special.dedup();

        Self {
            primary_axis,
            cross_axis,
            child_participation_rule,
            allowed_propagation_families: families.into_boxed_slice(),
            admitted_cycle_families: cycle_families.into_boxed_slice(),
            allowed_sizing_modes: sizing.into_boxed_slice(),
            intrinsic_return_policy,
            sibling_grouping_rule,
            overflow_policy,
            special_input_requirements: special.into_boxed_slice(),
            denial_policy: UiLayoutOperatorDenialPolicy::RejectUnsupportedPropagationAuthority,
        }
    }

    pub fn allowed_propagation_families(&self) -> &[UiConstraintPropagationEdgeFamily] { &self.allowed_propagation_families }
    pub fn primary_axis(&self) -> UiLayoutOperatorPrimaryAxis { self.primary_axis }
    pub fn admitted_cycle_families(&self) -> &[UiConstraintPropagationEdgeFamily] { &self.admitted_cycle_families }
    pub fn cross_axis(&self) -> UiLayoutOperatorCrossAxis { self.cross_axis }
    pub fn child_participation_rule(&self) -> UiLayoutOperatorChildParticipationRule {
        self.child_participation_rule
    }

    pub fn allowed_sizing_modes(&self) -> &[UiLayoutOperatorSizingMode] {
        &self.allowed_sizing_modes
    }

    pub fn intrinsic_return_policy(&self) -> UiLayoutOperatorIntrinsicReturnPolicy {
        self.intrinsic_return_policy
    }

    pub fn sibling_grouping_rule(&self) -> UiLayoutOperatorSiblingGroupingRule {
        self.sibling_grouping_rule
    }

    pub fn overflow_policy(&self) -> UiLayoutOperatorOverflowPolicy {
        self.overflow_policy
    }

    pub fn special_input_requirements(&self) -> &[UiLayoutOperatorSpecialInputRequirement] {
        &self.special_input_requirements
    }

    pub fn denial_policy(&self) -> UiLayoutOperatorDenialPolicy {
        self.denial_policy
    }

    pub(crate) fn identity_digest(&self) -> u64 {
        let digest = self.allowed_propagation_families.iter().fold(
            stable_text_digest("worth-ui.layout-operator-semantics"),
            |digest, family| {
                digest.rotate_left(11) ^ stable_text_digest(family_name(*family)).rotate_left(7)
            },
        );
        let digest = self
            .admitted_cycle_families
            .iter()
            .fold(digest, |digest, family| {
                digest.rotate_left(11)
                    ^ stable_text_digest(family_name(*family)).rotate_left(9)
                    ^ stable_text_digest("worth-ui.layout-operator-cycle-family").rotate_left(5)
            });
        let digest = self
            .allowed_sizing_modes
            .iter()
            .fold(digest, |digest, mode| {
                digest.rotate_left(11) ^ stable_text_digest(sizing_mode_name(*mode)).rotate_left(13)
            });
        let digest = self
            .special_input_requirements
            .iter()
            .fold(digest, |digest, requirement| {
                digest.rotate_left(11)
                    ^ stable_text_digest(special_input_requirement_name(*requirement))
                        .rotate_left(17)
            });
        digest
            ^ stable_text_digest(primary_axis_name(self.primary_axis)).rotate_left(5)
            ^ stable_text_digest(cross_axis_name(self.cross_axis)).rotate_left(9)
            ^ stable_text_digest(child_participation_rule_name(self.child_participation_rule))
                .rotate_left(15)
            ^ stable_text_digest(intrinsic_return_policy_name(self.intrinsic_return_policy))
                .rotate_left(19)
            ^ stable_text_digest(sibling_grouping_rule_name(self.sibling_grouping_rule))
                .rotate_left(23)
            ^ stable_text_digest(overflow_policy_name(self.overflow_policy)).rotate_left(29)
            ^ stable_text_digest(denial_policy_name(self.denial_policy)).rotate_left(31)
    }
}

fn family_name(family: UiConstraintPropagationEdgeFamily) -> &'static str {
    match family {
        UiConstraintPropagationEdgeFamily::ParentAvailableSpace => "parent-available-space",
        UiConstraintPropagationEdgeFamily::ChildIntrinsicContribution => "child-intrinsic",
        UiConstraintPropagationEdgeFamily::SiblingNegotiation => "sibling-negotiation",
        UiConstraintPropagationEdgeFamily::EqualShareDistribution => "equal-share",
        UiConstraintPropagationEdgeFamily::BoundedReconciliation => "bounded-reconciliation",
        UiConstraintPropagationEdgeFamily::ViewportInput => "viewport-input",
        UiConstraintPropagationEdgeFamily::ScrollViewportInput => "scroll-viewport-input",
        UiConstraintPropagationEdgeFamily::PortalAnchorInput => "portal-anchor-input",
        UiConstraintPropagationEdgeFamily::DurableResizeInput => "durable-resize-input",
    }
}

fn sizing_mode_name(mode: UiLayoutOperatorSizingMode) -> &'static str {
    match mode {
        UiLayoutOperatorSizingMode::AvailableSpace => "available-space",
        UiLayoutOperatorSizingMode::IntrinsicContent => "intrinsic-content",
        UiLayoutOperatorSizingMode::BoundedContent => "bounded-content",
        UiLayoutOperatorSizingMode::ViewportBound => "viewport-bound",
        UiLayoutOperatorSizingMode::PortalAnchorBound => "portal-anchor-bound",
    }
}

fn intrinsic_return_policy_name(policy: UiLayoutOperatorIntrinsicReturnPolicy) -> &'static str {
    match policy {
        UiLayoutOperatorIntrinsicReturnPolicy::None => "none",
        UiLayoutOperatorIntrinsicReturnPolicy::ChildrenToParent => "children-to-parent",
    }
}

fn sibling_grouping_rule_name(rule: UiLayoutOperatorSiblingGroupingRule) -> &'static str {
    match rule {
        UiLayoutOperatorSiblingGroupingRule::None => "none",
        UiLayoutOperatorSiblingGroupingRule::StablePeerGroup => "stable-peer-group",
    }
}

fn overflow_policy_name(policy: UiLayoutOperatorOverflowPolicy) -> &'static str {
    match policy {
        UiLayoutOperatorOverflowPolicy::None => "none",
        UiLayoutOperatorOverflowPolicy::Clip => "clip",
        UiLayoutOperatorOverflowPolicy::ScrollViewport => "scroll-viewport",
        UiLayoutOperatorOverflowPolicy::PortalAnchorBounded => "portal-anchor-bounded",
    }
}

fn special_input_requirement_name(
    requirement: UiLayoutOperatorSpecialInputRequirement,
) -> &'static str {
    match requirement {
        UiLayoutOperatorSpecialInputRequirement::ViewportExtent => "viewport-extent",
        UiLayoutOperatorSpecialInputRequirement::ScrollViewportExtent => "scroll-viewport-extent",
        UiLayoutOperatorSpecialInputRequirement::PortalAnchorRect => "portal-anchor-rect",
    }
}

fn denial_policy_name(policy: UiLayoutOperatorDenialPolicy) -> &'static str {
    match policy {
        UiLayoutOperatorDenialPolicy::RejectUnsupportedPropagationAuthority => {
            "reject-unsupported-propagation-authority"
        }
    }
}
