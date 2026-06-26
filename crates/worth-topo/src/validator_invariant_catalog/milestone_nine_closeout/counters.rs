use std::collections::BTreeSet;

use crate::validator_invariant_catalog::milestone_nine_closeout::{
    WorthTopologyMilestoneNineDeletionLedgerReport, WorthTopologyMilestoneNineResidueAuditReport,
    WorthTopologyMilestoneNineSourceFirewallReport,
};
use crate::validator_invariant_catalog::WorthTopologyOperatorCertificationCutoverCloseout;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTopologyMilestoneNineCloseoutCounters {
    selected_obligation_count: usize,
    worth_family_count: usize,
    enforcement_receipt_count: usize,
    graph_read_receipt_count: usize,
    budget_denial_count: usize,
    support_pin_count: usize,
    executor_row_count: usize,
    execution_backed_adoption_proof_count: usize,
    residue_manifest_count: usize,
    deletion_ledger_row_count: usize,
    capped_residue_row_count: usize,
    stale_residue_row_count: usize,
    uncapped_authority_count: usize,
    source_firewall_violation_count: usize,
    scanned_source_file_count: usize,
    whole_view_certification_only_count: usize,
    counters_digest: String,
}

impl WorthTopologyMilestoneNineCloseoutCounters {
    pub(in crate::validator_invariant_catalog) fn from_reports(
        cutover: &WorthTopologyOperatorCertificationCutoverCloseout,
        deletion_ledger: &WorthTopologyMilestoneNineDeletionLedgerReport,
        residue_audit: &WorthTopologyMilestoneNineResidueAuditReport,
        source_firewall: &WorthTopologyMilestoneNineSourceFirewallReport,
    ) -> Self {
        let selected_obligation_count = cutover.selected_obligation_closeout_rows().len();
        let worth_family_count = cutover
            .selected_obligation_closeout_rows()
            .iter()
            .map(|row| row.worth_family_identity_digest())
            .collect::<BTreeSet<_>>()
            .len();
        let enforcement_receipt_count = cutover
            .selected_obligation_closeout_rows()
            .iter()
            .filter(|row| !row.enforcement_receipt_digest().is_empty())
            .count();
        let executor_row_count = cutover
            .selected_obligation_closeout_rows()
            .iter()
            .filter(|row| row.query_execution_status() == "executed")
            .count();
        let graph_read_receipt_count = cutover
            .selected_obligation_closeout_rows()
            .iter()
            .filter(|row| !row.query_execution_row_digest().is_empty())
            .count();
        let support_pin_count = usize::from(!cutover.support_pin_digest().is_empty());
        let execution_backed_adoption_proof_count =
            usize::from(!cutover.execution_proof_digest().is_empty());
        let residue_manifest_count = usize::from(!residue_audit.report_digest().is_empty());
        let budget_denial_count = cutover
            .selected_obligation_closeout_rows()
            .iter()
            .filter(|row| row.query_execution_status() == "budget-exceeded")
            .count();
        let mut digest_parts = vec![
            "worth-topo-milestone-nine-closeout-counters-v1".to_string(),
            format!("selected-obligation:{selected_obligation_count}"),
            format!("worth-family:{worth_family_count}"),
            format!("enforcement-receipt:{enforcement_receipt_count}"),
            format!("graph-read-receipt:{graph_read_receipt_count}"),
            format!("budget-denial:{budget_denial_count}"),
            format!("support-pin:{support_pin_count}"),
            format!("executor-row:{executor_row_count}"),
            format!("adoption-proof:{execution_backed_adoption_proof_count}"),
            format!("residue-manifest:{residue_manifest_count}"),
            format!("deletion-ledger-row:{}", deletion_ledger.rows().len()),
            format!(
                "capped-residue-row:{}",
                residue_audit.capped_residue_count()
            ),
            format!("stale-residue-row:{}", residue_audit.stale_residue_count()),
            format!(
                "uncapped-authority:{}",
                residue_audit.uncapped_authority_count()
            ),
            format!(
                "source-firewall-violation:{}",
                source_firewall.violations().len()
            ),
            format!(
                "scanned-source-file:{}",
                source_firewall.scanned_file_count()
            ),
            format!(
                "whole-view-certification-only:{}",
                deletion_ledger.whole_view_certification_only_count()
            ),
        ];
        digest_parts.extend(
            cutover
                .selected_obligation_closeout_rows()
                .iter()
                .map(|row| format!("row:{}", row.row_digest())),
        );
        Self {
            selected_obligation_count,
            worth_family_count,
            enforcement_receipt_count,
            graph_read_receipt_count,
            budget_denial_count,
            support_pin_count,
            executor_row_count,
            execution_backed_adoption_proof_count,
            residue_manifest_count,
            deletion_ledger_row_count: deletion_ledger.rows().len(),
            capped_residue_row_count: residue_audit.capped_residue_count(),
            stale_residue_row_count: residue_audit.stale_residue_count(),
            uncapped_authority_count: residue_audit.uncapped_authority_count(),
            source_firewall_violation_count: source_firewall.violations().len(),
            scanned_source_file_count: source_firewall.scanned_file_count(),
            whole_view_certification_only_count: deletion_ledger
                .whole_view_certification_only_count(),
            counters_digest: digest_parts.join("|"),
        }
    }

    pub const fn selected_obligation_count(&self) -> usize {
        self.selected_obligation_count
    }

    pub const fn worth_family_count(&self) -> usize {
        self.worth_family_count
    }

    pub const fn enforcement_receipt_count(&self) -> usize {
        self.enforcement_receipt_count
    }

    pub const fn graph_read_receipt_count(&self) -> usize {
        self.graph_read_receipt_count
    }

    pub const fn budget_denial_count(&self) -> usize {
        self.budget_denial_count
    }

    pub const fn support_pin_count(&self) -> usize {
        self.support_pin_count
    }

    pub const fn executor_row_count(&self) -> usize {
        self.executor_row_count
    }

    pub const fn execution_backed_adoption_proof_count(&self) -> usize {
        self.execution_backed_adoption_proof_count
    }

    pub const fn residue_manifest_count(&self) -> usize {
        self.residue_manifest_count
    }

    pub const fn deletion_ledger_row_count(&self) -> usize {
        self.deletion_ledger_row_count
    }

    pub const fn capped_residue_row_count(&self) -> usize {
        self.capped_residue_row_count
    }

    pub const fn stale_residue_row_count(&self) -> usize {
        self.stale_residue_row_count
    }

    pub const fn uncapped_authority_count(&self) -> usize {
        self.uncapped_authority_count
    }

    pub const fn source_firewall_violation_count(&self) -> usize {
        self.source_firewall_violation_count
    }

    pub const fn scanned_source_file_count(&self) -> usize {
        self.scanned_source_file_count
    }

    pub const fn whole_view_certification_only_count(&self) -> usize {
        self.whole_view_certification_only_count
    }

    pub fn counters_digest(&self) -> &str {
        &self.counters_digest
    }
}
