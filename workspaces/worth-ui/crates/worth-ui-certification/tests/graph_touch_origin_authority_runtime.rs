use worth_ui::facade::app::WorthUi;
use worth_ui::facade::declaration::UiDeclarationArtifact;
use worth_ui::facade::graph::{
    UiGraphTouchAspectPosture, UiGraphTouchAspects, UiGraphTouchDenial, UiGraphTouchOriginClass,
    UiGraphTouchTiming, UiGraphWorldProfile,
};
use worth_ui_certification::scenario::installed_query_world;
use worth_ui_certification::{
    WorthUiCertificationBuilderExt, WorthUiRustAuthoredDeclarationFixture,
};
use worth_ui_dsl::{
    UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey, UiDslSourceProvenance,
    UiDslStructuralToken,
};
use worth_ui_test_support::WorthUiApplicationBuilderCertificationExt;

#[test]
fn origin_authority_cannot_be_remixed_onto_unrelated_graph_targets() {
    let app = touch_app(UiGraphWorldProfile::authoritative());
    let graph = app.graph();
    let region_id = graph_node_identity(graph, region_artifact(&app));

    let declaration_origin = graph
        .touches()
        .declaration_change_receipt(control_artifact(&app))
        .expect("control declaration should admit a declaration-change origin");
    let mismatched_declaration_touch = graph.touches().from_node(
        declaration_origin,
        UiGraphTouchTiming::PostMutation,
        region_id,
        UiGraphTouchAspects::new().structural(UiGraphTouchAspectPosture::Invalidated),
    );

    assert!(matches!(
        mismatched_declaration_touch,
        Err(UiGraphTouchDenial::OriginDoesNotAuthorizeGraphNode {
            origin_class: UiGraphTouchOriginClass::DeclarationChange,
            graph_node_identity,
        }) if graph_node_identity == region_id
    ));

    let query_world = touch_app(query_snapshot_world_profile(
        "snapshot:graph-touch-remix",
        ["worth-ui.graph", "touch", "remix"],
    ));
    let query_origin = query_world
        .graph()
        .touches()
        .query_binding_change_receipt()
        .expect("query-backed world should admit query-change origin");
    let direct_query_touch = query_world.graph().touches().from_node(
        query_origin,
        UiGraphTouchTiming::PostMutation,
        graph_node_identity(query_world.graph(), control_artifact(&query_world)),
        UiGraphTouchAspects::new().query_binding(UiGraphTouchAspectPosture::Invalidated),
    );

    assert!(matches!(
        direct_query_touch,
        Err(UiGraphTouchDenial::OriginDoesNotAuthorizeGraphNode {
            origin_class: UiGraphTouchOriginClass::QueryBindingChange,
            ..
        })
    ));
}

fn touch_app(world_profile: UiGraphWorldProfile) -> worth_ui::facade::app::WorthUiApp {
    WorthUi::app()
        .bind_certification_host_adapter(
            worth_ui_host_contract::UiCertificationHostBindingGrant::for_certification(),
            worth_ui_host_headless::WorthUiHeadlessHost,
        )
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .with_graph_world_profile(world_profile)
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named(
                "worth-ui.certification.graph-touch-origin-authority",
            )
            .with_semantic_artifact_spec(control_spec())
            .with_semantic_artifact_spec(region_spec()),
        )
        .freeze()
        .expect("application preparation should succeed")
}

fn control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.save"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/graph_touch_origin_authority_runtime.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:save"))
}

fn region_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.region.sidebar"),
        UiDslSemanticFamily::Region,
        UiDslSourceProvenance::file_authored("app/graph_touch_origin_authority_runtime.wui", 1),
    )
    .with_structural_token(UiDslStructuralToken::new("region:sidebar"))
}

fn control_artifact(app: &worth_ui::facade::app::WorthUiApp) -> &UiDeclarationArtifact {
    artifact_from_file_provenance(app, "app/graph_touch_origin_authority_runtime.wui", 0)
}

fn region_artifact(app: &worth_ui::facade::app::WorthUiApp) -> &UiDeclarationArtifact {
    artifact_from_file_provenance(app, "app/graph_touch_origin_authority_runtime.wui", 1)
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

fn graph_node_identity(
    graph: worth_ui::facade::graph::UiGraphAuthority<'_>,
    artifact: &UiDeclarationArtifact,
) -> worth_ui::facade::graph::UiGraphNodeIdentity {
    graph
        .lookup()
        .declaration_instances(artifact.identity())
        .value()
        .first()
        .copied()
        .expect("declaration should admit one graph node")
}

fn query_snapshot_world_profile(
    snapshot_label: &str,
    schema_basis_parts: [&str; 3],
) -> UiGraphWorldProfile {
    let binding = schema_basis_parts.join(".").replace('-', "_");
    installed_query_world::settled_query_world_profile(
        worth_ui::facade::declaration::ViewBindingId::new(binding.clone()).unwrap(),
        format!("{binding}.{snapshot_label}").replace('-', "_"),
    )
}
