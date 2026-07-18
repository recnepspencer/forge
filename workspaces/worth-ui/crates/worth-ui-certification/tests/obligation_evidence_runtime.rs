use worth_ui::facade::inspection::{
    UiEvidenceAuthorityKind, UiEvidenceFamily, UiEvidenceMaterializedDetail, UiEvidenceRichness,
    UiInspectionObligationDenialPosture, UiInspectionObligationRelevanceDetail, UiInspectionQuery,
    UiInspectionRelevance, UiInspectionScope, UiInspectionTarget, UiRelevanceFamily,
    UiRelevanceFilter,
};
use worth_ui::facade::obligations::UiObligationEvidenceDecision;
use worth_ui_runtime::facade::obligations::UiSelectedObligationEvidenceProjection;

use worth_ui_certification::scenario::obligation_dispatch_prerequisite as obligation_dispatch_prerequisite_support;

#[test]
fn selected_verdict_and_admission_paths_retain_typed_evidence_handles() {
    let app = obligation_dispatch_prerequisite_support::application_authority::query_touch_app();
    let touch = obligation_dispatch_prerequisite_support::graph_touches::query_touch(&app);
    let target =
        obligation_dispatch_prerequisite_support::admission_targets::graph_aligned_query_target(
            &touch,
        );
    let selected = app
        .admission()
        .select_obligations_for_target(&touch, target.clone());

    let selected_from_set = selected_projection_set_from_selected_set(&selected);
    let selected_from_index =
        selected_projection_set_from_records(selected.evidence_index().records());
    let selected_from_selected_inspect = selected_projection_set_from_receipt(
        &selected.inspect(
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
        ),
    );

    assert_eq!(
        selected.selected_obligation_handles().len(),
        selected.obligations().len()
    );
    assert!(
        selected
            .evidence_index()
            .records()
            .iter()
            .any(|record| record.decision() == UiObligationEvidenceDecision::NotSelected),
        "query-touch fixture must keep plausible non-selected candidates around the selected set"
    );
    assert_eq!(selected_from_set, selected_from_index);
    assert_eq!(selected_from_set, selected_from_selected_inspect);

    let report = app.admission().admit_selected_obligations(&selected);
    let selected_from_report_inspect = selected_projection_set_from_receipt(
        &report.inspect(
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
        ),
    );
    assert_eq!(selected_from_set, selected_from_report_inspect);

    let dispatch_records = report
        .evidence_index()
        .records()
        .iter()
        .filter(|record| record.decision() == UiObligationEvidenceDecision::Dispatch)
        .collect::<Vec<_>>();
    assert!(!dispatch_records.is_empty());
    assert!(dispatch_records.iter().all(|record| {
        record.dispatch_posture().is_some()
            && record.verdict_posture().is_none()
            && record.denial_posture().is_none()
    }));

    assert_eq!(
        report.verdict_evidence_handles().len(),
        report.verdicts().len()
    );
    for verdict in report.verdicts() {
        let evidence = report
            .evidence_index()
            .record(verdict.evidence_handle())
            .expect("verdict evidence handle should resolve");
        assert_eq!(evidence.handle(), verdict.evidence_handle());
        assert_eq!(evidence.decision(), UiObligationEvidenceDecision::Verdict);
        assert_eq!(evidence.family(), verdict.family());
        assert_eq!(evidence.dispatch_posture(), None);
        assert_eq!(
            evidence.verdict_posture().map(|posture| posture.class()),
            Some(verdict.class())
        );
        assert_eq!(
            evidence
                .verdict_posture()
                .map(|posture| posture.stop_posture()),
            Some(verdict.stop_posture())
        );
        assert_eq!(
            evidence.touch_identity_digest(),
            Some(touch.identity_digest())
        );
        assert_eq!(evidence.selection_reasons(), verdict.selection_reasons());
        let selected_evidence = selected
            .obligations()
            .iter()
            .find(|entry| verdict.selected_identity() == Some(entry.identity()))
            .map(|entry| {
                selected
                    .evidence_index()
                    .record(entry.evidence_handle())
                    .expect("selected evidence should remain indexed")
            })
            .expect("selected evidence should exist for verdict");
        assert_ne!(selected_evidence.handle(), evidence.handle());
        assert_eq!(
            selected_evidence.prerequisite_sources(),
            evidence.prerequisite_sources()
        );
    }
    let admission_records = report
        .evidence_index()
        .records()
        .iter()
        .filter(|record| record.decision() == UiObligationEvidenceDecision::Admission)
        .collect::<Vec<_>>();
    assert_eq!(admission_records.len(), 1);
    assert!(admission_records.iter().all(|record| {
        record.dispatch_posture().is_none()
            && record.verdict_posture().is_none()
            && record.touch_identity_digest() == Some(touch.identity_digest())
    }));

    let denied_report = app.admission().report(
        obligation_dispatch_prerequisite_support::admission_targets::wrong_query_basis_target(
            &touch,
        ),
    );
    let denial_receipt = denied_report.inspect(
        UiInspectionQuery::new(
            UiInspectionTarget::obligation_graph_node(
                touch.target().graph_node_identity().digest(),
            ),
            UiInspectionScope::graph(),
        )
        .with_relevance(
            obligation_relevance(
                UiInspectionObligationRelevanceDetail::new().with_denial_posture(
                    UiInspectionObligationDenialPosture::WrongQueryBasis {
                        required: worth_ui::facade::inspection::UiInspectionAdmissionQueryBasis::GraphAligned,
                        observed: worth_ui::facade::inspection::UiInspectionAdmissionQueryBasis::WrongWorldProjection,
                    },
                ),
            ),
        )
        .with_richness(UiEvidenceRichness::materialized_detail()),
    );
    let denied_rows = denial_receipt
        .evidence_slice()
        .and_then(|slice| slice.materialized_detail())
        .and_then(|detail| match detail {
            UiEvidenceMaterializedDetail::Obligation(receipt) => Some(receipt.projections()),
            _ => None,
        })
        .expect("obligation evidence receipt should be materialized through the evidence slice");
    assert_eq!(denied_rows.len(), 1);
    let denial = &denied_rows[0];
    assert_eq!(
        denial.decision(),
        worth_ui::facade::inspection::UiInspectionObligationDecision::Admission
    );
    assert_eq!(
        denial.denial_posture(),
        Some(UiInspectionObligationDenialPosture::WrongQueryBasis {
            required: worth_ui::facade::inspection::UiInspectionAdmissionQueryBasis::GraphAligned,
            observed:
                worth_ui::facade::inspection::UiInspectionAdmissionQueryBasis::WrongWorldProjection,
        })
    );

    let evidence_slice = denial_receipt
        .evidence_slice()
        .expect("obligation inspection should carry a typed evidence slice");
    assert_eq!(evidence_slice.refs().len(), denied_rows.len());
    assert_eq!(evidence_slice.family_summaries().len(), 1);
    assert_eq!(
        evidence_slice.family_summaries()[0].family(),
        UiEvidenceFamily::Obligation
    );
    assert_eq!(evidence_slice.family_summaries()[0].ref_count(), 1);
    assert!(matches!(
        evidence_slice.materialized_detail(),
        Some(UiEvidenceMaterializedDetail::Obligation(_))
    ));
    let evidence_ref = evidence_slice.refs()[0];
    assert_eq!(evidence_ref.family(), UiEvidenceFamily::Obligation);
    assert_eq!(
        evidence_ref.authority_binding().artifact_identity().kind(),
        UiEvidenceAuthorityKind::AdmissionReport
    );
    assert_ne!(
        evidence_ref
            .authority_binding()
            .artifact_identity()
            .digest(),
        touch.target().graph_node_identity().digest(),
        "admission evidence provenance must bind to the owning report artifact, not the graph node"
    );
}

