use std::collections::BTreeSet;

use worth_ui::facade::app::{WorthUi, WorthUiApp};
use worth_ui::facade::inspection::{
    UiEvidenceFamily, UiEvidenceLinkKind, UiEvidenceRichness, UiInspectionAspectRelevanceDetail,
    UiInspectionQuery, UiInspectionRelevance, UiInspectionRelevanceOutcome, UiInspectionScope,
    UiInspectionTarget, UiInspectionTargetClass, UiRelevanceFamily, UiRelevanceFilter,
};
use worth_ui_dsl::{
    UiDslAspectName, UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily,
    UiDslSemanticKey, UiDslSourceProvenance, UiDslStructuralToken, WorthUiDslPackage,
};
use worth_ui_runtime::facade::graph::{
    project_aspect_evidence_refs, UiAspectEvidenceLane, UiAspectEvidenceRefProjection,
    UiAspectEvidenceSubjectKind, UiGraphNodeIdentity, UiMountedReceiptIdentity,
};

const ALPHA_MODULE_PATH: &str = "app/aspect_cert_alpha.wui";
const BETA_MODULE_PATH: &str = "app/aspect_cert_beta.wui";
const GAMMA_MODULE_PATH: &str = "app/aspect_cert_gamma.wui";
const DELTA_MODULE_PATH: &str = "app/aspect_cert_delta.wui";
const SHARED_ASPECT: &str = "content.text";
const COMPETING_PUBLISHED_ASPECT: &str = "appearance.background";
const COMPETING_CONSUMED_ASPECT: &str = "interaction.operability";

#[test]
fn shared_aspect_queries_stay_family_local_and_explicitly_cover_receipts() {
    let app = aspect_identity_app();
    let all_graph_node_digests = all_graph_node_digests(&app);
    let all_mounted_receipt_digests = all_mounted_receipt_digests(&app);
    let published_receipt = app.inspect(published_aspect_query(" Content.Text "));
    let published_provenance_receipt =
        app.inspect(published_aspect_with_provenance_query(SHARED_ASPECT));
    let consumed_receipt = app.inspect(consumed_aspect_query(SHARED_ASPECT));
    let consumed_provenance_receipt =
        app.inspect(consumed_aspect_with_provenance_query(SHARED_ASPECT));
    let competing_published = app.inspect(published_aspect_query(COMPETING_PUBLISHED_ASPECT));
    let competing_consumed = app.inspect(consumed_aspect_query(COMPETING_CONSUMED_ASPECT));
    let published_facts = aspect_neighborhood_facts_from_receipt(
        SHARED_ASPECT,
        &published_receipt,
        &all_graph_node_digests,
        &all_mounted_receipt_digests,
    );
    let published_provenance_facts = aspect_neighborhood_facts_from_receipt(
        SHARED_ASPECT,
        &published_provenance_receipt,
        &all_graph_node_digests,
        &all_mounted_receipt_digests,
    );
    let consumed_facts = aspect_neighborhood_facts_from_receipt(
        SHARED_ASPECT,
        &consumed_receipt,
        &all_graph_node_digests,
        &all_mounted_receipt_digests,
    );
    let consumed_provenance_facts = aspect_neighborhood_facts_from_receipt(
        SHARED_ASPECT,
        &consumed_provenance_receipt,
        &all_graph_node_digests,
        &all_mounted_receipt_digests,
    );
    let competing_published_facts = aspect_neighborhood_facts_from_receipt(
        COMPETING_PUBLISHED_ASPECT,
        &competing_published,
        &all_graph_node_digests,
        &all_mounted_receipt_digests,
    );
    let competing_consumed_facts = aspect_neighborhood_facts_from_receipt(
        COMPETING_CONSUMED_ASPECT,
        &competing_consumed,
        &all_graph_node_digests,
        &all_mounted_receipt_digests,
    );

    assert_eq!(
        published_receipt.relevance_outcome(),
        UiInspectionRelevanceOutcome::Matched
    );
    assert_eq!(
        consumed_receipt.relevance_outcome(),
        UiInspectionRelevanceOutcome::Matched
    );
    assert_eq!(published_facts, published_provenance_facts);
    assert_eq!(consumed_facts, consumed_provenance_facts);
    assert_membership(
        &published_facts,
        UiAspectEvidenceLane::Published,
        &expected_graph_node_digests(&app, &[ALPHA_MODULE_PATH, GAMMA_MODULE_PATH]),
        &expected_mounted_receipt_digests(&app, &[ALPHA_MODULE_PATH, GAMMA_MODULE_PATH]),
    );
    assert_membership(
        &consumed_facts,
        UiAspectEvidenceLane::Consumed,
        &expected_graph_node_digests(&app, &[ALPHA_MODULE_PATH, GAMMA_MODULE_PATH]),
        &expected_mounted_receipt_digests(&app, &[ALPHA_MODULE_PATH, GAMMA_MODULE_PATH]),
    );
    assert_membership(
        &competing_published_facts,
        UiAspectEvidenceLane::Published,
        &expected_graph_node_digests(&app, &[BETA_MODULE_PATH]),
        &expected_mounted_receipt_digests(&app, &[BETA_MODULE_PATH]),
    );
    assert_membership(
        &competing_consumed_facts,
        UiAspectEvidenceLane::Consumed,
        &expected_graph_node_digests(&app, &[DELTA_MODULE_PATH]),
        &expected_mounted_receipt_digests(&app, &[DELTA_MODULE_PATH]),
    );
    assert_lane_identity_distinctness(&published_facts, &consumed_facts);
}

