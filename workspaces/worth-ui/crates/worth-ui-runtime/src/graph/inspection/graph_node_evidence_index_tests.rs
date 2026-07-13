use worth_ui_dsl::{
    UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken,
};
use worth_ui_inspection::{
    UiEvidenceAuthorityGeneration, UiEvidenceFamily, UiEvidenceRichness,
    UiInspectionObligationFamily, UiInspectionObligationRelevanceDetail, UiInspectionQuery,
    UiInspectionRelevance, UiInspectionRelevanceOutcome, UiInspectionScope, UiInspectionTarget,
    UiInspectionTargetClass, UiRelevanceFamily, UiRelevanceFilter,
};

use super::UiGraphNodeEvidenceIndex;
use crate::declaration::UiDeclarationArtifact;
use crate::evidence::UiEvidenceRef;
use crate::facade::{
    WorthUi,
    WorthUiApp,
    WorthUiDslPackage,
};
use crate::graph::UiGraphNodeIdentity;

const ALPHA_MODULE_PATH: &str = "app/graph_node_alpha.wui";
const BETA_MODULE_PATH: &str = "app/graph_node_beta.wui";

#[test]
fn graph_node_identity_lookup_is_indexed_and_zero_scan() {
    let app = graph_identity_app();
    let index =
        UiGraphNodeEvidenceIndex::rebuild(app.declaration_artifacts(), app.graph().snapshot());
    let alpha_lookup = lookup_for_module_path(&app, &index, ALPHA_MODULE_PATH);
    let beta_lookup = lookup_for_module_path(&app, &index, BETA_MODULE_PATH);

    assert_lookup_is_indexed(alpha_lookup.cost());
    assert_lookup_is_indexed(beta_lookup.cost());
    assert_node_neighborhood_families(alpha_lookup.neighborhood().refs());
    assert_node_neighborhood_families(beta_lookup.neighborhood().refs());
    assert_refs_exclude_other_node(
        alpha_lookup.neighborhood().refs(),
        beta_lookup.neighborhood().refs(),
    );
}

#[test]
fn public_graph_identity_lookup_returns_runtime_local_neighborhood() {
    let app = graph_identity_app();
    let alpha = graph_node_identity_for_module_path(&app, ALPHA_MODULE_PATH);
    let beta = graph_node_identity_for_module_path(&app, BETA_MODULE_PATH);
    let observation_before = app.inspection_observation();
    let alpha_first = app.inspect(graph_identity_query(alpha));
    let alpha_second = app.inspect(graph_identity_query(alpha));
    let beta_receipt = app.inspect(graph_identity_query(beta));
    let observation_after = app.inspection_observation();
    let alpha_first_slice = alpha_first
        .evidence_slice()
        .expect("graph node identity should return indexed graph-local refs");
    let alpha_second_slice = alpha_second
        .evidence_slice()
        .expect("repeated graph node identity query should remain indexed");
    let beta_slice = beta_receipt
        .evidence_slice()
        .expect("second graph node should return its own neighborhood");

    assert_eq!(
        observation_after.graph_node_evidence_index_rebuild_count(),
        observation_before.graph_node_evidence_index_rebuild_count(),
    );
    assert_eq!(
        alpha_first.relevance_outcome(),
        UiInspectionRelevanceOutcome::Matched
    );
    assert_eq!(
        alpha_first.authority_generation(),
        Some(UiEvidenceAuthorityGeneration::new(
            app.graph().generation().as_u64()
        ))
    );
    assert_eq!(
        alpha_first.evidence_slice_ref(),
        alpha_second.evidence_slice_ref()
    );
    assert_eq!(alpha_first_slice.refs(), alpha_second_slice.refs());
    assert_node_neighborhood_families(alpha_first_slice.refs());
    assert_node_neighborhood_families(beta_slice.refs());
    assert_refs_exclude_other_node(alpha_first_slice.refs(), beta_slice.refs());
}

