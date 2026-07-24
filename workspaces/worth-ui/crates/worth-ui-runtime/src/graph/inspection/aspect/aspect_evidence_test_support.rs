use std::collections::BTreeSet;

use super::super::super::super::facade::{WorthUi, WorthUiApp, WorthUiDslPackage};
use crate::graph::{
    project_aspect_evidence_refs, UiAspectEvidenceLane, UiAspectEvidenceRefProjection,
    UiAspectEvidenceSubjectKind, UiGraphMountEligibilityIdentity, UiGraphNodeIdentity,
};
use worth_ui_dsl::{
    UiDslAspectName, UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily,
    UiDslSemanticKey, UiDslSourceProvenance, UiDslStructuralToken,
};
use worth_ui_inspection::{
    UiEvidenceFamily, UiEvidenceRichness, UiInspectionAspectRelevanceDetail, UiInspectionQuery,
    UiInspectionRelevance, UiInspectionScope, UiInspectionTarget, UiRelevanceFamily,
    UiRelevanceFilter,
};

pub(super) const ALPHA_MODULE_PATH: &str = "app/aspect_alpha.wui";
pub(super) const BETA_MODULE_PATH: &str = "app/aspect_beta.wui";
pub(super) const GAMMA_MODULE_PATH: &str = "app/aspect_gamma.wui";
pub(super) const DELTA_MODULE_PATH: &str = "app/aspect_delta.wui";
pub(super) const SHARED_ASPECT: &str = "content.text";
pub(super) const COMPETING_PUBLISHED_ASPECT: &str = "appearance.background";
pub(super) const COMPETING_CONSUMED_ASPECT: &str = "interaction.operability";

