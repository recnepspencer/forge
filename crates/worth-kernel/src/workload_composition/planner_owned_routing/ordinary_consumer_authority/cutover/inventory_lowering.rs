use super::{
    owner_name, phase_eleven_consumer_sweep_rows, PendingWorthWorkloadOrdinaryConsumerCutoverRow,
    WorthWorkloadOrdinaryConsumerCutover, WorthWorkloadOrdinaryConsumerCutoverError,
    WorthWorkloadOrdinaryConsumerCutoverErrorKind, WorthWorkloadOrdinaryConsumerCutoverPosture,
};
use crate::workload_composition::performance_trace::{trace_note, trace_scope};
use crate::workload_composition::{
    BatchAdmissionExecutionReceipt, ConflictBatchAdmissionCertificationPosture,
    ConflictBatchAdmissionDisposition, ConflictBatchAdmissionInventory,
    ConflictBatchAdmissionInventoryRow, ConflictBatchAdmissionSurfaceIdentity,
};

use super::super::batch_execution_receipt::current_worth_workload_ordinary_consumer_batch_execution_receipt;
use super::super::route_witness::{
    current_completed_split_batch_execution_cluster_witness,
    current_lookup_consumed_batch_execution_cluster_witness,
    current_replay_undo_boundary_batch_execution_cluster_witness,
};

pub(crate) fn ordinary_consumer_cutover_from_inventory(
    inventory: &ConflictBatchAdmissionInventory,
) -> Result<WorthWorkloadOrdinaryConsumerCutover, WorthWorkloadOrdinaryConsumerCutoverError> {
    trace_scope("ordinary_consumer_cutover_from_inventory", || {
        let lowered_rows = lower_phase_eleven_cutover_rows(inventory)?;
        if lowered_rows.is_empty() {
            return Err(WorthWorkloadOrdinaryConsumerCutoverError::new(
                WorthWorkloadOrdinaryConsumerCutoverErrorKind::MissingInventory,
                "phase 13 ordinary-consumer cutover requires phase-11 consumer sweep rows",
            ));
        }
        cutover_from_lowered_rows(lowered_rows)
    })
}

pub(super) fn lower_phase_eleven_cutover_rows(
    inventory: &ConflictBatchAdmissionInventory,
) -> Result<
    Vec<PendingWorthWorkloadOrdinaryConsumerCutoverRow>,
    WorthWorkloadOrdinaryConsumerCutoverError,
> {
    phase_eleven_consumer_sweep_rows(inventory)
        .map(PendingWorthWorkloadOrdinaryConsumerCutoverRow::from_phase_eleven_inventory_row)
        .collect()
}

pub(super) fn cutover_from_lowered_rows(
    lowered_rows: Vec<PendingWorthWorkloadOrdinaryConsumerCutoverRow>,
) -> Result<WorthWorkloadOrdinaryConsumerCutover, WorthWorkloadOrdinaryConsumerCutoverError> {
    let route_witnesses = lowered_rows
        .iter()
        .filter_map(PendingWorthWorkloadOrdinaryConsumerCutoverRow::route_witness)
        .collect::<Vec<_>>();
    trace_note(format!(
        "ordinary consumer cutover rows={}, selected_plan_witnesses={}",
        lowered_rows.len(),
        route_witnesses.len()
    ));
    let batch_execution_receipt =
        current_worth_workload_ordinary_consumer_batch_execution_receipt(&route_witnesses)?;
    Ok(bind_rows_to_batch_execution_receipt(
        lowered_rows,
        batch_execution_receipt,
    ))
}

fn bind_rows_to_batch_execution_receipt(
    lowered_rows: Vec<PendingWorthWorkloadOrdinaryConsumerCutoverRow>,
    batch_execution_receipt: BatchAdmissionExecutionReceipt,
) -> WorthWorkloadOrdinaryConsumerCutover {
    let rows = lowered_rows
        .into_iter()
        .map(|row| row.bind_receipt(&batch_execution_receipt))
        .collect();
    WorthWorkloadOrdinaryConsumerCutover::new(batch_execution_receipt, rows)
}

impl PendingWorthWorkloadOrdinaryConsumerCutoverRow {
    pub(super) fn from_phase_eleven_inventory_row(
        row: ConflictBatchAdmissionInventoryRow,
    ) -> Result<Self, WorthWorkloadOrdinaryConsumerCutoverError> {
        let (posture, route_witness) = cutover_posture_and_route_witness_for_row(&row)?;
        Ok(Self {
            surface_name: row.surface_name().to_string(),
            owner: owner_name(row.owner()).to_string(),
            blocker: row.blocker().to_string(),
            removal_trigger: row.removal_trigger().to_string(),
            posture,
            route_witness,
        })
    }
}

fn cutover_posture_and_route_witness_for_row(
    row: &ConflictBatchAdmissionInventoryRow,
) -> Result<
    (
        WorthWorkloadOrdinaryConsumerCutoverPosture,
        Option<super::WorthWorkloadOrdinaryConsumerCurrentRouteWitness>,
    ),
    WorthWorkloadOrdinaryConsumerCutoverError,
> {
    match row.surface_identity() {
        ConflictBatchAdmissionSurfaceIdentity::WorthWorkloadAdmitLookupConsumedWorkload
            if is_migrated_ordinary_production_row(row) =>
        {
            Ok((
                WorthWorkloadOrdinaryConsumerCutoverPosture::SelectedPlanDrivenOrdinaryConsumer,
                Some(current_lookup_consumed_batch_execution_cluster_witness()?),
            ))
        }
        ConflictBatchAdmissionSurfaceIdentity::CompletedBooleanSplitHandoffAdmitDownstreamSplitConsumption
            if is_migrated_ordinary_production_row(row) =>
        {
            Ok((
                WorthWorkloadOrdinaryConsumerCutoverPosture::SelectedPlanDrivenOrdinaryConsumer,
                Some(current_completed_split_batch_execution_cluster_witness()?),
            ))
        }
        ConflictBatchAdmissionSurfaceIdentity::BooleanSplitReplayUndoBoundaryAdmission
            if is_migrated_ordinary_production_row(row) =>
        {
            Ok((
                WorthWorkloadOrdinaryConsumerCutoverPosture::SelectedPlanDrivenOrdinaryConsumer,
                Some(current_replay_undo_boundary_batch_execution_cluster_witness()?),
            ))
        }
        ConflictBatchAdmissionSurfaceIdentity::PlanarBooleanLoopRuntimeRegistrationProof
            if row.disposition() == ConflictBatchAdmissionDisposition::Cap =>
        {
            Ok((
                WorthWorkloadOrdinaryConsumerCutoverPosture::QueryProofAccompanimentOnly,
                None,
            ))
        }
        ConflictBatchAdmissionSurfaceIdentity::BooleanChainIntegrationHandoff
            if row.disposition() == ConflictBatchAdmissionDisposition::Cap =>
        {
            Ok((
                WorthWorkloadOrdinaryConsumerCutoverPosture::ReplayUndoCloseoutOnly,
                None,
            ))
        }
        _ => Ok((
            WorthWorkloadOrdinaryConsumerCutoverPosture::CoveredOrdinaryConsumerDependency,
            None,
        )),
    }
}

fn is_migrated_ordinary_production_row(row: &ConflictBatchAdmissionInventoryRow) -> bool {
    row.disposition() == ConflictBatchAdmissionDisposition::Migrate
        && row.certification_posture()
            == ConflictBatchAdmissionCertificationPosture::OrdinaryProductionReachable
}