#[test]
fn shared_published_and_consumed_aspect_queries_keep_relationships_and_provenance_in_parity() {
    let app = aspect_identity_app();
    let all_graph_node_digests = all_graph_node_digests(&app);
    let all_mounted_receipt_digests = all_mounted_receipt_digests(&app);
    let published_receipt = app.inspect(published_aspect_with_provenance_query(SHARED_ASPECT));
    let consumed_receipt = app.inspect(consumed_aspect_with_provenance_query(SHARED_ASPECT));
    let published_facts = aspect_neighborhood_facts_from_receipt(
        SHARED_ASPECT,
        &published_receipt,
        &all_graph_node_digests,
        &all_mounted_receipt_digests,
    );
    let consumed_facts = aspect_neighborhood_facts_from_receipt(
        SHARED_ASPECT,
        &consumed_receipt,
        &all_graph_node_digests,
        &all_mounted_receipt_digests,
    );

    assert_eq!(
        published_facts.graph_node_digests,
        consumed_facts.graph_node_digests,
    );
    assert_eq!(
        published_facts.mounted_receipt_digests,
        consumed_facts.mounted_receipt_digests,
    );
    assert_eq!(
        declaration_ref_digests(
            published_receipt
                .evidence_slice()
                .expect("published slice")
                .refs()
        ),
        declaration_ref_digests(
            consumed_receipt
                .evidence_slice()
                .expect("consumed slice")
                .refs()
        ),
    );
    assert_eq!(
        declaration_ref_digests(
            published_receipt
                .evidence_slice()
                .expect("published slice")
                .refs()
        ),
        declaration_artifact_digests(&app, &[ALPHA_MODULE_PATH, GAMMA_MODULE_PATH]),
    );
    assert_lane_identity_distinctness(&published_facts, &consumed_facts);
}

#[test]
fn aspect_targets_reject_unrelated_family_filters_and_missing_targets() {
    let app = aspect_identity_app();
    let wrong_family_receipt = app.inspect(
        UiInspectionQuery::new(
            UiInspectionTarget::published_aspect(SHARED_ASPECT),
            UiInspectionScope::graph(),
        )
        .with_relevance(UiInspectionRelevance::local(UiRelevanceFilter::family(
            UiRelevanceFamily::Obligation,
        )))
        .with_richness(UiEvidenceRichness::refs_only()),
    );
    let widened_receipt = app.inspect(
        UiInspectionQuery::new(
            UiInspectionTarget::consumed_aspect(SHARED_ASPECT),
            UiInspectionScope::graph(),
        )
        .with_relevance(
            UiInspectionRelevance::local(
                UiRelevanceFilter::family(UiRelevanceFamily::Aspect)
                    .include_link(UiEvidenceLinkKind::Explains),
            )
            .with_aspect_detail(UiInspectionAspectRelevanceDetail::new()),
        )
        .with_richness(UiEvidenceRichness::refs_only()),
    );
    let missing_receipt = app.inspect(published_aspect_query("missing.aspect"));

    assert_eq!(
        wrong_family_receipt.relevance_outcome(),
        UiInspectionRelevanceOutcome::NotApplicableToTarget {
            target: UiInspectionTargetClass::PublishedAspect,
        }
    );
    assert!(wrong_family_receipt.evidence_slice().is_none());
    assert_eq!(
        widened_receipt.relevance_outcome(),
        UiInspectionRelevanceOutcome::NotApplicableToTarget {
            target: UiInspectionTargetClass::ConsumedAspect,
        }
    );
    assert!(widened_receipt.evidence_slice().is_none());
    assert_eq!(
        missing_receipt.relevance_outcome(),
        UiInspectionRelevanceOutcome::UnsupportedScope {
            scope: UiInspectionScope::Graph,
        }
    );
    assert!(missing_receipt.support_report().is_some());
    assert!(missing_receipt.evidence_slice().is_none());
}

