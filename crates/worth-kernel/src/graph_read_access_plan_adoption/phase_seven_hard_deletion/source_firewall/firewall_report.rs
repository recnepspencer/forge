use super::super::stable_digest;
use super::firewall_region_row::WorthGraphReadAccessHardDeletionSourceFirewallRegionRow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessHardDeletionSourceFirewallReport {
    region_rows: Vec<WorthGraphReadAccessHardDeletionSourceFirewallRegionRow>,
    scanned_region_count: usize,
    scanned_source_count: usize,
    forbidden_pattern_count: usize,
    violation_count: usize,
    report_digest: String,
}

impl WorthGraphReadAccessHardDeletionSourceFirewallReport {
    pub(crate) fn new(
        region_rows: Vec<WorthGraphReadAccessHardDeletionSourceFirewallRegionRow>,
        forbidden_pattern_count: usize,
        violation_count: usize,
    ) -> Self {
        let scanned_region_count = region_rows.len();
        let scanned_source_count = region_rows
            .iter()
            .map(|row| row.scanned_source_count())
            .sum::<usize>();
        let report_digest = stable_digest(
            &std::iter::once(
                "worth_graph_read_access_hard_deletion_source_firewall_report_v1".to_string(),
            )
            .chain(region_rows.iter().map(|row| {
                format!(
                    "region:{}:{}:{}",
                    row.region(),
                    row.root_identity(),
                    row.row_digest()
                )
            }))
            .chain([
                format!("regions:{scanned_region_count}"),
                format!("scanned:{scanned_source_count}"),
                format!("patterns:{forbidden_pattern_count}"),
                format!("violations:{violation_count}"),
            ])
            .collect::<Vec<_>>(),
        );
        Self {
            region_rows,
            scanned_region_count,
            scanned_source_count,
            forbidden_pattern_count,
            violation_count,
            report_digest,
        }
    }

    pub fn region_rows(&self) -> &[WorthGraphReadAccessHardDeletionSourceFirewallRegionRow] {
        &self.region_rows
    }

    pub const fn scanned_region_count(&self) -> usize {
        self.scanned_region_count
    }

    pub const fn scanned_source_count(&self) -> usize {
        self.scanned_source_count
    }

    pub const fn forbidden_pattern_count(&self) -> usize {
        self.forbidden_pattern_count
    }

    pub const fn violation_count(&self) -> usize {
        self.violation_count
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

#[cfg(test)]
mod adversarial_source_firewall_report {
    use super::*;

    impl WorthGraphReadAccessHardDeletionSourceFirewallReport {
        pub(crate) fn with_violation_for_tests(&self) -> Self {
            let mut report = self.clone();
            report.violation_count = self.violation_count + 1;
            report.report_digest = stable_digest(&[
                "worth_graph_read_access_hard_deletion_source_firewall_report_adversarial_violation_v1"
                    .to_string(),
                format!("source:{}", self.report_digest),
                format!("violations:{}", report.violation_count),
            ]);
            report
        }
    }
}
