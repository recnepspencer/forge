use crate::validator_invariant_catalog::operator_certification_cutover::{
    WorthTopologyOperatorCertificationCutoverSourceFirewallReport,
    WorthTopologyOperatorCertificationOldExpectationResidueReport,
    WorthTopologyOperatorSelectedObligationCloseoutRow,
    WorthTopologyOperatorSelectedObligationSupportPostureRow,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTopologyOperatorCertificationCutoverCounters {
    selected_obligation_closeout_row_count: usize,
    support_posture_row_count: usize,
    old_expectation_residue_row_count: usize,
    capped_old_expectation_residue_row_count: usize,
    uncapped_old_expectation_authority_count: usize,
    source_firewall_violation_count: usize,
    scanned_source_file_count: usize,
    executed_obligation_count: usize,
    visible_unsupported_or_diagnostic_count: usize,
    counters_digest: String,
}

impl WorthTopologyOperatorCertificationCutoverCounters {
    pub(in crate::validator_invariant_catalog) fn from_rows(
        closeout_rows: &[WorthTopologyOperatorSelectedObligationCloseoutRow],
        support_rows: &[WorthTopologyOperatorSelectedObligationSupportPostureRow],
        residue: &WorthTopologyOperatorCertificationOldExpectationResidueReport,
        firewall: &WorthTopologyOperatorCertificationCutoverSourceFirewallReport,
    ) -> Self {
        let executed_obligation_count = closeout_rows
            .iter()
            .filter(|row| row.query_execution_status() == "executed")
            .count();
        let visible_unsupported_or_diagnostic_count = closeout_rows
            .iter()
            .filter(|row| {
                matches!(
                    row.query_support_status(),
                    "unsupported" | "diagnostic-only" | "deferred-to-backstop"
                )
            })
            .count();
        let counters_digest = [
            "worth-topo-operator-certification-cutover-counters-v1",
            &closeout_rows.len().to_string(),
            &support_rows.len().to_string(),
            &residue.rows().len().to_string(),
            &residue.capped_authority_count().to_string(),
            &residue.uncapped_authority_count().to_string(),
            &firewall.violations().len().to_string(),
            &firewall.scanned_file_count().to_string(),
            &executed_obligation_count.to_string(),
            &visible_unsupported_or_diagnostic_count.to_string(),
        ]
        .join("|");
        Self {
            selected_obligation_closeout_row_count: closeout_rows.len(),
            support_posture_row_count: support_rows.len(),
            old_expectation_residue_row_count: residue.rows().len(),
            capped_old_expectation_residue_row_count: residue.capped_authority_count(),
            uncapped_old_expectation_authority_count: residue.uncapped_authority_count(),
            source_firewall_violation_count: firewall.violations().len(),
            scanned_source_file_count: firewall.scanned_file_count(),
            executed_obligation_count,
            visible_unsupported_or_diagnostic_count,
            counters_digest,
        }
    }

    pub const fn selected_obligation_closeout_row_count(&self) -> usize {
        self.selected_obligation_closeout_row_count
    }

    pub const fn support_posture_row_count(&self) -> usize {
        self.support_posture_row_count
    }

    pub const fn old_expectation_residue_row_count(&self) -> usize {
        self.old_expectation_residue_row_count
    }

    pub const fn capped_old_expectation_residue_row_count(&self) -> usize {
        self.capped_old_expectation_residue_row_count
    }

    pub const fn uncapped_old_expectation_authority_count(&self) -> usize {
        self.uncapped_old_expectation_authority_count
    }

    pub const fn source_firewall_violation_count(&self) -> usize {
        self.source_firewall_violation_count
    }

    pub const fn scanned_source_file_count(&self) -> usize {
        self.scanned_source_file_count
    }

    pub const fn executed_obligation_count(&self) -> usize {
        self.executed_obligation_count
    }

    pub const fn visible_unsupported_or_diagnostic_count(&self) -> usize {
        self.visible_unsupported_or_diagnostic_count
    }

    pub fn counters_digest(&self) -> &str {
        &self.counters_digest
    }
}
