use crate::workload_platform::evidence_lookup_source_firewall::{
    current_evidence_lookup_source_firewall_report,
    EvidenceLookupSourceFirewallCoveredRootKind as CoveredRootKind,
    EvidenceLookupSourceFirewallExceptionKind as ExceptionKind,
    EvidenceLookupSourceFirewallRowPosture,
};

#[test]
fn current_workspace_report_names_phase_fourteen_firewall_truth() {
    let report = current_evidence_lookup_source_firewall_report()
        .expect("current workspace firewall report");

    assert_eq!(report.covered_root_inventory().len(), 9);
    assert_eq!(
        report.covered_roots().len(),
        report.covered_root_inventory().len()
    );
    assert!(report
        .covered_root_inventory()
        .iter()
        .any(|root| root.kind() == CoveredRootKind::DocumentationReportCodec));
    assert!(report
        .covered_root_inventory()
        .iter()
        .any(|root| root.kind() == CoveredRootKind::CertificationCodec));
    assert_eq!(
        report.counters().allowed_exception_row_count(),
        report.allowed_exception_rows().len()
    );
    assert_eq!(
        report.counters().forbidden_row_count(),
        report.forbidden_rows().len()
    );
    assert!(report.exception_summaries().iter().any(|summary| {
        summary.kind() == ExceptionKind::DocumentationReportCodec && summary.row_count() > 0
    }));
    assert!(report.exception_summaries().iter().any(|summary| {
        summary.kind() == ExceptionKind::CertificationOnlyCodec && summary.row_count() > 0
    }));
    assert!(report.allowed_exception_rows().iter().all(|row| {
        row.posture() == EvidenceLookupSourceFirewallRowPosture::AllowedNamedException
            && !row.claims_lookup_execution_authority()
    }));
}
