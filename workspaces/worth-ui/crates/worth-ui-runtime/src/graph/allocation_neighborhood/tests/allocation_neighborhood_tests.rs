use crate::facade::WorthUiRustAuthoredDeclarationFixture;
use worth_ui_dsl::{
    UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken,
};
use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::declaration::{
    UiDeclaredMeasurementBasisSource, UiDeclaredMeasurementConstraintModifier,
    UiDeclaredMeasurementEvidenceRequirement, UiDeclaredMeasurementMode,
    UiDeclaredMeasurementOwnershipPosture, UiDeclaredMeasurementPolicyPosture,
};
use crate::evidence::measurement::projection::fact_test_support::{
    capability_report, display_field_projection_context, host_result_font_metrics,
    host_result_portal_anchor, host_result_scroll_container_viewport, host_result_viewport_extent,
    scroll_viewport_policy, synthetic_declaration_identity,
};
use crate::evidence::{
    admit_measurement_basis, consume_declared_measurement_projection_facts,
    MeasurementEvidenceInput, UiAllocationNeighborhood, UiAllocationNeighborhoodClass,
    UiAllocationNeighborhoodMember,
};
use crate::facade::WorthUi;
use crate::graph::allocation_neighborhood_test_support::snapshot_with_admitted_layout;
use crate::graph::{UiGraphNodeIdentity, UiGraphWorldProfile};

#[test]
fn container_neighborhood_uses_parent_peers_and_excludes_non_layout_siblings() {
    let (_, _, world_profile) =
        display_field_projection_context("allocation-neighborhood-container");
    let app = neighborhood_app(world_profile.clone());
    let root_node = graph_node_identity_for_provenance(&app, 0);
    let peer_node = graph_node_identity_for_provenance(&app, 1);
    let diagnostic_node = graph_node_identity_for_provenance(&app, 2);
    let admitted_snapshot = snapshot_with_admitted_layout(&app, &[root_node, peer_node]);
    let touch = app
        .try_query_touch_for_node(root_node)
        .expect("query-world touch should admit");
    let selected = app.admission().select_obligations(&touch);
    let basis = admit_measurement_basis(
        synthetic_declaration_identity("allocation-neighborhood-container"),
        root_node,
        world_profile,
        UiEvidenceAuthorityGeneration::new(17),
        &container_policy(),
        &[],
    );

    let neighborhood = selected
        .admit_allocation_neighborhood(&admitted_snapshot, &basis)
        .expect("container neighborhood should admit");

    let member_ids = member_ids(&neighborhood);
    assert_eq!(
        neighborhood.neighborhood_class(),
        UiAllocationNeighborhoodClass::ContainerPeerGroup
    );
    assert!(member_ids.contains(&root_node));
    assert!(
        member_ids.contains(&peer_node),
        "peer node missing from container neighborhood; root_topology={:?}; peer_topology={:?}; members={member_ids:?}",
        app.graph_snapshot().topology().node_topology(root_node),
        app.graph_snapshot().topology().node_topology(peer_node),
    );
    assert!(
        !member_ids.contains(&diagnostic_node),
        "non-layout sibling must not silently join the peer group"
    );
    assert!(neighborhood
        .members()
        .iter()
        .all(UiAllocationNeighborhoodMember::layout_participates));
}

#[test]
fn viewport_neighborhood_admits_parent_peers_without_non_layout_broadening() {
    let generation = UiEvidenceAuthorityGeneration::new(17);
    let declaration_identity = synthetic_declaration_identity("allocation-neighborhood-viewport");
    let (prerequisites, attempt, world_profile) =
        display_field_projection_context("allocation-neighborhood-viewport");
    let app = neighborhood_app(world_profile.clone());
    let root_node = graph_node_identity_for_provenance(&app, 0);
    let peer_node = graph_node_identity_for_provenance(&app, 1);
    let diagnostic_node = graph_node_identity_for_provenance(&app, 2);
    let admitted_snapshot = snapshot_with_admitted_layout(&app, &[root_node, peer_node]);
    let touch = app
        .try_query_touch_for_node(root_node)
        .expect("query-world touch should admit");
    let selected = app.admission().select_obligations(&touch);
    let receipt = consume_declared_measurement_projection_facts(
        declaration_identity.clone(),
        generation,
        &scroll_viewport_policy(),
        prerequisites,
        &attempt,
    )
    .expect("query receipt should admit");
    let capability_report = capability_report(77);
    let basis = admit_measurement_basis(
        declaration_identity,
        root_node,
        world_profile,
        generation,
        &scroll_viewport_policy(),
        &[
            MeasurementEvidenceInput::settled_query_fact(&receipt),
            MeasurementEvidenceInput::host_capability_report(&capability_report),
            MeasurementEvidenceInput::host_measurement_result(&host_result_font_metrics(
                31,
                &capability_report,
                generation,
            )),
            MeasurementEvidenceInput::host_measurement_result(&host_result_viewport_extent(
                32,
                &capability_report,
                generation,
            )),
        ],
    );

    let neighborhood = selected
        .admit_allocation_neighborhood(&admitted_snapshot, &basis)
        .expect("viewport neighborhood should admit");

    let member_ids = member_ids(&neighborhood);
    assert_eq!(
        neighborhood.neighborhood_class(),
        UiAllocationNeighborhoodClass::Viewport
    );
    assert!(member_ids.contains(&root_node));
    assert!(
        member_ids.contains(&peer_node),
        "viewport neighborhood should preserve parent-slot peers for bounded planning when the operator admits layout participation"
    );
    assert!(
        !member_ids.contains(&diagnostic_node),
        "viewport scope must still honor layout participation as the gate"
    );
}

