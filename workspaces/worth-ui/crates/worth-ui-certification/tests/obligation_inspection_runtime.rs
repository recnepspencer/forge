use worth_ui::facade::inspection::{
    UiEvidenceMaterializedDetail, UiEvidenceRichness, UiInspectionEvidenceSource,
    UiInspectionObligationDecision, UiInspectionObligationFamily,
    UiInspectionObligationNonSelectionReason, UiInspectionObligationRelevanceDetail,
    UiInspectionObligationSelectionReason, UiInspectionObligationVerdictClass, UiInspectionQuery,
    UiInspectionRelevance, UiInspectionScope, UiInspectionTarget, UiRelevanceFamily,
    UiRelevanceFilter,
};

#[path = "fixtures/obligation_dispatch_prerequisite_support/mod.rs"]
pub mod obligation_dispatch_prerequisite_support;

#[test]
fn obligation_inspection_answers_selected_and_not_selected_from_retained_evidence() {
    let app = obligation_dispatch_prerequisite_support::apps::query_touch_app();
    let touch = obligation_dispatch_prerequisite_support::touches::query_touch(&app);
    let target =
        obligation_dispatch_prerequisite_support::targets::graph_aligned_query_target(&touch);
    let selected = app
        .admission()
        .select_obligations_for_target(&touch, target);

    let selected_projection = selected.inspect(
        UiInspectionQuery::new(
            UiInspectionTarget::obligation_touch(
                touch.target().graph_node_identity().digest(),
                touch.identity_digest(),
            ),
            UiInspectionScope::graph(),
        )
        .with_relevance(obligation_relevance(
            UiInspectionObligationRelevanceDetail::new()
                .with_family(UiInspectionObligationFamily::QueryBindingRequirement),
        ))
        .with_richness(UiEvidenceRichness::materialized_detail()),
    );
    let selected_query = selected_projection
        .evidence_slice()
        .and_then(|slice| slice.materialized_detail())
        .and_then(obligation_detail)
        .expect("obligation evidence receipt should be materialized through the evidence slice")
        .projections()
        .iter()
        .find(|projection| projection.decision() == UiInspectionObligationDecision::Selected)
        .expect("query binding family should remain inspectable as selected");
    assert_eq!(
        selected_query.family(),
        Some(UiInspectionObligationFamily::QueryBindingRequirement)
    );
    assert!(selected_query
        .selection_reasons()
        .contains(&UiInspectionObligationSelectionReason::GraphQueryBindingAttachment));

    let not_selected_projection = selected.inspect(
        UiInspectionQuery::new(
            UiInspectionTarget::obligation_touch(
                touch.target().graph_node_identity().digest(),
                touch.identity_digest(),
            ),
            UiInspectionScope::graph(),
        )
        .with_relevance(obligation_relevance(
            UiInspectionObligationRelevanceDetail::new()
                .with_family(UiInspectionObligationFamily::HostCapabilityRequirement),
        ))
        .with_richness(UiEvidenceRichness::materialized_detail()),
    );
    let not_selected = not_selected_projection
        .evidence_slice()
        .and_then(|slice| slice.materialized_detail())
        .and_then(obligation_detail)
        .expect("obligation evidence receipt should be materialized through the evidence slice")
        .projections()
        .iter()
        .find(|projection| projection.decision() == UiInspectionObligationDecision::NotSelected)
        .expect("non-selected motion family should remain inspectable");
    assert_eq!(
        not_selected.family(),
        Some(UiInspectionObligationFamily::HostCapabilityRequirement)
    );
    assert_eq!(
        not_selected.non_selection_reason(),
        Some(UiInspectionObligationNonSelectionReason::RuleDidNotMatch)
    );
}

#[test]
fn evidence_index_filters_by_graph_touch_family_and_prerequisite_source() {
    let app = obligation_dispatch_prerequisite_support::apps::query_touch_app();
    let touch = obligation_dispatch_prerequisite_support::touches::query_touch(&app);
    let target =
        obligation_dispatch_prerequisite_support::targets::graph_aligned_query_target(&touch);
    let report = app.admission().admit_selected_obligations(
        &app.admission()
            .select_obligations_for_target(&touch, target),
    );

    let filtered = report.inspect(
        UiInspectionQuery::new(
            UiInspectionTarget::obligation_touch(
                touch.target().graph_node_identity().digest(),
                touch.identity_digest(),
            ),
            UiInspectionScope::graph(),
        )
        .with_relevance(obligation_relevance(
            UiInspectionObligationRelevanceDetail::new()
                .with_family(UiInspectionObligationFamily::QueryBindingRequirement)
                .with_prerequisite_source(UiInspectionEvidenceSource::QueryInspection),
        ))
        .with_richness(UiEvidenceRichness::materialized_detail()),
    );
    let filtered = filtered
        .evidence_slice()
        .and_then(|slice| slice.materialized_detail())
        .and_then(obligation_detail)
        .expect("obligation evidence receipt should be materialized through the evidence slice");

    assert!(filtered
        .projections()
        .iter()
        .all(|projection| projection.graph_node_digest()
            == touch.target().graph_node_identity().digest()));
    assert!(filtered
        .projections()
        .iter()
        .all(|projection| projection.touch_identity_digest() == Some(touch.identity_digest())));
    assert!(filtered
        .projections()
        .iter()
        .all(|projection| projection.family()
            == Some(UiInspectionObligationFamily::QueryBindingRequirement)));
    assert!(filtered.projections().iter().all(|projection| projection
        .prerequisite_sources()
        .contains(&UiInspectionEvidenceSource::QueryInspection)));
}

