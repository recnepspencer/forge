use worth_ui::facade::app::{WorthUi, WorthUiApp};
use worth_ui::facade::declaration::UiDeclarationArtifact;
use worth_ui::facade::inspection::{
    UiEvidenceFamily, UiEvidenceRichness, UiInspectionObligationFamily,
    UiInspectionObligationRelevanceDetail, UiInspectionQuery, UiInspectionRelevance,
    UiInspectionRelevanceOutcome, UiInspectionScope, UiInspectionTarget, UiInspectionTargetClass,
    UiRelevanceFamily, UiRelevanceFilter,
};
use worth_ui_certification::{
    WorthUiCertificationBuilderExt, WorthUiRustAuthoredDeclarationFixture,
};
use worth_ui_dsl::{
    UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken,
};

const ALPHA_MODULE_PATH: &str = "app/graph_evidence_alpha.wui";
const BETA_MODULE_PATH: &str = "app/graph_evidence_beta.wui";

#[test]
fn graph_node_identity_lookup_stays_runtime_truth_distinct_from_authored_lookup() {
    let app = graph_identity_app();
    let alpha_artifact = control_artifact(&app, ALPHA_MODULE_PATH);
    let beta_artifact = control_artifact(&app, BETA_MODULE_PATH);
    let alpha_graph_receipt = app.inspect(graph_identity_query(graph_node_identity(
        &app,
        alpha_artifact,
    )));
    let beta_graph_receipt = app.inspect(graph_identity_query(graph_node_identity(
        &app,
        beta_artifact,
    )));
    let alpha_authored_receipt = app.inspect(declaration_identity_query(alpha_artifact));
    let beta_authored_receipt = app.inspect(declaration_identity_query(beta_artifact));
    let alpha_graph_slice = alpha_graph_receipt
        .evidence_slice()
        .expect("graph node identity should return graph-local evidence refs");
    let beta_graph_slice = beta_graph_receipt
        .evidence_slice()
        .expect("second graph node should return its own graph-local evidence refs");
    let alpha_authored_slice = alpha_authored_receipt
        .evidence_slice()
        .expect("declaration identity should return authored-side evidence refs");
    let beta_authored_slice = beta_authored_receipt
        .evidence_slice()
        .expect("second declaration identity should return authored-side evidence refs");

    assert_eq!(
        alpha_graph_receipt.relevance_outcome(),
        UiInspectionRelevanceOutcome::Matched
    );
    assert_eq!(
        beta_graph_receipt.relevance_outcome(),
        UiInspectionRelevanceOutcome::Matched
    );
    assert_eq!(
        alpha_authored_receipt.relevance_outcome(),
        UiInspectionRelevanceOutcome::Matched
    );
    assert_eq!(
        beta_authored_receipt.relevance_outcome(),
        UiInspectionRelevanceOutcome::Matched
    );
    assert_ne!(
        alpha_graph_receipt.evidence_slice_ref(),
        alpha_authored_receipt.evidence_slice_ref()
    );
    assert_ne!(
        beta_graph_receipt.evidence_slice_ref(),
        beta_authored_receipt.evidence_slice_ref()
    );
    assert_ne!(
        alpha_graph_receipt.evidence_slice_ref(),
        beta_graph_receipt.evidence_slice_ref()
    );
    assert_node_exposes_runtime_families(alpha_graph_slice.refs());
    assert_node_exposes_runtime_families(beta_graph_slice.refs());
    assert_refs_exclude_other_node(alpha_graph_slice.refs(), beta_graph_slice.refs());
    assert_eq!(
        declaration_and_admission_refs(alpha_graph_slice.refs()),
        alpha_authored_slice.refs().to_vec(),
        "graph identity should preserve declaration/admission correspondence for node alpha without collapsing into the authored lane",
    );
    assert_eq!(
        declaration_and_admission_refs(beta_graph_slice.refs()),
        beta_authored_slice.refs().to_vec(),
        "graph identity should preserve declaration/admission correspondence for node beta without collapsing into the authored lane",
    );
}

