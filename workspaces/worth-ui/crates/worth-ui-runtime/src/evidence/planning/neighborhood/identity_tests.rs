use worth_ui_dsl::{
    UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey, UiDslSourceProvenance,
    UiDslStructuralToken, WorthUiDslPackage,
};
use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::declaration::{
    UiDeclarationOrderingGuarantee, UiDeclarationPlanningOperatorKind,
    UiDeclarationRepetitionPosture, UiDeclaredMeasurementBasisSource,
    UiDeclaredMeasurementConstraintModifier, UiDeclaredMeasurementMode,
    UiDeclaredMeasurementPolicyPosture,
};
use crate::evidence::measurement::projection::fact_test_support::synthetic_declaration_identity;
use crate::evidence::{
    admit_measurement_basis, UiAllocationNeighborhood, UiAllocationNeighborhoodClass,
    UiAllocationNeighborhoodMember, UiAllocationNeighborhoodMemberRole,
    UiAllocationNeighborhoodMembershipRule, UiLayoutOperatorContainmentKind,
    UiLayoutOperatorFamily, UiLayoutOperatorPlanningContract,
    UiLayoutOperatorSlotParticipationKind, UiMeasurementDependencyMap,
};
use crate::facade::WorthUi;
use crate::graph::allocation_neighborhood_test_support::snapshot_with_admitted_layout;
use crate::graph::{
    UiGraphAxisParticipation, UiGraphGeneration, UiGraphNodeIdentity, UiGraphParticipationStatus,
    UiGraphWorldProfile, UiRepeatedInstanceBasis,
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
    let left = UiLayoutOperatorPlanningContract::new(
        UiDeclarationPlanningOperatorKind::Control,
        UiLayoutOperatorFamily::Control,
        UiLayoutOperatorContainmentKind::Control,
        None,
        UiLayoutOperatorSlotParticipationKind::DeclaredParticipant,
        UiDeclarationOrderingGuarantee::NotSemanticallyClaimed,
        UiDeclarationRepetitionPosture::NotAdmitted,
        UiAllocationNeighborhoodClass::ContainerPeerGroup,
        UiAllocationNeighborhoodMembershipRule::ParentSlotPeerGroup,
        Some(UiDeclaredMeasurementMode::HugHeight),
        Some(UiDeclaredMeasurementConstraintModifier::Bounded),
        None,
        None,
        vec![],
    );
    let right = UiLayoutOperatorPlanningContract::new(
        UiDeclarationPlanningOperatorKind::Mosaic,
        UiLayoutOperatorFamily::Mosaic,
        UiLayoutOperatorContainmentKind::Mosaic,
        None,
        UiLayoutOperatorSlotParticipationKind::None,
        UiDeclarationOrderingGuarantee::NotSemanticallyClaimed,
        UiDeclarationRepetitionPosture::NotAdmitted,
        UiAllocationNeighborhoodClass::LocalIntrinsicContent,
        UiAllocationNeighborhoodMembershipRule::RootOnly,
        Some(UiDeclaredMeasurementMode::HugHeight),
        Some(UiDeclaredMeasurementConstraintModifier::Bounded),
        None,
        None,
        vec![],
    );

    assert_ne!(left.identity(), right.identity());
}

