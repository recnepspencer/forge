use std::sync::Arc;

use worth_ui::facade::app::WorthUi;
use worth_ui::facade::declaration::UiDeclarationArtifact;
use worth_ui::facade::graph::{
    admit_runtime_current_snapshot_basis, snapshot_resolution_report, QueryExternalIdentityToken,
    QueryExternalSchemaBasisToken, UiGraphAxisParticipation, UiGraphParticipationAxis,
    UiGraphParticipationStatus, UiGraphWorldProfile,
};
use worth_ui_dsl::{
    UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken, WorthUiDslPackage,
};

pub fn touch_app(world_profile: UiGraphWorldProfile) -> worth_ui::facade::app::WorthUiApp {
    WorthUi::app()
        .with_graph_world_profile(world_profile)
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.graph-touch")
                .with_semantic_artifact_spec(control_spec())
                .with_semantic_artifact_spec(region_spec())
                .with_semantic_artifact_spec(mosaic_spec()),
        )
        .freeze()
}

pub fn control_artifact(app: &worth_ui::facade::app::WorthUiApp) -> &UiDeclarationArtifact {
    artifact_from_file_provenance(app, "app/graph_touch_runtime.wui", 0)
}

pub fn region_artifact(app: &worth_ui::facade::app::WorthUiApp) -> &UiDeclarationArtifact {
    artifact_from_file_provenance(app, "app/graph_touch_runtime.wui", 1)
}

pub fn mosaic_artifact(app: &worth_ui::facade::app::WorthUiApp) -> &UiDeclarationArtifact {
    artifact_from_file_provenance(app, "app/graph_touch_runtime.wui", 2)
}

pub fn graph_node_identity(
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

pub fn mounted_receipt_transition(
    app: &worth_ui::facade::app::WorthUiApp,
    artifact: &UiDeclarationArtifact,
) -> worth_ui::facade::graph::UiGraphMountedReceiptTransition {
    let graph = app.graph();
    let graph_node_identity = graph_node_identity(graph, artifact);
    let control_node = graph
        .lookup()
        .graph_node(graph_node_identity)
        .expect("graph should resolve control node")
        .value();

    graph
        .mounted_receipt_transition_for_node(
            graph_node_identity,
            control_node
                .participation_posture()
                .axis(UiGraphParticipationAxis::Mounted),
            UiGraphAxisParticipation::runtime_mutation(UiGraphParticipationStatus::Admitted),
        )
        .expect("mounted admission should yield one graph-owned transition")
}

pub fn query_snapshot_world_profile(
    snapshot_label: &str,
    schema_basis_parts: [&str; 3],
) -> UiGraphWorldProfile {
    let snapshot_identity =
        worth_ui::facade::graph::WorthQuerySnapshotIdentity::admit_external_token(
            QueryExternalIdentityToken::new(Arc::<str>::from(snapshot_label)),
        );
    let basis = admit_runtime_current_snapshot_basis(
        snapshot_identity.evidence_identity(),
        QueryExternalSchemaBasisToken::from_domain_parts(
            &schema_basis_parts
                .into_iter()
                .map(str::to_owned)
                .collect::<Vec<_>>(),
        ),
    )
    .expect("runtime current snapshot basis should resolve");

    UiGraphWorldProfile::query_snapshot_basis(basis.clone(), snapshot_resolution_report(&basis))
        .expect("query snapshot basis world profile should admit matching report")
}

fn control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.save"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/graph_touch_runtime.wui", 0),
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
        UiDslSourceProvenance::file_authored("app/graph_touch_runtime.wui", 1),
    )
    .with_structural_token(UiDslStructuralToken::new("region:sidebar"))
}

fn mosaic_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.mosaic.workspace"),
        UiDslSemanticFamily::Mosaic,
        UiDslSourceProvenance::file_authored("app/graph_touch_runtime.wui", 2),
    )
    .with_structural_token(UiDslStructuralToken::new("mosaic:workspace"))
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
