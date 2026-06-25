use crate::graph_read_access_plan_adoption::{
    WorthGraphReadAccessBatchAccountingReport, WorthGraphReadAccessBoundedExecutionContract,
    WorthGraphReadAccessCounterAccountingReport,
    WorthGraphReadAccessExecutionReceiptAccountingPhaseSevenSeed,
    WorthGraphReadAccessPostureCapRow, WorthGraphReadAccessReceiptAccountingReport,
    WorthGraphReadAccessSliceCutoverProof, WorthGraphReadAccessSpatialDensePostureProjection,
    WorthGraphReadAccessSpatialDenseSourceFirewallReport,
};

use super::capped_residue::WorthGraphReadAccessHardDeletionCappedResidueReport;
use super::deletion_proof::WorthGraphReadAccessHardDeletionProofReport;
use super::source_firewall::WorthGraphReadAccessHardDeletionSourceFirewallReport;
use super::stable_digest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessHardDeletionPhaseEightSeed {
    phase_seven_closeout_digest: String,
    phase_six_closeout_digest: String,
    receipt_accounting_report: WorthGraphReadAccessReceiptAccountingReport,
    counter_accounting_report: WorthGraphReadAccessCounterAccountingReport,
    batch_accounting_report: WorthGraphReadAccessBatchAccountingReport,
    prior_source_firewall_report: WorthGraphReadAccessSpatialDenseSourceFirewallReport,
    bounded_execution_contract: WorthGraphReadAccessBoundedExecutionContract,
    phase_four_cutover_proof: WorthGraphReadAccessSliceCutoverProof,
    deletion_proof_report: WorthGraphReadAccessHardDeletionProofReport,
    capped_residue_report: WorthGraphReadAccessHardDeletionCappedResidueReport,
    source_firewall_report: WorthGraphReadAccessHardDeletionSourceFirewallReport,
    posture_projections: Vec<WorthGraphReadAccessSpatialDensePostureProjection>,
    cap_rows: Vec<WorthGraphReadAccessPostureCapRow>,
    seed_digest: String,
}

pub(crate) struct WorthGraphReadAccessHardDeletionPhaseEightSeedInput {
    pub phase_seven_closeout_digest: String,
    pub source_seed: WorthGraphReadAccessExecutionReceiptAccountingPhaseSevenSeed,
    pub deletion_proof_report: WorthGraphReadAccessHardDeletionProofReport,
    pub capped_residue_report: WorthGraphReadAccessHardDeletionCappedResidueReport,
    pub source_firewall_report: WorthGraphReadAccessHardDeletionSourceFirewallReport,
}

impl WorthGraphReadAccessHardDeletionPhaseEightSeed {
    pub(crate) fn from_input(input: WorthGraphReadAccessHardDeletionPhaseEightSeedInput) -> Self {
        let seed_digest = stable_digest(&[
            "worth_graph_read_access_hard_deletion_phase_eight_seed_v1".to_string(),
            format!("phase_seven:{}", input.phase_seven_closeout_digest),
            format!(
                "phase_six:{}",
                input.source_seed.phase_six_closeout_digest()
            ),
            format!(
                "receipt:{}",
                input
                    .source_seed
                    .receipt_accounting_report()
                    .report_digest()
            ),
            format!(
                "counter:{}",
                input
                    .source_seed
                    .counter_accounting_report()
                    .report_digest()
            ),
            format!(
                "batch:{}",
                input.source_seed.batch_accounting_report().report_digest()
            ),
            format!("deletion:{}", input.deletion_proof_report.report_digest()),
            format!("residue:{}", input.capped_residue_report.report_digest()),
            format!("firewall:{}", input.source_firewall_report.report_digest()),
            format!(
                "phase_four_cutover:{}",
                input
                    .source_seed
                    .phase_four_cutover_proof()
                    .cutover_digest()
            ),
        ]);
        Self {
            phase_seven_closeout_digest: input.phase_seven_closeout_digest,
            phase_six_closeout_digest: input.source_seed.phase_six_closeout_digest().to_string(),
            receipt_accounting_report: input.source_seed.receipt_accounting_report().clone(),
            counter_accounting_report: input.source_seed.counter_accounting_report().clone(),
            batch_accounting_report: input.source_seed.batch_accounting_report().clone(),
            prior_source_firewall_report: input.source_seed.source_firewall_report().clone(),
            bounded_execution_contract: input.source_seed.bounded_execution_contract().clone(),
            phase_four_cutover_proof: input.source_seed.phase_four_cutover_proof().clone(),
            deletion_proof_report: input.deletion_proof_report,
            capped_residue_report: input.capped_residue_report,
            source_firewall_report: input.source_firewall_report,
            posture_projections: input.source_seed.posture_projections().to_vec(),
            cap_rows: input.source_seed.cap_rows().to_vec(),
            seed_digest,
        }
    }

    pub fn phase_seven_closeout_digest(&self) -> &str {
        &self.phase_seven_closeout_digest
    }

