use worth_spatial::facade::evidence_lookup_source_firewall::{
    current_evidence_lookup_source_firewall_report, EvidenceLookupForbiddenAuthorityKind,
    EvidenceLookupSourceFirewallCounters, EvidenceLookupSourceFirewallCoveredRoot,
    EvidenceLookupSourceFirewallCoveredRootKind, EvidenceLookupSourceFirewallError,
    EvidenceLookupSourceFirewallErrorKind, EvidenceLookupSourceFirewallExceptionSummary,
    EvidenceLookupSourceFirewallOutcome, EvidenceLookupSourceFirewallReport,
    EvidenceLookupSourceFirewallRow, EvidenceLookupSourceFirewallRowPosture,
};

#[test]
fn spatial_public_api_exports_lookup_source_firewall_report() {
    let _: fn() -> Result<EvidenceLookupSourceFirewallReport, EvidenceLookupSourceFirewallError> =
        current_evidence_lookup_source_firewall_report;
}

#[test]
fn spatial_public_api_exposes_lookup_source_firewall_read_contract() {
    let report =
        current_evidence_lookup_source_firewall_report().expect("current firewall report reads");

    let _: fn(&EvidenceLookupSourceFirewallReport) -> &[EvidenceLookupSourceFirewallCoveredRoot] =
        EvidenceLookupSourceFirewallReport::covered_root_inventory;
    let _: fn(&EvidenceLookupSourceFirewallReport) -> &[String] =
        EvidenceLookupSourceFirewallReport::covered_roots;
    let _: fn(
        &EvidenceLookupSourceFirewallReport,
    ) -> &[EvidenceLookupSourceFirewallExceptionSummary] =
        EvidenceLookupSourceFirewallReport::exception_summaries;
    let _: fn(&EvidenceLookupSourceFirewallReport) -> &[EvidenceLookupSourceFirewallRow] =
        EvidenceLookupSourceFirewallReport::rows;
    let _: fn(&EvidenceLookupSourceFirewallReport) -> &EvidenceLookupSourceFirewallCounters =
        EvidenceLookupSourceFirewallReport::counters;
    let _: fn(&EvidenceLookupSourceFirewallReport) -> EvidenceLookupSourceFirewallOutcome =
        EvidenceLookupSourceFirewallReport::outcome;
    let _: fn(&EvidenceLookupSourceFirewallReport) -> &str =
        EvidenceLookupSourceFirewallReport::firewall_digest;
    let _: fn(&EvidenceLookupSourceFirewallReport) -> bool =
        EvidenceLookupSourceFirewallReport::claims_lookup_execution_authority;

    assert_eq!(
        report.covered_roots().len(),
        report.counters().scanned_root_count()
    );
    assert!(report.covered_root_inventory().iter().any(|root| {
        root.kind() == EvidenceLookupSourceFirewallCoveredRootKind::DocumentationReportCodec
    }));
    assert!(report
        .exception_summaries()
        .iter()
        .any(|summary| summary.row_count() > 0));
    assert_eq!(report.rows().len(), report.counters().total_row_count());
    assert!(!report.firewall_digest().is_empty());
    assert!(matches!(
        report.outcome(),
        EvidenceLookupSourceFirewallOutcome::Clean
            | EvidenceLookupSourceFirewallOutcome::ExceptionsOnly
            | EvidenceLookupSourceFirewallOutcome::ForbiddenAuthorityPresent
    ));
}

#[test]
fn spatial_public_api_exposes_lookup_source_firewall_row_and_error_contract() {
    let _: fn(&EvidenceLookupSourceFirewallRow) -> &str =
        EvidenceLookupSourceFirewallRow::source_path;
    let _: fn(&EvidenceLookupSourceFirewallRow) -> &str =
        EvidenceLookupSourceFirewallRow::matched_surface;
    let _: fn(&EvidenceLookupSourceFirewallRow) -> EvidenceLookupForbiddenAuthorityKind =
        EvidenceLookupSourceFirewallRow::forbidden_authority_kind;
    let _: fn(&EvidenceLookupSourceFirewallRow) -> EvidenceLookupSourceFirewallRowPosture =
        EvidenceLookupSourceFirewallRow::posture;
    let _: fn(&EvidenceLookupSourceFirewallRow) -> &str = EvidenceLookupSourceFirewallRow::reason;
    let _: fn(&EvidenceLookupSourceFirewallRow) -> bool =
        EvidenceLookupSourceFirewallRow::claims_lookup_execution_authority;
    let _: fn(&EvidenceLookupSourceFirewallError) -> EvidenceLookupSourceFirewallErrorKind =
        EvidenceLookupSourceFirewallError::kind;
    let _: fn(&EvidenceLookupSourceFirewallError) -> &str =
        EvidenceLookupSourceFirewallError::detail;
    let _: fn(&EvidenceLookupSourceFirewallCoveredRoot) -> &str =
        EvidenceLookupSourceFirewallCoveredRoot::source_path;
    let _: fn(
        &EvidenceLookupSourceFirewallCoveredRoot,
    ) -> EvidenceLookupSourceFirewallCoveredRootKind =
        EvidenceLookupSourceFirewallCoveredRoot::kind;
    let _: fn(&EvidenceLookupSourceFirewallExceptionSummary) -> usize =
        EvidenceLookupSourceFirewallExceptionSummary::row_count;
    let _: fn(&EvidenceLookupSourceFirewallCounters) -> usize =
        EvidenceLookupSourceFirewallCounters::certification_only_exception_row_count;
    let _: fn(&EvidenceLookupSourceFirewallCounters) -> usize =
        EvidenceLookupSourceFirewallCounters::documentation_report_exception_row_count;
    let _: fn(&EvidenceLookupSourceFirewallCounters) -> usize =
        EvidenceLookupSourceFirewallCounters::test_support_exception_row_count;
}
