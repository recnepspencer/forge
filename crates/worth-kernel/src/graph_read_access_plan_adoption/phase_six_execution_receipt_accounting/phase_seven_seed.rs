use crate::graph_read_access_plan_adoption::{
    WorthGraphReadAccessBoundedExecutionContract, WorthGraphReadAccessPostureCapRow,
    WorthGraphReadAccessSliceCutoverProof,
    WorthGraphReadAccessSpatialDensePostureProjection,
    WorthGraphReadAccessSpatialDenseSourceFirewallReport,
};

use super::batch_accounting::WorthGraphReadAccessBatchAccountingReport;
use super::counter_accounting::WorthGraphReadAccessCounterAccountingReport;
use super::receipt_accounting::WorthGraphReadAccessReceiptAccountingReport;
use super::stable_digest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessExecutionReceiptAccountingPhaseSevenSeed {
    phase_six_closeout_digest: String,
    phase_five_closeout_digest: String,
    receipt_accounting_report: WorthGraphReadAccessReceiptAccountingReport,
    counter_accounting_report: WorthGraphReadAccessCounterAccountingReport,
    batch_accounting_report: WorthGraphReadAccessBatchAccountingReport,
    source_firewall_report: WorthGraphReadAccessSpatialDenseSourceFirewallReport,
    bounded_execution_contract: WorthGraphReadAccessBoundedExecutionContract,
    phase_four_cutover_proof: WorthGraphReadAccessSliceCutoverProof,
    posture_projections: Vec<WorthGraphReadAccessSpatialDensePostureProjection>,
    cap_rows: Vec<WorthGraphReadAccessPostureCapRow>,
    seed_digest: String,
}

pub(crate) struct WorthGraphReadAccessExecutionReceiptAccountingPhaseSevenSeedInput {
    pub phase_six_closeout_digest: String,
    pub phase_five_closeout_digest: String,
    pub receipt_accounting_report: WorthGraphReadAccessReceiptAccountingReport,
    pub counter_accounting_report: WorthGraphReadAccessCounterAccountingReport,
    pub batch_accounting_report: WorthGraphReadAccessBatchAccountingReport,
    pub source_firewall_report: WorthGraphReadAccessSpatialDenseSourceFirewallReport,
    pub bounded_execution_contract: WorthGraphReadAccessBoundedExecutionContract,
    pub phase_four_cutover_proof: WorthGraphReadAccessSliceCutoverProof,
    pub posture_projections: Vec<WorthGraphReadAccessSpatialDensePostureProjection>,
    pub cap_rows: Vec<WorthGraphReadAccessPostureCapRow>,
}

impl WorthGraphReadAccessExecutionReceiptAccountingPhaseSevenSeed {
    pub(crate) fn from_input(
        input: WorthGraphReadAccessExecutionReceiptAccountingPhaseSevenSeedInput,
    ) -> Self {
        let seed_digest = stable_digest(
            &std::iter::once(
                "worth_graph_read_access_execution_receipt_accounting_phase_seven_seed_v1"
                    .to_string(),
            )
            .chain([
                format!("phase_six:{}", input.phase_six_closeout_digest),
                format!("phase_five:{}", input.phase_five_closeout_digest),
                format!(
                    "receipt:{}",
                    input.receipt_accounting_report.report_digest()
                ),
                format!(
                    "counter:{}",
                    input.counter_accounting_report.report_digest()
                ),
                format!("batch:{}", input.batch_accounting_report.report_digest()),
                format!("firewall:{}", input.source_firewall_report.report_digest()),
                format!(
                    "bounded:{}",
                    input.bounded_execution_contract.contract_digest()
                ),
                format!(
                    "phase_four_cutover:{}",
                    input.phase_four_cutover_proof.cutover_digest()
                ),
            ])
            .chain(
                input
                    .posture_projections
                    .iter()
                    .map(|projection| format!("projection:{}", projection.projection_digest())),
            )
            .chain(
                input
                    .cap_rows
                    .iter()
                    .map(|row| format!("cap:{}", row.row_digest())),
            )
            .collect::<Vec<_>>(),
        );
        Self {
            phase_six_closeout_digest: input.phase_six_closeout_digest,
            phase_five_closeout_digest: input.phase_five_closeout_digest,
            receipt_accounting_report: input.receipt_accounting_report,
            counter_accounting_report: input.counter_accounting_report,
            batch_accounting_report: input.batch_accounting_report,
            source_firewall_report: input.source_firewall_report,
            bounded_execution_contract: input.bounded_execution_contract,
            phase_four_cutover_proof: input.phase_four_cutover_proof,
            posture_projections: input.posture_projections,
            cap_rows: input.cap_rows,
            seed_digest,
        }
    }

    pub fn phase_six_closeout_digest(&self) -> &str {
        &self.phase_six_closeout_digest
    }

    pub fn phase_five_closeout_digest(&self) -> &str {
        &self.phase_five_closeout_digest
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

    pub const fn source_firewall_report(
        &self,
    ) -> &WorthGraphReadAccessSpatialDenseSourceFirewallReport {
        &self.source_firewall_report
    }

    pub const fn bounded_execution_contract(
        &self,
    ) -> &WorthGraphReadAccessBoundedExecutionContract {
        &self.bounded_execution_contract
    }

    pub const fn phase_four_cutover_proof(&self) -> &WorthGraphReadAccessSliceCutoverProof {
        &self.phase_four_cutover_proof
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
