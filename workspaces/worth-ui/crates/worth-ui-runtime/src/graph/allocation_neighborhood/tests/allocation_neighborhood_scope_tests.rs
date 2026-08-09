use crate::facade::WorthUiRustAuthoredDeclarationFixture;
use worth_ui_dsl::{
    UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken,
};
use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::declaration::{
    UiDeclaredMeasurementConstraintModifier, UiDeclaredMeasurementMode,
    UiDeclaredMeasurementPolicyPosture,
};
use crate::evidence::measurement::projection::fact_test_support::{
    display_field_projection_context, synthetic_declaration_identity,
};
use crate::evidence::{admit_measurement_basis, UiAllocationNeighborhoodMember};
use crate::facade::WorthUi;
use crate::graph::allocation_neighborhood_test_support::snapshot_with_admitted_layout;
use crate::graph::{
    UiGraphNodeIdentity, UiGraphParticipationReasonCode, UiGraphParticipationStatus,
    UiGraphWorldProfile,
};

#[test]
fn local_intrinsic_neighborhood_stays_root_scoped_even_with_parent_peers() {
    let (_, _, world_profile) = display_field_projection_context("allocation-neighborhood-local");
    let app = neighborhood_app(world_profile.clone());
    let root_node = graph_node_identity_for_provenance(&app, 0);
    let peer_node = graph_node_identity_for_provenance(&app, 1);
    let admitted_snapshot = snapshot_with_admitted_layout(&app, &[root_node]);
    let touch = app
        .try_query_touch_for_node(root_node)
        .expect("query-world touch should admit");
    let selected = app.admission().select_obligations(&touch);
    let basis = admit_measurement_basis(
        synthetic_declaration_identity("allocation-neighborhood-local"),
        root_node,
        world_profile,
        UiEvidenceAuthorityGeneration::new(17),
        &local_intrinsic_policy(),
        &[],
    );

    let neighborhood = selected
        .admit_allocation_neighborhood(&admitted_snapshot, &basis)
        .expect("local intrinsic neighborhood should admit");

    assert_eq!(member_ids(&neighborhood), vec![root_node]);
    assert!(
        !member_ids(&neighborhood).contains(&peer_node),
        "local intrinsic dependency lineage must not silently widen to container peers"
    );
}

#[test]
fn deferred_layout_peer_does_not_enter_container_neighborhood() {
    let (_, _, world_profile) =
        display_field_projection_context("allocation-neighborhood-container-deferred");
    let app = neighborhood_app(world_profile.clone());
    let root_node = graph_node_identity_for_provenance(&app, 0);
    let peer_node = graph_node_identity_for_provenance(&app, 1);
    let admitted_snapshot = snapshot_with_admitted_layout(&app, &[root_node]);
    let touch = app
        .try_query_touch_for_node(root_node)
        .expect("query-world touch should admit");
    let selected = app.admission().select_obligations(&touch);
    let basis = admit_measurement_basis(
        synthetic_declaration_identity("allocation-neighborhood-container-deferred"),
        root_node,
        world_profile,
        UiEvidenceAuthorityGeneration::new(17),
        &container_policy(),
        &[],
    );

    let neighborhood = selected
        .admit_allocation_neighborhood(&admitted_snapshot, &basis)
        .expect("container neighborhood should admit");

    let deferred_peer = neighborhood
        .members()
        .iter()
        .find(|member| member.graph_node_identity() == peer_node);
    if let Some(member) = deferred_peer {
        assert_eq!(
            member.layout_participation().status(),
            UiGraphParticipationStatus::Admitted,
            "deferred layout participants must not survive the neighborhood gate"
        );
    }
}

#[test]
fn deferred_layout_participation_reason_does_not_count_as_neighborhood_membership() {
    let authority = super::super::UiAllocationNeighborhoodMintAuthority::mint();
    let participation = crate::graph::UiGraphAxisParticipation::new(
        UiGraphParticipationStatus::Deferred,
        crate::graph::UiGraphParticipationReasonSource::ReservedRuntimeMutation,
        UiGraphParticipationReasonCode::LayoutAxisAwaitsRuntimeMutation,
        crate::graph::UiGraphParticipationEvidenceHandle::ReservedRuntimeMutationLane,
    );

    assert!(!UiAllocationNeighborhoodMember::new_for_graph_test(
        UiGraphNodeIdentity::new(900),
        900,
        crate::graph::UiRepeatedInstanceBasis::unavailable(),
        participation,
        crate::evidence::UiAllocationNeighborhoodMemberRole::ScopedParticipant,
        None,
        &authority,
    )
    .layout_participates());
}

fn member_ids(
    neighborhood: &crate::evidence::UiAllocationNeighborhood,
) -> Vec<UiGraphNodeIdentity> {
    neighborhood
        .members()
        .iter()
        .map(UiAllocationNeighborhoodMember::graph_node_identity)
        .collect()
}

fn local_intrinsic_policy() -> UiDeclaredMeasurementPolicyPosture {
    UiDeclaredMeasurementPolicyPosture::new(
        Some(UiDeclaredMeasurementMode::HugHeight),
        None,
        None,
        None,
        vec![],
    )
    .expect("local intrinsic measurement policy should admit")
}

fn container_policy() -> UiDeclaredMeasurementPolicyPosture {
    UiDeclaredMeasurementPolicyPosture::new(
        Some(UiDeclaredMeasurementMode::HugHeight),
        Some(UiDeclaredMeasurementConstraintModifier::Bounded),
        None,
        None,
        vec![],
    )
    .expect("container measurement policy should admit")
}

fn neighborhood_app(world_profile: UiGraphWorldProfile) -> crate::facade::WorthUiApp {
    WorthUi::app()
        .bind_certification_host()
        .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse())
        .with_graph_world_profile(world_profile)
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named(
                "worth-ui.runtime.graph.allocation-neighborhood.scope",
            )
            .with_semantic_artifact_spec(control_spec(
                "workflow_editor.control.primary",
                UiDslSemanticFamily::Control,
                0,
                "control:primary",
                Some("touch:press"),
            ))
            .with_semantic_artifact_spec(control_spec(
                "workflow_editor.control.sibling",
                UiDslSemanticFamily::Control,
                1,
                "control:sibling",
                Some("touch:press"),
            ))
            .with_semantic_artifact_spec(control_spec(
                "workflow_editor.diagnostic_surface.lint",
                UiDslSemanticFamily::DiagnosticSurface,
                2,
                "diagnostic-surface:lint",
                None,
            )),
        )
        .freeze()
        .expect("application preparation should succeed")
}

fn control_spec(
    semantic_key: &str,
    family: UiDslSemanticFamily,
    declaration_index: usize,
    structural_token: &str,
    posture_token: Option<&str>,
) -> UiDslSemanticArtifactSpec {
    let spec = UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new(semantic_key),
        family,
        UiDslSourceProvenance::file_authored(
            "app/allocation_neighborhood_scope_tests.wui",
            declaration_index,
        ),
    )
    .with_structural_token(UiDslStructuralToken::new(structural_token));

    let spec = if matches!(family, UiDslSemanticFamily::Control) {
        spec.with_structural_token(UiDslStructuralToken::new("slot:footer"))
            .with_structural_token(UiDslStructuralToken::new("operator:stack"))
    } else {
        spec
    };

    if let Some(posture_token) = posture_token {
        spec.with_posture_token(UiDslPostureToken::new(posture_token))
    } else {
        spec
    }
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
            provenance.module_path() == "app/allocation_neighborhood_scope_tests.wui"
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