    pub fn phase_six_closeout_digest(&self) -> &str {
        &self.phase_six_closeout_digest
    }

    pub const fn receipt_accounting_report(&self) -> &WorthGraphReadAccessReceiptAccountingReport {
        &self.receipt_accounting_report
    }

    pub const fn counter_accounting_report(&self) -> &WorthGraphReadAccessCounterAccountingReport {
        &self.counter_accounting_report
    }

    pub const fn batch_accounting_report(&self) -> &WorthGraphReadAccessBatchAccountingReport {
        &self.batch_accounting_report
    }

    pub const fn prior_source_firewall_report(
        &self,
    ) -> &WorthGraphReadAccessSpatialDenseSourceFirewallReport {
        &self.prior_source_firewall_report
    }

    pub const fn bounded_execution_contract(
        &self,
    ) -> &WorthGraphReadAccessBoundedExecutionContract {
        &self.bounded_execution_contract
    }

    pub const fn phase_four_cutover_proof(&self) -> &WorthGraphReadAccessSliceCutoverProof {
        &self.phase_four_cutover_proof
    }

    pub const fn deletion_proof_report(&self) -> &WorthGraphReadAccessHardDeletionProofReport {
        &self.deletion_proof_report
    }

    pub const fn capped_residue_report(
        &self,
    ) -> &WorthGraphReadAccessHardDeletionCappedResidueReport {
        &self.capped_residue_report
    }

    pub const fn source_firewall_report(
        &self,
    ) -> &WorthGraphReadAccessHardDeletionSourceFirewallReport {
        &self.source_firewall_report
    }

    pub fn posture_projections(&self) -> &[WorthGraphReadAccessSpatialDensePostureProjection] {
        &self.posture_projections
    }

    pub fn cap_rows(&self) -> &[WorthGraphReadAccessPostureCapRow] {
        &self.cap_rows
    }

    pub fn seed_digest(&self) -> &str {
        &self.seed_digest
    }

    pub const fn claims_validator_selection(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod adversarial_phase_eight_seed {
    use crate::graph_read_access_plan_adoption::WorthGraphReadAccessReceiptStatus;

    use super::*;

    impl WorthGraphReadAccessHardDeletionPhaseEightSeed {
        pub(crate) fn with_empty_receipt_accounting_for_tests(&self) -> Self {
            let mut seed = self.clone();
            seed.receipt_accounting_report =
                WorthGraphReadAccessReceiptAccountingReport::from_rows_for_tests(Vec::new());
            seed
        }

        pub(crate) fn with_only_pending_admitted_receipts_for_tests(&self) -> Self {
            let row = self
                .receipt_accounting_report
                .rows()
                .first()
                .expect("production Phase 8 seed should have receipt rows")
                .with_status_for_tests(
                    WorthGraphReadAccessReceiptStatus::AdmittedPlanRequiresExecutionReceipt,
                );
            let mut seed = self.clone();
            seed.receipt_accounting_report =
                WorthGraphReadAccessReceiptAccountingReport::from_rows_for_tests(vec![row]);
            seed
        }

        pub(crate) fn with_empty_counter_accounting_for_tests(&self) -> Self {
            let mut seed = self.clone();
            seed.counter_accounting_report =
                WorthGraphReadAccessCounterAccountingReport::from_rows_for_tests(Vec::new());
            seed
        }

        pub(crate) fn with_empty_batch_accounting_for_tests(&self) -> Self {
            let mut seed = self.clone();
            seed.batch_accounting_report =
                WorthGraphReadAccessBatchAccountingReport::empty_for_tests();
            seed
        }

        pub(crate) fn with_lost_batch_receipt_association_for_tests(&self) -> Self {
            let mut seed = self.clone();
            seed.batch_accounting_report = self
                .batch_accounting_report
                .with_lost_per_read_association_for_tests();
            seed
        }

        pub(crate) fn with_caller_owned_graph_work_for_tests(&self) -> Self {
            let mut seed = self.clone();
            seed.counter_accounting_report = self
                .counter_accounting_report
                .with_caller_owned_graph_work_for_tests();
            seed.batch_accounting_report = self
                .batch_accounting_report
                .with_caller_owned_graph_work_for_tests();
            seed
        }

        pub(crate) fn with_unresolved_deletion_for_tests(&self) -> Self {
            let mut seed = self.clone();
            seed.deletion_proof_report =
                self.deletion_proof_report.with_unresolved_path_for_tests();
            seed
        }

        pub(crate) fn with_uncapped_residue_for_tests(&self) -> Self {
            let mut seed = self.clone();
            seed.capped_residue_report =
                self.capped_residue_report.with_uncapped_residue_for_tests();
            seed
        }

        pub(crate) fn with_source_firewall_violation_for_tests(&self) -> Self {
            let mut seed = self.clone();
            seed.source_firewall_report = self.source_firewall_report.with_violation_for_tests();
            seed
        }
    }
}
