use crate::graph_read_access_plan_adoption::WorthGraphReadAccessPhaseFourSeed;

#[cfg(test)]
use crate::construction::query_access_planning::PrimitiveConstructionConsumedQueryAccess;

use super::counters::WorthGraphReadAccessFirstVerticalSliceCounters;
use super::cutover_proof::{project_cutover_for_slice, WorthGraphReadAccessSliceCutoverProof};
use super::errors::{
    WorthGraphReadAccessFirstVerticalSliceError, WorthGraphReadAccessFirstVerticalSliceErrorKind,
};
#[cfg(test)]
use super::execution_binding::bind_selected_slice_to_construction_execution;
use super::execution_binding::WorthGraphReadAccessExecutedVerticalSlice;
use super::phase_five_seed::WorthGraphReadAccessFirstVerticalSliceSeed;
#[cfg(test)]
use super::query_plan_projection::project_query_plan_for_executed_slice;
use super::query_plan_projection::{
    project_query_plan_for_selected_slice, WorthGraphReadAccessSlicePlanProjection,
};
#[cfg(test)]
use super::receipt_boundary::project_receipt_for_executed_slice;
use super::receipt_boundary::{
    project_receipt_for_plan_projection, WorthGraphReadAccessSliceReceiptProjection,
};
use super::slice_selection::{
    select_first_vertical_slice, WorthGraphReadAccessSelectedVerticalSlice,
};
use super::stable_digest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessFirstVerticalSliceCloseout {
    phase_four_seed_digest: String,
    selected_slice: WorthGraphReadAccessSelectedVerticalSlice,
    executed_slice: Option<WorthGraphReadAccessExecutedVerticalSlice>,
    plan_projection: WorthGraphReadAccessSlicePlanProjection,
    receipt_projection: WorthGraphReadAccessSliceReceiptProjection,
    cutover_proof: WorthGraphReadAccessSliceCutoverProof,
    counters: WorthGraphReadAccessFirstVerticalSliceCounters,
    phase_five_seed: WorthGraphReadAccessFirstVerticalSliceSeed,
    closeout_digest: String,
}

pub fn current_worth_graph_read_access_first_vertical_slice_closeout(
    seed: &WorthGraphReadAccessPhaseFourSeed,
) -> Result<
    WorthGraphReadAccessFirstVerticalSliceCloseout,
    WorthGraphReadAccessFirstVerticalSliceError,
> {
    reject_invalid_seed_claims(seed)?;
    if seed.resolved_postures().is_empty() {
        return Err(WorthGraphReadAccessFirstVerticalSliceError::new(
            WorthGraphReadAccessFirstVerticalSliceErrorKind::EmptyPhaseFourSeed,
        ));
    }

    let selected_slice = select_first_vertical_slice(seed)?;
    let plan_projection = project_query_plan_for_selected_slice(&selected_slice);
    let receipt_projection = project_receipt_for_plan_projection(&selected_slice, &plan_projection);
    Ok(assemble_first_vertical_slice_closeout(
        seed,
        selected_slice,
        None,
        plan_projection,
        receipt_projection,
    ))
}

#[cfg(test)]
pub(crate) fn current_worth_graph_read_access_first_vertical_slice_closeout_with_construction_execution(
    seed: &WorthGraphReadAccessPhaseFourSeed,
    consumed_access: &PrimitiveConstructionConsumedQueryAccess,
) -> Result<
    WorthGraphReadAccessFirstVerticalSliceCloseout,
    WorthGraphReadAccessFirstVerticalSliceError,
> {
    reject_invalid_seed_claims(seed)?;
    if seed.resolved_postures().is_empty() {
        return Err(WorthGraphReadAccessFirstVerticalSliceError::new(
            WorthGraphReadAccessFirstVerticalSliceErrorKind::EmptyPhaseFourSeed,
        ));
    }

    let selected_slice = select_first_vertical_slice(seed)?;
    let executed_slice =
        bind_selected_slice_to_construction_execution(&selected_slice, consumed_access)?;
    let plan_projection = project_query_plan_for_executed_slice(&selected_slice, &executed_slice);
    let receipt_projection =
        project_receipt_for_executed_slice(&selected_slice, &plan_projection, &executed_slice);
    Ok(assemble_first_vertical_slice_closeout(
        seed,
        selected_slice,
        Some(executed_slice),
        plan_projection,
        receipt_projection,
    ))
}

