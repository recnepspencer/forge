use worth_ui::facade::inspection::{
    UiEvidenceMaterializedDetail, UiEvidenceRichness, UiInspectionObligationRelevanceDetail,
    UiInspectionQuery, UiInspectionRelevance, UiInspectionScope, UiInspectionTarget,
    UiRelevanceFamily, UiRelevanceFilter,
};
use worth_ui::facade::obligations::UiObligationEvidenceDecision;
use worth_ui_runtime::facade::obligations::UiSelectedObligationEvidenceProjection;

use worth_ui_certification::scenario::obligation_dispatch_prerequisite as obligation_dispatch_prerequisite_support;

#[test]
fn public_selected_rows_exclude_plausible_but_not_selected_candidates() {
    let app = obligation_dispatch_prerequisite_support::application_authority::query_touch_app();
    let touch = obligation_dispatch_prerequisite_support::graph_touches::query_touch(&app);
    let target =
        obligation_dispatch_prerequisite_support::admission_targets::graph_aligned_query_target(
            &touch,
        );
    let selected = app
        .admission()
        .select_obligations_for_target(&touch, target.clone());
    let report = app.admission().admit_selected_obligations(&selected);

    assert!(
        selected
            .evidence_index()
            .records()
            .iter()
            .any(|record| record.decision() == UiObligationEvidenceDecision::NotSelected),
        "fixture must contain non-selected candidates so selected-row exactness is adversarial"
    );

    let expected = selected_projection_set_from_selected_set(&selected);
    let selected_public = selected_projection_set_from_receipt(
        &selected.inspect(
            UiInspectionQuery::new(
                UiInspectionTarget::obligation_touch(
                    touch.target().graph_node_identity().digest(),
                    touch.identity_digest(),
                ),
                UiInspectionScope::graph(),
            )
            .with_relevance(obligation_relevance())
            .with_richness(UiEvidenceRichness::materialized_detail()),
        ),
    );
    let report_public = selected_projection_set_from_receipt(
        &report.inspect(
            UiInspectionQuery::new(
                UiInspectionTarget::obligation_touch(
                    touch.target().graph_node_identity().digest(),
                    touch.identity_digest(),
                ),
                UiInspectionScope::graph(),
            )
            .with_relevance(obligation_relevance())
            .with_richness(UiEvidenceRichness::materialized_detail()),
        ),
    );

    assert_eq!(expected, selected_public);
    assert_eq!(expected, report_public);
}

fn obligation_relevance() -> UiInspectionRelevance {
    UiInspectionRelevance::local(UiRelevanceFilter::family(UiRelevanceFamily::Obligation))
        .with_obligation_detail(UiInspectionObligationRelevanceDetail::new())
}

fn selected_projection_set_from_selected_set(
    selected: &worth_ui_runtime::facade::obligations::UiSelectedObligationSet,
) -> Vec<UiSelectedObligationEvidenceProjection> {
    let mut projections = selected
        .obligations()
        .iter()
        .map(|obligation| {
            UiSelectedObligationEvidenceProjection::from_selected_obligation_set_entry(
                selected, obligation,
            )
        })
        .collect::<Vec<_>>();
    projections.sort_by_key(UiSelectedObligationEvidenceProjection::handle_digest);
    projections
}

fn selected_projection_set_from_receipt(
    receipt: &worth_ui::facade::inspection::UiInspectionReceipt,
) -> Vec<UiSelectedObligationEvidenceProjection> {
    let mut projections = receipt
        .evidence_slice()
        .and_then(|slice| slice.materialized_detail())
        .and_then(|detail| match detail {
            UiEvidenceMaterializedDetail::Obligation(receipt) => Some(receipt.projections()),
            _ => None,
        })
        .map(|projections| {
            projections
                .iter()
                .filter_map(UiSelectedObligationEvidenceProjection::from_selected_projection)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    projections.sort_by_key(UiSelectedObligationEvidenceProjection::handle_digest);
    projections
}