#[test]
fn distinct_containment_kinds_do_not_collapse_with_identical_policy_and_scope() {
    let left = UiLayoutOperatorPlanningContract::new(
        UiDeclarationPlanningOperatorKind::Control,
        UiLayoutOperatorFamily::Control,
        UiLayoutOperatorContainmentKind::Control,
        None,
        UiLayoutOperatorSlotParticipationKind::DeclaredParticipant,
        UiDeclarationOrderingGuarantee::NotSemanticallyClaimed,
        UiDeclarationRepetitionPosture::NotAdmitted,
        UiAllocationNeighborhoodClass::ContainerPeerGroup,
        UiAllocationNeighborhoodMembershipRule::ParentSlotPeerGroup,
        Some(UiDeclaredMeasurementMode::HugHeight),
        Some(UiDeclaredMeasurementConstraintModifier::Bounded),
        Some(UiDeclaredMeasurementBasisSource::PortalAnchor),
        None,
        vec![],
    );
    let right = UiLayoutOperatorPlanningContract::new(
        UiDeclarationPlanningOperatorKind::Control,
        UiLayoutOperatorFamily::Control,
        UiLayoutOperatorContainmentKind::DiagnosticSurface,
        None,
        UiLayoutOperatorSlotParticipationKind::DeclaredParticipant,
        UiDeclarationOrderingGuarantee::NotSemanticallyClaimed,
        UiDeclarationRepetitionPosture::NotAdmitted,
        UiAllocationNeighborhoodClass::ContainerPeerGroup,
        UiAllocationNeighborhoodMembershipRule::ParentSlotPeerGroup,
        Some(UiDeclaredMeasurementMode::HugHeight),
        Some(UiDeclaredMeasurementConstraintModifier::Bounded),
        Some(UiDeclaredMeasurementBasisSource::PortalAnchor),
        None,
        vec![],
    );

    assert_ne!(left.identity(), right.identity());
}

#[test]
fn distinct_slot_participation_kinds_do_not_collapse_with_identical_family_and_policy() {
    let left = UiLayoutOperatorPlanningContract::new(
        UiDeclarationPlanningOperatorKind::Control,
        UiLayoutOperatorFamily::Control,
        UiLayoutOperatorContainmentKind::Control,
        None,
        UiLayoutOperatorSlotParticipationKind::DeclaredParticipant,
        UiDeclarationOrderingGuarantee::NotSemanticallyClaimed,
        UiDeclarationRepetitionPosture::NotAdmitted,
        UiAllocationNeighborhoodClass::ContainerPeerGroup,
        UiAllocationNeighborhoodMembershipRule::ParentSlotPeerGroup,
        Some(UiDeclaredMeasurementMode::HugHeight),
        Some(UiDeclaredMeasurementConstraintModifier::Bounded),
        Some(UiDeclaredMeasurementBasisSource::PortalAnchor),
        None,
        vec![],
    );
    let right = UiLayoutOperatorPlanningContract::new(
        UiDeclarationPlanningOperatorKind::Control,
        UiLayoutOperatorFamily::Control,
        UiLayoutOperatorContainmentKind::Control,
        None,
        UiLayoutOperatorSlotParticipationKind::None,
        UiDeclarationOrderingGuarantee::NotSemanticallyClaimed,
        UiDeclarationRepetitionPosture::NotAdmitted,
        UiAllocationNeighborhoodClass::ContainerPeerGroup,
        UiAllocationNeighborhoodMembershipRule::ParentSlotPeerGroup,
        Some(UiDeclaredMeasurementMode::HugHeight),
        Some(UiDeclaredMeasurementConstraintModifier::Bounded),
        Some(UiDeclaredMeasurementBasisSource::PortalAnchor),
        None,
        vec![],
    );

    assert_ne!(left.identity(), right.identity());
}