#[test]
fn graph_node_identity_obligation_family_returns_indexed_graph_local_neighborhood() {
    let app = graph_identity_app();
    let index =
        UiGraphNodeEvidenceIndex::rebuild(app.declaration_artifacts(), app.graph().snapshot());
    let alpha = graph_node_identity_for_module_path(&app, ALPHA_MODULE_PATH);
    let beta = graph_node_identity_for_module_path(&app, BETA_MODULE_PATH);
    let alpha_lookup = lookup_for_graph_node_identity(&index, alpha);
    let beta_lookup = lookup_for_graph_node_identity(&index, beta);
    let observation_before = app.inspection_observation();
    let alpha_receipt = app.inspect(all_obligation_graph_identity_query(alpha));
    let alpha_repeat = app.inspect(all_obligation_graph_identity_query(alpha));
    let beta_receipt = app.inspect(all_obligation_graph_identity_query(beta));
    let structural_receipt = app.inspect(structural_obligation_graph_identity_query(alpha));
    let observation_after = app.inspection_observation();
    let alpha_slice = alpha_receipt
        .evidence_slice()
        .expect("graph-keyed obligation query should return graph-local obligation refs");
    let alpha_repeat_slice = alpha_repeat
        .evidence_slice()
        .expect("repeated graph-keyed obligation query should stay on the retained lane");
    let beta_slice = beta_receipt
        .evidence_slice()
        .expect("second graph node should return only its obligation neighborhood");

    assert_lookup_is_indexed(alpha_lookup.cost());
    assert_lookup_is_indexed(beta_lookup.cost());
    assert_eq!(
        observation_after.graph_node_evidence_index_rebuild_count(),
        observation_before.graph_node_evidence_index_rebuild_count(),
    );
    assert_eq!(
        obligation_refs(alpha_lookup.neighborhood().refs()),
        alpha_slice.refs().to_vec()
    );
    assert_eq!(
        alpha_receipt.evidence_slice_ref(),
        alpha_repeat.evidence_slice_ref()
    );
    assert_eq!(alpha_slice.refs(), alpha_repeat_slice.refs());
    assert_eq!(
        structural_receipt.relevance_outcome(),
        UiInspectionRelevanceOutcome::NotApplicableToTarget {
            target: UiInspectionTargetClass::GraphNodeIdentity,
        }
    );
    assert!(structural_receipt.evidence_slice().is_none());
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
}

#[test]
fn rebuilding_graph_node_index_from_authority_preserves_public_lookup_answers() {
    let mut app = graph_identity_app();
    let alpha = graph_node_identity_for_module_path(&app, ALPHA_MODULE_PATH);
    let before = app.inspect(graph_identity_query(alpha));
    let obligation_before = app.inspect(all_obligation_graph_identity_query(alpha));
    let observation_before = app.inspection_observation();

    app.rebuild_graph_evidence_indexes_from_authority();

    let observation_after = app.inspection_observation();
    let rebuilt_index =
        UiGraphNodeEvidenceIndex::rebuild(app.declaration_artifacts(), app.graph().snapshot());
    let rebuilt_lookup = lookup_for_graph_node_identity(&rebuilt_index, alpha);
    let after = app.inspect(graph_identity_query(alpha));
    let obligation_after = app.inspect(all_obligation_graph_identity_query(alpha));

    assert_lookup_is_indexed(rebuilt_lookup.cost());
    assert_eq!(
        observation_after.graph_node_evidence_index_rebuild_count(),
        observation_before.graph_node_evidence_index_rebuild_count() + 1,
    );
    assert_eq!(before.authority_generation(), after.authority_generation());
    assert_eq!(before.evidence_slice_ref(), after.evidence_slice_ref());
    assert_eq!(
        before.evidence_slice().map(|slice| slice.refs().to_vec()),
        after.evidence_slice().map(|slice| slice.refs().to_vec())
    );
    assert_eq!(
        obligation_before.authority_generation(),
        obligation_after.authority_generation()
    );
    assert_eq!(
        obligation_before.evidence_slice_ref(),
        obligation_after.evidence_slice_ref()
    );
    assert_eq!(
        obligation_before
            .evidence_slice()
            .map(|slice| slice.refs().to_vec()),
        obligation_after
            .evidence_slice()
            .map(|slice| slice.refs().to_vec())
    );
}

