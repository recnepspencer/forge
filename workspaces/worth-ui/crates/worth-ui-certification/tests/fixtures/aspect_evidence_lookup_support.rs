use std::collections::BTreeSet;

use worth_ui::facade::app::{WorthUi, WorthUiApp};
use worth_ui::facade::inspection::{
    UiEvidenceFamily, UiEvidenceRichness, UiInspectionAspectRelevanceDetail, UiInspectionQuery,
    UiInspectionRelevance, UiInspectionScope, UiInspectionTarget, UiRelevanceFamily,
    UiRelevanceFilter,
};
use worth_ui_dsl::{
    UiDslAspectName, UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily,
    UiDslSemanticKey, UiDslSourceProvenance, UiDslStructuralToken, WorthUiDslPackage,
};
use worth_ui_runtime::facade::graph::{
    project_aspect_evidence_refs, UiAspectEvidenceLane, UiAspectEvidenceRefProjection,
    UiAspectEvidenceSubjectKind, UiGraphNodeIdentity, UiMountedReceiptIdentity,
};

use super::{
    ALPHA_MODULE_PATH, BETA_MODULE_PATH, COMPETING_CONSUMED_ASPECT, COMPETING_PUBLISHED_ASPECT,
    DELTA_MODULE_PATH, GAMMA_MODULE_PATH, SHARED_ASPECT,
};

pub fn aspect_identity_app() -> WorthUiApp {
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
        .expect("application preparation should succeed")
}

pub fn published_aspect_query(aspect_name: &str) -> UiInspectionQuery {
    base_aspect_query(UiInspectionTarget::published_aspect(aspect_name))
}

pub fn published_aspect_with_provenance_query(aspect_name: &str) -> UiInspectionQuery {
    aspect_query_with_provenance(UiInspectionTarget::published_aspect(aspect_name))
}

pub fn consumed_aspect_query(aspect_name: &str) -> UiInspectionQuery {
    base_aspect_query(UiInspectionTarget::consumed_aspect(aspect_name))
}

pub fn consumed_aspect_with_provenance_query(aspect_name: &str) -> UiInspectionQuery {
    aspect_query_with_provenance(UiInspectionTarget::consumed_aspect(aspect_name))
}

pub fn expected_graph_node_digests(app: &WorthUiApp, module_paths: &[&str]) -> BTreeSet<u64> {
    module_paths
        .iter()
        .map(|path| graph_node_identity(app, path).digest())
        .collect()
}

pub fn expected_mounted_receipt_digests(app: &WorthUiApp, module_paths: &[&str]) -> BTreeSet<u64> {
    module_paths
        .iter()
        .map(|path| mounted_receipt_identity(app, path).digest())
        .collect()
}

pub fn all_graph_node_digests(app: &WorthUiApp) -> Vec<u64> {
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

pub fn all_mounted_receipt_digests(app: &WorthUiApp) -> Vec<u64> {
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

pub fn declaration_artifact_digests(app: &WorthUiApp, module_paths: &[&str]) -> Vec<u64> {
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

pub fn declaration_ref_digests(refs: &[worth_ui::facade::inspection::UiEvidenceRef]) -> Vec<u64> {
    let mut digests = refs
        .iter()
        .filter(|evidence_ref| evidence_ref.family() == UiEvidenceFamily::Declaration)
        .map(|evidence_ref| evidence_ref.identity().digest())
        .collect::<Vec<_>>();
    digests.sort_unstable();
    digests
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AspectNeighborhoodFacts {
    pub lanes: BTreeSet<UiAspectEvidenceLane>,
    pub graph_node_digests: BTreeSet<u64>,
    pub mounted_receipt_digests: BTreeSet<u64>,
    pub entries: BTreeSet<UiAspectEvidenceRefProjection>,
}

pub fn aspect_neighborhood_facts_from_receipt(
    aspect_name: &str,
    receipt: &worth_ui::facade::inspection::UiInspectionReceipt,
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

pub fn assert_membership(
    facts: &AspectNeighborhoodFacts,
    lane: UiAspectEvidenceLane,
    graph_node_digests: &BTreeSet<u64>,
    mounted_receipt_digests: &BTreeSet<u64>,
) {
    assert_eq!(facts.lanes, BTreeSet::from([lane]));
    assert_eq!(facts.graph_node_digests, *graph_node_digests);
    assert_eq!(facts.mounted_receipt_digests, *mounted_receipt_digests);
}

pub fn assert_lane_identity_distinctness(
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
