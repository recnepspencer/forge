use crate::declaration::{
    UiDeclarationOrderingGuarantee, UiDeclarationPlanningOperatorKind,
    UiDeclarationRepetitionPosture, UiDeclaredMeasurementBasisSource,
};
use crate::evidence::{
    UiAllocationConstraintSet, UiAllocationConstraintSummary, UiAllocationConstraintSummaryInput,
    UiConstraintAvailableSpacePosture, UiConstraintAxisScope, UiConstraintBoundedMinMaxRequirement,
    UiConstraintCycleParticipationPosture, UiConstraintEqualShareGroup,
    UiConstraintNormalizationPosture, UiConstraintParentAvailableSpace,
    UiConstraintPropagationEdge, UiConstraintPropagationEdgeFamily,
    UiConstraintPropagationEdgePayload, UiConstraintResizePermissionPosture,
    UiConstraintSiblingNegotiationFixedPointPolicy, UiConstraintSiblingNegotiationMode,
    UiConstraintSiblingNegotiationSolveOrder, UiConstraintSpecialInputPosture,
    UiLayoutOperatorChildParticipationRule, UiLayoutOperatorContainmentKind,
    UiLayoutOperatorFamily, UiLayoutOperatorPlanningContract,
    UiLayoutOperatorPlanningContractInput, UiLayoutOperatorPrimaryAxis,
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
    let mosaic = UiLayoutOperatorPlanningContract::new(UiLayoutOperatorPlanningContractInput {
        operator_kind: UiDeclarationPlanningOperatorKind::Mosaic,
        operator_family: UiLayoutOperatorFamily::Mosaic,
        containment_kind: UiLayoutOperatorContainmentKind::Mosaic,
        mosaic_sizing_contract_id: None,
        slot_participation_kind: UiLayoutOperatorSlotParticipationKind::None,
        ordering_guarantee: UiDeclarationOrderingGuarantee::NotSemanticallyClaimed,
        repetition_posture: UiDeclarationRepetitionPosture::NotAdmitted,
        neighborhood_class: crate::evidence::UiAllocationNeighborhoodClass::LocalIntrinsicContent,
        membership_rule: crate::evidence::UiAllocationNeighborhoodMembershipRule::RootOnly,
        measurement_mode: None,
        constraint_modifier: None,
        basis_source: None,
        ownership_posture: None,
        evidence_requirements: vec![],
    });
    let overlay = contract(UiDeclarationPlanningOperatorKind::Overlay);
    let scroll = UiLayoutOperatorPlanningContract::new(UiLayoutOperatorPlanningContractInput {
        operator_kind: UiDeclarationPlanningOperatorKind::Scroll,
        operator_family: UiLayoutOperatorFamily::Control,
        containment_kind: UiLayoutOperatorContainmentKind::Control,
        mosaic_sizing_contract_id: None,
        slot_participation_kind: UiLayoutOperatorSlotParticipationKind::DeclaredParticipant,
        ordering_guarantee: UiDeclarationOrderingGuarantee::NotSemanticallyClaimed,
        repetition_posture: UiDeclarationRepetitionPosture::NotAdmitted,
        neighborhood_class: crate::evidence::UiAllocationNeighborhoodClass::ScrollContainer,
        membership_rule: crate::evidence::UiAllocationNeighborhoodMembershipRule::RootOnly,
        measurement_mode: None,
        constraint_modifier: None,
        basis_source: Some(UiDeclaredMeasurementBasisSource::ScrollViewport),
        ownership_posture: None,
        evidence_requirements: vec![],
    });
    let portal = UiLayoutOperatorPlanningContract::new(UiLayoutOperatorPlanningContractInput {
        operator_kind: UiDeclarationPlanningOperatorKind::PortalAnchor,
        operator_family: UiLayoutOperatorFamily::Control,
        containment_kind: UiLayoutOperatorContainmentKind::Control,
        mosaic_sizing_contract_id: None,
        slot_participation_kind: UiLayoutOperatorSlotParticipationKind::DeclaredParticipant,
        ordering_guarantee: UiDeclarationOrderingGuarantee::NotSemanticallyClaimed,
        repetition_posture: UiDeclarationRepetitionPosture::NotAdmitted,
        neighborhood_class: crate::evidence::UiAllocationNeighborhoodClass::PortalAnchor,
        membership_rule: crate::evidence::UiAllocationNeighborhoodMembershipRule::RootOnly,
        measurement_mode: None,
        constraint_modifier: None,
        basis_source: Some(UiDeclaredMeasurementBasisSource::PortalAnchor),
        ownership_posture: None,
        evidence_requirements: vec![],
    });

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
        UiAllocationConstraintSummary::new(UiAllocationConstraintSummaryInput {
            incoming_available_space: Some(UiConstraintAxisScope::Both),
            incoming_available_space_posture: Some(
                UiConstraintAvailableSpacePosture::AdmittedPositiveExtent,
            ),
            intrinsic_contribution_requirements: Some(UiConstraintAxisScope::Both),
            sibling_negotiation_mode: UiConstraintSiblingNegotiationMode::StablePeerTwoDimensional,
            equal_share_group: UiConstraintEqualShareGroup::StablePeerTwoDimensional,
            bounded_min_max_requirements: UiConstraintBoundedMinMaxRequirement::BothAxes,
            viewport_requirement: UiConstraintSpecialInputPosture::Required,
            scroll_owner_requirement: UiConstraintSpecialInputPosture::NotRequired,
            portal_anchor_requirement: UiConstraintSpecialInputPosture::NotRequired,
            resize_permission_posture: UiConstraintResizePermissionPosture::DurableAuthorityLane,
            unit_posture: None,
            coordinate_space: None,
            rounding_posture: None,
        }),
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
    UiLayoutOperatorPlanningContract::new(UiLayoutOperatorPlanningContractInput {
        operator_kind,
        operator_family: UiLayoutOperatorFamily::Control,
        containment_kind: UiLayoutOperatorContainmentKind::Control,
        mosaic_sizing_contract_id: None,
        slot_participation_kind: UiLayoutOperatorSlotParticipationKind::DeclaredParticipant,
        ordering_guarantee: UiDeclarationOrderingGuarantee::NotSemanticallyClaimed,
        repetition_posture: UiDeclarationRepetitionPosture::NotAdmitted,
        neighborhood_class: crate::evidence::UiAllocationNeighborhoodClass::ContainerPeerGroup,
        membership_rule:
            crate::evidence::UiAllocationNeighborhoodMembershipRule::ParentSlotPeerGroup,
        measurement_mode: None,
        constraint_modifier: None,
        basis_source: None,
        ownership_posture: None,
        evidence_requirements: vec![],
    })
}

fn default_summary() -> UiAllocationConstraintSummary {
    UiAllocationConstraintSummary::new(UiAllocationConstraintSummaryInput {
        incoming_available_space: Some(UiConstraintAxisScope::Both),
        incoming_available_space_posture: None,
        intrinsic_contribution_requirements: Some(UiConstraintAxisScope::Both),
        sibling_negotiation_mode: UiConstraintSiblingNegotiationMode::StablePeerPrimaryAxis,
        equal_share_group: UiConstraintEqualShareGroup::None,
        bounded_min_max_requirements: UiConstraintBoundedMinMaxRequirement::PrimaryAxis,
        viewport_requirement: UiConstraintSpecialInputPosture::NotRequired,
        scroll_owner_requirement: UiConstraintSpecialInputPosture::NotRequired,
        portal_anchor_requirement: UiConstraintSpecialInputPosture::NotRequired,
        resize_permission_posture: UiConstraintResizePermissionPosture::None,
        unit_posture: None,
        coordinate_space: None,
        rounding_posture: None,
    })
}
