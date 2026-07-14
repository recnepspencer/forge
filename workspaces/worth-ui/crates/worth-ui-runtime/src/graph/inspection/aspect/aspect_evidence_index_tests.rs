use super::aspect_evidence_test_support::{
    all_graph_node_digests, all_mounted_receipt_digests, aspect_identity_app,
    aspect_neighborhood_facts, aspect_neighborhood_facts_from_receipt, aspect_ref_count,
    assert_indexed_lookup, assert_lane_identity_distinctness, assert_membership,
    consumed_aspect_query, consumed_aspect_with_provenance_query, declaration_artifact_digests,
    declaration_ref_digests, expected_declaration_indexes, expected_graph_node_digests,
    expected_mounted_receipt_digests, published_aspect_query, ALPHA_MODULE_PATH, BETA_MODULE_PATH,
    COMPETING_CONSUMED_ASPECT, COMPETING_PUBLISHED_ASPECT, DELTA_MODULE_PATH, GAMMA_MODULE_PATH,
    SHARED_ASPECT,
};
use super::UiGraphAspectEvidenceIndexes;
use crate::facade::inspection_bridge::UiInspectionReceipt;
use crate::graph::{UiAspectEvidenceLane, UiGraphNodeEvidenceIndex};
use worth_ui_inspection::{
    UiEvidenceLinkKind, UiEvidenceRichness, UiInspectionAspectRelevanceDetail, UiInspectionQuery,
    UiInspectionRelevance, UiInspectionRelevanceOutcome, UiInspectionScope, UiInspectionTarget,
    UiInspectionTargetClass, UiRelevanceFamily, UiRelevanceFilter,
};

