use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::declaration::{
    UiDeclarationOrderingGuarantee, UiDeclarationPlanningOperatorKind,
    UiDeclarationRepetitionPosture, UiDeclaredMeasurementBasisSource,
    UiDeclaredMeasurementConstraintModifier, UiDeclaredMeasurementMode,
};
use crate::evidence::measurement::projection::fact_test_support::synthetic_declaration_identity;
use crate::evidence::{
    admit_measurement_basis, UiAllocationNeighborhoodClass, UiAllocationNeighborhoodMembershipRule,
    UiLayoutOperatorContainmentKind, UiLayoutOperatorFamily, UiLayoutOperatorPlanningContract,
    UiLayoutOperatorPlanningContractInput, UiLayoutOperatorSlotParticipationKind,
};
use crate::graph::allocation_neighborhood_test_support::snapshot_with_admitted_layout;
use crate::graph::UiGraphWorldProfile;

#[path = "identity_test_support.rs"]
mod identity_test_support;

use identity_test_support::{
    container_policy, control_contract_input, distinct_operator_kind_app, equivalent_contract_app,
    graph_node_identity_for_provenance, synthetic_neighborhood,
};

#[test]
fn allocation_neighborhood_identity_includes_layout_operator_contract_identity() {
    let left = synthetic_neighborhood(101);
    let right = synthetic_neighborhood(202);

    assert_ne!(
        left.layout_operator_contract_identity_digest(),
        right.layout_operator_contract_identity_digest()
    );
    assert_ne!(left.identity(), right.identity());
    assert_ne!(
        left.identity().identity_digest(),
        right.identity().identity_digest()
    );
}

#[test]
fn equivalent_declared_operator_contracts_converge_across_declarations() {
    let world_profile = UiGraphWorldProfile::authoritative();
    let app = equivalent_contract_app(world_profile.clone());
    let left_node = graph_node_identity_for_provenance(&app, 0);
    let right_node = graph_node_identity_for_provenance(&app, 1);
    let admitted_snapshot = snapshot_with_admitted_layout(&app, &[left_node, right_node]);
    let policy = container_policy();
    let left_basis = admit_measurement_basis(
        synthetic_declaration_identity("operator-contract-left"),
        left_node,
        world_profile.clone(),
        UiEvidenceAuthorityGeneration::new(5),
        &policy,
        &[],
    );
    let right_basis = admit_measurement_basis(
        synthetic_declaration_identity("operator-contract-right"),
        right_node,
        world_profile,
        UiEvidenceAuthorityGeneration::new(5),
        &policy,
        &[],
    );

    let left = left_basis
        .admit_allocation_neighborhood_from_graph(&admitted_snapshot)
        .expect("left declaration should admit allocation neighborhood");
    let right = right_basis
        .admit_allocation_neighborhood_from_graph(&admitted_snapshot)
        .expect("right declaration should admit allocation neighborhood");

    assert_eq!(
        left.layout_operator_planning_contract(),
        right.layout_operator_planning_contract()
    );
    assert_eq!(
        left.layout_operator_planning_contract().containment_kind(),
        UiLayoutOperatorContainmentKind::Control
    );
    assert_eq!(
        left.layout_operator_planning_contract()
            .slot_participation_kind(),
        UiLayoutOperatorSlotParticipationKind::DeclaredParticipant
    );
}

#[test]
fn distinct_planning_contracts_do_not_collapse_with_shared_policy_knobs() {
    let left = UiLayoutOperatorPlanningContract::new(UiLayoutOperatorPlanningContractInput {
        operator_kind: UiDeclarationPlanningOperatorKind::Control,
        operator_family: UiLayoutOperatorFamily::Control,
        containment_kind: UiLayoutOperatorContainmentKind::Control,
        mosaic_sizing_contract_id: None,
        slot_participation_kind: UiLayoutOperatorSlotParticipationKind::DeclaredParticipant,
        ordering_guarantee: UiDeclarationOrderingGuarantee::NotSemanticallyClaimed,
        repetition_posture: UiDeclarationRepetitionPosture::NotAdmitted,
        neighborhood_class: UiAllocationNeighborhoodClass::ContainerPeerGroup,
        membership_rule: UiAllocationNeighborhoodMembershipRule::ParentSlotPeerGroup,
        measurement_mode: Some(UiDeclaredMeasurementMode::HugHeight),
        constraint_modifier: Some(UiDeclaredMeasurementConstraintModifier::Bounded),
        basis_source: None,
        ownership_posture: None,
        evidence_requirements: vec![],
    });
    let right = UiLayoutOperatorPlanningContract::new(UiLayoutOperatorPlanningContractInput {
        operator_kind: UiDeclarationPlanningOperatorKind::Mosaic,
        operator_family: UiLayoutOperatorFamily::Mosaic,
        containment_kind: UiLayoutOperatorContainmentKind::Mosaic,
        mosaic_sizing_contract_id: None,
        slot_participation_kind: UiLayoutOperatorSlotParticipationKind::None,
        ordering_guarantee: UiDeclarationOrderingGuarantee::NotSemanticallyClaimed,
        repetition_posture: UiDeclarationRepetitionPosture::NotAdmitted,
        neighborhood_class: UiAllocationNeighborhoodClass::LocalIntrinsicContent,
        membership_rule: UiAllocationNeighborhoodMembershipRule::RootOnly,
        measurement_mode: Some(UiDeclaredMeasurementMode::HugHeight),
        constraint_modifier: Some(UiDeclaredMeasurementConstraintModifier::Bounded),
        basis_source: None,
        ownership_posture: None,
        evidence_requirements: vec![],
    });

    assert_ne!(left.identity(), right.identity());
}

