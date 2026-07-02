use std::panic::{catch_unwind, AssertUnwindSafe};

use worth_ui::facade::app::WorthUi;
use worth_ui::facade::declaration::UiDeclarationArtifact;
use worth_ui::facade::graph::{
    UiGraphContainmentClaim, UiGraphNodeIdentity, UiGraphParentResolutionClaim,
};
use worth_ui_dsl::{
    UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken, WorthUiDslPackage,
};

#[test]
fn public_freeze_materializes_parent_child_slot_topology_as_graph_truth() {
    let app = WorthUi::app()
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.graph-topology.slot")
                .with_semantic_artifact_spec(slotted_control_spec()),
        )
        .freeze();
    let graph = app.graph();
    let root_page = root_page_artifact(&app);
    let control = artifact_from_file_provenance(&app, "app/graph_topology.wui", 0);
    let root_page_id = graph_node_identity(graph, root_page);
    let control_id = graph_node_identity(graph, control);
    let root_topology = graph
        .inspection()
        .inspect_topology_node(root_page_id)
        .expect("topology inspection should resolve admitted node topology")
        .value();
    let control_topology = graph
        .inspection()
        .inspect_topology_node(control_id)
        .expect("topology inspection should resolve admitted node topology")
        .value();

    assert_eq!(root_topology.parent_node_identity(), None);
    assert_eq!(root_topology.containment_claim(), &UiGraphContainmentClaim::RootPage);
    assert_eq!(
        root_topology.parent_resolution_claim(),
        &UiGraphParentResolutionClaim::RootPage
    );
    assert_eq!(
        root_topology
            .page_membership()
            .expect("root page should own page membership")
            .page_node_identity(),
        root_page_id
    );

    assert_eq!(control_topology.parent_node_identity(), Some(root_page_id));
    assert_eq!(
        control_topology.parent_resolution_claim(),
        &UiGraphParentResolutionClaim::ContainedByRootPage
    );
    assert_eq!(
        control_topology.containment_claim(),
        &UiGraphContainmentClaim::Control {
            control_name: "save".into(),
        }
    );
    assert_eq!(
        control_topology
            .slot_topology()
            .expect("slotted control should publish slot topology")
            .slot_name(),
        "footer"
    );
    assert_eq!(
        control_topology
            .page_membership()
            .expect("control should inherit page membership from root page")
            .page_node_identity(),
        root_page_id
    );

    assert!(graph
        .lookup()
        .child_nodes(root_page_id)
        .value()
        .contains(&control_id));
    assert!(graph
        .lookup()
        .slot_occupants(root_page_id, "footer")
        .value()
        .contains(&control_id));
}

#[test]
fn public_freeze_exposes_explicit_region_and_mosaic_membership_indexes() {
    let app = WorthUi::app()
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.graph-topology.membership")
                .with_semantic_artifact_spec(region_spec())
                .with_semantic_artifact_spec(mosaic_spec()),
        )
        .freeze();
    let graph = app.graph();
    let root_page = root_page_artifact(&app);
    let region = artifact_from_file_provenance(&app, "app/graph_topology.wui", 1);
    let mosaic = artifact_from_file_provenance(&app, "app/graph_topology.wui", 2);
    let root_page_id = graph_node_identity(graph, root_page);
    let region_id = graph_node_identity(graph, region);
    let mosaic_id = graph_node_identity(graph, mosaic);
    let region_topology = graph
        .inspection()
        .inspect_topology_node(region_id)
        .expect("topology inspection should resolve admitted node topology")
        .value();
    let mosaic_topology = graph
        .inspection()
        .inspect_topology_node(mosaic_id)
        .expect("topology inspection should resolve admitted node topology")
        .value();

    assert_eq!(
        region_topology.containment_claim(),
        &UiGraphContainmentClaim::Region {
            region_name: "sidebar".into(),
        }
    );
    assert_eq!(
        mosaic_topology.containment_claim(),
        &UiGraphContainmentClaim::Mosaic {
            mosaic_name: "workspace".into(),
        }
    );
    assert_eq!(
        region_topology
            .region_membership()
            .expect("region declaration should materialize region membership")
            .region_name(),
        "sidebar"
    );
    assert_eq!(
        mosaic_topology
            .mosaic_membership()
            .expect("mosaic declaration should materialize mosaic membership")
            .mosaic_name(),
        "workspace"
    );
    assert!(graph
        .lookup()
        .region_members("sidebar")
        .value()
        .contains(&region_id));
    assert!(graph
        .lookup()
        .mosaic_members("workspace")
        .value()
        .contains(&mosaic_id));
    assert!(graph
        .lookup()
        .page_members(root_page_id)
        .value()
        .contains(&region_id));
    assert!(graph
        .lookup()
        .page_members(root_page_id)
        .value()
        .contains(&mosaic_id));
}

