use crate::graph_read_access_plan_adoption::{
    WorthGraphReadAccessBatchAccountingReport, WorthGraphReadAccessBoundedExecutionContract,
    WorthGraphReadAccessCounterAccountingReport, WorthGraphReadAccessSliceCutoverProof,
};

use super::closeout_counters::WorthGraphReadAccessPlanAdoptionCloseoutCounters;
use super::proof_exports::{
    WorthGraphReadAccessPlanAdoptionDeletionExport, WorthGraphReadAccessPlanAdoptionPostureExport,
    WorthGraphReadAccessPlanAdoptionReceiptExport, WorthGraphReadAccessPlanAdoptionResidueExport,
    WorthGraphReadAccessPlanAdoptionSourceFirewallExport,
};
use super::stable_digest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessPlanAdoptionMilestoneNineSeed {
    milestone_eight_closeout_digest: String,
    receipt_export: WorthGraphReadAccessPlanAdoptionReceiptExport,
    posture_export: WorthGraphReadAccessPlanAdoptionPostureExport,
    closeout_counters: WorthGraphReadAccessPlanAdoptionCloseoutCounters,
    counter_accounting_report: WorthGraphReadAccessCounterAccountingReport,
    batch_accounting_report: WorthGraphReadAccessBatchAccountingReport,
    deletion_export: WorthGraphReadAccessPlanAdoptionDeletionExport,
    residue_export: WorthGraphReadAccessPlanAdoptionResidueExport,
    source_firewall_export: WorthGraphReadAccessPlanAdoptionSourceFirewallExport,
    bounded_execution_contract: WorthGraphReadAccessBoundedExecutionContract,
    phase_four_cutover_proof: WorthGraphReadAccessSliceCutoverProof,
    seed_digest: String,
}

pub(crate) struct WorthGraphReadAccessPlanAdoptionMilestoneNineSeedInput {
    pub milestone_eight_closeout_digest: String,
    pub receipt_export: WorthGraphReadAccessPlanAdoptionReceiptExport,
    pub posture_export: WorthGraphReadAccessPlanAdoptionPostureExport,
    pub closeout_counters: WorthGraphReadAccessPlanAdoptionCloseoutCounters,
    pub counter_accounting_report: WorthGraphReadAccessCounterAccountingReport,
    pub batch_accounting_report: WorthGraphReadAccessBatchAccountingReport,
    pub deletion_export: WorthGraphReadAccessPlanAdoptionDeletionExport,
    pub residue_export: WorthGraphReadAccessPlanAdoptionResidueExport,
    pub source_firewall_export: WorthGraphReadAccessPlanAdoptionSourceFirewallExport,
    pub bounded_execution_contract: WorthGraphReadAccessBoundedExecutionContract,
    pub phase_four_cutover_proof: WorthGraphReadAccessSliceCutoverProof,
}

impl WorthGraphReadAccessPlanAdoptionMilestoneNineSeed {
    pub(in crate::graph_read_access_plan_adoption::phase_eight_public_closeout) fn from_input(
        input: WorthGraphReadAccessPlanAdoptionMilestoneNineSeedInput,
    ) -> Self {
        let seed_digest = stable_digest(&[
            "worth_graph_read_access_plan_adoption_milestone_nine_seed_v1".to_string(),
            format!("closeout:{}", input.milestone_eight_closeout_digest),
            format!("receipt:{}", input.receipt_export.export_digest()),
            format!("posture:{}", input.posture_export.export_digest()),
            format!("counters:{}", input.closeout_counters.counter_digest()),
            format!(
                "counter_report:{}",
                input.counter_accounting_report.report_digest()
            ),
            format!("batch:{}", input.batch_accounting_report.report_digest()),
            format!("deletion:{}", input.deletion_export.export_digest()),
            format!("residue:{}", input.residue_export.export_digest()),
            format!("firewall:{}", input.source_firewall_export.export_digest()),
            format!(
                "bounded_execution:{}",
                input.bounded_execution_contract.contract_digest()
            ),
            format!(
                "phase_four_cutover:{}",
                input.phase_four_cutover_proof.cutover_digest()
            ),
        ]);
        Self {
            milestone_eight_closeout_digest: input.milestone_eight_closeout_digest,
            receipt_export: input.receipt_export,
            posture_export: input.posture_export,
            closeout_counters: input.closeout_counters,
            counter_accounting_report: input.counter_accounting_report,
            batch_accounting_report: input.batch_accounting_report,
            deletion_export: input.deletion_export,
            residue_export: input.residue_export,
            source_firewall_export: input.source_firewall_export,
            bounded_execution_contract: input.bounded_execution_contract,
            phase_four_cutover_proof: input.phase_four_cutover_proof,
            seed_digest,
        }
    }

    pub fn milestone_eight_closeout_digest(&self) -> &str {
        &self.milestone_eight_closeout_digest
    }

    pub const fn receipts(&self) -> &WorthGraphReadAccessPlanAdoptionReceiptExport {
        &self.receipt_export
    }

    pub const fn postures(&self) -> &WorthGraphReadAccessPlanAdoptionPostureExport {
        &self.posture_export
    }

    pub const fn counters(&self) -> &WorthGraphReadAccessPlanAdoptionCloseoutCounters {
        &self.closeout_counters
    }

    pub const fn counter_accounting_report(&self) -> &WorthGraphReadAccessCounterAccountingReport {
        &self.counter_accounting_report
    }

    pub const fn batch_accounting_report(&self) -> &WorthGraphReadAccessBatchAccountingReport {
        &self.batch_accounting_report
    }

    pub const fn deletion(&self) -> &WorthGraphReadAccessPlanAdoptionDeletionExport {
        &self.deletion_export
    }

    pub const fn residue(&self) -> &WorthGraphReadAccessPlanAdoptionResidueExport {
        &self.residue_export
    }

    pub const fn source_firewall(&self) -> &WorthGraphReadAccessPlanAdoptionSourceFirewallExport {
        &self.source_firewall_export
    }

    pub const fn bounded_execution_contract(
        &self,
    ) -> &WorthGraphReadAccessBoundedExecutionContract {
        &self.bounded_execution_contract
    }

    pub const fn phase_four_cutover_proof(&self) -> &WorthGraphReadAccessSliceCutoverProof {
        &self.phase_four_cutover_proof
    }

    pub fn seed_digest(&self) -> &str {
        &self.seed_digest
    }

    pub const fn claims_validator_selection(&self) -> bool {
        false
    }
}
