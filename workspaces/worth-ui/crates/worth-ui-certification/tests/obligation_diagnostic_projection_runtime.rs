use worth_ui::facade::inspection::{
    UiEvidenceMaterializedDetail, UiEvidenceRef, UiEvidenceRichness,
    UiInspectionObligationDecision, UiInspectionQuery, UiInspectionScope, UiInspectionTarget,
};

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

    let inspection = report.inspect(
        UiInspectionQuery::new(
            UiInspectionTarget::obligation_touch(
                touch.target().graph_node_identity().digest(),
                touch.identity_digest(),
            ),
            UiInspectionScope::graph(),
        )
        .with_richness(UiEvidenceRichness::materialized_detail()),
    );
    let diagnostics = report.diagnostic_projection();

    let inspection_rows = inspection
        .evidence_slice()
        .and_then(|slice| slice.materialized_detail())
        .and_then(obligation_detail)
        .expect("obligation evidence receipt should be materialized through the evidence slice")
        .projections();
    let inspection_refs = inspection
        .evidence_slice()
        .expect("inspection should expose a public evidence slice")
        .refs();

    assert_parity_for_decision(
        inspection_refs,
        inspection_rows,
        diagnostics.rows(),
        UiInspectionObligationDecision::Selected,
    );
    assert_parity_for_decision(
        inspection_refs,
        inspection_rows,
        diagnostics.rows(),
        UiInspectionObligationDecision::NotSelected,
    );
    assert_parity_for_decision(
        inspection_refs,
        inspection_rows,
        diagnostics.rows(),
        UiInspectionObligationDecision::Verdict,
    );

    let denied_report = app
        .admission()
        .report(obligation_dispatch_prerequisite_support::wrong_query_basis_target(&touch));
    let denied_inspection = denied_report.inspect(
        UiInspectionQuery::new(
            UiInspectionTarget::obligation_graph_node(
                touch.target().graph_node_identity().digest(),
            ),
            UiInspectionScope::graph(),
        )
        .with_richness(UiEvidenceRichness::materialized_detail()),
    );
    let denied_projection = denied_report.diagnostic_projection();
    assert_parity_for_decision(
        denied_inspection
            .evidence_slice()
            .expect("denied inspection should expose a public evidence slice")
            .refs(),
        denied_inspection
            .evidence_slice()
            .and_then(|slice| slice.materialized_detail())
            .and_then(obligation_detail)
            .expect("obligation evidence receipt should be materialized through the evidence slice")
            .projections(),
        denied_projection.rows(),
        UiInspectionObligationDecision::Admission,
    );
}

fn obligation_detail(
    detail: &UiEvidenceMaterializedDetail,
) -> Option<&worth_ui::facade::inspection::UiInspectionObligationEvidenceReceipt> {
    match detail {
        UiEvidenceMaterializedDetail::Obligation(receipt) => Some(receipt),
        _ => None,
    }
}

fn assert_parity_for_decision(
    inspection_refs: &[UiEvidenceRef],
    inspection_rows: &[worth_ui::facade::inspection::UiInspectionObligationReasonProjection],
    diagnostic_rows: &[worth_ui::facade::obligations::UiObligationDiagnosticRow],
    decision: UiInspectionObligationDecision,
) {
    let inspection_row = inspection_rows
        .iter()
        .find(|projection| projection.decision() == decision)
        .unwrap_or_else(|| panic!("{decision:?} inspection row should exist"));
    let inspection_ref = inspection_refs
        .iter()
        .find(|reference| reference.handle().handle_digest() == inspection_row.handle_digest())
        .unwrap_or_else(|| panic!("{decision:?} public evidence ref should exist"));
    let diagnostic_row = diagnostic_rows
        .iter()
        .find(|row| row.handle_digest() == inspection_ref.handle().handle_digest())
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
