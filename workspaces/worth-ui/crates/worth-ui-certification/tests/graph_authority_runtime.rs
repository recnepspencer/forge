use worth_ui::facade::app::WorthUi;
use worth_ui::facade::declaration::UiDeclarationArtifact;
use worth_ui::facade::graph::{
    UiGraphInstantiationPlan, UiGraphWorldDifferenceKind, UiGraphWorldProfile,
    UiPreviewSessionIdentity, UiRepeatedInstanceBasisDenial, UiRepeatedInstanceBasisKind,
};
use worth_ui_certification::scenario::installed_query_world;
use worth_ui_dsl::{
    UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken, WorthUiDslPackage,
};

#[test]
fn public_app_freeze_exposes_committed_graph_authority_with_typed_identity_basis() {
    let app = WorthUi::app()
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.graph-authority")
                .with_semantic_artifact_spec(primary_control_spec()),
        )
        .freeze()
        .expect("application preparation should succeed");

    let artifact = artifact_from_file_provenance(&app, "app/graph_authority_primary.wui", 0);
    let graph = app.graph();
    let graph_node_identity = graph_node_identity(graph, artifact);
    let node = graph
        .lookup()
        .graph_node(graph_node_identity)
        .expect("graph snapshot should resolve admitted graph node identity")
        .value();

    assert_eq!(graph.generation().as_u64(), 1);
    assert_eq!(graph.world_profile(), &UiGraphWorldProfile::authoritative());
    assert_eq!(node.declaration_identity(), artifact.identity());
    assert_eq!(
        node.repeated_instance_basis().kind(),
        UiRepeatedInstanceBasisKind::DeclarationKeyed
    );
    assert!(node.attachment_posture().query_binding_attached());
    assert!(node.attachment_posture().service_usage_attached());
    assert_eq!(
        graph.compare_to(graph).kind(),
        UiGraphWorldDifferenceKind::SameWorldEquivalent
    );
}

#[test]
fn unrelated_sibling_churn_does_not_rewrite_primary_runtime_graph_identity() {
    let baseline = WorthUi::app()
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.graph-identity.stable")
                .with_semantic_artifact_spec(primary_control_spec())
                .with_semantic_artifact_spec(secondary_control_spec()),
        )
        .freeze()
        .expect("application preparation should succeed");
    let churned = WorthUi::app()
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.graph-identity.stable")
                .with_semantic_artifact_spec(primary_control_spec())
                .with_semantic_artifact_spec(unrelated_inserted_control_spec())
                .with_semantic_artifact_spec(secondary_control_spec()),
        )
        .freeze()
        .expect("application preparation should succeed");

    let baseline_artifact =
        artifact_from_file_provenance(&baseline, "app/graph_authority_primary.wui", 0);
    let churned_artifact =
        artifact_from_file_provenance(&churned, "app/graph_authority_primary.wui", 0);

    let baseline_graph_node_identity = baseline
        .graph()
        .lookup()
        .declaration_instances(baseline_artifact.identity())
        .value()
        .first()
        .copied()
        .expect("baseline primary declaration should admit one graph node");
    let churned_graph_node_identity = churned
        .graph()
        .lookup()
        .declaration_instances(churned_artifact.identity())
        .value()
        .first()
        .copied()
        .expect("churned primary declaration should admit one graph node");

    assert_eq!(baseline_artifact.identity(), churned_artifact.identity());
    assert_eq!(baseline_graph_node_identity, churned_graph_node_identity);
}

#[test]
fn graph_world_profile_compare_distinguishes_preview_session_identity_worlds() {
    let alpha_world = UiGraphWorldProfile::preview_session_identity(
        UiPreviewSessionIdentity::new("preview-session:alpha").expect("preview identity"),
    );
    let beta_world = UiGraphWorldProfile::preview_session_identity(
        UiPreviewSessionIdentity::new("preview-session:beta").expect("preview identity"),
    );

    let alpha = WorthUi::app()
        .with_graph_world_profile(alpha_world.clone())
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.graph-world.alpha")
                .with_semantic_artifact_spec(primary_control_spec()),
        )
        .freeze()
        .expect("application preparation should succeed");
    let alpha_again = WorthUi::app()
        .with_graph_world_profile(alpha_world)
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.graph-world.alpha")
                .with_semantic_artifact_spec(primary_control_spec()),
        )
        .freeze()
        .expect("application preparation should succeed");
    let beta = WorthUi::app()
        .with_graph_world_profile(beta_world)
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.graph-world.alpha")
                .with_semantic_artifact_spec(primary_control_spec()),
        )
        .freeze()
        .expect("application preparation should succeed");

    assert_eq!(
        alpha.graph().compare_to(alpha_again.graph()).kind(),
        UiGraphWorldDifferenceKind::SameWorldEquivalent
    );
    assert_eq!(
        alpha.graph().compare_to(beta.graph()).kind(),
        UiGraphWorldDifferenceKind::SameDeclarationDifferentWorld
    );
}