#[test]
fn evidence_provenance_uses_owner_artifact_identity_instead_of_surrogate_target_digests() {
    let app = obligation_dispatch_prerequisite_support::application_authority::query_touch_app();
    let touch = obligation_dispatch_prerequisite_support::graph_touches::query_touch(&app);
    let target =
        obligation_dispatch_prerequisite_support::admission_targets::graph_aligned_query_target(
            &touch,
        );
    let selected = app
        .admission()
        .select_obligations_for_target(&touch, target.clone());
    let selected_receipt = selected.inspect(
        UiInspectionQuery::new(
            UiInspectionTarget::obligation_touch(
                touch.target().graph_node_identity().digest(),
                touch.identity_digest(),
            ),
            UiInspectionScope::graph(),
        )
        .with_relevance(obligation_relevance(
            UiInspectionObligationRelevanceDetail::new(),
        )),
    );
    let selected_slice = selected_receipt
        .evidence_slice()
        .expect("selected obligation inspection should expose an evidence slice");
    let selected_ref = selected_slice
        .refs()
        .first()
        .copied()
        .expect("selected obligation inspection should retain evidence refs");

    assert_eq!(
        selected_ref.authority_binding().artifact_identity().kind(),
        UiEvidenceAuthorityKind::ObligationAuthority
    );
    assert_ne!(
        selected_ref.authority_binding().artifact_identity().digest(),
        touch.identity_digest(),
        "selected obligation evidence provenance must bind to the selected-set artifact, not the touch digest"
    );

    let report = app.admission().admit_selected_obligations(&selected);
    let report_receipt = report.inspect(
        UiInspectionQuery::new(
            UiInspectionTarget::obligation_touch(
                touch.target().graph_node_identity().digest(),
                touch.identity_digest(),
            ),
            UiInspectionScope::graph(),
        )
        .with_richness(UiEvidenceRichness::materialized_detail()),
    );
    let report_slice = report_receipt
        .evidence_slice()
        .expect("admission report inspection should expose an evidence slice");
    let report_projections = report_slice
        .materialized_detail()
        .and_then(|detail| match detail {
            UiEvidenceMaterializedDetail::Obligation(receipt) => Some(receipt.projections()),
            _ => None,
        })
        .expect("report inspection should materialize obligation detail");
    let verdict_row = report_projections
        .iter()
        .find(|projection| {
            projection.decision()
                == worth_ui::facade::inspection::UiInspectionObligationDecision::Verdict
        })
        .expect("report inspection should retain a verdict row");
    let verdict_ref = report_slice
        .refs()
        .iter()
        .copied()
        .find(|reference| reference.handle().handle_digest() == verdict_row.handle_digest())
        .expect("verdict row should match a public evidence ref");

    assert_ne!(
        verdict_ref.authority_binding().artifact_identity().digest(),
        touch.identity_digest(),
        "verdict evidence provenance must bind to the verdict artifact, not the touch digest"
    );

    let dispatch_row = report_projections
        .iter()
        .find(|projection| {
            projection.decision()
                == worth_ui::facade::inspection::UiInspectionObligationDecision::Dispatch
        })
        .expect("report inspection should retain a dispatch row");
    let dispatch_ref = report_slice
        .refs()
        .iter()
        .copied()
        .find(|reference| reference.handle().handle_digest() == dispatch_row.handle_digest())
        .expect("dispatch row should match a public evidence ref");
    assert_eq!(
        dispatch_ref.authority_binding().artifact_identity().kind(),
        UiEvidenceAuthorityKind::ObligationAuthority
    );
    assert_ne!(
        dispatch_ref
            .authority_binding()
            .artifact_identity()
            .digest(),
        selected_ref
            .authority_binding()
            .artifact_identity()
            .digest()
    );
    assert_ne!(
        dispatch_ref
            .authority_binding()
            .artifact_identity()
            .digest(),
        verdict_ref.authority_binding().artifact_identity().digest()
    );
}

fn obligation_relevance(detail: UiInspectionObligationRelevanceDetail) -> UiInspectionRelevance {
    UiInspectionRelevance::local(UiRelevanceFilter::family(UiRelevanceFamily::Obligation))
        .with_obligation_detail(detail)
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

fn selected_projection_set_from_records(
    records: &[worth_ui_runtime::facade::obligations::UiObligationEvidenceRecord],
) -> Vec<UiSelectedObligationEvidenceProjection> {
    let mut projections = records
        .iter()
        .filter_map(UiSelectedObligationEvidenceProjection::from_selected_record)
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
