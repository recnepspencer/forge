mod firewall_region;
mod firewall_region_row;
mod firewall_report;
mod firewall_violation;
mod forbidden_pattern;
mod source_roots;
mod source_text_scan;
mod workspace_scan;

pub use firewall_region_row::WorthGraphReadAccessHardDeletionSourceFirewallRegionRow;
pub use firewall_report::WorthGraphReadAccessHardDeletionSourceFirewallReport;
pub use firewall_violation::WorthGraphReadAccessHardDeletionSourceFirewallViolation;
#[cfg(test)]
pub(crate) use source_text_scan::{scan_source, scan_source_for_region};
pub(crate) use workspace_scan::scan_workspace;

#[cfg(test)]
pub(crate) use forbidden_pattern::forbidden_pattern_audit_rows;
