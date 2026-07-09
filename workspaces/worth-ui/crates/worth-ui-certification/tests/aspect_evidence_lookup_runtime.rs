use worth_ui::facade::inspection::{
    UiEvidenceLinkKind, UiEvidenceRichness, UiInspectionAspectRelevanceDetail, UiInspectionQuery,
    UiInspectionRelevance, UiInspectionRelevanceOutcome, UiInspectionScope, UiInspectionTarget,
    UiInspectionTargetClass, UiRelevanceFamily, UiRelevanceFilter,
};
use worth_ui_runtime::facade::graph::UiAspectEvidenceLane;

const ALPHA_MODULE_PATH: &str = "app/aspect_cert_alpha.wui";
const BETA_MODULE_PATH: &str = "app/aspect_cert_beta.wui";
const GAMMA_MODULE_PATH: &str = "app/aspect_cert_gamma.wui";
const DELTA_MODULE_PATH: &str = "app/aspect_cert_delta.wui";
const SHARED_ASPECT: &str = "content.text";
const COMPETING_PUBLISHED_ASPECT: &str = "appearance.background";
const COMPETING_CONSUMED_ASPECT: &str = "interaction.operability";

#[path = "fixtures/aspect_evidence_lookup_support.rs"]
mod aspect_evidence_lookup_support;

use aspect_evidence_lookup_support::{
    all_graph_node_digests, all_mounted_receipt_digests, aspect_identity_app,
    aspect_neighborhood_facts_from_receipt, assert_lane_identity_distinctness, assert_membership,
    consumed_aspect_query, consumed_aspect_with_provenance_query, declaration_artifact_digests,
    declaration_ref_digests, expected_graph_node_digests, expected_mounted_receipt_digests,
    published_aspect_query, published_aspect_with_provenance_query,
};

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
