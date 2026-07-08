use crate::declaration::{
    UiDeclarationOrderingGuarantee, UiDeclarationPlanningOperatorKind,
    UiDeclarationRepetitionPosture, UiDeclaredMeasurementBasisSource,
};
use crate::evidence::{
    UiAllocationConstraintSet, UiAllocationConstraintSummary, UiConstraintAxisScope,
    UiConstraintAvailableSpacePosture,
    UiConstraintBoundedMinMaxRequirement, UiConstraintCycleParticipationPosture,
    UiConstraintEqualShareGroup, UiConstraintNormalizationPosture, UiConstraintParentAvailableSpace,
    UiConstraintPropagationEdge, UiConstraintPropagationEdgeFamily,
    UiConstraintPropagationEdgePayload, UiConstraintResizePermissionPosture,
    UiConstraintSiblingNegotiationFixedPointPolicy, UiConstraintSiblingNegotiationMode,
    UiConstraintSiblingNegotiationSolveOrder, UiConstraintSpecialInputPosture,
    UiLayoutOperatorChildParticipationRule, UiLayoutOperatorContainmentKind,
    UiLayoutOperatorFamily, UiLayoutOperatorPlanningContract, UiLayoutOperatorPrimaryAxis,
    UiLayoutOperatorSlotParticipationKind, UiLayoutOperatorSpecialInputRequirement,
};

#[test]
fn allocation_constraint_set_canonicalizes_unsorted_edges() {
    let constraint_set = UiAllocationConstraintSet::new(
        17,
        contract(UiDeclarationPlanningOperatorKind::Stack).identity(),
        default_summary(),
        vec![
            UiConstraintPropagationEdge::new(
                UiConstraintPropagationEdgeFamily::SiblingNegotiation,
                200,
                300,
                UiConstraintPropagationEdgePayload::SiblingNegotiation {
                    axis_scope: UiConstraintAxisScope::Both,
                    group_identity_digest: 501,
                    negotiation_identity_digest: 701,
                    fixed_point_policy:
                        UiConstraintSiblingNegotiationFixedPointPolicy::AdmittedStablePeerMutual,
                    solve_order:
                        UiConstraintSiblingNegotiationSolveOrder::BeforeEqualShareAndBounds,
                },
                UiConstraintCycleParticipationPosture::Acyclic,
            ),
            UiConstraintPropagationEdge::new(
                UiConstraintPropagationEdgeFamily::ParentAvailableSpace,
                100,
                300,
                UiConstraintPropagationEdgePayload::ParentAvailableSpace(
                    UiConstraintParentAvailableSpace::new(
                        UiConstraintAxisScope::Both,
                        UiConstraintAvailableSpacePosture::DeclaredExtentUnknown,
                        UiConstraintBoundedMinMaxRequirement::BothAxes,
                        UiConstraintNormalizationPosture::deferred(),
                    ),
                ),
                UiConstraintCycleParticipationPosture::Acyclic,
            ),
        ],
    );

    assert_eq!(
        constraint_set.propagation_edges()[0].family(),
        UiConstraintPropagationEdgeFamily::ParentAvailableSpace
    );
    assert_eq!(
        constraint_set.propagation_edges()[1].family(),
        UiConstraintPropagationEdgeFamily::SiblingNegotiation
    );
}

#[test]
fn propagation_edge_identity_preserves_cycle_posture() {
    let acyclic = UiConstraintPropagationEdge::new(
        UiConstraintPropagationEdgeFamily::ParentAvailableSpace,
        100,
        200,
        UiConstraintPropagationEdgePayload::ParentAvailableSpace(
            UiConstraintParentAvailableSpace::new(
                UiConstraintAxisScope::Primary,
                UiConstraintAvailableSpacePosture::DeclaredExtentUnknown,
                UiConstraintBoundedMinMaxRequirement::PrimaryAxis,
                UiConstraintNormalizationPosture::deferred(),
            ),
        ),
        UiConstraintCycleParticipationPosture::Acyclic,
    );
    let fixed_point = UiConstraintPropagationEdge::new(
        UiConstraintPropagationEdgeFamily::ParentAvailableSpace,
        100,
        200,
        UiConstraintPropagationEdgePayload::ParentAvailableSpace(
            UiConstraintParentAvailableSpace::new(
                UiConstraintAxisScope::Primary,
                UiConstraintAvailableSpacePosture::DeclaredExtentUnknown,
                UiConstraintBoundedMinMaxRequirement::PrimaryAxis,
                UiConstraintNormalizationPosture::deferred(),
            ),
        ),
        UiConstraintCycleParticipationPosture::AdmittedFixedPoint,
    );

    assert_ne!(acyclic.identity_digest(), fixed_point.identity_digest());
    assert_ne!(acyclic, fixed_point);
}

