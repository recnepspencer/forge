use worth_ui::facade::app::{WorthUi, WorthUiApp};
use worth_ui::facade::graph::{UiGraphInstantiationPlan, UiGraphNodeIdentity};
use worth_ui::facade::inspection::{
    UiEvidenceAuthorityGeneration, UiEvidenceMaterializedDetail,
    UiInspectionObligationEvidenceReceipt, UiInspectionObligationFamily,
    UiInspectionObligationRelevanceDetail, UiInspectionQuery, UiInspectionRelevance,
    UiInspectionScope, UiInspectionTarget, UiRelevanceFamily, UiRelevanceFilter,
};
use worth_ui_certification::{
    WorthUiCertificationBuilderExt, WorthUiRustAuthoredDeclarationFixture,
};
use worth_ui_dsl::{
    UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey, UiDslSourceProvenance,
    UiDslStructuralToken,
};

pub(super) fn app_generation(app: &WorthUiApp) -> UiEvidenceAuthorityGeneration {
    UiEvidenceAuthorityGeneration::new(app.graph().generation().as_u64())
}

pub(super) fn obligation_query(
    graph_node_digest: u64,
    touch_identity_digest: u64,
) -> UiInspectionQuery {
    UiInspectionQuery::new(
        UiInspectionTarget::obligation_touch(graph_node_digest, touch_identity_digest),
        UiInspectionScope::graph(),
    )
    .with_relevance(obligation_relevance())
}

pub(super) fn graph_evidence_app() -> WorthUiApp {
    WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named(
                "worth-ui.certification.evidence.references",
            )
            .with_semantic_artifact_spec(
                UiDslSemanticArtifactSpec::new(
                    UiDslSemanticKey::new("ui.graph.evidence.reference"),
                    UiDslSemanticFamily::Control,
                    UiDslSourceProvenance::file_authored("app/evidence_reference_runtime.wui", 0),
                )
                .with_structural_token(UiDslStructuralToken::new("control:test")),
            ),
        )
        .freeze()
        .map(
            worth_ui_runtime::facade::entry::WorthUiCertificationApplicationTransition::activate_headless,
        )
        .expect("application preparation should succeed")
}

pub(super) fn first_graph_node_identity(app: &WorthUiApp) -> UiGraphNodeIdentity {
    let declaration_identity = app
        .declaration_artifacts()
        .iter()
        .find(|artifact| {
            artifact.provenance().source_provenance().module_path()
                == "app/evidence_reference_runtime.wui"
        })
        .expect("graph evidence fixture declaration should exist")
        .identity();
    app.graph()
        .lookup()
        .declaration_instances(declaration_identity)
        .value()[0]
}

pub(super) fn successor_graph_commit(
    app: &WorthUiApp,
) -> worth_ui::facade::graph::UiGraphMutationCommitResult {
    let handoffs = app
        .declaration_artifacts()
        .iter()
        .map(|artifact| {
            artifact
                .graph_handoff()
                .expect("graph evidence app declarations should lower to graph handoffs")
        })
        .collect::<Vec<_>>();

    UiGraphInstantiationPlan::admit_handoffs(&handoffs, &[])
        .expect("graph evidence app declarations should admit successor graph planning")
        .commit_successor_generation(app.graph())
        .expect("admitted graph plan should commit one real successor generation")
}

pub(super) fn obligation_detail(
    detail: &UiEvidenceMaterializedDetail,
) -> &UiInspectionObligationEvidenceReceipt {
    match detail {
        UiEvidenceMaterializedDetail::Obligation(receipt) => receipt,
        _ => panic!("expected obligation materialized detail"),
    }
}

pub(super) fn obligation_relevance() -> UiInspectionRelevance {
    UiInspectionRelevance::local(UiRelevanceFilter::family(UiRelevanceFamily::Obligation))
        .with_obligation_detail(
            UiInspectionObligationRelevanceDetail::new()
                .with_family(UiInspectionObligationFamily::QueryBindingRequirement),
        )
}
