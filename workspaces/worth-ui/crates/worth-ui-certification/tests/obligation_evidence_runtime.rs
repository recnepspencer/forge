use worth_ui::facade::inspection::{
    UiInspectionObligationDenialPosture, UiInspectionQuery, UiInspectionScope, UiInspectionTarget,
};
use worth_ui::facade::obligations::UiObligationEvidenceDecision;

#[path = "fixtures/obligation_dispatch_prerequisite_support/mod.rs"]
mod obligation_dispatch_prerequisite_support;

#[test]
fn selected_verdict_and_admission_paths_retain_typed_evidence_handles() {
    let app = obligation_dispatch_prerequisite_support::query_touch_app();
    let touch = obligation_dispatch_prerequisite_support::query_touch(&app);
    let target = obligation_dispatch_prerequisite_support::graph_aligned_query_target(&touch);
    let selected = app
        .admission()
        .select_obligations_for_target(&touch, target.clone());

    assert_eq!(
        selected.selected_obligation_handles().len(),
        selected.obligations().len()
    );
    for selected_obligation in selected.obligations() {
        let record = selected
            .evidence_index()
            .record(selected_obligation.evidence_handle())
            .expect("selected obligation handle should resolve");
        assert_eq!(record.family(), Some(selected_obligation.family()));
        assert_eq!(record.decision(), UiObligationEvidenceDecision::Selected);
        assert_eq!(
            record.touch_identity_digest(),
            Some(touch.identity_digest())
        );
        assert_eq!(
            record.selection_reasons(),
            selected_obligation.selection_reasons()
        );
    }

    let report = app.admission().admit_selected_obligations(&selected);
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

    let denied_report = app
        .admission()
        .report(obligation_dispatch_prerequisite_support::wrong_query_basis_target(&touch));
    let denial_receipt = denied_report.inspect(
        UiInspectionQuery::new(
            UiInspectionTarget::obligation_graph_node(
                touch.target().graph_node_identity().digest(),
            ),
            UiInspectionScope::graph(),
        )
        .with_obligation_evidence(
            worth_ui::facade::inspection::UiInspectionObligationEvidenceQuery::new()
                .with_denial_posture(UiInspectionObligationDenialPosture::WrongQueryBasis {
                    required: worth_ui::facade::inspection::UiInspectionAdmissionQueryBasis::GraphAligned,
                    observed: worth_ui::facade::inspection::UiInspectionAdmissionQueryBasis::WrongWorldProjection,
                }),
        ),
    );
    let denied_rows = denial_receipt
        .obligation_evidence()
        .expect("obligation evidence receipt should be present")
        .projections();
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
}
