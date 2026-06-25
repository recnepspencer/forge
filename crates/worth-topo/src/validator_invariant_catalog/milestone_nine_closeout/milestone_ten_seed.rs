use crate::validator_invariant_catalog::milestone_nine_closeout::{
    stable_digest, WorthTopologyMilestoneNineCloseoutCounters,
    WorthTopologyMilestoneNineDeletionLedgerReport, WorthTopologyMilestoneNineResidueAuditReport,
    WorthTopologyMilestoneNineSourceFirewallReport,
};
use crate::validator_invariant_catalog::WorthTopologyOperatorCertificationCutoverCloseout;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTopologyMilestoneTenSeed {
    phase_nine_closeout_digest: String,
    phase_six_seed_digest: String,
    phase_eight_cutover_seed_digest: String,
    phase_seven_enforcement_seed_digest: String,
    routing_closure_digest: String,
    selected_obligation_row_digests: Vec<String>,
    query_selected_obligation_digests: Vec<String>,
    worth_family_identity_digests: Vec<String>,
    enforcement_receipt_digests: Vec<String>,
    query_execution_envelope_digest: String,
    query_execution_row_digests: Vec<String>,
    support_posture_row_digests: Vec<String>,
    support_posture_digests: Vec<String>,
    diagnostic_witness_digest_summary: Vec<String>,
    execution_backed_adoption_manifest_digest: String,
    support_pin_digest: String,
    support_matrix_digest: String,
    residue_manifest_digest: String,
    local_ceremony_audit_digest: String,
    in_memory_proof_digest: String,
    execution_proof_digest: String,
    deletion_ledger_digest: String,
    residue_audit_digest: String,
    source_firewall_digest: String,
    counters_digest: String,
    claims_invalidation_planning: bool,
    seed_digest: String,
}