#[test]
fn parent_available_space_payload_preserves_normalization_posture() {
    let edge = UiConstraintPropagationEdge::new(
        UiConstraintPropagationEdgeFamily::ParentAvailableSpace,
        100,
        200,
        UiConstraintPropagationEdgePayload::ParentAvailableSpace(
            UiConstraintParentAvailableSpace::new(
                UiConstraintAxisScope::Primary,
                UiConstraintAvailableSpacePosture::DeclaredExtentUnknown,
                UiConstraintBoundedMinMaxRequirement::PrimaryAxis,
                UiConstraintNormalizationPosture::explicit(
                    crate::evidence::UiMeasurementUnitPosture::LogicalPx,
                    crate::evidence::UiMeasurementCoordinateSpace::Viewport,
                    crate::evidence::UiMeasurementRoundingPosture::ExactFloat,
                ),
            ),
        ),
        UiConstraintCycleParticipationPosture::Acyclic,
    );

    let downward = edge
        .payload()
        .parent_available_space()
        .expect("edge should preserve parent available-space witness");
    assert_eq!(downward.axis_scope(), UiConstraintAxisScope::Primary);
    assert_eq!(
        downward.bounded_min_max_requirement(),
        UiConstraintBoundedMinMaxRequirement::PrimaryAxis
    );
    assert_eq!(
        downward.normalization_posture(),
        UiConstraintNormalizationPosture::explicit(
            crate::evidence::UiMeasurementUnitPosture::LogicalPx,
            crate::evidence::UiMeasurementCoordinateSpace::Viewport,
            crate::evidence::UiMeasurementRoundingPosture::ExactFloat,
        )
    );
}

#[test]
fn named_operator_contracts_publish_closed_propagation_semantics() {
    let stack = contract(UiDeclarationPlanningOperatorKind::Stack);
    let row = contract(UiDeclarationPlanningOperatorKind::Row);
    let grid = contract(UiDeclarationPlanningOperatorKind::Grid);
    let split = contract(UiDeclarationPlanningOperatorKind::Split);
    let mosaic = UiLayoutOperatorPlanningContract::new(
        UiDeclarationPlanningOperatorKind::Mosaic,
        UiLayoutOperatorFamily::Mosaic,
        UiLayoutOperatorContainmentKind::Mosaic,
        None,
        UiLayoutOperatorSlotParticipationKind::None,
        UiDeclarationOrderingGuarantee::NotSemanticallyClaimed,
        UiDeclarationRepetitionPosture::NotAdmitted,
        crate::evidence::UiAllocationNeighborhoodClass::LocalIntrinsicContent,
        crate::evidence::UiAllocationNeighborhoodMembershipRule::RootOnly,
        None,
        None,
        None,
        None,
        vec![],
    );
    let overlay = contract(UiDeclarationPlanningOperatorKind::Overlay);
    let scroll = UiLayoutOperatorPlanningContract::new(
        UiDeclarationPlanningOperatorKind::Scroll,
        UiLayoutOperatorFamily::Control,
        UiLayoutOperatorContainmentKind::Control,
        None,
        UiLayoutOperatorSlotParticipationKind::DeclaredParticipant,
        UiDeclarationOrderingGuarantee::NotSemanticallyClaimed,
        UiDeclarationRepetitionPosture::NotAdmitted,
        crate::evidence::UiAllocationNeighborhoodClass::ScrollContainer,
        crate::evidence::UiAllocationNeighborhoodMembershipRule::RootOnly,
        None,
        None,
        Some(UiDeclaredMeasurementBasisSource::ScrollViewport),
        None,
        vec![],
    );
    let portal = UiLayoutOperatorPlanningContract::new(
        UiDeclarationPlanningOperatorKind::PortalAnchor,
        UiLayoutOperatorFamily::Control,
        UiLayoutOperatorContainmentKind::Control,
        None,
        UiLayoutOperatorSlotParticipationKind::DeclaredParticipant,
        UiDeclarationOrderingGuarantee::NotSemanticallyClaimed,
        UiDeclarationRepetitionPosture::NotAdmitted,
        crate::evidence::UiAllocationNeighborhoodClass::PortalAnchor,
        crate::evidence::UiAllocationNeighborhoodMembershipRule::RootOnly,
        None,
        None,
        Some(UiDeclaredMeasurementBasisSource::PortalAnchor),
        None,
        vec![],
    );

    for contract in [&stack, &row, &grid, &split, &mosaic] {
        assert!(contract
            .semantics()
            .allowed_propagation_families()
            .contains(&UiConstraintPropagationEdgeFamily::SiblingNegotiation));
        assert!(contract
            .semantics()
            .allowed_propagation_families()
            .contains(&UiConstraintPropagationEdgeFamily::ChildIntrinsicContribution));
    }
    assert!(grid
        .semantics()
        .allowed_propagation_families()
        .contains(&UiConstraintPropagationEdgeFamily::EqualShareDistribution));
    assert!(split
        .semantics()
        .allowed_propagation_families()
        .contains(&UiConstraintPropagationEdgeFamily::EqualShareDistribution));
    assert!(split
        .semantics()
        .allowed_propagation_families()
        .contains(&UiConstraintPropagationEdgeFamily::DurableResizeInput));
    assert!(grid
        .semantics()
        .admitted_cycle_families()
        .contains(&UiConstraintPropagationEdgeFamily::EqualShareDistribution));
    assert!(stack
        .semantics()
        .allowed_propagation_families()
        .contains(&UiConstraintPropagationEdgeFamily::BoundedReconciliation));
    assert!(!overlay
        .semantics()
        .allowed_propagation_families()
        .contains(&UiConstraintPropagationEdgeFamily::SiblingNegotiation));
    assert_eq!(
        stack.semantics().primary_axis(),
        UiLayoutOperatorPrimaryAxis::Vertical
    );
    assert_eq!(
        row.semantics().primary_axis(),
        UiLayoutOperatorPrimaryAxis::Horizontal
    );
    assert_eq!(
        grid.semantics().child_participation_rule(),
        UiLayoutOperatorChildParticipationRule::GridCellPeers
    );
    assert!(scroll
        .semantics()
        .special_input_requirements()
        .contains(&UiLayoutOperatorSpecialInputRequirement::ScrollViewportExtent));
    assert!(portal
        .semantics()
        .special_input_requirements()
        .contains(&UiLayoutOperatorSpecialInputRequirement::PortalAnchorRect));
}