pub(super) fn aspect_identity_app() -> WorthUiApp {
    WorthUi::app()
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.runtime.aspect-evidence-index")
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

pub(super) fn published_aspect_query(aspect_name: &str) -> UiInspectionQuery {
    base_aspect_query(UiInspectionTarget::published_aspect(aspect_name))
}

pub(super) fn consumed_aspect_query(aspect_name: &str) -> UiInspectionQuery {
    base_aspect_query(UiInspectionTarget::consumed_aspect(aspect_name))
}

pub(super) fn consumed_aspect_with_provenance_query(aspect_name: &str) -> UiInspectionQuery {
    UiInspectionQuery::new(
        UiInspectionTarget::consumed_aspect(aspect_name),
        UiInspectionScope::graph(),
    )
    .with_relevance(
        UiInspectionRelevance::local(UiRelevanceFilter::family(UiRelevanceFamily::Aspect))
            .with_aspect_detail(
                UiInspectionAspectRelevanceDetail::new().include_direct_provenance_refs(),
            ),
    )
    .with_richness(UiEvidenceRichness::refs_only())
}

pub(super) fn expected_declaration_indexes(app: &WorthUiApp, module_paths: &[&str]) -> Vec<usize> {
    module_paths
        .iter()
        .map(|path| declaration_artifact_index(app, path))
        .collect()
}

pub(super) fn expected_graph_node_digests(
    app: &WorthUiApp,
    module_paths: &[&str],
) -> BTreeSet<u64> {
    module_paths
        .iter()
        .map(|path| graph_node_identity(app, path).digest())
        .collect()
}

pub(super) fn expected_mount_eligibility_digests(
    app: &WorthUiApp,
    module_paths: &[&str],
) -> BTreeSet<u64> {
    module_paths
        .iter()
        .map(|path| mount_eligibility_identity(app, path).digest())
        .collect()
}

pub(super) fn all_graph_node_digests(app: &WorthUiApp) -> Vec<u64> {
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

pub(super) fn all_mount_eligibility_digests(app: &WorthUiApp) -> Vec<u64> {
    expected_mount_eligibility_digests(
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

pub(super) fn declaration_ref_digests(
    app: &WorthUiApp,
    receipt: &crate::facade::inspection_bridge::UiInspectionReceipt,
) -> Vec<u64> {
    let mut digests = receipt
        .evidence_slice()
        .expect("query should return an evidence slice")
        .refs()
        .iter()
        .filter(|evidence_ref| evidence_ref.family() == UiEvidenceFamily::Declaration)
        .map(|evidence_ref| evidence_ref.identity().digest())
        .collect::<Vec<_>>();
    digests.sort_unstable();
    assert_eq!(
        digests,
        declaration_artifact_digests(app, &[ALPHA_MODULE_PATH, GAMMA_MODULE_PATH])
    );
    digests
}

pub(super) fn aspect_ref_count(
    receipt: &crate::facade::inspection_bridge::UiInspectionReceipt,
) -> usize {
    receipt
        .evidence_slice()
        .expect("aspect query should return an evidence slice")
        .refs()
        .iter()
        .filter(|evidence_ref| evidence_ref.family() == UiEvidenceFamily::Aspect)
        .count()
}

pub(super) fn assert_indexed_lookup(
    cost: super::aspect_evidence_neighborhood::UiAspectEvidenceLookupCost,
) {
    assert_eq!(cost.aspect_identity_index_lookups(), 1);
    assert_eq!(cost.aspect_scan_count(), 0);
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct AspectNeighborhoodFacts {
    pub(super) lanes: BTreeSet<UiAspectEvidenceLane>,
    pub(super) graph_node_digests: BTreeSet<u64>,
    pub(super) mount_eligibility_digests: BTreeSet<u64>,
    pub(super) entries: BTreeSet<UiAspectEvidenceRefProjection>,
}

pub(super) fn aspect_neighborhood_facts_from_receipt(
    aspect_name: &str,
    receipt: &crate::facade::inspection_bridge::UiInspectionReceipt,
    all_graph_node_digests: &[u64],
    all_mount_eligibility_digests: &[u64],
) -> AspectNeighborhoodFacts {
    aspect_neighborhood_facts(
        aspect_name,
        receipt
            .evidence_slice()
            .expect("query should return an evidence slice")
            .refs(),
        all_graph_node_digests,
        all_mount_eligibility_digests,
    )
}

pub(super) fn aspect_neighborhood_facts(
    aspect_name: &str,
    refs: &[crate::evidence::UiEvidenceRef],
    all_graph_node_digests: &[u64],
    all_mount_eligibility_digests: &[u64],
) -> AspectNeighborhoodFacts {
    let projections = project_aspect_evidence_refs(
        refs,
        aspect_name,
        all_graph_node_digests,
        all_mount_eligibility_digests,
    );

    assert_eq!(
        projections.len(),
        refs.iter()
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
        mount_eligibility_digests: projections
            .iter()
            .filter(|projection| {
                projection.subject_kind() == UiAspectEvidenceSubjectKind::MountEligibility
            })
            .map(|projection| projection.subject_digest())
            .collect(),
        entries: projections,
    }
}

pub(super) fn assert_membership(
    facts: &AspectNeighborhoodFacts,
    lane: UiAspectEvidenceLane,
    graph_node_digests: &BTreeSet<u64>,
    mount_eligibility_digests: &BTreeSet<u64>,
) {
    assert_eq!(
        facts.lanes,
        BTreeSet::from([lane]),
        "aspect lane membership should stay exact",
    );
    assert_eq!(
        facts.graph_node_digests, *graph_node_digests,
        "graph-node membership should stay exact",
    );
    assert_eq!(
        facts.mount_eligibility_digests, *mount_eligibility_digests,
        "mount-eligibility membership should stay exact",
    );
}

pub(super) fn assert_lane_identity_distinctness(
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
            "published and consumed aspect refs should keep distinct exact identities for the same subject",
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

fn mount_eligibility_identity(
    app: &WorthUiApp,
    module_path: &str,
) -> UiGraphMountEligibilityIdentity {
    app.graph()
        .lookup()
        .mount_eligibility_slot_for_node(graph_node_identity(app, module_path))
        .expect("graph node should own a mount eligibility slot")
        .value()
        .mount_eligibility_identity()
}

pub(super) fn declaration_artifact_digests(app: &WorthUiApp, module_paths: &[&str]) -> Vec<u64> {
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
