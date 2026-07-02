use worth_ui::facade::inspection::{
    UiInspectionEvidenceSource, UiInspectionObligationDecision,
    UiInspectionObligationEvidenceQuery, UiInspectionObligationFamily,
    UiInspectionObligationNonSelectionReason, UiInspectionObligationSelectionReason,
    UiInspectionQuery, UiInspectionScope, UiInspectionTarget,
};

#[path = "fixtures/obligation_dispatch_prerequisite_support/mod.rs"]
mod obligation_dispatch_prerequisite_support;

#[test]
fn obligation_inspection_answers_selected_and_not_selected_from_retained_evidence() {
    let app = obligation_dispatch_prerequisite_support::query_touch_app();
    let touch = obligation_dispatch_prerequisite_support::query_touch(&app);
    let target = obligation_dispatch_prerequisite_support::graph_aligned_query_target(&touch);
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
        .with_obligation_evidence(
            UiInspectionObligationEvidenceQuery::new()
                .with_family(UiInspectionObligationFamily::QueryBindingRequirement),
        ),
    );
    let selected_query = selected_projection
        .obligation_evidence()
        .expect("obligation evidence receipt should be present")
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
        .with_obligation_evidence(
            UiInspectionObligationEvidenceQuery::new()
                .with_family(UiInspectionObligationFamily::HostCapabilityRequirement),
        ),
    );
    let not_selected = not_selected_projection
        .obligation_evidence()
        .expect("obligation evidence receipt should be present")
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
    let app = obligation_dispatch_prerequisite_support::query_touch_app();
    let touch = obligation_dispatch_prerequisite_support::query_touch(&app);
    let target = obligation_dispatch_prerequisite_support::graph_aligned_query_target(&touch);
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
        .with_obligation_evidence(
            UiInspectionObligationEvidenceQuery::new()
                .with_family(UiInspectionObligationFamily::QueryBindingRequirement)
                .with_prerequisite_source(UiInspectionEvidenceSource::QueryInspection),
        ),
    );
    let filtered = filtered
        .obligation_evidence()
        .expect("obligation evidence receipt should be present");

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
fn denial_posture_filter_excludes_non_matching_admission_rows() {
    use worth_ui::facade::inspection::{
        UiInspectionAdmissionQueryBasis, UiInspectionAdmissionStaleEvidence,
        UiInspectionObligationDenialPosture,
    };

    let app = obligation_dispatch_prerequisite_support::query_touch_app();
    let touch = obligation_dispatch_prerequisite_support::query_touch(&app);

    let wrong_basis_report = app
        .admission()
        .report(obligation_dispatch_prerequisite_support::wrong_query_basis_target(&touch));
    let stale_report = app
        .admission()
        .report(obligation_dispatch_prerequisite_support::stale_query_basis_target(&touch));

    let wrong_basis_receipt = wrong_basis_report.inspect(
        UiInspectionQuery::new(
            UiInspectionTarget::obligation_graph_node(
                touch.target().graph_node_identity().digest(),
            ),
            UiInspectionScope::graph(),
        )
        .with_obligation_evidence(
            UiInspectionObligationEvidenceQuery::new().with_denial_posture(
                UiInspectionObligationDenialPosture::WrongQueryBasis {
                    required: UiInspectionAdmissionQueryBasis::GraphAligned,
                    observed: UiInspectionAdmissionQueryBasis::WrongWorldProjection,
                },
            ),
        ),
    );
    let wrong_basis_rows = wrong_basis_receipt
        .obligation_evidence()
        .expect("obligation evidence receipt should be present")
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
        .with_obligation_evidence(
            UiInspectionObligationEvidenceQuery::new().with_denial_posture(
                UiInspectionObligationDenialPosture::Stale {
                    required: UiInspectionAdmissionQueryBasis::GraphAligned,
                    observed: UiInspectionAdmissionQueryBasis::StaleReceipt,
                    evidence: UiInspectionAdmissionStaleEvidence::QueryReceiptExpired,
                },
            ),
        ),
    );
    assert!(wrong_basis_mismatch
        .obligation_evidence()
        .expect("obligation evidence receipt should be present")
        .projections()
        .is_empty());

    let stale_receipt = stale_report.inspect(
        UiInspectionQuery::new(
            UiInspectionTarget::obligation_graph_node(
                touch.target().graph_node_identity().digest(),
            ),
            UiInspectionScope::graph(),
        )
        .with_obligation_evidence(
            UiInspectionObligationEvidenceQuery::new().with_denial_posture(
                UiInspectionObligationDenialPosture::Stale {
                    required: UiInspectionAdmissionQueryBasis::GraphAligned,
                    observed: UiInspectionAdmissionQueryBasis::StaleReceipt,
                    evidence: UiInspectionAdmissionStaleEvidence::QueryReceiptExpired,
                },
            ),
        ),
    );
    let stale_rows = stale_receipt
        .obligation_evidence()
        .expect("obligation evidence receipt should be present")
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
