use worth_ui::facade::inspection::{UiInspectionQuery, UiInspectionScope, UiInspectionTarget};
use worth_ui::facade::obligations::UiObligationEvidenceDecision;
use worth_ui_runtime::facade::admission::UiAdmissionReport;

#[path = "fixtures/obligation_dispatch_prerequisite_support/mod.rs"]
mod obligation_dispatch_prerequisite_support;

#[test]
fn diagnostic_projection_derives_from_the_same_evidence_as_inspection() {
    let app = obligation_dispatch_prerequisite_support::query_touch_app();
    let touch = obligation_dispatch_prerequisite_support::query_touch(&app);
    let target = obligation_dispatch_prerequisite_support::graph_aligned_query_target(&touch);
    let report = app.admission().admit_selected_obligations(
        &app.admission()
            .select_obligations_for_target(&touch, target),
    );

    let inspection = report.inspect(UiInspectionQuery::new(
        UiInspectionTarget::obligation_touch(
            touch.target().graph_node_identity().digest(),
            touch.identity_digest(),
        ),
        UiInspectionScope::graph(),
    ));
    let diagnostics = report.diagnostic_projection();

    let inspection_rows = inspection
        .obligation_evidence()
        .expect("obligation evidence receipt should be present")
        .projections();

    assert_parity_for_decision(
        inspection_rows,
        diagnostics.rows(),
        &report,
        UiObligationEvidenceDecision::Selected,
    );
    assert_parity_for_decision(
        inspection_rows,
        diagnostics.rows(),
        &report,
        UiObligationEvidenceDecision::NotSelected,
    );
    assert_parity_for_decision(
        inspection_rows,
        diagnostics.rows(),
        &report,
        UiObligationEvidenceDecision::Verdict,
    );

    let denied_report = app
        .admission()
        .report(obligation_dispatch_prerequisite_support::wrong_query_basis_target(&touch));
    let denied_inspection = denied_report.inspect(UiInspectionQuery::new(
        UiInspectionTarget::obligation_graph_node(touch.target().graph_node_identity().digest()),
        UiInspectionScope::graph(),
    ));
    let denied_projection = denied_report.diagnostic_projection();
    assert_parity_for_decision(
        denied_inspection
            .obligation_evidence()
            .expect("obligation evidence receipt should be present")
            .projections(),
        denied_projection.rows(),
        &denied_report,
        UiObligationEvidenceDecision::Admission,
    );
}

fn assert_parity_for_decision(
    inspection_rows: &[worth_ui::facade::inspection::UiInspectionObligationReasonProjection],
    diagnostic_rows: &[worth_ui::facade::obligations::UiObligationDiagnosticRow],
    report: &UiAdmissionReport,
    decision: UiObligationEvidenceDecision,
) {
    let handle = report
        .evidence_index()
        .records()
        .iter()
        .find(|record| record.decision() == decision)
        .unwrap_or_else(|| panic!("{decision:?} evidence should exist"))
        .handle()
        .digest();

    let inspection_row = inspection_rows
        .iter()
        .find(|projection| projection.handle_digest() == handle)
        .unwrap_or_else(|| panic!("{decision:?} inspection row should exist"));
    let diagnostic_row = diagnostic_rows
        .iter()
        .find(|row| row.handle_digest() == handle)
        .unwrap_or_else(|| panic!("{decision:?} diagnostic row should exist"));

    assert_eq!(inspection_row.family(), diagnostic_row.family());
    assert_eq!(inspection_row.decision(), diagnostic_row.decision());
    assert_eq!(
        inspection_row.denial_posture(),
        diagnostic_row.denial_posture()
    );
    assert_eq!(
        inspection_row.selection_reasons(),
        diagnostic_row.selection_reasons()
    );
    assert_eq!(
        inspection_row.non_selection_reason(),
        diagnostic_row.non_selection_reason()
    );
    assert_eq!(
        inspection_row.legality_reason(),
        diagnostic_row.legality_reason()
    );
    assert_eq!(
        inspection_row.prerequisite_sources(),
        diagnostic_row.prerequisite_sources()
    );
}