#[test]
fn shared_aspect_lookups_are_indexed_receipt_backed_and_in_parity() {
    let app = aspect_identity_app();
    let graph_node_index =
        UiGraphNodeEvidenceIndex::rebuild(app.declaration_artifacts(), app.graph().snapshot());
    let aspect_indexes =
        UiGraphAspectEvidenceIndexes::rebuild(app.graph().snapshot(), &graph_node_index);
    let all_graph_node_digests = all_graph_node_digests(&app);
    let all_mounted_receipt_digests = all_mounted_receipt_digests(&app);
    let shared_published = aspect_indexes
        .lookup_published_aspect(SHARED_ASPECT)
        .unwrap();
    let shared_consumed = aspect_indexes
        .lookup_consumed_aspect(SHARED_ASPECT)
        .unwrap();
    let competing_published = aspect_indexes
        .lookup_published_aspect(COMPETING_PUBLISHED_ASPECT)
        .unwrap();
    let competing_consumed = aspect_indexes
        .lookup_consumed_aspect(COMPETING_CONSUMED_ASPECT)
        .unwrap();
    let public_shared_published = app.inspect(published_aspect_query(SHARED_ASPECT));
    let public_shared_consumed = app.inspect(consumed_aspect_query(SHARED_ASPECT));
    let public_competing_published =
        app.inspect(published_aspect_query(COMPETING_PUBLISHED_ASPECT));
    let public_competing_consumed = app.inspect(consumed_aspect_query(COMPETING_CONSUMED_ASPECT));
    let shared_published_facts = aspect_neighborhood_facts(
        SHARED_ASPECT,
        shared_published.neighborhood().refs(),
        &all_graph_node_digests,
        &all_mounted_receipt_digests,
    );
    let shared_consumed_facts = aspect_neighborhood_facts(
        SHARED_ASPECT,
        shared_consumed.neighborhood().refs(),
        &all_graph_node_digests,
        &all_mounted_receipt_digests,
    );
    let competing_published_facts = aspect_neighborhood_facts(
        COMPETING_PUBLISHED_ASPECT,
        competing_published.neighborhood().refs(),
        &all_graph_node_digests,
        &all_mounted_receipt_digests,
    );
    let competing_consumed_facts = aspect_neighborhood_facts(
        COMPETING_CONSUMED_ASPECT,
        competing_consumed.neighborhood().refs(),
        &all_graph_node_digests,
        &all_mounted_receipt_digests,
    );

    assert_indexed_lookup(shared_published.cost());
    assert_indexed_lookup(shared_consumed.cost());
    assert_indexed_lookup(competing_published.cost());
    assert_indexed_lookup(competing_consumed.cost());
    assert_eq!(
        shared_published
            .neighborhood()
            .declaration_artifact_indexes(),
        shared_consumed
            .neighborhood()
            .declaration_artifact_indexes(),
    );
    assert_eq!(
        shared_published
            .neighborhood()
            .declaration_artifact_indexes(),
        expected_declaration_indexes(&app, &[ALPHA_MODULE_PATH, GAMMA_MODULE_PATH]),
    );
    assert_eq!(
        shared_published_facts,
        aspect_neighborhood_facts_from_receipt(
            SHARED_ASPECT,
            &public_shared_published,
            &all_graph_node_digests,
            &all_mounted_receipt_digests,
        ),
    );
    assert_eq!(
        shared_consumed_facts,
        aspect_neighborhood_facts_from_receipt(
            SHARED_ASPECT,
            &public_shared_consumed,
            &all_graph_node_digests,
            &all_mounted_receipt_digests,
        ),
    );
    assert_eq!(
        competing_published_facts,
        aspect_neighborhood_facts_from_receipt(
            COMPETING_PUBLISHED_ASPECT,
            &public_competing_published,
            &all_graph_node_digests,
            &all_mounted_receipt_digests,
        ),
    );
    assert_eq!(
        competing_consumed_facts,
        aspect_neighborhood_facts_from_receipt(
            COMPETING_CONSUMED_ASPECT,
            &public_competing_consumed,
            &all_graph_node_digests,
            &all_mounted_receipt_digests,
        ),
    );
    assert_membership(
        &shared_published_facts,
        UiAspectEvidenceLane::Published,
        &expected_graph_node_digests(&app, &[ALPHA_MODULE_PATH, GAMMA_MODULE_PATH]),
        &expected_mounted_receipt_digests(&app, &[ALPHA_MODULE_PATH, GAMMA_MODULE_PATH]),
    );
    assert_membership(
        &shared_consumed_facts,
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
    assert_lane_identity_distinctness(&shared_published_facts, &shared_consumed_facts);
}

#[test]
fn ordinary_aspect_queries_stay_local_and_do_not_rebuild_aspect_indexes() {
    let app = aspect_identity_app();
    let all_graph_node_digests = all_graph_node_digests(&app);
    let all_mounted_receipt_digests = all_mounted_receipt_digests(&app);
    let observation_before = app.inspection_observation();
    let first_published = app.inspect(published_aspect_query(" Content.Text "));
    let second_published = app.inspect(published_aspect_query(SHARED_ASPECT));
    let first_consumed = app.inspect(consumed_aspect_query(SHARED_ASPECT));
    let second_consumed = app.inspect(consumed_aspect_query(SHARED_ASPECT));
    let competing_published = app.inspect(published_aspect_query(COMPETING_PUBLISHED_ASPECT));
    let observation_after = app.inspection_observation();
    let first_published_facts = aspect_neighborhood_facts_from_receipt(
        SHARED_ASPECT,
        &first_published,
        &all_graph_node_digests,
        &all_mounted_receipt_digests,
    );
    let second_published_facts = aspect_neighborhood_facts_from_receipt(
        SHARED_ASPECT,
        &second_published,
        &all_graph_node_digests,
        &all_mounted_receipt_digests,
    );
    let first_consumed_facts = aspect_neighborhood_facts_from_receipt(
        SHARED_ASPECT,
        &first_consumed,
        &all_graph_node_digests,
        &all_mounted_receipt_digests,
    );
    let second_consumed_facts = aspect_neighborhood_facts_from_receipt(
        SHARED_ASPECT,
        &second_consumed,
        &all_graph_node_digests,
        &all_mounted_receipt_digests,
    );

    assert_eq!(
        observation_after.graph_aspect_evidence_index_rebuild_count(),
        observation_before.graph_aspect_evidence_index_rebuild_count(),
    );
    assert_eq!(
        observation_after.graph_node_evidence_index_rebuild_count(),
        observation_before.graph_node_evidence_index_rebuild_count(),
    );
    assert_eq!(
        first_published.relevance_outcome(),
        UiInspectionRelevanceOutcome::Matched
    );
    assert_eq!(
        first_consumed.relevance_outcome(),
        UiInspectionRelevanceOutcome::Matched
    );
    assert_same_slice(&first_published, &second_published);
    assert_same_slice(&first_consumed, &second_consumed);
    assert_eq!(first_published_facts, second_published_facts);
    assert_eq!(first_consumed_facts, second_consumed_facts);
    assert_membership(
        &first_published_facts,
        UiAspectEvidenceLane::Published,
        &expected_graph_node_digests(&app, &[ALPHA_MODULE_PATH, GAMMA_MODULE_PATH]),
        &expected_mounted_receipt_digests(&app, &[ALPHA_MODULE_PATH, GAMMA_MODULE_PATH]),
    );
    assert_membership(
        &first_consumed_facts,
        UiAspectEvidenceLane::Consumed,
        &expected_graph_node_digests(&app, &[ALPHA_MODULE_PATH, GAMMA_MODULE_PATH]),
        &expected_mounted_receipt_digests(&app, &[ALPHA_MODULE_PATH, GAMMA_MODULE_PATH]),
    );
    assert_lane_identity_distinctness(&first_published_facts, &first_consumed_facts);
    assert_eq!(aspect_ref_count(&competing_published), 2);
}

#[test]
fn consumed_aspect_query_can_include_direct_provenance_refs() {
    let app = aspect_identity_app();
    let local_receipt = app.inspect(consumed_aspect_query(SHARED_ASPECT));
    let provenance_receipt = app.inspect(consumed_aspect_with_provenance_query(SHARED_ASPECT));
    let wrong_family_receipt = app.inspect(
        UiInspectionQuery::new(
            UiInspectionTarget::consumed_aspect(SHARED_ASPECT),
            UiInspectionScope::graph(),
        )
        .with_relevance(UiInspectionRelevance::local(UiRelevanceFilter::family(
            UiRelevanceFamily::Declaration,
        )))
        .with_richness(UiEvidenceRichness::refs_only()),
    );

    assert_eq!(
        local_receipt.relevance_outcome(),
        UiInspectionRelevanceOutcome::Matched
    );
    assert_eq!(
        provenance_receipt.relevance_outcome(),
        UiInspectionRelevanceOutcome::Matched
    );
    assert_eq!(aspect_ref_count(&local_receipt), 4);
    assert_eq!(
        declaration_ref_digests(&app, &provenance_receipt),
        declaration_artifact_digests(&app, &[ALPHA_MODULE_PATH, GAMMA_MODULE_PATH]),
    );
    assert_eq!(
        wrong_family_receipt.relevance_outcome(),
        UiInspectionRelevanceOutcome::NotApplicableToTarget {
            target: UiInspectionTargetClass::ConsumedAspect,
        }
    );
    assert!(wrong_family_receipt.evidence_slice().is_none());
}

#[test]
fn aspect_targets_reject_unindexed_widening_and_missing_targets() {
    let app = aspect_identity_app();
    let widened_receipt = app.inspect(
        UiInspectionQuery::new(
            UiInspectionTarget::published_aspect(SHARED_ASPECT),
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
        widened_receipt.relevance_outcome(),
        UiInspectionRelevanceOutcome::NotApplicableToTarget {
            target: UiInspectionTargetClass::PublishedAspect,
        }
    );
    assert!(widened_receipt.evidence_slice().is_none());
    assert_eq!(
        missing_receipt.relevance_outcome(),
        UiInspectionRelevanceOutcome::UnsupportedScope {
            scope: UiInspectionScope::Graph
        }
    );
    assert!(missing_receipt.evidence_slice().is_none());
    assert!(missing_receipt.support_report().is_some());
}

#[test]
fn rebuilding_graph_node_index_preserves_aspect_lookup_answers_and_records_aspect_rebuilds() {
    let mut app = aspect_identity_app();
    let all_graph_node_digests = all_graph_node_digests(&app);
    let all_mounted_receipt_digests = all_mounted_receipt_digests(&app);
    let published_before = app.inspect(published_aspect_query(SHARED_ASPECT));
    let consumed_before = app.inspect(consumed_aspect_with_provenance_query(SHARED_ASPECT));
    let observation_before = app.inspection_observation();

    app.rebuild_graph_evidence_indexes_from_authority();

    let observation_after = app.inspection_observation();
    let published_after = app.inspect(published_aspect_query(SHARED_ASPECT));
    let consumed_after = app.inspect(consumed_aspect_with_provenance_query(SHARED_ASPECT));

    assert_eq!(
        observation_after.graph_node_evidence_index_rebuild_count(),
        observation_before.graph_node_evidence_index_rebuild_count() + 1,
    );
    assert_eq!(
        observation_after.graph_aspect_evidence_index_rebuild_count(),
        observation_before.graph_aspect_evidence_index_rebuild_count() + 1,
    );
    assert_same_slice(&published_before, &published_after);
    assert_same_slice(&consumed_before, &consumed_after);
    assert_eq!(
        aspect_neighborhood_facts_from_receipt(
            SHARED_ASPECT,
            &published_before,
            &all_graph_node_digests,
            &all_mounted_receipt_digests,
        ),
        aspect_neighborhood_facts_from_receipt(
            SHARED_ASPECT,
            &published_after,
            &all_graph_node_digests,
            &all_mounted_receipt_digests,
        ),
    );
    assert_eq!(
        declaration_ref_digests(&app, &consumed_before),
        declaration_ref_digests(&app, &consumed_after),
    );
    assert_eq!(
        aspect_neighborhood_facts_from_receipt(
            SHARED_ASPECT,
            &consumed_before,
            &all_graph_node_digests,
            &all_mounted_receipt_digests,
        ),
        aspect_neighborhood_facts_from_receipt(
            SHARED_ASPECT,
            &consumed_after,
            &all_graph_node_digests,
            &all_mounted_receipt_digests,
        ),
    );
}

fn assert_same_slice(left: &UiInspectionReceipt, right: &UiInspectionReceipt) {
    assert_eq!(left.evidence_slice_ref(), right.evidence_slice_ref());
}
