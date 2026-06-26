use crate::validator_invariant_catalog::operator_certification_cutover::{
    WorthTopologyOperatorCertificationCutoverCounters,
    WorthTopologyOperatorCertificationCutoverDenial,
    WorthTopologyOperatorCertificationCutoverDenialKind,
    WorthTopologyOperatorCertificationCutoverPhaseEightSeed,
    WorthTopologyOperatorCertificationCutoverSourceFirewallReport,
    WorthTopologyOperatorCertificationOldExpectationResidueReport,
    WorthTopologyOperatorSelectedObligationCloseoutRow,
    WorthTopologyOperatorSelectedObligationSupportPostureRow,
};
use crate::validator_invariant_catalog::{
    WorthTopologyLegalityCatalogError, WorthTopologySelectedGraphObligationEnforcementCloseout,
};

#[derive(Clone, Debug)]
pub struct WorthTopologyOperatorCertificationCutoverCloseout {
    phase_six_seed_digest: String,
    phase_seven_enforcement_seed_digest: String,
    selected_plan_digest: String,
    routing_closure_digest: String,
    query_execution_envelope_digest: String,
    execution_backed_adoption_manifest_digest: String,
    support_pin_digest: String,
    support_matrix_digest: String,
    residue_manifest_digest: String,
    local_ceremony_audit_digest: String,
    in_memory_proof_digest: String,
    execution_proof_digest: String,
    selected_obligation_closeout_rows: Vec<WorthTopologyOperatorSelectedObligationCloseoutRow>,
    support_posture_rows: Vec<WorthTopologyOperatorSelectedObligationSupportPostureRow>,
    old_expectation_residue: WorthTopologyOperatorCertificationOldExpectationResidueReport,
    source_firewall: WorthTopologyOperatorCertificationCutoverSourceFirewallReport,
    counters: WorthTopologyOperatorCertificationCutoverCounters,
    phase_eight_seed: WorthTopologyOperatorCertificationCutoverPhaseEightSeed,
    closeout_digest: String,
}

impl WorthTopologyOperatorCertificationCutoverCloseout {
    pub fn from_selected_graph_obligation_enforcement(
        enforcement_closeout: &WorthTopologySelectedGraphObligationEnforcementCloseout,
    ) -> Result<Self, WorthTopologyLegalityCatalogError> {
        let old_expectation_residue =
            WorthTopologyOperatorCertificationOldExpectationResidueReport::current_capped_migration_residue();
        let source_firewall =
            WorthTopologyOperatorCertificationCutoverSourceFirewallReport::current_with_capped_residue(
                &old_expectation_residue,
            );
        Self::from_selected_graph_obligation_enforcement_with_reports(
            enforcement_closeout,
            source_firewall,
            old_expectation_residue,
        )
    }

