use worth_ui::facade::app::{
    WorthUi, WorthUiApplicationPreparationDenial, WorthUiApplicationPreparationPhase,
};
use worth_ui::facade::declaration::{UiDeclarationArtifact, UiDeclarationStructuralRole};
use worth_ui::facade::graph::{
    UiGraphInstantiationPlan, UiGraphLookupCostClass, UiGraphLookupFamily,
    UiGraphTopologyLocalDenial, UiGraphWorldDifferenceKind, UiGraphWorldProfile,
};
use worth_ui_certification::{
    WorthUiCertificationBuilderExt, WorthUiRustAuthoredDeclarationFixture,
};
use worth_ui_dsl::{
    UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey, UiDslSourceProvenance,
    UiDslStructuralToken,
};

#[test]
fn initial_graph_commit_publishes_one_coherent_generation_transition() {
    let app = mutation_app();
    let handoff = control_artifact(&app)
        .graph_handoff()
        .expect("control declaration should lower to graph handoff");
    let root_page_handoff = root_page_artifact(&app)
        .graph_handoff()
        .expect("bootstrap root page should lower to graph handoff");
    let commit =
        UiGraphInstantiationPlan::admit_handoffs(&[root_page_handoff, handoff.clone()], &[])
            .expect("sealed handoffs should admit graph mutation plan")
            .commit_initial_generation(UiGraphWorldProfile::authoritative())
            .expect("admitted graph mutation should commit coherently");
    let graph = commit.graph();
    let declaration_lookup = graph.lookup().declaration_instances(handoff.identity());
    let graph_node_identity = declaration_lookup
        .value()
        .first()
        .copied()
        .expect("committed generation should publish declaration correspondence");
    let node_lookup = graph
        .lookup()
        .graph_node(graph_node_identity)
        .expect("committed generation should publish node identity lookup");

    assert_eq!(commit.committed_generation(), graph.generation());
    assert_eq!(graph.generation().as_u64(), 1);
    assert_eq!(
        declaration_lookup.receipt().family(),
        UiGraphLookupFamily::DeclarationCorrespondence
    );
    assert_eq!(
        declaration_lookup.receipt().cost_class(),
        UiGraphLookupCostClass::IndexedSet
    );
    assert_eq!(
        node_lookup.receipt().family(),
        UiGraphLookupFamily::NodeIdentity
    );
    assert_eq!(
        node_lookup.receipt().cost_class(),
        UiGraphLookupCostClass::IndexedScalar
    );
}

#[test]
fn denied_graph_mutation_publishes_no_replacement_snapshot() {
    let app = mutation_app();
    let prior_graph = app.graph();
    let handoff = control_artifact(&app)
        .graph_handoff()
        .expect("control declaration should lower to graph handoff");
    let denial = UiGraphInstantiationPlan::admit_handoffs(&[handoff.clone(), handoff], &[])
        .expect("basis-free duplicate handoffs should localize denial")
        .commit_initial_generation(UiGraphWorldProfile::authoritative())
        .expect_err("denied graph mutation must not publish graph authority");

    assert_eq!(denial.local_denials().len(), 2);
    assert_eq!(app.graph().generation(), prior_graph.generation());
    assert_eq!(
        app.graph().compare_to(prior_graph).kind(),
        UiGraphWorldDifferenceKind::SameWorldEquivalent
    );
}

#[test]
fn public_freeze_denies_graph_commit_before_publishing_graph_authority() {
    let denial = match WorthUi::app()
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named(
                "worth-ui.certification.graph-mutation.freeze-denial",
            )
            .with_semantic_artifact_spec(extra_root_page_spec())
            .with_semantic_artifact_spec(control_spec()),
        )
        .freeze()
    {
        Ok(_) => panic!("invalid root topology must deny application preparation"),
        Err(denial) => denial,
    };

    assert_eq!(
        denial.phase(),
        WorthUiApplicationPreparationPhase::GraphCommit
    );
    let WorthUiApplicationPreparationDenial::GraphCommit(denial) = denial else {
        panic!("expected graph-commit denial");
    };
    assert_eq!(denial.local_denials().len(), 3);
    assert!(denial.local_denials().iter().all(|local| {
        local.topology_denial()
            == Some(&UiGraphTopologyLocalDenial::RootPageCardinality {
                observed_root_pages: 2,
            })
    }));
}

fn mutation_app() -> worth_ui::facade::app::WorthUiApp {
    WorthUi::app()
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named("worth-ui.certification.graph-mutation")
                .with_semantic_artifact_spec(control_spec()),
        )
        .freeze()
        .expect("application preparation should succeed")
}

fn control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.save"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/graph_mutation_runtime.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:save"))
}

fn extra_root_page_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.page.authored_root"),
        UiDslSemanticFamily::Page,
        UiDslSourceProvenance::file_authored("app/graph_mutation_root_denial.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("page:product-root"))
}

fn control_artifact(app: &worth_ui::facade::app::WorthUiApp) -> &UiDeclarationArtifact {
    artifact_from_file_provenance(app, "app/graph_mutation_runtime.wui", 0)
}

fn root_page_artifact(app: &worth_ui::facade::app::WorthUiApp) -> &UiDeclarationArtifact {
    app.declaration_artifacts()
        .iter()
        .find(|artifact| {
            artifact
                .graph_handoff()
                .map(|handoff| handoff.role() == UiDeclarationStructuralRole::Page)
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
            panic!(
                "expected declaration artifact for {module_path}#{declaration_index} on freeze path"
            )
        })
}
