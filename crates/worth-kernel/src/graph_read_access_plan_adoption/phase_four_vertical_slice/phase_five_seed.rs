use crate::graph_read_access_plan_adoption::{
    WorthGraphReadAccessPhaseFourSeed, WorthGraphReadAccessPostureCapRow,
    WorthGraphReadAccessResolvedPosture,
};

use super::cutover_proof::WorthGraphReadAccessSliceCutoverProof;
use super::query_plan_projection::WorthGraphReadAccessSlicePlanProjection;
use super::receipt_boundary::WorthGraphReadAccessSliceReceiptProjection;
use super::slice_selection::WorthGraphReadAccessSelectedVerticalSlice;
use super::stable_digest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessFirstVerticalSliceSeed {
    phase_four_closeout_digest: String,
    selected_slice: WorthGraphReadAccessSelectedVerticalSlice,
    plan_projection: WorthGraphReadAccessSlicePlanProjection,
    receipt_projection: WorthGraphReadAccessSliceReceiptProjection,
    cutover_proof: WorthGraphReadAccessSliceCutoverProof,
    unresolved_postures: Vec<WorthGraphReadAccessResolvedPosture>,
    cap_rows: Vec<WorthGraphReadAccessPostureCapRow>,
    seed_digest: String,
}

impl WorthGraphReadAccessFirstVerticalSliceSeed {
    pub(crate) fn from_products(
        phase_four_closeout_digest: &str,
        phase_four_seed: &WorthGraphReadAccessPhaseFourSeed,
        selected_slice: &WorthGraphReadAccessSelectedVerticalSlice,
        plan_projection: &WorthGraphReadAccessSlicePlanProjection,
        receipt_projection: &WorthGraphReadAccessSliceReceiptProjection,
        cutover_proof: &WorthGraphReadAccessSliceCutoverProof,
    ) -> Self {
        let unresolved_postures = phase_four_seed
            .resolved_postures()
            .iter()
            .filter(|posture| posture.row_digest() != selected_slice.source_posture_row_digest())
            .cloned()
            .collect::<Vec<_>>();
        let cap_rows = phase_four_seed.cap_rows().to_vec();
        let seed_digest = stable_digest(&[
            "worth_graph_read_access_first_vertical_slice_seed_v1".to_string(),
            format!("phase_four_closeout:{phase_four_closeout_digest}"),
            format!("selected_slice:{}", selected_slice.slice_digest()),
            format!("plan_projection:{}", plan_projection.projection_digest()),
            format!(
                "receipt_projection:{}",
                receipt_projection.projection_digest()
            ),
            format!("cutover:{}", cutover_proof.cutover_digest()),
            format!("unresolved_posture_count:{}", unresolved_postures.len()),
            format!("cap_row_count:{}", cap_rows.len()),
        ]);
        Self {
            phase_four_closeout_digest: phase_four_closeout_digest.to_string(),
            selected_slice: selected_slice.clone(),
            plan_projection: plan_projection.clone(),
            receipt_projection: receipt_projection.clone(),
            cutover_proof: cutover_proof.clone(),
            unresolved_postures,
            cap_rows,
            seed_digest,
        }
    }

    pub fn phase_four_closeout_digest(&self) -> &str {
        &self.phase_four_closeout_digest
    }

    pub const fn selected_slice(&self) -> &WorthGraphReadAccessSelectedVerticalSlice {
        &self.selected_slice
    }

    pub const fn plan_projection(&self) -> &WorthGraphReadAccessSlicePlanProjection {
        &self.plan_projection
    }

    pub const fn receipt_projection(&self) -> &WorthGraphReadAccessSliceReceiptProjection {
        &self.receipt_projection
    }

    pub const fn cutover_proof(&self) -> &WorthGraphReadAccessSliceCutoverProof {
        &self.cutover_proof
    }

    pub fn unresolved_postures(&self) -> &[WorthGraphReadAccessResolvedPosture] {
        &self.unresolved_postures
    }

    pub fn cap_rows(&self) -> &[WorthGraphReadAccessPostureCapRow] {
        &self.cap_rows
    }

    pub fn seed_digest(&self) -> &str {
        &self.seed_digest
    }

    pub const fn claims_access_plan_consumption(&self) -> bool {
        self.receipt_projection.claims_access_plan_consumption()
    }

    pub const fn claims_graph_read_execution(&self) -> bool {
        self.receipt_projection.claims_graph_read_execution()
    }

    pub const fn claims_graph_read_receipt(&self) -> bool {
        self.receipt_projection.claims_graph_read_receipt()
    }

    pub const fn claims_validator_selection(&self) -> bool {
        false
    }
}