#[test]
fn constraint_set_summary_preserves_typed_planning_facts() {
    let constraint_set = UiAllocationConstraintSet::new(
        19,
        contract(UiDeclarationPlanningOperatorKind::Grid).identity(),
        UiAllocationConstraintSummary::new(
            Some(UiConstraintAxisScope::Both),
            Some(UiConstraintAvailableSpacePosture::AdmittedPositiveExtent),
            Some(UiConstraintAxisScope::Both),
            UiConstraintSiblingNegotiationMode::StablePeerTwoDimensional,
            UiConstraintEqualShareGroup::StablePeerTwoDimensional,
            UiConstraintBoundedMinMaxRequirement::BothAxes,
            UiConstraintSpecialInputPosture::Required,
            UiConstraintSpecialInputPosture::NotRequired,
            UiConstraintSpecialInputPosture::NotRequired,
            UiConstraintResizePermissionPosture::DurableAuthorityLane,
            None,
            None,
            None,
        ),
        vec![],
    );

    let summary = constraint_set.summary();
    assert_eq!(
        summary.sibling_negotiation_mode(),
        UiConstraintSiblingNegotiationMode::StablePeerTwoDimensional
    );
    assert_eq!(
        summary.equal_share_group(),
        UiConstraintEqualShareGroup::StablePeerTwoDimensional
    );
    assert_eq!(
        summary.incoming_available_space_posture(),
        Some(UiConstraintAvailableSpacePosture::AdmittedPositiveExtent)
    );
    assert_eq!(
        summary.bounded_min_max_requirements(),
        UiConstraintBoundedMinMaxRequirement::BothAxes
    );
    assert_eq!(
        summary.resize_permission_posture(),
        UiConstraintResizePermissionPosture::DurableAuthorityLane
    );
}

fn contract(operator_kind: UiDeclarationPlanningOperatorKind) -> UiLayoutOperatorPlanningContract {
    UiLayoutOperatorPlanningContract::new(
        operator_kind,
        UiLayoutOperatorFamily::Control,
        UiLayoutOperatorContainmentKind::Control,
        None,
        UiLayoutOperatorSlotParticipationKind::DeclaredParticipant,
        UiDeclarationOrderingGuarantee::NotSemanticallyClaimed,
        UiDeclarationRepetitionPosture::NotAdmitted,
        crate::evidence::UiAllocationNeighborhoodClass::ContainerPeerGroup,
        crate::evidence::UiAllocationNeighborhoodMembershipRule::ParentSlotPeerGroup,
        None,
        None,
        None,
        None,
        vec![],
    )
}

fn default_summary() -> UiAllocationConstraintSummary {
    UiAllocationConstraintSummary::new(
        Some(UiConstraintAxisScope::Both),
        None,
        Some(UiConstraintAxisScope::Both),
        UiConstraintSiblingNegotiationMode::StablePeerPrimaryAxis,
        UiConstraintEqualShareGroup::None,
        UiConstraintBoundedMinMaxRequirement::PrimaryAxis,
        UiConstraintSpecialInputPosture::NotRequired,
        UiConstraintSpecialInputPosture::NotRequired,
        UiConstraintSpecialInputPosture::NotRequired,
        UiConstraintResizePermissionPosture::None,
        None,
        None,
        None,
    )
}
