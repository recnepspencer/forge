mod counters;
mod covered_root;
mod error;
mod exception;
mod exception_summary;
mod report;
mod row;
mod scan_roots;
mod semantic_shape;

#[cfg(test)]
mod tests;

pub use counters::EvidenceLookupSourceFirewallCounters;
pub use covered_root::{
    EvidenceLookupSourceFirewallCoveredRoot, EvidenceLookupSourceFirewallCoveredRootKind,
};
pub use error::{EvidenceLookupSourceFirewallError, EvidenceLookupSourceFirewallErrorKind};
pub use exception_summary::EvidenceLookupSourceFirewallExceptionSummary;
pub use report::{
    current_evidence_lookup_source_firewall_report, EvidenceLookupSourceFirewallOutcome,
    EvidenceLookupSourceFirewallReport,
};
pub use row::{
    EvidenceLookupForbiddenAuthorityKind, EvidenceLookupSourceFirewallExceptionKind,
    EvidenceLookupSourceFirewallRow, EvidenceLookupSourceFirewallRowPosture,
};

#[cfg(test)]
pub(crate) use report::source_firewall_report_for_snapshot_root;