#[test]
fn topology_indexes_locate_nodes_while_attachment_posture_stays_on_node_truth() {
    let app = WorthUi::app()
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.graph-topology.attachment")
                .with_semantic_artifact_spec(slotted_control_spec()),
        )
        .freeze();
    let graph = app.graph();
    let root_page = root_page_artifact(&app);
    let root_page_id = graph_node_identity(graph, root_page);
    let control_id = graph
        .lookup()
        .slot_occupants(root_page_id, "footer")
        .value()
        .first()
        .copied()
        .expect("slot lookup should resolve the control node through graph indexes");
    let control = graph
        .inspection()
        .inspect_graph_node(control_id)
        .expect("graph node identity from topology index should resolve node truth")
        .value();

    assert!(control.attachment_posture().query_binding_attached());
    assert!(control.attachment_posture().service_usage_attached());
}

#[test]
fn graph_topology_keeps_root_contained_claims_explicit_without_generic_membership_tags() {
    let app = WorthUi::app()
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.graph-topology.containment-claims")
                .with_semantic_artifact_spec(page_set_spec())
                .with_semantic_artifact_spec(local_composition_spec())
                .with_semantic_artifact_spec(diagnostic_surface_spec()),
        )
        .freeze();
    let graph = app.graph();
    let root_page = root_page_artifact(&app);
    let page_set = artifact_from_file_provenance(&app, "app/graph_topology_claims.wui", 0);
    let local_composition =
        artifact_from_file_provenance(&app, "app/graph_topology_claims.wui", 1);
    let diagnostic_surface =
        artifact_from_file_provenance(&app, "app/graph_topology_claims.wui", 2);
    let root_page_id = graph_node_identity(graph, root_page);
    let page_set_topology = graph
        .inspection()
        .inspect_topology_node(graph_node_identity(graph, page_set))
        .expect("topology inspection should resolve admitted node topology")
        .value();
    let local_composition_topology = graph
        .inspection()
        .inspect_topology_node(graph_node_identity(graph, local_composition))
        .expect("topology inspection should resolve admitted node topology")
        .value();
    let diagnostic_surface_topology = graph
        .inspection()
        .inspect_topology_node(graph_node_identity(graph, diagnostic_surface))
        .expect("topology inspection should resolve admitted node topology")
        .value();

    for topology in [
        &page_set_topology,
        &local_composition_topology,
        &diagnostic_surface_topology,
    ] {
        assert_eq!(topology.parent_node_identity(), Some(root_page_id));
        assert_eq!(
            topology.parent_resolution_claim(),
            &UiGraphParentResolutionClaim::ContainedByRootPage
        );
        assert_eq!(
            topology
                .page_membership()
                .expect("root-contained graph node should have explicit page membership")
                .page_node_identity(),
            root_page_id
        );
    }

    assert_eq!(
        page_set_topology.containment_claim(),
        &UiGraphContainmentClaim::PageSet {
            page_set_name: "shell".into(),
        }
    );
    assert_eq!(
        local_composition_topology.containment_claim(),
        &UiGraphContainmentClaim::LocalComposition {
            local_composition_name: "inspector".into(),
        }
    );
    assert_eq!(
        diagnostic_surface_topology.containment_claim(),
        &UiGraphContainmentClaim::DiagnosticSurface {
            diagnostic_surface_name: "lint".into(),
        }
    );
}