fn synthetic_neighborhood(
    layout_operator_contract_identity_digest: u64,
) -> UiAllocationNeighborhood {
    UiAllocationNeighborhood::new_with_authority(
        UiGraphNodeIdentity::new(801),
        UiGraphGeneration::initial(),
        77,
        88,
        UiLayoutOperatorPlanningContract::new(
            UiDeclarationPlanningOperatorKind::Control,
            UiLayoutOperatorFamily::Control,
            UiLayoutOperatorContainmentKind::Control,
            None,
            UiLayoutOperatorSlotParticipationKind::DeclaredParticipant,
            UiDeclarationOrderingGuarantee::NotSemanticallyClaimed,
            UiDeclarationRepetitionPosture::NotAdmitted,
            UiAllocationNeighborhoodClass::ContainerPeerGroup,
            UiAllocationNeighborhoodMembershipRule::ParentSlotPeerGroup,
            None,
            None,
            None,
            None,
            vec![if layout_operator_contract_identity_digest == 101 {
                crate::declaration::UiDeclaredMeasurementEvidenceRequirement::HostFontMetrics
            } else {
                crate::declaration::UiDeclaredMeasurementEvidenceRequirement::PortalAnchorMetrics
            }],
        ),
        UiMeasurementDependencyMap::new(vec![]),
        UiAllocationNeighborhoodClass::ContainerPeerGroup,
        UiAllocationNeighborhoodMembershipRule::ParentSlotPeerGroup,
        vec![UiAllocationNeighborhoodMember::new(
            UiGraphNodeIdentity::new(801),
            801,
            UiRepeatedInstanceBasis::unavailable(),
            UiGraphAxisParticipation::runtime_mutation(UiGraphParticipationStatus::Admitted),
            UiAllocationNeighborhoodMemberRole::Root,
            None,
        )],
    )
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

fn equivalent_contract_app(world_profile: UiGraphWorldProfile) -> crate::facade::WorthUiApp {
    WorthUi::app()
        .with_graph_world_profile(world_profile)
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.runtime.evidence.allocation-contract")
                .with_semantic_artifact_spec(control_spec(
                    "workflow_editor.control.primary",
                    0,
                    "control:primary",
                    Some("operator:stack"),
                ))
                .with_semantic_artifact_spec(control_spec(
                    "workflow_editor.control.secondary",
                    1,
                    "control:secondary",
                    Some("operator:stack"),
                )),
        )
        .freeze()
}

fn distinct_operator_kind_app(world_profile: UiGraphWorldProfile) -> crate::facade::WorthUiApp {
    WorthUi::app()
        .with_graph_world_profile(world_profile)
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.runtime.evidence.operator-kind")
                .with_semantic_artifact_spec(control_spec(
                    "workflow_editor.control.stack",
                    0,
                    "control:left-pane",
                    Some("operator:stack"),
                ))
                .with_semantic_artifact_spec(control_spec(
                    "workflow_editor.control.row",
                    1,
                    "control:right-pane",
                    Some("operator:row"),
                )),
        )
        .freeze()
}

fn control_spec(
    semantic_key: &str,
    declaration_index: usize,
    structural_token: &str,
    operator_token: Option<&str>,
) -> UiDslSemanticArtifactSpec {
    let spec = UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new(semantic_key),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored(
            "app/allocation_neighborhood_identity_tests.wui",
            declaration_index,
        ),
    )
    .with_structural_token(UiDslStructuralToken::new(structural_token))
    .with_structural_token(UiDslStructuralToken::new("slot:footer"));

    match operator_token {
        Some(operator_token) => {
            spec.with_structural_token(UiDslStructuralToken::new(operator_token))
        }
        None => spec,
    }
}

fn container_policy() -> UiDeclaredMeasurementPolicyPosture {
    UiDeclaredMeasurementPolicyPosture::new(
        Some(UiDeclaredMeasurementMode::HugHeight),
        Some(UiDeclaredMeasurementConstraintModifier::Bounded),
        None,
        None,
        vec![],
    )
    .expect("container policy should admit")
}

fn graph_node_identity_for_provenance(
    app: &crate::facade::WorthUiApp,
    declaration_index: usize,
) -> UiGraphNodeIdentity {
    let artifact = app
        .declaration_artifacts()
        .iter()
        .find(|artifact| {
            let provenance = artifact.provenance().source_provenance();
            provenance.module_path() == "app/allocation_neighborhood_identity_tests.wui"
                && provenance.declaration_index() == declaration_index
        })
        .expect("expected declaration artifact for requested provenance row");
    app.graph()
        .lookup()
        .declaration_instances(artifact.identity())
        .value()
        .first()
        .copied()
        .expect("declaration should project one graph node")
}
