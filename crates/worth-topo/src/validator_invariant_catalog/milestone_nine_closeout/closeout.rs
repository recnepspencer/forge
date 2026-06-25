use crate::validator_invariant_catalog::milestone_nine_closeout::{
    stable_digest, WorthTopologyMilestoneNineCloseoutCounters,
    WorthTopologyMilestoneNineCloseoutDenial, WorthTopologyMilestoneNineCloseoutDenialKind,
    WorthTopologyMilestoneNineDeletionLedgerReport, WorthTopologyMilestoneNinePublicProof,
    WorthTopologyMilestoneNineResidueAuditReport, WorthTopologyMilestoneNineSourceFirewallReport,
    WorthTopologyMilestoneTenSeed,
};
use crate::validator_invariant_catalog::{
    WorthTopologyLegalityCatalogError, WorthTopologyOperatorCertificationCutoverCloseout,
    WorthTopologyOperatorCertificationCutoverPhaseEightSeed,
};

#[derive(Clone, Debug)]
pub struct WorthTopologyMilestoneNineCloseout {
    phase_eight_cutover_seed_digest: String,
    operator_cutover_closeout_digest: String,
    deletion_ledger: WorthTopologyMilestoneNineDeletionLedgerReport,
    residue_audit: WorthTopologyMilestoneNineResidueAuditReport,
    source_firewall: WorthTopologyMilestoneNineSourceFirewallReport,
    counters: WorthTopologyMilestoneNineCloseoutCounters,
    public_proof: WorthTopologyMilestoneNinePublicProof,
    milestone_ten_seed: WorthTopologyMilestoneTenSeed,
    closeout_digest: String,
}

impl WorthTopologyMilestoneNineCloseout {
    pub fn from_operator_cutover(
        phase_eight_seed: &WorthTopologyOperatorCertificationCutoverPhaseEightSeed,
        cutover: &WorthTopologyOperatorCertificationCutoverCloseout,
    ) -> Result<Self, WorthTopologyLegalityCatalogError> {
        if phase_eight_seed.seed_digest() != cutover.phase_eight_seed().seed_digest() {
            return Err(milestone_nine_denial(
                WorthTopologyMilestoneNineCloseoutDenialKind::PhaseEightSeedMismatch,
                cutover.phase_eight_seed().seed_digest(),
                "Milestone 9 closeout requires the live Phase 8 seed from the operator cutover",
            ));
        }
        if cutover.selected_obligation_closeout_rows().is_empty() {
            return Err(milestone_nine_denial(
                WorthTopologyMilestoneNineCloseoutDenialKind::EmptySelectedObligationProof,
                cutover.closeout_digest(),
                "Milestone 9 closeout requires selected obligation rows, not selection-only proof",
            ));
        }
        if missing_execution_backed_adoption_proof(cutover) {
            return Err(milestone_nine_denial(
                WorthTopologyMilestoneNineCloseoutDenialKind::MissingExecutionBackedAdoptionProof,
                cutover.closeout_digest(),
                "Milestone 9 closeout requires Consumer Kit adoption, support, residue, and execution proof digests",
            ));
        }
        let deletion_ledger =
            WorthTopologyMilestoneNineDeletionLedgerReport::from_operator_cutover(cutover);
        let residue_audit =
            WorthTopologyMilestoneNineResidueAuditReport::from_cutover_and_deletion_ledger(
                cutover,
                &deletion_ledger,
            );
        let source_firewall =
            WorthTopologyMilestoneNineSourceFirewallReport::current_with_deletion_ledger(
                &deletion_ledger,
            );
        Self::from_operator_cutover_with_reports_for_tests(
            phase_eight_seed,
            cutover,
            deletion_ledger,
            residue_audit,
            source_firewall,
        )
    }

