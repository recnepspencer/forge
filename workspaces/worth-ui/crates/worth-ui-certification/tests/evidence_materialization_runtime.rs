use worth_ui::facade::inspection::{
    UiEvidenceMaterializedDetail, UiEvidenceRichness, UiInspectionObligationFamily,
    UiInspectionObligationRelevanceDetail, UiInspectionQuery, UiInspectionRelevance,
    UiInspectionScope, UiInspectionTarget, UiRelevanceFamily, UiRelevanceFilter,
};

use worth_ui_certification::scenario::obligation_dispatch_prerequisite as obligation_dispatch_prerequisite_support;

#[test]
fn refs_first_expansion_matches_direct_rich_request_for_the_same_obligation_handle() {
    let app = obligation_dispatch_prerequisite_support::application_authority::query_touch_app();
    let touch = obligation_dispatch_prerequisite_support::graph_touches::query_touch(&app);
    let target =
        obligation_dispatch_prerequisite_support::admission_targets::graph_aligned_query_target(
            &touch,
        );
    let selected = app
        .admission()
        .select_obligations_for_target(&touch, target);
    let refs_only_receipt = selected.inspect(
        obligation_touch_query(
            touch.target().graph_node_identity().digest(),
            touch.identity_digest(),
        )
        .with_richness(UiEvidenceRichness::refs_only()),
    );
    let evidence_ref = refs_only_receipt
        .evidence_slice()
        .expect("refs-only obligation inspection should retain a slice")
        .refs()[0];

    let expansion = selected.expand_evidence_ref(evidence_ref, UiEvidenceRichness::summary());
    let expanded_detail = obligation_detail(
        expansion
            .materialized_detail()
            .expect("selected expansion should materialize one obligation receipt"),
    );
    let direct_receipt = selected.inspect(
        obligation_handle_query(evidence_ref.handle().handle_digest())
            .with_richness(UiEvidenceRichness::materialized_detail()),
    );
    let direct_detail = obligation_detail(
        direct_receipt
            .evidence_slice()
            .expect("direct rich handle inspection should retain a slice")
            .materialized_detail()
            .expect("direct rich handle inspection should materialize obligation detail"),
    );

    assert!(expansion.outcome().is_available());
    assert_eq!(expanded_detail.refs(), direct_detail.refs());
    assert_eq!(expanded_detail.projections(), direct_detail.projections());
}

#[test]
fn refs_only_obligation_inspection_does_not_materialize_before_explicit_expansion() {
    let app = obligation_dispatch_prerequisite_support::application_authority::query_touch_app();
    let touch = obligation_dispatch_prerequisite_support::graph_touches::query_touch(&app);
    let target =
        obligation_dispatch_prerequisite_support::admission_targets::graph_aligned_query_target(
            &touch,
        );
    let selected = app
        .admission()
        .select_obligations_for_target(&touch, target);
    let observation_before = selected.inspection_observation();
    let receipt = selected.inspect(
        obligation_touch_query(
            touch.target().graph_node_identity().digest(),
            touch.identity_digest(),
        )
        .with_richness(UiEvidenceRichness::refs_only()),
    );
    let observation_after_refs_only = selected.inspection_observation();
    let evidence_ref = receipt
        .evidence_slice()
        .expect("refs-only obligation inspection should retain a slice")
        .refs()[0];

    assert_eq!(
        observation_after_refs_only.rich_artifact_materialization_count()
            - observation_before.rich_artifact_materialization_count(),
        0
    );
    assert!(receipt
        .evidence_slice()
        .expect("refs-only obligation inspection should retain a slice")
        .materialized_detail()
        .is_none());

    let _ = selected.expand_evidence_ref(evidence_ref, UiEvidenceRichness::summary());
    let observation_after_expand = selected.inspection_observation();

    assert_eq!(
        observation_after_expand.rich_artifact_materialization_count()
            - observation_after_refs_only.rich_artifact_materialization_count(),
        1
    );
}

fn obligation_detail(
    detail: &UiEvidenceMaterializedDetail,
) -> &worth_ui::facade::inspection::UiInspectionObligationEvidenceReceipt {
    match detail {
        UiEvidenceMaterializedDetail::Obligation(receipt) => receipt,
        _ => panic!("expected obligation materialized detail"),
    }
}

fn obligation_touch_query(graph_node_digest: u64, touch_identity_digest: u64) -> UiInspectionQuery {
    UiInspectionQuery::new(
        UiInspectionTarget::obligation_touch(graph_node_digest, touch_identity_digest),
        UiInspectionScope::graph(),
    )
    .with_relevance(obligation_relevance())
}

fn obligation_handle_query(handle_digest: u64) -> UiInspectionQuery {
    UiInspectionQuery::new(
        UiInspectionTarget::obligation_evidence_handle(handle_digest),
        UiInspectionScope::graph(),
    )
    .with_relevance(obligation_relevance())
}

fn obligation_relevance() -> UiInspectionRelevance {
    UiInspectionRelevance::local(UiRelevanceFilter::family(UiRelevanceFamily::Obligation))
        .with_obligation_detail(
            UiInspectionObligationRelevanceDetail::new()
                .with_family(UiInspectionObligationFamily::QueryBindingRequirement),
        )
}
