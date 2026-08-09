use crate::declaration::{
    UiDeclarationOrderingGuarantee, UiDeclarationPlanningOperatorKind,
    UiDeclarationRepetitionPosture, UiDeclaredMeasurementBasisSource,
    UiDeclaredMeasurementConstraintModifier, UiDeclaredMeasurementMode,
    UiDeclaredMeasurementPolicyPosture,
};
use crate::evidence::{
    UiAllocationNeighborhood, UiAllocationNeighborhoodClass, UiAllocationNeighborhoodMember,
    UiAllocationNeighborhoodMemberRole, UiAllocationNeighborhoodMembershipRule,
    UiLayoutOperatorContainmentKind, UiLayoutOperatorFamily, UiLayoutOperatorPlanningContract,
    UiLayoutOperatorPlanningContractInput, UiLayoutOperatorSlotParticipationKind,
    UiMeasurementDependencyMap,
};
use crate::facade::{WorthUi, WorthUiRustAuthoredDeclarationFixture};
use crate::graph::{
    UiGraphAxisParticipation, UiGraphGeneration, UiGraphNodeIdentity, UiGraphParticipationStatus,
    UiGraphWorldProfile, UiRepeatedInstanceBasis,
};
use worth_ui_dsl::{
    UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey, UiDslSourceProvenance,
    UiDslStructuralToken,
};

pub(super) fn synthetic_neighborhood(
    layout_operator_contract_identity_digest: u64,
) -> UiAllocationNeighborhood {
    let authority = super::super::UiAllocationNeighborhoodEvidenceTestAuthority::mint();
    UiAllocationNeighborhood::new_for_evidence_test(
        super::super::UiAllocationNeighborhoodInput {
            root_graph_node_identity: UiGraphNodeIdentity::new(801),
            graph_generation: UiGraphGeneration::initial(),
            world_identity_digest: 77,
            graph_snapshot_authority_digest: 77
                ^ UiGraphGeneration::initial().as_u64().rotate_left(11),
            measurement_basis_identity_digest: 88,
            layout_operator_planning_contract: UiLayoutOperatorPlanningContract::new(
                control_contract_input(
                    UiLayoutOperatorContainmentKind::Control,
                    UiLayoutOperatorSlotParticipationKind::DeclaredParticipant,
                    None,
                    vec![if layout_operator_contract_identity_digest == 101 {
                        crate::declaration::UiDeclaredMeasurementEvidenceRequirement::HostFontMetrics
                    } else {
                        crate::declaration::UiDeclaredMeasurementEvidenceRequirement::PortalAnchorMetrics
                    }],
                ),
            ),
            dependency_map: UiMeasurementDependencyMap::new(vec![]),
            neighborhood_class: UiAllocationNeighborhoodClass::ContainerPeerGroup,
            membership_rule: UiAllocationNeighborhoodMembershipRule::ParentSlotPeerGroup,
            members: vec![UiAllocationNeighborhoodMember::new_for_evidence_test(
                UiGraphNodeIdentity::new(801),
                801,
                UiRepeatedInstanceBasis::unavailable(),
                UiGraphAxisParticipation::runtime_mutation(UiGraphParticipationStatus::Admitted),
                UiAllocationNeighborhoodMemberRole::Root,
                None,
                &authority,
            )],
        },
        &authority,
    )
}

pub(super) fn control_contract_input(
    containment_kind: UiLayoutOperatorContainmentKind,
    slot_participation_kind: UiLayoutOperatorSlotParticipationKind,
    basis_source: Option<UiDeclaredMeasurementBasisSource>,
    evidence_requirements: Vec<crate::declaration::UiDeclaredMeasurementEvidenceRequirement>,
) -> UiLayoutOperatorPlanningContractInput {
    UiLayoutOperatorPlanningContractInput {
        operator_kind: UiDeclarationPlanningOperatorKind::Control,
        operator_family: UiLayoutOperatorFamily::Control,
        containment_kind,
        mosaic_sizing_contract_id: None,
        slot_participation_kind,
        ordering_guarantee: UiDeclarationOrderingGuarantee::NotSemanticallyClaimed,
        repetition_posture: UiDeclarationRepetitionPosture::NotAdmitted,
        neighborhood_class: UiAllocationNeighborhoodClass::ContainerPeerGroup,
        membership_rule: UiAllocationNeighborhoodMembershipRule::ParentSlotPeerGroup,
        measurement_mode: Some(UiDeclaredMeasurementMode::HugHeight),
        constraint_modifier: Some(UiDeclaredMeasurementConstraintModifier::Bounded),
        basis_source,
        ownership_posture: None,
        evidence_requirements,
    }
}

pub(super) fn equivalent_contract_app(
    world_profile: UiGraphWorldProfile,
) -> crate::facade::WorthUiApp {
    WorthUi::app()
        .bind_certification_host()
        .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse())
        .with_graph_world_profile(world_profile)
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named(
                "worth-ui.runtime.evidence.allocation-contract",
            )
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
        .expect("application preparation should succeed")
}

pub(super) fn distinct_operator_kind_app(
    world_profile: UiGraphWorldProfile,
) -> crate::facade::WorthUiApp {
    WorthUi::app()
        .bind_certification_host()
        .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse())
        .with_graph_world_profile(world_profile)
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named("worth-ui.runtime.evidence.operator-kind")
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
        .expect("application preparation should succeed")
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

    if let Some(operator_token) = operator_token {
        return spec.with_structural_token(UiDslStructuralToken::new(operator_token));
    }
    spec
}

pub(super) fn container_policy() -> UiDeclaredMeasurementPolicyPosture {
    UiDeclaredMeasurementPolicyPosture::new(
        Some(UiDeclaredMeasurementMode::HugHeight),
        Some(UiDeclaredMeasurementConstraintModifier::Bounded),
        None,
        None,
        vec![],
    )
    .expect("container policy should admit")
}

pub(super) fn graph_node_identity_for_provenance(
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