#[test]
fn graph_world_profile_compare_distinguishes_settled_query_bindings() {
    let alpha_world = query_snapshot_world_profile("snapshot:equal-looking");
    let beta_world = query_snapshot_world_profile("snapshot:equal-looking");

    let alpha = WorthUi::app()
        .with_graph_world_profile(alpha_world.clone())
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.graph-world.query")
                .with_semantic_artifact_spec(primary_control_spec()),
        )
        .freeze()
        .expect("application preparation should succeed");
    let alpha_again = WorthUi::app()
        .with_graph_world_profile(alpha_world)
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.graph-world.query")
                .with_semantic_artifact_spec(primary_control_spec()),
        )
        .freeze()
        .expect("application preparation should succeed");
    let beta = WorthUi::app()
        .with_graph_world_profile(beta_world)
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.graph-world.query")
                .with_semantic_artifact_spec(primary_control_spec()),
        )
        .freeze()
        .expect("application preparation should succeed");

    assert_eq!(
        alpha.graph().compare_to(alpha_again.graph()).kind(),
        UiGraphWorldDifferenceKind::SameWorldEquivalent
    );
    assert_eq!(
        alpha.graph().compare_to(beta.graph()).kind(),
        UiGraphWorldDifferenceKind::SameDeclarationDifferentWorld
    );
}

#[test]
fn graph_instantiation_plan_denies_basis_free_runtime_multiplicity_before_snapshot_commit() {
    let app = WorthUi::app()
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.graph-basis.denial")
                .with_semantic_artifact_spec(primary_control_spec()),
        )
        .freeze()
        .expect("application preparation should succeed");

    let handoff = artifact_from_file_provenance(&app, "app/graph_authority_primary.wui", 0)
        .graph_handoff()
        .expect("declaration artifact should lower to sealed handoff");
    let plan = UiGraphInstantiationPlan::admit_handoffs(&[handoff.clone(), handoff], &[])
        .expect("sealed handoff admission should localize basis-free multiplicity denial");

    assert!(plan.node_entries().is_empty());
    assert_eq!(plan.local_denials().len(), 2);
    for denial in plan.local_denials() {
        assert_eq!(
            denial.repeated_instance_basis_denial(),
            Some(&UiRepeatedInstanceBasisDenial::BasisFreeRuntimeIdentityDenied)
        );
    }
    let denial = plan
        .commit_initial_generation(UiGraphWorldProfile::authoritative())
        .expect_err("denied graph mutation must not publish graph authority");

    assert_eq!(denial.local_denials(), plan.local_denials());
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
            panic!(
                "expected declaration artifact for {module_path}#{declaration_index} on freeze path"
            )
        })
}

fn primary_control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.save"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/graph_authority_primary.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:save"))
    .with_structural_token(UiDslStructuralToken::new("slot:footer"))
    .with_posture_token(UiDslPostureToken::new("query-binding:attached:view"))
    .with_posture_token(UiDslPostureToken::new("service:portal"))
}

fn secondary_control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.cancel"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/graph_authority_secondary.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:cancel"))
    .with_structural_token(UiDslStructuralToken::new("slot:footer"))
}

fn unrelated_inserted_control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.help"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/graph_authority_help.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:help"))
    .with_structural_token(UiDslStructuralToken::new("slot:header"))
}

fn query_snapshot_world_profile(snapshot_label: &str) -> UiGraphWorldProfile {
    installed_query_world::settled_query_world_profile(
        worth_ui::facade::registry::ViewBindingId::new("workspace.binding.shared").unwrap(),
        snapshot_label.replace('-', "_"),
    )
}