fn aspect_identity_app() -> WorthUiApp {
    WorthUi::app()
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.aspect-evidence-lookup")
                .with_semantic_artifact_spec(
                    control_spec(
                        "ui.workflow.aspect.alpha",
                        ALPHA_MODULE_PATH,
                        "query-binding:attached:view",
                    )
                    .with_published_aspect(UiDslAspectName::new(SHARED_ASPECT))
                    .with_consumed_aspect(UiDslAspectName::new(SHARED_ASPECT)),
                )
                .with_semantic_artifact_spec(
                    control_spec(
                        "ui.workflow.aspect.beta",
                        BETA_MODULE_PATH,
                        "service:portal",
                    )
                    .with_published_aspect(UiDslAspectName::new(COMPETING_PUBLISHED_ASPECT)),
                )
                .with_semantic_artifact_spec(
                    control_spec(
                        "ui.workflow.aspect.gamma",
                        GAMMA_MODULE_PATH,
                        "query-binding:attached:view",
                    )
                    .with_published_aspect(UiDslAspectName::new(SHARED_ASPECT))
                    .with_consumed_aspect(UiDslAspectName::new(SHARED_ASPECT)),
                )
                .with_semantic_artifact_spec(
                    control_spec(
                        "ui.workflow.aspect.delta",
                        DELTA_MODULE_PATH,
                        "service:portal",
                    )
                    .with_consumed_aspect(UiDslAspectName::new(COMPETING_CONSUMED_ASPECT)),
                ),
        )
        .freeze()
}

fn published_aspect_query(aspect_name: &str) -> UiInspectionQuery {
    base_aspect_query(UiInspectionTarget::published_aspect(aspect_name))
}

fn published_aspect_with_provenance_query(aspect_name: &str) -> UiInspectionQuery {
    aspect_query_with_provenance(UiInspectionTarget::published_aspect(aspect_name))
}

fn consumed_aspect_query(aspect_name: &str) -> UiInspectionQuery {
    base_aspect_query(UiInspectionTarget::consumed_aspect(aspect_name))
}

fn consumed_aspect_with_provenance_query(aspect_name: &str) -> UiInspectionQuery {
    aspect_query_with_provenance(UiInspectionTarget::consumed_aspect(aspect_name))
}

fn base_aspect_query(target: UiInspectionTarget) -> UiInspectionQuery {
    UiInspectionQuery::new(target, UiInspectionScope::graph())
        .with_relevance(UiInspectionRelevance::local(UiRelevanceFilter::family(
            UiRelevanceFamily::Aspect,
        )))
        .with_richness(UiEvidenceRichness::refs_only())
}