fn graph_identity_app() -> WorthUiApp {
    WorthUi::app()
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.runtime.graph-node-evidence-index")
                .with_semantic_artifact_spec(control_spec(
                    "ui.workflow.graph_identity.alpha",
                    ALPHA_MODULE_PATH,
                    "query-binding:attached:view",
                ))
                .with_semantic_artifact_spec(control_spec(
                    "ui.workflow.graph_identity.beta",
                    BETA_MODULE_PATH,
                    "service:portal",
                )),
        )
        .freeze()
}

fn graph_identity_query(graph_node_identity: UiGraphNodeIdentity) -> UiInspectionQuery {
    UiInspectionQuery::new(
        UiInspectionTarget::graph_node_identity(graph_node_identity.digest()),
        UiInspectionScope::graph(),
    )
    .with_relevance(UiInspectionRelevance::local(
        UiRelevanceFilter::target_local(),
    ))
    .with_richness(UiEvidenceRichness::refs_only())
}

fn all_obligation_graph_identity_query(
    graph_node_identity: UiGraphNodeIdentity,
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
    graph_node_identity: UiGraphNodeIdentity,
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

fn lookup_for_module_path<'a>(
    app: &WorthUiApp,
    index: &'a UiGraphNodeEvidenceIndex,
    module_path: &str,
) -> super::graph_node_evidence_index::UiGraphNodeEvidenceLookup<'a> {
    lookup_for_graph_node_identity(index, graph_node_identity_for_module_path(app, module_path))
}

fn lookup_for_graph_node_identity<'a>(
    index: &'a UiGraphNodeEvidenceIndex,
    graph_node_identity: UiGraphNodeIdentity,
) -> super::graph_node_evidence_index::UiGraphNodeEvidenceLookup<'a> {
    index
        .lookup_graph_node_identity(graph_node_identity)
        .expect("graph node identity should resolve through the graph evidence index")
}

fn graph_node_identity_for_module_path(app: &WorthUiApp, module_path: &str) -> UiGraphNodeIdentity {
    let artifact = declaration_artifact_for_module_path(app, module_path);
    app.graph()
        .lookup()
        .declaration_instances(artifact.identity())
        .value()
        .first()
        .copied()
        .expect("declaration should admit one graph node")
}

fn declaration_artifact_for_module_path<'a>(
    app: &'a WorthUiApp,
    module_path: &str,
) -> &'a UiDeclarationArtifact {
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

fn assert_lookup_is_indexed(cost: super::graph_node_evidence_index::UiGraphNodeEvidenceLookupCost) {
    assert_eq!(cost.graph_node_identity_index_lookups(), 1);
    assert_eq!(cost.graph_node_scan_count(), 0);
}

fn assert_node_neighborhood_families(refs: &[UiEvidenceRef]) {
    assert!(refs
        .iter()
        .any(|evidence_ref| evidence_ref.family() == UiEvidenceFamily::Declaration));
    assert!(refs
        .iter()
        .any(|evidence_ref| evidence_ref.family() == UiEvidenceFamily::Admission));
    assert!(refs
        .iter()
        .any(|evidence_ref| evidence_ref.family() == UiEvidenceFamily::Graph));
    assert!(refs
        .iter()
        .any(|evidence_ref| evidence_ref.family() == UiEvidenceFamily::Obligation));
}

fn obligation_refs(refs: &[UiEvidenceRef]) -> Vec<UiEvidenceRef> {
    refs.iter()
        .copied()
        .filter(|evidence_ref| evidence_ref.family() == UiEvidenceFamily::Obligation)
        .collect()
}

fn assert_refs_exclude_other_node(left: &[UiEvidenceRef], right: &[UiEvidenceRef]) {
    assert!(left.iter().all(|left_ref| !right.contains(left_ref)));
    assert!(right.iter().all(|right_ref| !left.contains(right_ref)));
}