    pub(in crate::validator_invariant_catalog) fn from_selected_graph_obligation_enforcement_with_reports(
        enforcement_closeout: &WorthTopologySelectedGraphObligationEnforcementCloseout,
        source_firewall: WorthTopologyOperatorCertificationCutoverSourceFirewallReport,
        old_expectation_residue: WorthTopologyOperatorCertificationOldExpectationResidueReport,
    ) -> Result<Self, WorthTopologyLegalityCatalogError> {
        if enforcement_closeout.enforcement_receipts().is_empty() {
            return Err(cutover_denial(
                WorthTopologyOperatorCertificationCutoverDenialKind::EmptyEnforcementReceiptSet,
                enforcement_closeout.closeout_digest(),
                "operator certification cutover requires selected graph obligation receipts",
            ));
        }
        if !source_firewall.is_clean() {
            return Err(cutover_denial(
                WorthTopologyOperatorCertificationCutoverDenialKind::SourceFirewallViolation,
                source_firewall.report_digest(),
                "old validator expectation authority appeared in the operator cutover proof path",
            ));
        }
        if !old_expectation_residue.is_capped() {
            return Err(cutover_denial(
                WorthTopologyOperatorCertificationCutoverDenialKind::UncappedOldExpectationAuthority,
                old_expectation_residue.report_digest(),
                "old validator expectation authority must be capped as comparison or deletion proof",
            ));
        }
        let selected_obligation_closeout_rows = enforcement_closeout
            .enforcement_receipts()
            .iter()
            .map(WorthTopologyOperatorSelectedObligationCloseoutRow::from_enforcement_receipt)
            .collect::<Vec<_>>();
        let support_posture_rows =
            WorthTopologyOperatorSelectedObligationSupportPostureRow::from_closeout_rows(
                &selected_obligation_closeout_rows,
            );
        let counters = WorthTopologyOperatorCertificationCutoverCounters::from_rows(
            &selected_obligation_closeout_rows,
            &support_posture_rows,
            &old_expectation_residue,
            &source_firewall,
        );
        let closeout_digest = cutover_closeout_digest(
            enforcement_closeout
                .phase_seven_seed()
                .phase_six_seed_digest(),
            enforcement_closeout.phase_seven_seed().seed_digest(),
            enforcement_closeout.selected_plan_digest(),
            enforcement_closeout
                .phase_seven_seed()
                .routing_closure_digest(),
            enforcement_closeout.query_execution_envelope_digest(),
            enforcement_closeout
                .phase_seven_seed()
                .execution_backed_adoption_manifest_digest(),
            enforcement_closeout.phase_seven_seed().support_pin_digest(),
            enforcement_closeout
                .phase_seven_seed()
                .support_matrix_digest(),
            enforcement_closeout
                .phase_seven_seed()
                .residue_manifest_digest(),
            enforcement_closeout
                .phase_seven_seed()
                .local_ceremony_audit_digest(),
            enforcement_closeout
                .phase_seven_seed()
                .in_memory_proof_digest(),
            enforcement_closeout
                .phase_seven_seed()
                .execution_proof_digest(),
            old_expectation_residue.report_digest(),
            source_firewall.report_digest(),
            counters.counters_digest(),
            &selected_obligation_closeout_rows,
            &support_posture_rows,
        );
        let phase_eight_seed =
            WorthTopologyOperatorCertificationCutoverPhaseEightSeed::from_closeout(
                enforcement_closeout.phase_seven_seed().seed_digest(),
                &closeout_digest,
                &counters,
                &selected_obligation_closeout_rows,
            );
        Ok(Self {
            phase_six_seed_digest: enforcement_closeout
                .phase_seven_seed()
                .phase_six_seed_digest()
                .to_string(),
            phase_seven_enforcement_seed_digest: enforcement_closeout
                .phase_seven_seed()
                .seed_digest()
                .to_string(),
            selected_plan_digest: enforcement_closeout.selected_plan_digest().to_string(),
            routing_closure_digest: enforcement_closeout
                .phase_seven_seed()
                .routing_closure_digest()
                .to_string(),
            query_execution_envelope_digest: enforcement_closeout
                .query_execution_envelope_digest()
                .to_string(),
            execution_backed_adoption_manifest_digest: enforcement_closeout
                .phase_seven_seed()
                .execution_backed_adoption_manifest_digest()
                .to_string(),
            support_pin_digest: enforcement_closeout
                .phase_seven_seed()
                .support_pin_digest()
                .to_string(),
            support_matrix_digest: enforcement_closeout
                .phase_seven_seed()
                .support_matrix_digest()
                .to_string(),
            residue_manifest_digest: enforcement_closeout
                .phase_seven_seed()
                .residue_manifest_digest()
                .to_string(),
            local_ceremony_audit_digest: enforcement_closeout
                .phase_seven_seed()
                .local_ceremony_audit_digest()
                .to_string(),
            in_memory_proof_digest: enforcement_closeout
                .phase_seven_seed()
                .in_memory_proof_digest()
                .to_string(),
            execution_proof_digest: enforcement_closeout
                .phase_seven_seed()
                .execution_proof_digest()
                .to_string(),
            selected_obligation_closeout_rows,
            support_posture_rows,
            old_expectation_residue,
            source_firewall,
            counters,
            phase_eight_seed,
            closeout_digest,
        })
    }

    pub fn selected_obligation_closeout_rows(
        &self,
    ) -> &[WorthTopologyOperatorSelectedObligationCloseoutRow] {
        &self.selected_obligation_closeout_rows
    }

    pub fn support_posture_rows(
        &self,
    ) -> &[WorthTopologyOperatorSelectedObligationSupportPostureRow] {
        &self.support_posture_rows
    }

    pub const fn old_expectation_residue(
        &self,
    ) -> &WorthTopologyOperatorCertificationOldExpectationResidueReport {
        &self.old_expectation_residue
    }

