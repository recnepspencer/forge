use std::collections::BTreeSet;
use std::sync::Arc;
use worth_ui_certification::{
    WorthUiCertificationBuilderExt, WorthUiRustAuthoredDeclarationFixture,
};

use worth_ui::facade::app::WorthUi;
use worth_ui::facade::declaration::UiDeclarationArtifact;
use worth_ui::facade::graph::{
    UiGraphInstantiationPlan, UiGraphLookupCostClass, UiGraphLookupFamily,
    UiGraphParticipationAxis, UiGraphWorldProfile, UiRuntimeDataInstanceKeyToken,
    UiRuntimeInstanceBasisAdmission,
};
use worth_ui_dsl::{
    UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken,
};

#[test]
fn ordinary_graph_lookup_is_bounded_and_receipt_backed() {
    let app = lookup_app();
    let graph = app.graph();
    let root_page = root_page_artifact(&app);
    let control = artifact_from_file_provenance(&app, "app/graph_lookup_runtime.wui", 0);
    let root_page_id = graph_node_identity(graph, root_page);
    let control_id = graph_node_identity(graph, control);
    let node_lookup = graph
        .lookup()
        .graph_node(control_id)
        .expect("node identity lookup should resolve committed node");
    let declaration_lookup = graph.lookup().declaration_instances(control.identity());
    let child_lookup = graph.lookup().child_nodes(root_page_id);
    let slot_lookup = graph.lookup().slot_occupants(root_page_id, "footer");
    let participation_lookup = graph
        .lookup()
        .page_participation(root_page_id, UiGraphParticipationAxis::QueryBound);
    let receipt_lookup = graph
        .lookup()
        .mount_eligibility_slot_for_node(control_id)
        .expect("mount eligibility lookup should resolve the committed slot");

    assert_eq!(
        node_lookup.receipt().family(),
        UiGraphLookupFamily::NodeIdentity
    );
    assert_eq!(
        node_lookup.receipt().cost_class(),
        UiGraphLookupCostClass::IndexedScalar
    );
    assert_eq!(node_lookup.value().graph_node_identity(), control_id);

    assert_eq!(
        declaration_lookup.receipt().family(),
        UiGraphLookupFamily::DeclarationCorrespondence
    );
    assert_eq!(
        declaration_lookup.receipt().cost_class(),
        UiGraphLookupCostClass::IndexedSet
    );
    assert_eq!(declaration_lookup.value(), &[control_id]);

    assert_eq!(
        child_lookup.receipt().family(),
        UiGraphLookupFamily::ParentChild
    );
    assert_eq!(
        child_lookup.receipt().cost_class(),
        UiGraphLookupCostClass::IndexedSet
    );
    assert!(child_lookup.value().contains(&control_id));

    assert_eq!(
        slot_lookup.receipt().family(),
        UiGraphLookupFamily::SlotOccupancy
    );
    assert_eq!(
        slot_lookup.receipt().cost_class(),
        UiGraphLookupCostClass::IndexedSet
    );
    assert!(slot_lookup.value().contains(&control_id));

    assert_eq!(
        participation_lookup.receipt().family(),
        UiGraphLookupFamily::PageParticipation
    );
    assert_eq!(
        participation_lookup.receipt().cost_class(),
        UiGraphLookupCostClass::IndexedNeighborhood
    );
    assert!(participation_lookup
        .value()
        .iter()
        .any(|member| member.member_node_identity() == control_id));

    assert_eq!(
        receipt_lookup.receipt().family(),
        UiGraphLookupFamily::MountEligibilitySlot
    );
    assert_eq!(
        receipt_lookup.receipt().cost_class(),
        UiGraphLookupCostClass::IndexedScalar
    );
    assert_eq!(receipt_lookup.value().graph_node_identity(), control_id);
}

