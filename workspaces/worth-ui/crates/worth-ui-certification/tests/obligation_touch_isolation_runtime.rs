use worth_ui::facade::admission::WorthUiAdmissionExt;
use worth_ui::facade::inspection::{
    UiEvidenceRichness, UiInspectionObligationDecision, UiInspectionObligationRelevanceDetail,
    UiInspectionQuery, UiInspectionRelevance, UiInspectionScope, UiInspectionTarget,
    UiRelevanceFamily, UiRelevanceFilter,
};

use worth_ui_certification::scenario::obligation_dispatch_prerequisite as obligation_dispatch_prerequisite_support;

#[test]
fn touch_queries_do_not_leak_same_node_admission_rows_across_distinct_touches() {
    let app = obligation_dispatch_prerequisite_support::application_authority::query_touch_app();
    let query_touch = obligation_dispatch_prerequisite_support::graph_touches::query_touch(&app);
    let structural_touch =
        obligation_dispatch_prerequisite_support::graph_touches::structural_touch(&app);
    let target =
        obligation_dispatch_prerequisite_support::admission_targets::graph_aligned_query_target(
            &query_touch,
        );
    let report = app.admission().admit_selected_obligations(
        &app.admission()
            .select_obligations_for_target(&query_touch, target),
    );

    let matching_rows = projected_decisions(
        &report.inspect(
            UiInspectionQuery::new(
                UiInspectionTarget::obligation_touch(
                    query_touch.target().graph_node_identity().digest(),
                    query_touch.identity_digest(),
                ),
                UiInspectionScope::graph(),
            )
            .with_relevance(obligation_relevance())
            .with_richness(UiEvidenceRichness::materialized_detail()),
        ),
    );
    assert!(matching_rows.contains(&UiInspectionObligationDecision::Admission));

    let mismatched_rows = projected_decisions(
        &report.inspect(
            UiInspectionQuery::new(
                UiInspectionTarget::obligation_touch(
                    structural_touch.target().graph_node_identity().digest(),
                    structural_touch.identity_digest(),
                ),
                UiInspectionScope::graph(),
            )
            .with_relevance(obligation_relevance())
            .with_richness(UiEvidenceRichness::materialized_detail()),
        ),
    );
    assert!(
        mismatched_rows.is_empty(),
        "same-node touch queries must not absorb admission rows from a different admitted work"
    );
}

fn obligation_relevance() -> UiInspectionRelevance {
    UiInspectionRelevance::local(UiRelevanceFilter::family(UiRelevanceFamily::Obligation))
        .with_obligation_detail(UiInspectionObligationRelevanceDetail::new())
}

fn projected_decisions(
    receipt: &worth_ui::facade::inspection::UiInspectionReceipt,
) -> Vec<UiInspectionObligationDecision> {
    receipt
        .evidence_slice()
        .and_then(|slice| slice.materialized_detail())
        .and_then(|detail| match detail {
            worth_ui::facade::inspection::UiEvidenceMaterializedDetail::Obligation(receipt) => {
                Some(receipt.projections())
            }
            _ => None,
        })
        .map(|projections| {
            projections
                .iter()
                .map(|projection| projection.decision())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}