#[test]
fn scroll_container_neighborhood_admits_parent_peers_without_region_broadening() {
    let generation = UiEvidenceAuthorityGeneration::new(17);
    let declaration_identity = synthetic_declaration_identity("allocation-neighborhood-scroll");
    let (_, _, world_profile) = display_field_projection_context("allocation-neighborhood-scroll");
    let app = neighborhood_app(world_profile.clone());
    let root_node = graph_node_identity_for_provenance(&app, 0);
    let peer_node = graph_node_identity_for_provenance(&app, 1);
    let diagnostic_node = graph_node_identity_for_provenance(&app, 2);
    let admitted_snapshot = snapshot_with_admitted_layout(&app, &[root_node, peer_node]);
    let touch = app
        .try_query_touch_for_node(root_node)
        .expect("query-world touch should admit");
    let selected = app.admission().select_obligations(&touch);
    let capability_report = capability_report(77);
    let basis = admit_measurement_basis(
        declaration_identity,
        root_node,
        world_profile,
        generation,
        &scroll_container_policy(),
        &[
            MeasurementEvidenceInput::host_capability_report(&capability_report),
            MeasurementEvidenceInput::host_measurement_result(
                &host_result_scroll_container_viewport(41, &capability_report, generation),
            ),
        ],
    );

    let neighborhood = selected
        .admit_allocation_neighborhood(&admitted_snapshot, &basis)
        .expect("scroll-container neighborhood should admit");

    assert_eq!(
        neighborhood.neighborhood_class(),
        UiAllocationNeighborhoodClass::ScrollContainer
    );
    assert!(member_ids(&neighborhood).contains(&root_node));
    assert!(
        member_ids(&neighborhood).contains(&peer_node),
        "scroll-container neighborhood should preserve parent-slot peers for bounded planning"
    );
    assert!(
        !member_ids(&neighborhood).contains(&diagnostic_node),
        "scroll-container neighborhood must not widen to unrelated non-layout members"
    );
}

#[test]
fn portal_anchor_neighborhood_stays_root_scoped_without_page_broadening() {
    let generation = UiEvidenceAuthorityGeneration::new(17);
    let declaration_identity = synthetic_declaration_identity("allocation-neighborhood-portal");
    let (prerequisites, attempt, world_profile) =
        display_field_projection_context("allocation-neighborhood-portal");
    let app = neighborhood_app(world_profile.clone());
    let root_node = graph_node_identity_for_provenance(&app, 0);
    let peer_node = graph_node_identity_for_provenance(&app, 1);
    let admitted_snapshot = snapshot_with_admitted_layout(&app, &[root_node]);
    let touch = app
        .try_query_touch_for_node(root_node)
        .expect("query-world touch should admit");
    let selected = app.admission().select_obligations(&touch);
    let receipt = consume_declared_measurement_projection_facts(
        declaration_identity.clone(),
        generation,
        &scroll_viewport_policy(),
        prerequisites,
        &attempt,
    )
    .expect("query receipt should admit");
    let capability_report = capability_report(77);
    let basis = admit_measurement_basis(
        declaration_identity,
        root_node,
        world_profile,
        generation,
        &portal_anchor_policy(),
        &[
            MeasurementEvidenceInput::settled_query_fact(&receipt),
            MeasurementEvidenceInput::host_capability_report(&capability_report),
            MeasurementEvidenceInput::host_measurement_result(&host_result_font_metrics(
                51,
                &capability_report,
                generation,
            )),
            MeasurementEvidenceInput::host_measurement_result(&host_result_portal_anchor(
                52,
                &capability_report,
                generation,
            )),
        ],
    );

    let neighborhood = selected
        .admit_allocation_neighborhood(&admitted_snapshot, &basis)
        .expect("portal-anchor neighborhood should admit");

    assert_eq!(
        neighborhood.neighborhood_class(),
        UiAllocationNeighborhoodClass::PortalAnchor
    );
    assert_eq!(member_ids(&neighborhood), vec![root_node]);
    assert!(
        !member_ids(&neighborhood).contains(&peer_node),
        "portal-anchor neighborhood must not widen to unrelated page members by default"
    );
}

fn member_ids(neighborhood: &UiAllocationNeighborhood) -> Vec<UiGraphNodeIdentity> {
    neighborhood
        .members()
        .iter()
        .map(UiAllocationNeighborhoodMember::graph_node_identity)
        .collect()
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

fn portal_anchor_policy() -> UiDeclaredMeasurementPolicyPosture {
    UiDeclaredMeasurementPolicyPosture::new(
        None,
        None,
        Some(UiDeclaredMeasurementBasisSource::PortalAnchor),
        Some(UiDeclaredMeasurementOwnershipPosture::PortalAnchorBasisRequired),
        vec![UiDeclaredMeasurementEvidenceRequirement::PortalAnchorMetrics],
    )
    .expect("portal anchor measurement policy should admit")
}

fn scroll_container_policy() -> UiDeclaredMeasurementPolicyPosture {
    UiDeclaredMeasurementPolicyPosture::new(
        Some(UiDeclaredMeasurementMode::HugHeight),
        Some(UiDeclaredMeasurementConstraintModifier::Bounded),
        None,
        Some(UiDeclaredMeasurementOwnershipPosture::ScrollContainerBasis),
        vec![],
    )
    .expect("scroll container measurement policy should admit")
}

fn neighborhood_app(world_profile: UiGraphWorldProfile) -> crate::facade::WorthUiApp {
    WorthUi::app()
        .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse())
        .with_graph_world_profile(world_profile)
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named(
                "worth-ui.runtime.graph.allocation-neighborhood",
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
            "app/allocation_neighborhood_tests.wui",
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
            provenance.module_path() == "app/allocation_neighborhood_tests.wui"
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
