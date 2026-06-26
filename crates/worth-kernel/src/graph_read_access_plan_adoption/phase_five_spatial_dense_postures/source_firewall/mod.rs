mod firewall_report;
mod firewall_violation;
mod forbidden_pattern;
mod source_roots;

pub(crate) use firewall_report::scan_workspace;
pub use firewall_report::WorthGraphReadAccessSpatialDenseSourceFirewallReport;
pub use firewall_violation::WorthGraphReadAccessSpatialDenseSourceFirewallViolation;

pub fn reject_spatial_dense_local_graph_read_residue(
    source_path: &str,
    source_text: &str,
) -> Result<
    WorthGraphReadAccessSpatialDenseSourceFirewallReport,
    WorthGraphReadAccessSpatialDenseSourceFirewallViolation,
> {
    firewall_report::scan_source(source_path, source_text)
}