#[test]
fn distinct_containment_kinds_do_not_collapse_with_identical_policy_and_scope() {
    let left = UiLayoutOperatorPlanningContract::new(UiLayoutOperatorPlanningContractInput {
        operator_kind: UiDeclarationPlanningOperatorKind::Control,
        operator_family: UiLayoutOperatorFamily::Control,
        containment_kind: UiLayoutOperatorContainmentKind::Control,
        mosaic_sizing_contract_id: None,
        slot_participation_kind: UiLayoutOperatorSlotParticipationKind::DeclaredParticipant,
        ordering_guarantee: UiDeclarationOrderingGuarantee::NotSemanticallyClaimed,
        repetition_posture: UiDeclarationRepetitionPosture::NotAdmitted,
        neighborhood_class: UiAllocationNeighborhoodClass::ContainerPeerGroup,
        membership_rule: UiAllocationNeighborhoodMembershipRule::ParentSlotPeerGroup,
        measurement_mode: Some(UiDeclaredMeasurementMode::HugHeight),
        constraint_modifier: Some(UiDeclaredMeasurementConstraintModifier::Bounded),
        basis_source: Some(UiDeclaredMeasurementBasisSource::PortalAnchor),
        ownership_posture: None,
        evidence_requirements: vec![],
    });
    let right = UiLayoutOperatorPlanningContract::new(UiLayoutOperatorPlanningContractInput {
        operator_kind: UiDeclarationPlanningOperatorKind::Control,
        operator_family: UiLayoutOperatorFamily::Control,
        containment_kind: UiLayoutOperatorContainmentKind::DiagnosticSurface,
        mosaic_sizing_contract_id: None,
        slot_participation_kind: UiLayoutOperatorSlotParticipationKind::DeclaredParticipant,
        ordering_guarantee: UiDeclarationOrderingGuarantee::NotSemanticallyClaimed,
        repetition_posture: UiDeclarationRepetitionPosture::NotAdmitted,
        neighborhood_class: UiAllocationNeighborhoodClass::ContainerPeerGroup,
        membership_rule: UiAllocationNeighborhoodMembershipRule::ParentSlotPeerGroup,
        measurement_mode: Some(UiDeclaredMeasurementMode::HugHeight),
        constraint_modifier: Some(UiDeclaredMeasurementConstraintModifier::Bounded),
        basis_source: Some(UiDeclaredMeasurementBasisSource::PortalAnchor),
        ownership_posture: None,
        evidence_requirements: vec![],
    });

    assert_ne!(left.identity(), right.identity());
}

#[test]
fn distinct_slot_participation_kinds_do_not_collapse_with_identical_family_and_policy() {
    let left = UiLayoutOperatorPlanningContract::new(control_contract_input(
        UiLayoutOperatorContainmentKind::Control,
        UiLayoutOperatorSlotParticipationKind::DeclaredParticipant,
        Some(UiDeclaredMeasurementBasisSource::PortalAnchor),
        vec![],
    ));
    let right = UiLayoutOperatorPlanningContract::new(control_contract_input(
        UiLayoutOperatorContainmentKind::Control,
        UiLayoutOperatorSlotParticipationKind::None,
        Some(UiDeclaredMeasurementBasisSource::PortalAnchor),
        vec![],
    ));

    assert_ne!(left.identity(), right.identity());
}

#[test]
fn spec_named_operator_kinds_do_not_collapse_with_identical_policy_and_scope() {
    let world_profile = UiGraphWorldProfile::authoritative();
    let app = distinct_operator_kind_app(world_profile.clone());
    let left_node = graph_node_identity_for_provenance(&app, 0);
    let right_node = graph_node_identity_for_provenance(&app, 1);
    let admitted_snapshot = snapshot_with_admitted_layout(&app, &[left_node, right_node]);
    let policy = container_policy();
    let left_basis = admit_measurement_basis(
        synthetic_declaration_identity("operator-kind-left"),
        left_node,
        world_profile.clone(),
        UiEvidenceAuthorityGeneration::new(9),
        &policy,
        &[],
    );
    let right_basis = admit_measurement_basis(
        synthetic_declaration_identity("operator-kind-right"),
        right_node,
        world_profile,
        UiEvidenceAuthorityGeneration::new(9),
        &policy,
        &[],
    );

    let left = left_basis
        .admit_allocation_neighborhood_from_graph(&admitted_snapshot)
        .expect("stack declaration should admit allocation neighborhood");
    let right = right_basis
        .admit_allocation_neighborhood_from_graph(&admitted_snapshot)
        .expect("row declaration should admit allocation neighborhood");

    assert_eq!(
        left.layout_operator_planning_contract().operator_kind(),
        UiDeclarationPlanningOperatorKind::Stack
    );
    assert_eq!(
        right.layout_operator_planning_contract().operator_kind(),
        UiDeclarationPlanningOperatorKind::Row
    );
    assert_ne!(
        left.layout_operator_planning_contract(),
        right.layout_operator_planning_contract()
    );
    assert_ne!(
        left.layout_operator_contract_identity_digest(),
        right.layout_operator_contract_identity_digest()
    );
}