#[test]
fn graph_node_identity_accepts_obligation_family_through_indexed_graph_runtime_lane() {
    let app = graph_identity_app();
    let alpha = graph_node_identity(&app, control_artifact(&app, ALPHA_MODULE_PATH));
    let beta = graph_node_identity(&app, control_artifact(&app, BETA_MODULE_PATH));
    let alpha_receipt = app.inspect(all_obligation_graph_identity_query(alpha));
    let beta_receipt = app.inspect(all_obligation_graph_identity_query(beta));
    let structural_receipt = app.inspect(structural_obligation_graph_identity_query(alpha));
    let alpha_slice = alpha_receipt
        .evidence_slice()
        .expect("graph identity obligation neighborhood should exist");
    let beta_slice = beta_receipt
        .evidence_slice()
        .expect("second graph-node obligation neighborhood should exist");

    assert_eq!(
        alpha_receipt.relevance_outcome(),
        UiInspectionRelevanceOutcome::Matched
    );
    assert_eq!(
        beta_receipt.relevance_outcome(),
        UiInspectionRelevanceOutcome::Matched
    );
    assert!(!alpha_slice.refs().is_empty());
    assert!(!beta_slice.refs().is_empty());
    assert!(alpha_slice
        .refs()
        .iter()
        .all(|evidence_ref| evidence_ref.family() == UiEvidenceFamily::Obligation));
    assert!(beta_slice
        .refs()
        .iter()
        .all(|evidence_ref| evidence_ref.family() == UiEvidenceFamily::Obligation));
    assert_refs_exclude_other_node(alpha_slice.refs(), beta_slice.refs());
    assert_eq!(
        structural_receipt.relevance_outcome(),
        UiInspectionRelevanceOutcome::NotApplicableToTarget {
            target: UiInspectionTargetClass::GraphNodeIdentity,
        }
    );
    assert!(structural_receipt.evidence_slice().is_none());
}

#[test]
fn graph_node_identity_rejects_aspect_family_until_indexed_lane_exists() {
    let app = graph_identity_app();
    let receipt = app.inspect(
        UiInspectionQuery::new(
            UiInspectionTarget::graph_node_identity(
                graph_node_identity(&app, control_artifact(&app, ALPHA_MODULE_PATH)).digest(),
            ),
            UiInspectionScope::graph(),
        )
        .with_relevance(UiInspectionRelevance::local(UiRelevanceFilter::family(
            UiRelevanceFamily::Aspect,
        )))
        .with_richness(UiEvidenceRichness::refs_only()),
    );

    assert_eq!(
        receipt.relevance_outcome(),
        UiInspectionRelevanceOutcome::NotApplicableToTarget {
            target: UiInspectionTargetClass::GraphNodeIdentity,
        }
    );
    assert!(receipt.evidence_slice().is_none());
}

fn graph_identity_app() -> WorthUiApp {
    WorthUi::app()
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named(
                "worth-ui.certification.graph-evidence-lookup",
            )
            .with_semantic_artifact_spec(control_spec(
                "ui.workflow.graph_evidence.alpha",
                ALPHA_MODULE_PATH,
                "query-binding:attached:view",
            ))
            .with_semantic_artifact_spec(control_spec(
                "ui.workflow.graph_evidence.beta",
                BETA_MODULE_PATH,
                "service:portal",
            )),
        )
        .freeze()
        .expect("application preparation should succeed")
}

fn graph_identity_query(
    graph_node_identity: worth_ui::facade::graph::UiGraphNodeIdentity,
) -> UiInspectionQuery {
    UiInspectionQuery::new(
        UiInspectionTarget::graph_node_identity(graph_node_identity.digest()),
        UiInspectionScope::graph(),
    )
    .with_relevance(UiInspectionRelevance::local(
        UiRelevanceFilter::target_local(),
    ))
    .with_richness(UiEvidenceRichness::refs_only())
}

