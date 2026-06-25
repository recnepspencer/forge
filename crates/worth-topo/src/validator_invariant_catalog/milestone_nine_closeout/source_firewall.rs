use std::collections::BTreeSet;

use crate::validator_invariant_catalog::milestone_nine_closeout::{
    WorthTopologyMilestoneNineAuthorityOccurrenceInventory,
    WorthTopologyMilestoneNineDeletionLedgerReport,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTopologyMilestoneNineSourceFirewallReport {
    scanned_file_count: usize,
    scanned_source_paths: Vec<String>,
    deletion_ledger_allowed_paths: Vec<String>,
    violations: Vec<String>,
    report_digest: String,
}

impl WorthTopologyMilestoneNineSourceFirewallReport {
    pub const CURRENT_SCAN_SOURCE_PATHS: [&'static str; 8] = [
        "certification/topology_operator_closeout/acceptance_rows/validator_family_coverage.rs",
        "certification/topology_operator_closeout/validation_breadth_row.rs",
        "runtime_support.rs",
        "topology_operators/application/declaration_entry/execution_finalize.rs",
        "topology_operators/declaration_entry/mod.rs",
        "topology_operators/edge_split_blueprint/required_phase_1_validator_lanes.rs",
        "topology_operators/loop_reconstruction_blueprint/phase_2_inventory/validator_rows.rs",
        "validation/rule_registry.rs",
    ];

    pub(in crate::validator_invariant_catalog) fn current_with_deletion_ledger(
        deletion_ledger: &WorthTopologyMilestoneNineDeletionLedgerReport,
    ) -> Self {
        let inventory =
            WorthTopologyMilestoneNineAuthorityOccurrenceInventory::current_from_deletion_ledger(
                deletion_ledger,
            );
        Self::from_authority_occurrence_inventory(deletion_ledger, &inventory)
    }

    #[cfg(test)]
    pub(in crate::validator_invariant_catalog) fn from_source_pairs_with_deletion_ledger(
        sources: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
        deletion_ledger: &WorthTopologyMilestoneNineDeletionLedgerReport,
    ) -> Self {
        let inventory =
            WorthTopologyMilestoneNineAuthorityOccurrenceInventory::from_source_pairs_and_deletion_ledger(
                sources,
                deletion_ledger,
                Self::forbidden_authority_patterns(),
            );
        Self::from_authority_occurrence_inventory(deletion_ledger, &inventory)
    }

    fn from_authority_occurrence_inventory(
        deletion_ledger: &WorthTopologyMilestoneNineDeletionLedgerReport,
        inventory: &WorthTopologyMilestoneNineAuthorityOccurrenceInventory,
    ) -> Self {
        let scanned_file_count = inventory.scanned_source_paths().len();
        let scanned_source_paths = inventory.scanned_source_paths().to_vec();
        let deletion_ledger_allowed_paths = deletion_ledger
            .rows()
            .iter()
            .map(|row| row.source_path())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .map(str::to_string)
            .collect::<Vec<_>>();
        let violations = inventory
            .violation_rows()
            .into_iter()
            .map(|row| {
                format!(
                    "{}::{}::observed-{}::allowed-{}::{}",
                    row.source_path(),
                    row.forbidden_pattern(),
                    row.observed_count(),
                    row.ledger_allowed_count(),
                    row.status().as_str()
                )
            })
            .collect::<Vec<_>>();
        let mut digest_parts = vec![
            "worth-topo-milestone-nine-source-firewall-report-v1".to_string(),
            format!("scanned-file-count:{scanned_file_count}"),
            format!("violation-count:{}", violations.len()),
            format!("authority-inventory:{}", inventory.inventory_digest()),
        ];
        digest_parts.extend(
            scanned_source_paths
                .iter()
                .map(|path| format!("scanned:{path}")),
        );
        digest_parts.extend(
            deletion_ledger_allowed_paths
                .iter()
                .map(|path| format!("allowed-deletion-ledger:{path}")),
        );
        digest_parts.extend(
            violations
                .iter()
                .map(|violation| format!("violation:{violation}")),
        );
        Self {
            scanned_file_count,
            scanned_source_paths,
            deletion_ledger_allowed_paths,
            violations,
            report_digest: digest_parts.join("|"),
        }
    }

    pub const fn scanned_file_count(&self) -> usize {
        self.scanned_file_count
    }

    pub fn scanned_source_paths(&self) -> &[String] {
        &self.scanned_source_paths
    }

    pub fn deletion_ledger_allowed_paths(&self) -> &[String] {
        &self.deletion_ledger_allowed_paths
    }

    pub fn is_clean(&self) -> bool {
        self.violations.is_empty()
    }

    pub fn violations(&self) -> &[String] {
        &self.violations
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }

    pub const fn forbidden_authority_patterns() -> &'static [&'static str] {
        super::authority_occurrence_inventory::FORBIDDEN_AUTHORITY_PATTERNS
    }
}
