use crate::validator_invariant_catalog::milestone_nine_closeout::{
    WorthTopologyMilestoneNineCloseoutCounters, WorthTopologyMilestoneNineDeletionLedgerReport,
    WorthTopologyMilestoneNineResidueAuditReport, WorthTopologyMilestoneNineSourceFirewallReport,
};
use crate::validator_invariant_catalog::WorthTopologyOperatorCertificationCutoverCloseout;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTopologyMilestoneNinePublicProof {
    closeout_digest: String,
    deletion_ledger_digest: String,
    residue_audit_digest: String,
    source_firewall_digest: String,
    counters_digest: String,
    selected_obligation_row_digests: Vec<String>,
    query_selected_obligation_digests: Vec<String>,
    enforcement_receipt_digests: Vec<String>,
    execution_proof_digest: String,
    support_pin_digest: String,
    support_matrix_digest: String,
    adoption_manifest_digest: String,
    residue_manifest_digest: String,
    local_ceremony_audit_digest: String,
    old_authority_closed: bool,
    ordinary_operator_paths_query_backed: bool,
    proof_digest: String,
}

impl WorthTopologyMilestoneNinePublicProof {
    pub(in crate::validator_invariant_catalog) fn from_parts(
        closeout_digest: &str,
        cutover: &WorthTopologyOperatorCertificationCutoverCloseout,
        deletion_ledger: &WorthTopologyMilestoneNineDeletionLedgerReport,
        residue_audit: &WorthTopologyMilestoneNineResidueAuditReport,
        source_firewall: &WorthTopologyMilestoneNineSourceFirewallReport,
        counters: &WorthTopologyMilestoneNineCloseoutCounters,
    ) -> Self {
        let selected_obligation_row_digests = cutover
            .selected_obligation_closeout_rows()
            .iter()
            .map(|row| row.row_digest().to_string())
            .collect::<Vec<_>>();
        let query_selected_obligation_digests = cutover
            .selected_obligation_closeout_rows()
            .iter()
            .map(|row| row.query_rule_identity_digest().to_string())
            .collect::<Vec<_>>();
        let enforcement_receipt_digests = cutover
            .selected_obligation_closeout_rows()
            .iter()
            .map(|row| row.enforcement_receipt_digest().to_string())
            .collect::<Vec<_>>();
        let old_authority_closed = deletion_ledger.closed_old_authority_count()
            == deletion_ledger.rows().len()
            && residue_audit.stale_residue_count() == 0
            && residue_audit.uncapped_authority_count() == 0
            && source_firewall.is_clean();
        let ordinary_operator_paths_query_backed =
            counters.selected_obligation_count() > 0 && counters.executor_row_count() > 0;
        let mut digest_parts = vec![
            "worth-topo-milestone-nine-public-proof-v1".to_string(),
            format!("closeout:{closeout_digest}"),
            format!("deletion-ledger:{}", deletion_ledger.report_digest()),
            format!("residue-audit:{}", residue_audit.report_digest()),
            format!("source-firewall:{}", source_firewall.report_digest()),
            format!("counters:{}", counters.counters_digest()),
            format!("execution-proof:{}", cutover.execution_proof_digest()),
            format!("support-pin:{}", cutover.support_pin_digest()),
            format!("support-matrix:{}", cutover.support_matrix_digest()),
            format!(
                "adoption-manifest:{}",
                cutover.execution_backed_adoption_manifest_digest()
            ),
            format!("residue-manifest:{}", cutover.residue_manifest_digest()),
            format!(
                "local-ceremony-audit:{}",
                cutover.local_ceremony_audit_digest()
            ),
            format!("old-authority-closed:{old_authority_closed}"),
            format!("ordinary-operator-paths-query-backed:{ordinary_operator_paths_query_backed}"),
        ];
        digest_parts.extend(
            selected_obligation_row_digests
                .iter()
                .map(|digest| format!("selected-obligation-row:{digest}")),
        );
        digest_parts.extend(
            query_selected_obligation_digests
                .iter()
                .map(|digest| format!("query-selected-obligation:{digest}")),
        );
        digest_parts.extend(
            enforcement_receipt_digests
                .iter()
                .map(|digest| format!("enforcement-receipt:{digest}")),
        );
        Self {
            closeout_digest: closeout_digest.to_string(),
            deletion_ledger_digest: deletion_ledger.report_digest().to_string(),
            residue_audit_digest: residue_audit.report_digest().to_string(),
            source_firewall_digest: source_firewall.report_digest().to_string(),
            counters_digest: counters.counters_digest().to_string(),
            selected_obligation_row_digests,
            query_selected_obligation_digests,
            enforcement_receipt_digests,
            execution_proof_digest: cutover.execution_proof_digest().to_string(),
            support_pin_digest: cutover.support_pin_digest().to_string(),
            support_matrix_digest: cutover.support_matrix_digest().to_string(),
            adoption_manifest_digest: cutover
                .execution_backed_adoption_manifest_digest()
                .to_string(),
            residue_manifest_digest: cutover.residue_manifest_digest().to_string(),
            local_ceremony_audit_digest: cutover.local_ceremony_audit_digest().to_string(),
            old_authority_closed,
            ordinary_operator_paths_query_backed,
            proof_digest: digest_parts.join("|"),
        }
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }

    pub fn deletion_ledger_digest(&self) -> &str {
        &self.deletion_ledger_digest
    }

    pub fn residue_audit_digest(&self) -> &str {
        &self.residue_audit_digest
    }

    pub fn source_firewall_digest(&self) -> &str {
        &self.source_firewall_digest
    }

    pub fn counters_digest(&self) -> &str {
        &self.counters_digest
    }

    pub fn selected_obligation_row_digests(&self) -> &[String] {
        &self.selected_obligation_row_digests
    }

    pub fn query_selected_obligation_digests(&self) -> &[String] {
        &self.query_selected_obligation_digests
    }

    pub fn enforcement_receipt_digests(&self) -> &[String] {
        &self.enforcement_receipt_digests
    }

    pub fn execution_proof_digest(&self) -> &str {
        &self.execution_proof_digest
    }

    pub fn support_pin_digest(&self) -> &str {
        &self.support_pin_digest
    }

    pub fn support_matrix_digest(&self) -> &str {
        &self.support_matrix_digest
    }

    pub fn adoption_manifest_digest(&self) -> &str {
        &self.adoption_manifest_digest
    }

    pub fn residue_manifest_digest(&self) -> &str {
        &self.residue_manifest_digest
    }

    pub fn local_ceremony_audit_digest(&self) -> &str {
        &self.local_ceremony_audit_digest
    }

    pub const fn old_authority_closed(&self) -> bool {
        self.old_authority_closed
    }

    pub const fn ordinary_operator_paths_query_backed(&self) -> bool {
        self.ordinary_operator_paths_query_backed
    }

    pub fn proof_digest(&self) -> &str {
        &self.proof_digest
    }
}
