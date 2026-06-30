use super::row::EvidenceLookupSourceFirewallExceptionKind;

pub(crate) fn named_exception_for_path(
    source_path: &str,
    test_support: bool,
) -> Option<(EvidenceLookupSourceFirewallExceptionKind, &'static str)> {
    if source_path == "crates/worth-spatial/src/certification/workload_evidence.rs" {
        return Some((
            EvidenceLookupSourceFirewallExceptionKind::CertificationOnlyCodec,
            "certification workload evidence is an allowed fixture/report codec and cannot satisfy ordinary lookup proof",
        ));
    }
    if source_path
        == "crates/worth-spatial/src/workload_platform/evidence_ledger/surface_inventory/rows.rs"
    {
        return Some((
            EvidenceLookupSourceFirewallExceptionKind::DocumentationReportCodec,
            "surface-inventory rows document or report legacy lookup residue and cannot satisfy ordinary lookup proof",
        ));
    }
    if test_support || source_path.contains("/test_support/") || source_path.contains("/tests/") {
        return Some((
            EvidenceLookupSourceFirewallExceptionKind::TestSupportFixture,
            "test support may mention lookup-shaped residue but remains non-authoritative fixture scaffolding",
        ));
    }
    None
}
