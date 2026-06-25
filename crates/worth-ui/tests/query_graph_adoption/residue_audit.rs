use worth_ui::facade::WorthUiQueryGraphAdoptionResidueAudit;

#[test]
fn residue_audit_rejects_local_graph_authority_as_final_proof() {
    let findings = WorthUiQueryGraphAdoptionResidueAudit::scan_source(
        "let graph_admission = WorthUiGraphTouchAdmission::admit(declaration);
         let rows = WorthUiGraphObligationRegistry::select(&declaration);
         let posture = WorthUiGraphQueryPosture::Untouched;
         let digest = admission_digest;",
    );

    assert!(findings.len() >= 4);
    assert!(findings
        .iter()
        .any(|finding| finding.source() == "WorthUiGraphTouchAdmission::admit("));
    assert!(findings
        .iter()
        .any(|finding| finding.source() == "WorthUiGraphObligationRegistry::select("));
    assert!(findings
        .iter()
        .any(|finding| finding.source() == "WorthUiGraphQueryPosture"));
    assert!(findings
        .iter()
        .any(|finding| finding.source() == "admission_digest"));
}

#[test]
fn query_owned_residue_audit_runs_for_query_graph_sources() {
    let report = WorthUiQueryGraphAdoptionResidueAudit::evaluate_query_owned_report()
        .expect("Query-owned residue audit should evaluate Worth query graph sources");

    assert_eq!(report.consumer_name(), "worth-ui");
    assert!(report.scanned_file_count() > 0);
    assert!(report.visited_node_count() > 0);
}

#[test]
fn residue_audit_allows_query_execution_adapter_source() {
    let findings = WorthUiQueryGraphAdoptionResidueAudit::scan_source(
        "let proof = mounted_interaction_adoption_proof()?;",
    );

    assert!(findings.is_empty());
}