    pub const fn source_firewall(
        &self,
    ) -> &WorthTopologyOperatorCertificationCutoverSourceFirewallReport {
        &self.source_firewall
    }

    pub const fn counters(&self) -> &WorthTopologyOperatorCertificationCutoverCounters {
        &self.counters
    }

    pub const fn phase_eight_seed(
        &self,
    ) -> &WorthTopologyOperatorCertificationCutoverPhaseEightSeed {
        &self.phase_eight_seed
    }

    pub fn phase_seven_enforcement_seed_digest(&self) -> &str {
        &self.phase_seven_enforcement_seed_digest
    }

    pub fn phase_six_seed_digest(&self) -> &str {
        &self.phase_six_seed_digest
    }

    pub fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }

    pub fn routing_closure_digest(&self) -> &str {
        &self.routing_closure_digest
    }

    pub fn query_execution_envelope_digest(&self) -> &str {
        &self.query_execution_envelope_digest
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

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }

    #[cfg(test)]
    pub(in crate::validator_invariant_catalog) fn without_execution_proof_for_tests(&self) -> Self {
        let mut cutover = self.clone();
        cutover.execution_proof_digest.clear();
        cutover
    }

    #[cfg(test)]
    pub(in crate::validator_invariant_catalog) fn without_selected_rows_for_tests(&self) -> Self {
        let mut cutover = self.clone();
        cutover.selected_obligation_closeout_rows.clear();
        cutover
    }

    #[cfg(test)]
    pub(in crate::validator_invariant_catalog) fn with_selected_rows_for_tests(
        &self,
        rows: Vec<WorthTopologyOperatorSelectedObligationCloseoutRow>,
    ) -> Self {
        let mut cutover = self.clone();
        cutover.selected_obligation_closeout_rows = rows;
        cutover
    }
}

fn cutover_closeout_digest(
    phase_six_seed_digest: &str,
    phase_seven_enforcement_seed_digest: &str,
    selected_plan_digest: &str,
    routing_closure_digest: &str,
    query_execution_envelope_digest: &str,
    execution_backed_adoption_manifest_digest: &str,
    support_pin_digest: &str,
    support_matrix_digest: &str,
    residue_manifest_digest: &str,
    local_ceremony_audit_digest: &str,
    in_memory_proof_digest: &str,
    execution_proof_digest: &str,
    old_expectation_residue_digest: &str,
    source_firewall_digest: &str,
    counters_digest: &str,
    rows: &[WorthTopologyOperatorSelectedObligationCloseoutRow],
    support_rows: &[WorthTopologyOperatorSelectedObligationSupportPostureRow],
) -> String {
    let mut digest_parts = vec![
        "worth-topo-operator-certification-cutover-closeout-v1".to_string(),
        format!("phase-six-seed:{phase_six_seed_digest}"),
        format!("phase-seven-enforcement-seed:{phase_seven_enforcement_seed_digest}"),
        format!("selected-plan:{selected_plan_digest}"),
        format!("routing-closure:{routing_closure_digest}"),
        format!("query-envelope:{query_execution_envelope_digest}"),
        format!("adoption-manifest:{execution_backed_adoption_manifest_digest}"),
        format!("support-pin:{support_pin_digest}"),
        format!("support-matrix:{support_matrix_digest}"),
        format!("residue-manifest:{residue_manifest_digest}"),
        format!("local-ceremony-audit:{local_ceremony_audit_digest}"),
        format!("in-memory-proof:{in_memory_proof_digest}"),
        format!("execution-proof:{execution_proof_digest}"),
        format!("old-expectation-residue:{old_expectation_residue_digest}"),
        format!("source-firewall:{source_firewall_digest}"),
        format!("counters:{counters_digest}"),
    ];
    digest_parts.extend(rows.iter().map(|row| format!("row:{}", row.row_digest())));
    digest_parts.extend(
        support_rows
            .iter()
            .map(|row| format!("support:{}", row.support_posture_row_digest())),
    );
    digest_parts.join("|")
}

fn cutover_denial(
    kind: WorthTopologyOperatorCertificationCutoverDenialKind,
    authority_digest: impl Into<String>,
    message: impl Into<String>,
) -> WorthTopologyLegalityCatalogError {
    WorthTopologyLegalityCatalogError::OperatorCertificationCutover(
        WorthTopologyOperatorCertificationCutoverDenial::new(kind, authority_digest, message),
    )
}