fn assemble_first_vertical_slice_closeout(
    seed: &WorthGraphReadAccessPhaseFourSeed,
    selected_slice: WorthGraphReadAccessSelectedVerticalSlice,
    executed_slice: Option<WorthGraphReadAccessExecutedVerticalSlice>,
    plan_projection: WorthGraphReadAccessSlicePlanProjection,
    receipt_projection: WorthGraphReadAccessSliceReceiptProjection,
) -> WorthGraphReadAccessFirstVerticalSliceCloseout {
    let cutover_proof = project_cutover_for_slice(&selected_slice, &receipt_projection);
    let counters = WorthGraphReadAccessFirstVerticalSliceCounters::from_products(
        &plan_projection,
        &receipt_projection,
    );
    let closeout_digest = first_vertical_slice_closeout_digest(
        seed,
        &selected_slice,
        &plan_projection,
        &receipt_projection,
        &cutover_proof,
    );
    let phase_five_seed = WorthGraphReadAccessFirstVerticalSliceSeed::from_products(
        &closeout_digest,
        seed,
        &selected_slice,
        &plan_projection,
        &receipt_projection,
        &cutover_proof,
    );

    WorthGraphReadAccessFirstVerticalSliceCloseout {
        phase_four_seed_digest: seed.seed_digest().to_string(),
        selected_slice,
        executed_slice,
        plan_projection,
        receipt_projection,
        cutover_proof,
        counters,
        phase_five_seed,
        closeout_digest,
    }
}

impl WorthGraphReadAccessFirstVerticalSliceCloseout {
    pub fn phase_four_seed_digest(&self) -> &str {
        &self.phase_four_seed_digest
    }

    pub const fn selected_slice(&self) -> &WorthGraphReadAccessSelectedVerticalSlice {
        &self.selected_slice
    }

    #[cfg(test)]
    pub(crate) const fn executed_slice(
        &self,
    ) -> Option<&WorthGraphReadAccessExecutedVerticalSlice> {
        self.executed_slice.as_ref()
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

    pub const fn counters(&self) -> &WorthGraphReadAccessFirstVerticalSliceCounters {
        &self.counters
    }

    pub const fn phase_five_seed(&self) -> &WorthGraphReadAccessFirstVerticalSliceSeed {
        &self.phase_five_seed
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }

    pub const fn claims_first_vertical_slice_migration(&self) -> bool {
        self.receipt_projection.claims_graph_read_execution()
    }

    pub const fn claims_access_plan_consumption(&self) -> bool {
        self.receipt_projection.claims_access_plan_consumption()
    }

    pub const fn claims_graph_read_execution(&self) -> bool {
        self.receipt_projection.claims_graph_read_execution()
    }

    pub const fn claims_graph_read_receipts(&self) -> bool {
        self.receipt_projection.claims_graph_read_receipt()
    }

    pub const fn claims_validator_selection(&self) -> bool {
        false
    }
}

fn reject_invalid_seed_claims(
    seed: &WorthGraphReadAccessPhaseFourSeed,
) -> Result<(), WorthGraphReadAccessFirstVerticalSliceError> {
    if seed.claims_access_plan_consumption() {
        return Err(WorthGraphReadAccessFirstVerticalSliceError::new(
            WorthGraphReadAccessFirstVerticalSliceErrorKind::SeedAlreadyClaimedAccessPlanConsumption,
        ));
    }
    if seed.claims_graph_read_execution() {
        return Err(WorthGraphReadAccessFirstVerticalSliceError::new(
            WorthGraphReadAccessFirstVerticalSliceErrorKind::SeedAlreadyClaimedGraphReadExecution,
        ));
    }
    if seed.claims_graph_read_receipt() {
        return Err(WorthGraphReadAccessFirstVerticalSliceError::new(
            WorthGraphReadAccessFirstVerticalSliceErrorKind::SeedAlreadyClaimedGraphReadReceipt,
        ));
    }
    Ok(())
}

fn first_vertical_slice_closeout_digest(
    seed: &WorthGraphReadAccessPhaseFourSeed,
    selected_slice: &WorthGraphReadAccessSelectedVerticalSlice,
    plan_projection: &WorthGraphReadAccessSlicePlanProjection,
    receipt_projection: &WorthGraphReadAccessSliceReceiptProjection,
    cutover_proof: &WorthGraphReadAccessSliceCutoverProof,
) -> String {
    stable_digest(&[
        "worth_graph_read_access_first_vertical_slice_closeout_v1".to_string(),
        format!("phase_four_seed:{}", seed.seed_digest()),
        format!("selected_slice:{}", selected_slice.slice_digest()),
        format!("plan_projection:{}", plan_projection.projection_digest()),
        format!(
            "receipt_projection:{}",
            receipt_projection.projection_digest()
        ),
        format!("cutover:{}", cutover_proof.cutover_digest()),
    ])
}
