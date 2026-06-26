use crate::graph_read_access_plan_adoption::{
    WorthGraphReadAccessPostureCapRow, WorthGraphReadAccessSliceCutoverProof,
    WorthGraphReadAccessSlicePlanProjection, WorthGraphReadAccessSliceReceiptProjection,
};

use super::batch_admission::WorthGraphReadAccessGroupedAdmissionReport;
use super::bounded_execution::WorthGraphReadAccessBoundedExecutionContract;
use super::query_posture_projection::WorthGraphReadAccessSpatialDensePostureProjection;
use super::source_firewall::WorthGraphReadAccessSpatialDenseSourceFirewallReport;
use super::stable_digest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessSpatialDensePhaseSixSeed {
    phase_five_closeout_digest: String,
    phase_four_closeout_digest: String,
    phase_four_plan_projection: WorthGraphReadAccessSlicePlanProjection,
    phase_four_receipt_projection: WorthGraphReadAccessSliceReceiptProjection,
    phase_four_cutover_proof: WorthGraphReadAccessSliceCutoverProof,
    posture_projections: Vec<WorthGraphReadAccessSpatialDensePostureProjection>,
    grouped_admission_report: WorthGraphReadAccessGroupedAdmissionReport,
    bounded_execution_contract: WorthGraphReadAccessBoundedExecutionContract,
    source_firewall_report: WorthGraphReadAccessSpatialDenseSourceFirewallReport,
    cap_rows: Vec<WorthGraphReadAccessPostureCapRow>,
    seed_digest: String,
}

pub(crate) struct WorthGraphReadAccessSpatialDensePhaseSixSeedInput {
    pub phase_five_closeout_digest: String,
    pub phase_four_closeout_digest: String,
    pub phase_four_plan_projection: WorthGraphReadAccessSlicePlanProjection,
    pub phase_four_receipt_projection: WorthGraphReadAccessSliceReceiptProjection,
    pub phase_four_cutover_proof: WorthGraphReadAccessSliceCutoverProof,
    pub posture_projections: Vec<WorthGraphReadAccessSpatialDensePostureProjection>,
    pub grouped_admission_report: WorthGraphReadAccessGroupedAdmissionReport,
    pub bounded_execution_contract: WorthGraphReadAccessBoundedExecutionContract,
    pub source_firewall_report: WorthGraphReadAccessSpatialDenseSourceFirewallReport,
    pub cap_rows: Vec<WorthGraphReadAccessPostureCapRow>,
}

impl WorthGraphReadAccessSpatialDensePhaseSixSeed {
    pub(crate) fn from_input(input: WorthGraphReadAccessSpatialDensePhaseSixSeedInput) -> Self {
        let seed_digest = stable_digest(
            &std::iter::once("worth_graph_read_access_spatial_dense_phase_six_seed_v1".to_string())
                .chain([
                    format!("phase_five_closeout:{}", input.phase_five_closeout_digest),
                    format!("phase_four_closeout:{}", input.phase_four_closeout_digest),
                    format!(
                        "phase_four_plan:{}",
                        input.phase_four_plan_projection.projection_digest()
                    ),
                    format!(
                        "phase_four_receipt:{}",
                        input.phase_four_receipt_projection.projection_digest()
                    ),
                    format!(
                        "phase_four_cutover:{}",
                        input.phase_four_cutover_proof.cutover_digest()
                    ),
                    format!("grouped:{}", input.grouped_admission_report.report_digest()),
                    format!(
                        "bounded:{}",
                        input.bounded_execution_contract.contract_digest()
                    ),
                    format!("firewall:{}", input.source_firewall_report.report_digest()),
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
            phase_five_closeout_digest: input.phase_five_closeout_digest,
            phase_four_closeout_digest: input.phase_four_closeout_digest,
            phase_four_plan_projection: input.phase_four_plan_projection,
            phase_four_receipt_projection: input.phase_four_receipt_projection,
            phase_four_cutover_proof: input.phase_four_cutover_proof,
            posture_projections: input.posture_projections,
            grouped_admission_report: input.grouped_admission_report,
            bounded_execution_contract: input.bounded_execution_contract,
            source_firewall_report: input.source_firewall_report,
            cap_rows: input.cap_rows,
            seed_digest,
        }
    }

    pub fn phase_five_closeout_digest(&self) -> &str {
        &self.phase_five_closeout_digest
    }

    pub fn phase_four_closeout_digest(&self) -> &str {
        &self.phase_four_closeout_digest
    }

    pub const fn phase_four_plan_projection(&self) -> &WorthGraphReadAccessSlicePlanProjection {
        &self.phase_four_plan_projection
    }

    pub const fn phase_four_receipt_projection(
        &self,
    ) -> &WorthGraphReadAccessSliceReceiptProjection {
        &self.phase_four_receipt_projection
    }

    pub const fn phase_four_cutover_proof(&self) -> &WorthGraphReadAccessSliceCutoverProof {
        &self.phase_four_cutover_proof
    }

    pub fn posture_projections(&self) -> &[WorthGraphReadAccessSpatialDensePostureProjection] {
        &self.posture_projections
    }

    pub const fn grouped_admission_report(&self) -> &WorthGraphReadAccessGroupedAdmissionReport {
        &self.grouped_admission_report
    }

    pub const fn bounded_execution_contract(
        &self,
    ) -> &WorthGraphReadAccessBoundedExecutionContract {
        &self.bounded_execution_contract
    }

    pub const fn source_firewall_report(
        &self,
    ) -> &WorthGraphReadAccessSpatialDenseSourceFirewallReport {
        &self.source_firewall_report
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

    #[cfg(test)]
    pub(crate) fn with_phase_four_receipt_projection_for_tests(
        &self,
        phase_four_receipt_projection: WorthGraphReadAccessSliceReceiptProjection,
    ) -> Self {
        Self::from_input(WorthGraphReadAccessSpatialDensePhaseSixSeedInput {
            phase_five_closeout_digest: self.phase_five_closeout_digest.clone(),
            phase_four_closeout_digest: self.phase_four_closeout_digest.clone(),
            phase_four_plan_projection: self.phase_four_plan_projection.clone(),
            phase_four_receipt_projection,
            phase_four_cutover_proof: self.phase_four_cutover_proof.clone(),
            posture_projections: self.posture_projections.clone(),
            grouped_admission_report: self.grouped_admission_report.clone(),
            bounded_execution_contract: self.bounded_execution_contract.clone(),
            source_firewall_report: self.source_firewall_report.clone(),
            cap_rows: self.cap_rows.clone(),
        })
    }
}