fn aspect_query_with_provenance(target: UiInspectionTarget) -> UiInspectionQuery {
    UiInspectionQuery::new(target, UiInspectionScope::graph())
        .with_relevance(
            UiInspectionRelevance::local(UiRelevanceFilter::family(UiRelevanceFamily::Aspect))
                .with_aspect_detail(
                    UiInspectionAspectRelevanceDetail::new().include_direct_provenance_refs(),
                ),
        )
        .with_richness(UiEvidenceRichness::refs_only())
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

fn expected_graph_node_digests(app: &WorthUiApp, module_paths: &[&str]) -> BTreeSet<u64> {
    module_paths
        .iter()
        .map(|path| graph_node_identity(app, path).digest())
        .collect()
}

fn expected_mounted_receipt_digests(app: &WorthUiApp, module_paths: &[&str]) -> BTreeSet<u64> {
    module_paths
        .iter()
        .map(|path| mounted_receipt_identity(app, path).digest())
        .collect()
}

fn all_graph_node_digests(app: &WorthUiApp) -> Vec<u64> {
    expected_graph_node_digests(
        app,
        &[
            ALPHA_MODULE_PATH,
            BETA_MODULE_PATH,
            GAMMA_MODULE_PATH,
            DELTA_MODULE_PATH,
        ],
    )
    .into_iter()
    .collect()
}

fn all_mounted_receipt_digests(app: &WorthUiApp) -> Vec<u64> {
    expected_mounted_receipt_digests(
        app,
        &[
            ALPHA_MODULE_PATH,
            BETA_MODULE_PATH,
            GAMMA_MODULE_PATH,
            DELTA_MODULE_PATH,
        ],
    )
    .into_iter()
    .collect()
}

fn declaration_artifact_index(app: &WorthUiApp, module_path: &str) -> usize {
    app.declaration_artifacts()
        .iter()
        .position(|artifact| artifact.provenance().source_provenance().module_path() == module_path)
        .expect("declaration artifact should resolve by module path")
}

fn graph_node_identity(app: &WorthUiApp, module_path: &str) -> UiGraphNodeIdentity {
    app.graph()
        .lookup()
        .declaration_instances(
            app.declaration_artifacts()[declaration_artifact_index(app, module_path)].identity(),
        )
        .value()
        .first()
        .copied()
        .expect("declaration should admit one graph node")
}

fn mounted_receipt_identity(app: &WorthUiApp, module_path: &str) -> UiMountedReceiptIdentity {
    app.graph()
        .lookup()
        .mounted_receipt_slot_for_node(graph_node_identity(app, module_path))
        .expect("graph node should own a mounted receipt slot")
        .value()
        .mounted_receipt_identity()
}

fn declaration_artifact_digests(app: &WorthUiApp, module_paths: &[&str]) -> Vec<u64> {
    let mut digests = module_paths
        .iter()
        .map(|path| {
            app.declaration_artifacts()[declaration_artifact_index(app, path)]
                .identity()
                .digest()
                .raw()
        })
        .collect::<Vec<_>>();
    digests.sort_unstable();
    digests
}

fn declaration_ref_digests(refs: &[worth_ui::facade::inspection::UiEvidenceRef]) -> Vec<u64> {
    let mut digests = refs
        .iter()
        .filter(|evidence_ref| evidence_ref.family() == UiEvidenceFamily::Declaration)
        .map(|evidence_ref| evidence_ref.identity().digest())
        .collect::<Vec<_>>();
    digests.sort_unstable();
    digests
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct AspectNeighborhoodFacts {
    lanes: BTreeSet<UiAspectEvidenceLane>,
    graph_node_digests: BTreeSet<u64>,
    mounted_receipt_digests: BTreeSet<u64>,
    entries: BTreeSet<UiAspectEvidenceRefProjection>,
}

fn aspect_neighborhood_facts_from_receipt(
    aspect_name: &str,
    receipt: &worth_ui::facade::UiInspectionReceipt,
    all_graph_node_digests: &[u64],
    all_mounted_receipt_digests: &[u64],
) -> AspectNeighborhoodFacts {
    let slice = receipt
        .evidence_slice()
        .expect("query should return an evidence slice");
    let projections = project_aspect_evidence_refs(
        slice.refs(),
        aspect_name,
        all_graph_node_digests,
        all_mounted_receipt_digests,
    );

    assert_eq!(
        projections.len(),
        slice
            .refs()
            .iter()
            .filter(|evidence_ref| evidence_ref.family() == UiEvidenceFamily::Aspect)
            .count(),
    );

    AspectNeighborhoodFacts {
        lanes: projections
            .iter()
            .map(|projection| projection.lane())
            .collect(),
        graph_node_digests: projections
            .iter()
            .filter(|projection| {
                projection.subject_kind() == UiAspectEvidenceSubjectKind::GraphNode
            })
            .map(|projection| projection.subject_digest())
            .collect(),
        mounted_receipt_digests: projections
            .iter()
            .filter(|projection| {
                projection.subject_kind() == UiAspectEvidenceSubjectKind::MountedReceipt
            })
            .map(|projection| projection.subject_digest())
            .collect(),
        entries: projections,
    }
}

fn assert_membership(
    facts: &AspectNeighborhoodFacts,
    lane: UiAspectEvidenceLane,
    graph_node_digests: &BTreeSet<u64>,
    mounted_receipt_digests: &BTreeSet<u64>,
) {
    assert_eq!(facts.lanes, BTreeSet::from([lane]));
    assert_eq!(facts.graph_node_digests, *graph_node_digests);
    assert_eq!(facts.mounted_receipt_digests, *mounted_receipt_digests);
}

fn assert_lane_identity_distinctness(
    published: &AspectNeighborhoodFacts,
    consumed: &AspectNeighborhoodFacts,
) {
    for published_entry in &published.entries {
        let matching_consumed = consumed
            .entries
            .iter()
            .find(|consumed_entry| {
                consumed_entry.subject_kind() == published_entry.subject_kind()
                    && consumed_entry.subject_digest() == published_entry.subject_digest()
            })
            .expect("shared membership should exist on both published and consumed lanes");
        assert_ne!(
            published_entry.evidence_identity_digest(),
            matching_consumed.evidence_identity_digest(),
        );
    }
}