#[test]
fn freeze_panics_when_topology_cannot_resolve_to_one_root_page() {
    let result = catch_unwind(AssertUnwindSafe(|| {
        let _ = WorthUi::app()
            .with_dsl_package(
                WorthUiDslPackage::named("worth-ui.certification.graph-topology.root-denial")
                    .with_semantic_artifact_spec(extra_root_page_spec())
                    .with_semantic_artifact_spec(slotted_control_spec()),
            )
            .freeze();
    }));

    let panic_message = panic_message(result.expect_err(
        "freeze path must panic when graph topology cannot resolve to one root page",
    ));
    assert!(
        panic_message.contains("freeze path must deny before publishing graph authority"),
        "expected topology denial panic to name unresolved topology path, got: {panic_message}"
    );
}

fn graph_node_identity(
    graph: worth_ui::facade::graph::UiGraphAuthority<'_>,
    artifact: &UiDeclarationArtifact,
) -> UiGraphNodeIdentity {
    graph
        .lookup()
        .declaration_instances(artifact.identity())
        .value()
        .first()
        .copied()
        .expect("declaration should admit one graph node")
}

fn root_page_artifact(app: &worth_ui::facade::app::WorthUiApp) -> &UiDeclarationArtifact {
    app.declaration_artifacts()
        .iter()
        .find(|artifact| {
            artifact
                .graph_handoff()
                .map(|handoff| handoff.role() == worth_ui::facade::declaration::UiDeclarationStructuralRole::Page)
                .unwrap_or(false)
        })
        .expect("bootstrap root page artifact should exist")
}

fn artifact_from_file_provenance<'a>(
    app: &'a worth_ui::facade::app::WorthUiApp,
    module_path: &str,
    declaration_index: usize,
) -> &'a UiDeclarationArtifact {
    app.declaration_artifacts()
        .iter()
        .find(|artifact| {
            let provenance = artifact.provenance().source_provenance();
            provenance.module_path() == module_path
                && provenance.declaration_index() == declaration_index
        })
        .unwrap_or_else(|| {
            panic!("expected declaration artifact for {module_path}#{declaration_index} on freeze path")
        })
}

fn slotted_control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.save"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/graph_topology.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:save"))
    .with_structural_token(UiDslStructuralToken::new("slot:footer"))
    .with_posture_token(UiDslPostureToken::new("query-binding:attached:view"))
    .with_posture_token(UiDslPostureToken::new("service:portal"))
}

fn region_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.region.sidebar"),
        UiDslSemanticFamily::Region,
        UiDslSourceProvenance::file_authored("app/graph_topology.wui", 1),
    )
    .with_structural_token(UiDslStructuralToken::new("region:sidebar"))
}

fn mosaic_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.mosaic.workspace"),
        UiDslSemanticFamily::Mosaic,
        UiDslSourceProvenance::file_authored("app/graph_topology.wui", 2),
    )
    .with_structural_token(UiDslStructuralToken::new("mosaic:workspace"))
}

fn page_set_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.page_set.shell"),
        UiDslSemanticFamily::PageSet,
        UiDslSourceProvenance::file_authored("app/graph_topology_claims.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("page-set:shell"))
}

fn local_composition_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.local_composition.inspector"),
        UiDslSemanticFamily::LocalComposition,
        UiDslSourceProvenance::file_authored("app/graph_topology_claims.wui", 1),
    )
    .with_structural_token(UiDslStructuralToken::new("local-composition:inspector"))
}

fn diagnostic_surface_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.diagnostic_surface.lint"),
        UiDslSemanticFamily::DiagnosticSurface,
        UiDslSourceProvenance::file_authored("app/graph_topology_claims.wui", 2),
    )
    .with_structural_token(UiDslStructuralToken::new("diagnostic-surface:lint"))
}

fn extra_root_page_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.page.authored_root"),
        UiDslSemanticFamily::Page,
        UiDslSourceProvenance::file_authored("app/graph_topology_root_denial.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("page:product-root"))
}

fn panic_message(payload: Box<dyn std::any::Any + Send>) -> String {
    match payload.downcast::<String>() {
        Ok(message) => *message,
        Err(payload) => match payload.downcast::<&'static str>() {
            Ok(message) => (*message).to_string(),
            Err(_) => "<non-string panic payload>".to_string(),
        },
    }
}