fn declaration_identity_query(control: &UiDeclarationArtifact) -> UiInspectionQuery {
    UiInspectionQuery::new(
        UiInspectionTarget::declaration_identity(control.identity().inspection_identity()),
        UiInspectionScope::graph(),
    )
    .with_relevance(UiInspectionRelevance::local(
        UiRelevanceFilter::target_local(),
    ))
    .with_richness(UiEvidenceRichness::refs_only())
}

fn all_obligation_graph_identity_query(
    graph_node_identity: worth_ui::facade::graph::UiGraphNodeIdentity,
) -> UiInspectionQuery {
    UiInspectionQuery::new(
        UiInspectionTarget::graph_node_identity(graph_node_identity.digest()),
        UiInspectionScope::graph(),
    )
    .with_relevance(UiInspectionRelevance::local(UiRelevanceFilter::family(
        UiRelevanceFamily::Obligation,
    )))
    .with_richness(UiEvidenceRichness::refs_only())
}

fn structural_obligation_graph_identity_query(
    graph_node_identity: worth_ui::facade::graph::UiGraphNodeIdentity,
) -> UiInspectionQuery {
    UiInspectionQuery::new(
        UiInspectionTarget::graph_node_identity(graph_node_identity.digest()),
        UiInspectionScope::graph(),
    )
    .with_relevance(
        UiInspectionRelevance::local(UiRelevanceFilter::family(UiRelevanceFamily::Obligation))
            .with_obligation_detail(
                UiInspectionObligationRelevanceDetail::new()
                    .with_family(UiInspectionObligationFamily::StructuralLegality),
            ),
    )
    .with_richness(UiEvidenceRichness::refs_only())
}

fn graph_node_identity(
    app: &WorthUiApp,
    artifact: &UiDeclarationArtifact,
) -> worth_ui::facade::graph::UiGraphNodeIdentity {
    app.graph()
        .lookup()
        .declaration_instances(artifact.identity())
        .value()
        .first()
        .copied()
        .expect("declaration should admit one graph node")
}

fn control_artifact<'a>(app: &'a WorthUiApp, module_path: &str) -> &'a UiDeclarationArtifact {
    app.declaration_artifacts()
        .iter()
        .find(|artifact| artifact.provenance().source_provenance().module_path() == module_path)
        .expect("control artifact should exist")
}

fn control_spec(
    semantic_key: &str,
    module_path: &str,
    posture_token: &str,
) -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new(semantic_key),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored(module_path, 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:workflow"))
    .with_posture_token(UiDslPostureToken::new(posture_token))
}

fn declaration_and_admission_refs(
    refs: &[worth_ui::facade::inspection::UiEvidenceRef],
) -> Vec<worth_ui::facade::inspection::UiEvidenceRef> {
    refs.iter()
        .copied()
        .filter(|evidence_ref| {
            matches!(
                evidence_ref.family(),
                UiEvidenceFamily::Declaration | UiEvidenceFamily::Admission
            )
        })
        .collect()
}

fn assert_node_exposes_runtime_families(refs: &[worth_ui::facade::inspection::UiEvidenceRef]) {
    assert!(refs
        .iter()
        .any(|evidence_ref| evidence_ref.family() == UiEvidenceFamily::Graph));
    assert!(refs
        .iter()
        .any(|evidence_ref| evidence_ref.family() == UiEvidenceFamily::Obligation));
}

fn assert_refs_exclude_other_node(
    left: &[worth_ui::facade::inspection::UiEvidenceRef],
    right: &[worth_ui::facade::inspection::UiEvidenceRef],
) {
    assert!(left.iter().all(|left_ref| !right.contains(left_ref)));
    assert!(right.iter().all(|right_ref| !left.contains(right_ref)));
}