    pub(in crate::validator_invariant_catalog) fn from_operator_cutover_with_reports_for_tests(
        phase_eight_seed: &WorthTopologyOperatorCertificationCutoverPhaseEightSeed,
        cutover: &WorthTopologyOperatorCertificationCutoverCloseout,
        deletion_ledger: WorthTopologyMilestoneNineDeletionLedgerReport,
        residue_audit: WorthTopologyMilestoneNineResidueAuditReport,
        source_firewall: WorthTopologyMilestoneNineSourceFirewallReport,
    ) -> Result<Self, WorthTopologyLegalityCatalogError> {
        if phase_eight_seed.seed_digest() != cutover.phase_eight_seed().seed_digest() {
            return Err(milestone_nine_denial(
                WorthTopologyMilestoneNineCloseoutDenialKind::PhaseEightSeedMismatch,
                cutover.phase_eight_seed().seed_digest(),
                "Milestone 9 closeout requires the live Phase 8 seed from the operator cutover",
            ));
        }
        if cutover.selected_obligation_closeout_rows().is_empty() {
            return Err(milestone_nine_denial(
                WorthTopologyMilestoneNineCloseoutDenialKind::EmptySelectedObligationProof,
                cutover.closeout_digest(),
                "Milestone 9 closeout requires selected obligation rows, not selection-only proof",
            ));
        }
        if missing_execution_backed_adoption_proof(cutover) {
            return Err(milestone_nine_denial(
                WorthTopologyMilestoneNineCloseoutDenialKind::MissingExecutionBackedAdoptionProof,
                cutover.closeout_digest(),
                "Milestone 9 closeout requires Consumer Kit adoption, support, residue, and execution proof digests",
            ));
        }
        if !source_firewall.is_clean() {
            return Err(milestone_nine_denial(
                WorthTopologyMilestoneNineCloseoutDenialKind::SourceFirewallViolation,
                source_firewall.report_digest(),
                "Milestone 9 source firewall found old authority outside deletion-ledger paths",
            ));
        }
        if residue_audit.uncapped_authority_count() > 0 {
            return Err(milestone_nine_denial(
                WorthTopologyMilestoneNineCloseoutDenialKind::UncappedOldAuthority,
                residue_audit.report_digest(),
                "Milestone 9 found old authority that is not capped by deletion evidence",
            ));
        }
        if residue_audit.stale_residue_count() > 0 {
            return Err(milestone_nine_denial(
                WorthTopologyMilestoneNineCloseoutDenialKind::StaleResidueWithoutDeletionLedger,
                residue_audit.report_digest(),
                "Milestone 9 residue must be backed by a deletion-ledger row",
            ));
        }
        let counters = WorthTopologyMilestoneNineCloseoutCounters::from_reports(
            cutover,
            &deletion_ledger,
            &residue_audit,
            &source_firewall,
        );
        if counters.executor_row_count() == 0 || counters.enforcement_receipt_count() == 0 {
            return Err(milestone_nine_denial(
                WorthTopologyMilestoneNineCloseoutDenialKind::SelectionOnlyProof,
                cutover.closeout_digest(),
                "Milestone 9 closeout requires execution-backed operator proof",
            ));
        }
        let closeout_digest = closeout_digest(
            phase_eight_seed.seed_digest(),
            cutover.closeout_digest(),
            &deletion_ledger,
            &residue_audit,
            &source_firewall,
            &counters,
        );
        let public_proof = WorthTopologyMilestoneNinePublicProof::from_parts(
            &closeout_digest,
            cutover,
            &deletion_ledger,
            &residue_audit,
            &source_firewall,
            &counters,
        );
        let milestone_ten_seed = WorthTopologyMilestoneTenSeed::from_closeout_parts(
            &closeout_digest,
            cutover,
            &deletion_ledger,
            &residue_audit,
            &source_firewall,
            &counters,
        );
        Ok(Self {
            phase_eight_cutover_seed_digest: phase_eight_seed.seed_digest().to_string(),
            operator_cutover_closeout_digest: cutover.closeout_digest().to_string(),
            deletion_ledger,
            residue_audit,
            source_firewall,
            counters,
            public_proof,
            milestone_ten_seed,
            closeout_digest,
        })
    }

    pub fn phase_eight_cutover_seed_digest(&self) -> &str {
        &self.phase_eight_cutover_seed_digest
    }

    pub fn operator_cutover_closeout_digest(&self) -> &str {
        &self.operator_cutover_closeout_digest
    }

    pub const fn deletion_ledger(&self) -> &WorthTopologyMilestoneNineDeletionLedgerReport {
        &self.deletion_ledger
    }

    pub const fn residue_audit(&self) -> &WorthTopologyMilestoneNineResidueAuditReport {
        &self.residue_audit
    }

    pub const fn source_firewall(&self) -> &WorthTopologyMilestoneNineSourceFirewallReport {
        &self.source_firewall
    }

    pub const fn counters(&self) -> &WorthTopologyMilestoneNineCloseoutCounters {
        &self.counters
    }

    pub const fn public_proof(&self) -> &WorthTopologyMilestoneNinePublicProof {
        &self.public_proof
    }

    pub const fn milestone_ten_seed(&self) -> &WorthTopologyMilestoneTenSeed {
        &self.milestone_ten_seed
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }
}

fn closeout_digest(
    phase_eight_seed_digest: &str,
    operator_cutover_closeout_digest: &str,
    deletion_ledger: &WorthTopologyMilestoneNineDeletionLedgerReport,
    residue_audit: &WorthTopologyMilestoneNineResidueAuditReport,
    source_firewall: &WorthTopologyMilestoneNineSourceFirewallReport,
    counters: &WorthTopologyMilestoneNineCloseoutCounters,
) -> String {
    stable_digest(&[
        "worth-topo-milestone-nine-closeout-v1".to_string(),
        format!("phase-eight-seed:{phase_eight_seed_digest}"),
        format!("operator-cutover:{operator_cutover_closeout_digest}"),
        format!("deletion-ledger:{}", deletion_ledger.report_digest()),
        format!("residue-audit:{}", residue_audit.report_digest()),
        format!("source-firewall:{}", source_firewall.report_digest()),
        format!("counters:{}", counters.counters_digest()),
    ])
}

fn missing_execution_backed_adoption_proof(
    cutover: &WorthTopologyOperatorCertificationCutoverCloseout,
) -> bool {
    [
        cutover.execution_backed_adoption_manifest_digest(),
        cutover.support_pin_digest(),
        cutover.support_matrix_digest(),
        cutover.residue_manifest_digest(),
        cutover.local_ceremony_audit_digest(),
        cutover.in_memory_proof_digest(),
        cutover.execution_proof_digest(),
    ]
    .iter()
    .any(|digest| digest.is_empty())
}

fn milestone_nine_denial(
    kind: WorthTopologyMilestoneNineCloseoutDenialKind,
    authority_digest: impl Into<String>,
    message: impl Into<String>,
) -> WorthTopologyLegalityCatalogError {
    WorthTopologyLegalityCatalogError::MilestoneNineCloseout(
        WorthTopologyMilestoneNineCloseoutDenial::new(kind, authority_digest, message),
    )
}