#[test]
fn graph_node_and_touch_routes_converge_on_the_same_retained_obligation_neighborhood() {
    let app = obligation_dispatch_prerequisite_support::apps::query_touch_app();
    let touch = obligation_dispatch_prerequisite_support::touches::query_touch(&app);
    let target =
        obligation_dispatch_prerequisite_support::targets::graph_aligned_query_target(&touch);
    let report = app.admission().admit_selected_obligations(
        &app.admission()
            .select_obligations_for_target(&touch, target),
    );

    let touch_rows = report
        .inspect(
            UiInspectionQuery::new(
                UiInspectionTarget::obligation_touch(
                    touch.target().graph_node_identity().digest(),
                    touch.identity_digest(),
                ),
                UiInspectionScope::graph(),
            )
            .with_relevance(obligation_relevance(
                UiInspectionObligationRelevanceDetail::new(),
            ))
            .with_richness(UiEvidenceRichness::materialized_detail()),
        )
        .evidence_slice()
        .and_then(|slice| slice.materialized_detail())
        .and_then(obligation_detail)
        .expect("touch obligation inspection should materialize retained detail")
        .projections()
        .iter()
        .map(obligation_shape)
        .collect::<Vec<_>>();
    let node_rows = report
        .inspect(
            UiInspectionQuery::new(
                UiInspectionTarget::obligation_graph_node(
                    touch.target().graph_node_identity().digest(),
                ),
                UiInspectionScope::graph(),
            )
            .with_relevance(obligation_relevance(
                UiInspectionObligationRelevanceDetail::new(),
            ))
            .with_richness(UiEvidenceRichness::materialized_detail()),
        )
        .evidence_slice()
        .and_then(|slice| slice.materialized_detail())
        .and_then(obligation_detail)
        .expect("graph-node obligation inspection should materialize retained detail")
        .projections()
        .iter()
        .map(obligation_shape)
        .collect::<Vec<_>>();

    assert_eq!(touch_rows, node_rows);
    let dispatch_row = touch_rows
        .iter()
        .find(|row| row.0 == UiInspectionObligationDecision::Dispatch)
        .expect("retained obligation neighborhood should include dispatch evidence");
    assert!(dispatch_row.3.is_some());
    assert_eq!(dispatch_row.4, None);
    assert_eq!(dispatch_row.5, None);
    assert_eq!(dispatch_row.6, None);

    let verdict_row = touch_rows
        .iter()
        .find(|row| row.0 == UiInspectionObligationDecision::Verdict)
        .expect("retained obligation neighborhood should include verdict evidence");
    assert_eq!(verdict_row.3, None);
    assert!(verdict_row.4.is_some());
    assert!(verdict_row.5.is_some());

    let admission_row = touch_rows
        .iter()
        .find(|row| row.0 == UiInspectionObligationDecision::Admission)
        .expect("retained obligation neighborhood should include admission evidence");
    assert_eq!(admission_row.2, Some(touch.identity_digest()));
    assert_eq!(admission_row.3, None);
    assert_eq!(admission_row.4, None);
    assert_eq!(admission_row.5, None);
}