impl WorthTopologyMilestoneTenSeed {
    pub(in crate::validator_invariant_catalog) fn from_closeout_parts(
        phase_nine_closeout_digest: &str,
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
        let worth_family_identity_digests = cutover
            .selected_obligation_closeout_rows()
            .iter()
            .map(|row| row.worth_family_identity_digest().to_string())
            .collect::<Vec<_>>();
        let enforcement_receipt_digests = cutover
            .selected_obligation_closeout_rows()
            .iter()
            .map(|row| row.enforcement_receipt_digest().to_string())
            .collect::<Vec<_>>();
        let query_execution_row_digests = cutover
            .selected_obligation_closeout_rows()
            .iter()
            .map(|row| row.query_execution_row_digest().to_string())
            .collect::<Vec<_>>();
        let support_posture_row_digests = cutover
            .support_posture_rows()
            .iter()
            .map(|row| row.support_posture_row_digest().to_string())
            .collect::<Vec<_>>();
        let support_posture_digests = cutover
            .selected_obligation_closeout_rows()
            .iter()
            .map(|row| row.query_support_posture_digest().to_string())
            .collect::<Vec<_>>();
        let diagnostic_witness_digest_summary = cutover
            .selected_obligation_closeout_rows()
            .iter()
            .filter_map(|row| row.diagnostic_witness_digest().map(str::to_string))
            .collect::<Vec<_>>();
        let claims_invalidation_planning = false;
        let mut digest_parts = vec![
            "worth-topo-milestone-ten-seed-v1".to_string(),
            format!("phase-nine-closeout:{phase_nine_closeout_digest}"),
            format!("phase-six-seed:{}", cutover.phase_six_seed_digest()),
            format!(
                "phase-eight-cutover-seed:{}",
                cutover.phase_eight_seed().seed_digest()
            ),
            format!(
                "phase-seven-enforcement-seed:{}",
                cutover.phase_seven_enforcement_seed_digest()
            ),
            format!("routing-closure:{}", cutover.routing_closure_digest()),
            format!(
                "query-execution-envelope:{}",
                cutover.query_execution_envelope_digest()
            ),
            format!(
                "adoption-manifest:{}",
                cutover.execution_backed_adoption_manifest_digest()
            ),
            format!("support-pin:{}", cutover.support_pin_digest()),
            format!("support-matrix:{}", cutover.support_matrix_digest()),
            format!("residue-manifest:{}", cutover.residue_manifest_digest()),
            format!(
                "local-ceremony-audit:{}",
                cutover.local_ceremony_audit_digest()
            ),
            format!("in-memory-proof:{}", cutover.in_memory_proof_digest()),
            format!("execution-proof:{}", cutover.execution_proof_digest()),
            format!("deletion-ledger:{}", deletion_ledger.report_digest()),
            format!("residue-audit:{}", residue_audit.report_digest()),
            format!("source-firewall:{}", source_firewall.report_digest()),
            format!("counters:{}", counters.counters_digest()),
            format!("claims-invalidation-planning:{claims_invalidation_planning}"),
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
        digest_parts.extend(
            query_execution_row_digests
                .iter()
                .map(|digest| format!("query-execution-row:{digest}")),
        );
        digest_parts.extend(
            support_posture_digests
                .iter()
                .map(|digest| format!("support-posture:{digest}")),
        );
        digest_parts.extend(
            diagnostic_witness_digest_summary
                .iter()
                .map(|digest| format!("diagnostic-witness:{digest}")),
        );
        Self {
            phase_nine_closeout_digest: phase_nine_closeout_digest.to_string(),
            phase_six_seed_digest: cutover.phase_six_seed_digest().to_string(),
            phase_eight_cutover_seed_digest: cutover.phase_eight_seed().seed_digest().to_string(),
            phase_seven_enforcement_seed_digest: cutover
                .phase_seven_enforcement_seed_digest()
                .to_string(),
            routing_closure_digest: cutover.routing_closure_digest().to_string(),
            selected_obligation_row_digests,
            query_selected_obligation_digests,
            worth_family_identity_digests,
            enforcement_receipt_digests,
            query_execution_envelope_digest: cutover.query_execution_envelope_digest().to_string(),
            query_execution_row_digests,
            support_posture_row_digests,
            support_posture_digests,
            diagnostic_witness_digest_summary,
            execution_backed_adoption_manifest_digest: cutover
                .execution_backed_adoption_manifest_digest()
                .to_string(),
            support_pin_digest: cutover.support_pin_digest().to_string(),
            support_matrix_digest: cutover.support_matrix_digest().to_string(),
            residue_manifest_digest: cutover.residue_manifest_digest().to_string(),
            local_ceremony_audit_digest: cutover.local_ceremony_audit_digest().to_string(),
            in_memory_proof_digest: cutover.in_memory_proof_digest().to_string(),
            execution_proof_digest: cutover.execution_proof_digest().to_string(),
            deletion_ledger_digest: deletion_ledger.report_digest().to_string(),
            residue_audit_digest: residue_audit.report_digest().to_string(),
            source_firewall_digest: source_firewall.report_digest().to_string(),
            counters_digest: counters.counters_digest().to_string(),
            claims_invalidation_planning,
            seed_digest: stable_digest(&digest_parts),
        }
    }

    pub fn phase_nine_closeout_digest(&self) -> &str {
        &self.phase_nine_closeout_digest
    }

    pub fn phase_six_seed_digest(&self) -> &str {
        &self.phase_six_seed_digest
    }

    pub fn phase_eight_cutover_seed_digest(&self) -> &str {
        &self.phase_eight_cutover_seed_digest
    }

    pub fn phase_seven_enforcement_seed_digest(&self) -> &str {
        &self.phase_seven_enforcement_seed_digest
    }

    pub fn routing_closure_digest(&self) -> &str {
        &self.routing_closure_digest
    }

    pub fn selected_obligation_row_digests(&self) -> &[String] {
        &self.selected_obligation_row_digests
    }

    pub fn query_selected_obligation_digests(&self) -> &[String] {
        &self.query_selected_obligation_digests
    }

    pub fn worth_family_identity_digests(&self) -> &[String] {
        &self.worth_family_identity_digests
    }

    pub fn enforcement_receipt_digests(&self) -> &[String] {
        &self.enforcement_receipt_digests
    }

    pub fn query_execution_envelope_digest(&self) -> &str {
        &self.query_execution_envelope_digest
    }

    pub fn query_execution_row_digests(&self) -> &[String] {
        &self.query_execution_row_digests
    }

    pub fn support_posture_row_digests(&self) -> &[String] {
        &self.support_posture_row_digests
    }

    pub fn support_posture_digests(&self) -> &[String] {
        &self.support_posture_digests
    }

    pub fn diagnostic_witness_digest_summary(&self) -> &[String] {
        &self.diagnostic_witness_digest_summary
    }

    pub fn execution_backed_adoption_manifest_digest(&self) -> &str {
        &self.execution_backed_adoption_manifest_digest
    }

    pub fn support_pin_digest(&self) -> &str {
        &self.support_pin_digest
    }

    pub fn support_matrix_digest(&self) -> &str {
        &self.support_matrix_digest
    }

    pub fn residue_manifest_digest(&self) -> &str {
        &self.residue_manifest_digest
    }

    pub fn local_ceremony_audit_digest(&self) -> &str {
        &self.local_ceremony_audit_digest
    }

    pub fn in_memory_proof_digest(&self) -> &str {
        &self.in_memory_proof_digest
    }

    pub fn execution_proof_digest(&self) -> &str {
        &self.execution_proof_digest
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

    pub const fn claims_invalidation_planning(&self) -> bool {
        self.claims_invalidation_planning
    }

    pub fn seed_digest(&self) -> &str {
        &self.seed_digest
    }
}