#[test]
fn declaration_correspondence_lookup_handles_zero_one_many_nodes_honestly() {
    let app = lookup_app();
    let graph = app.graph();
    let control = artifact_from_file_provenance(&app, "app/graph_lookup_runtime.wui", 0);
    let one_lookup = graph.lookup().declaration_instances(control.identity());

    assert_eq!(
        one_lookup.receipt().family(),
        UiGraphLookupFamily::DeclarationCorrespondence
    );
    assert_eq!(
        one_lookup.receipt().cost_class(),
        UiGraphLookupCostClass::IndexedSet
    );
    assert_eq!(one_lookup.value().len(), 1);

    let absent_app = absent_lookup_app();
    let absent_control =
        artifact_from_file_provenance(&absent_app, "app/graph_lookup_absent.wui", 0);
    let zero_lookup = graph
        .lookup()
        .declaration_instances(absent_control.identity());

    assert_eq!(
        zero_lookup.receipt().family(),
        UiGraphLookupFamily::DeclarationCorrespondence
    );
    assert_eq!(
        zero_lookup.receipt().cost_class(),
        UiGraphLookupCostClass::IndexedSet
    );
    assert!(zero_lookup.value().is_empty());

    let root_page_handoff = root_page_artifact(&app)
        .graph_handoff()
        .expect("bootstrap root page should lower to graph handoff");
    let control_handoff = control
        .graph_handoff()
        .expect("control declaration should lower to graph handoff");
    let runtime_basis_alpha = UiRuntimeInstanceBasisAdmission::admit_runtime_data_keyed(
        control.identity(),
        UiRuntimeDataInstanceKeyToken::new(Arc::<str>::from("row:user-7")),
    )
    .expect("runtime-keyed repeated instance should admit");
    let runtime_basis_beta = UiRuntimeInstanceBasisAdmission::admit_runtime_data_keyed(
        control.identity(),
        UiRuntimeDataInstanceKeyToken::new(Arc::<str>::from("row:user-8")),
    )
    .expect("runtime-keyed repeated instance should admit");
    let repeated_plan = UiGraphInstantiationPlan::admit_handoffs(
        &[root_page_handoff, control_handoff.clone(), control_handoff],
        &[runtime_basis_beta, runtime_basis_alpha],
    )
    .expect("runtime-keyed repeated instances should admit one node per basis");
    let repeated_commit = repeated_plan
        .commit_initial_generation(UiGraphWorldProfile::authoritative())
        .expect("repeated-instance plan should commit coherently");
    let repeated_graph = repeated_commit.graph();
    let many_lookup = repeated_graph
        .lookup()
        .declaration_instances(control.identity());

    assert_eq!(
        many_lookup.receipt().family(),
        UiGraphLookupFamily::DeclarationCorrespondence
    );
    assert_eq!(
        many_lookup.receipt().cost_class(),
        UiGraphLookupCostClass::IndexedSet
    );
    assert_eq!(many_lookup.value().len(), 2);
    assert_eq!(
        many_lookup
            .value()
            .iter()
            .copied()
            .collect::<BTreeSet<_>>()
            .len(),
        2
    );
    for graph_node_identity in many_lookup.value() {
        let node_lookup = repeated_graph
            .lookup()
            .graph_node(*graph_node_identity)
            .expect("declared repeated instance should resolve through bounded node lookup");
        assert_eq!(
            node_lookup.value().declaration_identity(),
            control.identity()
        );
    }
}

fn lookup_app() -> worth_ui::facade::app::WorthUiApp {
    WorthUi::app()
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named("worth-ui.certification.graph-lookup")
                .with_semantic_artifact_spec(control_spec()),
        )
        .freeze()
        .expect("application preparation should succeed")
}

fn absent_lookup_app() -> worth_ui::facade::app::WorthUiApp {
    WorthUi::app()
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named(
                "worth-ui.certification.graph-lookup.absent",
            )
            .with_semantic_artifact_spec(absent_control_spec()),
        )
        .freeze()
        .expect("application preparation should succeed")
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

fn root_page_artifact(app: &worth_ui::facade::app::WorthUiApp) -> &UiDeclarationArtifact {
    app.declaration_artifacts()
        .iter()
        .find(|artifact| {
            artifact
                .graph_handoff()
                .map(|handoff| {
                    handoff.role()
                        == worth_ui::facade::declaration::UiDeclarationStructuralRole::Page
                })
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
            panic!("expected declaration artifact for {module_path}#{declaration_index}")
        })
}

fn control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.save"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/graph_lookup_runtime.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:save"))
    .with_structural_token(UiDslStructuralToken::new("slot:footer"))
    .with_posture_token(UiDslPostureToken::new("query-binding:attached:view"))
}

fn absent_control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.discard"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/graph_lookup_absent.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:discard"))
}