#[test]
fn denial_posture_filter_excludes_non_matching_admission_rows() {
    use worth_ui::facade::inspection::{
        UiInspectionAdmissionQueryBasis, UiInspectionAdmissionStaleEvidence,
        UiInspectionObligationDenialPosture,
    };

    let app = obligation_dispatch_prerequisite_support::apps::query_touch_app();
    let touch = obligation_dispatch_prerequisite_support::touches::query_touch(&app);

    let wrong_basis_report = app.admission().report(
        obligation_dispatch_prerequisite_support::targets::wrong_query_basis_target(&touch),
    );
    let stale_report = app.admission().report(
        obligation_dispatch_prerequisite_support::targets::stale_query_basis_target(&touch),
    );

    let wrong_basis_receipt = wrong_basis_report.inspect(
        UiInspectionQuery::new(
            UiInspectionTarget::obligation_graph_node(
                touch.target().graph_node_identity().digest(),
            ),
            UiInspectionScope::graph(),
        )
        .with_relevance(obligation_relevance(
            UiInspectionObligationRelevanceDetail::new().with_denial_posture(
                UiInspectionObligationDenialPosture::WrongQueryBasis {
                    required: UiInspectionAdmissionQueryBasis::GraphAligned,
                    observed: UiInspectionAdmissionQueryBasis::WrongWorldProjection,
                },
            ),
        ))
        .with_richness(UiEvidenceRichness::materialized_detail()),
    );
    let wrong_basis_rows = wrong_basis_receipt
        .evidence_slice()
        .and_then(|slice| slice.materialized_detail())
        .and_then(obligation_detail)
        .expect("obligation evidence receipt should be materialized through the evidence slice")
        .projections();
    assert_eq!(wrong_basis_rows.len(), 1);
    assert!(wrong_basis_rows.iter().all(|projection| {
        projection.denial_posture()
            == Some(UiInspectionObligationDenialPosture::WrongQueryBasis {
                required: UiInspectionAdmissionQueryBasis::GraphAligned,
                observed: UiInspectionAdmissionQueryBasis::WrongWorldProjection,
            })
    }));

    let wrong_basis_mismatch = wrong_basis_report.inspect(
        UiInspectionQuery::new(
            UiInspectionTarget::obligation_graph_node(
                touch.target().graph_node_identity().digest(),
            ),
            UiInspectionScope::graph(),
        )
        .with_relevance(obligation_relevance(
            UiInspectionObligationRelevanceDetail::new().with_denial_posture(
                UiInspectionObligationDenialPosture::Stale {
                    required: UiInspectionAdmissionQueryBasis::GraphAligned,
                    observed: UiInspectionAdmissionQueryBasis::StaleReceipt,
                    evidence: UiInspectionAdmissionStaleEvidence::QueryReceiptExpired,
                },
            ),
        ))
        .with_richness(UiEvidenceRichness::materialized_detail()),
    );
    assert!(wrong_basis_mismatch
        .evidence_slice()
        .and_then(|slice| slice.materialized_detail())
        .and_then(obligation_detail)
        .expect("obligation evidence receipt should be materialized through the evidence slice")
        .projections()
        .is_empty());

    let stale_receipt = stale_report.inspect(
        UiInspectionQuery::new(
            UiInspectionTarget::obligation_graph_node(
                touch.target().graph_node_identity().digest(),
            ),
            UiInspectionScope::graph(),
        )
        .with_relevance(obligation_relevance(
            UiInspectionObligationRelevanceDetail::new().with_denial_posture(
                UiInspectionObligationDenialPosture::Stale {
                    required: UiInspectionAdmissionQueryBasis::GraphAligned,
                    observed: UiInspectionAdmissionQueryBasis::StaleReceipt,
                    evidence: UiInspectionAdmissionStaleEvidence::QueryReceiptExpired,
                },
            ),
        ))
        .with_richness(UiEvidenceRichness::materialized_detail()),
    );
    let stale_rows = stale_receipt
        .evidence_slice()
        .and_then(|slice| slice.materialized_detail())
        .and_then(obligation_detail)
        .expect("obligation evidence receipt should be materialized through the evidence slice")
        .projections();
    assert_eq!(stale_rows.len(), 1);
    assert!(stale_rows.iter().all(|projection| {
        projection.denial_posture()
            == Some(UiInspectionObligationDenialPosture::Stale {
                required: UiInspectionAdmissionQueryBasis::GraphAligned,
                observed: UiInspectionAdmissionQueryBasis::StaleReceipt,
                evidence: UiInspectionAdmissionStaleEvidence::QueryReceiptExpired,
            })
    }));
}

fn obligation_detail(
    detail: &UiEvidenceMaterializedDetail,
) -> Option<&worth_ui::facade::inspection::UiInspectionObligationEvidenceReceipt> {
    match detail {
        UiEvidenceMaterializedDetail::Obligation(receipt) => Some(receipt),
        _ => None,
    }
}

fn obligation_relevance(detail: UiInspectionObligationRelevanceDetail) -> UiInspectionRelevance {
    UiInspectionRelevance::local(UiRelevanceFilter::family(UiRelevanceFamily::Obligation))
        .with_obligation_detail(detail)
}

fn obligation_shape(
    projection: &worth_ui::facade::inspection::UiInspectionObligationReasonProjection,
) -> (
    UiInspectionObligationDecision,
    Option<UiInspectionObligationFamily>,
    Option<u64>,
    Option<worth_ui::facade::inspection::UiInspectionObligationDispatchPosture>,
    Option<UiInspectionObligationVerdictClass>,
    Option<worth_ui::facade::inspection::UiInspectionObligationVerdictPosture>,
    Option<worth_ui::facade::inspection::UiInspectionObligationDenialPosture>,
) {
    (
        projection.decision(),
        projection.family(),
        projection.touch_identity_digest(),
        projection.dispatch_posture(),
        projection.verdict_class(),
        projection.verdict_posture(),
        projection.denial_posture(),
    )
}
