use super::super::current_cutover_proof::current_worth_workload_ordinary_consumer_batch_execution_receipt;
use super::{
    PendingWorthWorkloadOrdinaryConsumerCutoverRow, WorthWorkloadOrdinaryConsumerCutover,
    WorthWorkloadOrdinaryConsumerCutoverError,
};
use crate::workload_composition::planner_owned_routing::ordinary_consumer_authority::current_replay_undo_boundary_batch_execution_cluster_witness_with_test_override;

pub(crate) fn ordinary_consumer_cutover_from_inventory_with_test_replay_undo_identity_override(
    inventory: &crate::workload_composition::ConflictBatchAdmissionInventory,
    boundary_proof_digest: &str,
    transaction_packet_identity: &str,
    replay_scope_identity: &str,
    undo_scope_identity: &str,
) -> Result<WorthWorkloadOrdinaryConsumerCutover, WorthWorkloadOrdinaryConsumerCutoverError> {
    let mut lowered_rows = inventory
        .rows()
        .iter()
        .filter(|row| {
            row.replacement_phase()
                == crate::workload_composition::ConflictBatchAdmissionReplacementPhase::PhaseElevenConsumerSweep
        })
        .cloned()
        .map(PendingWorthWorkloadOrdinaryConsumerCutoverRow::from_phase_eleven_inventory_row)
        .collect::<Result<Vec<_>, _>>()?;
    let replay_undo_surface = "admit_boolean_split_replay_undo_boundary";
    let replay_undo_row = lowered_rows
        .iter_mut()
        .find(|row| row.surface_name == replay_undo_surface)
        .expect("replay/undo selected-plan row should lower from the current inventory");
    replay_undo_row.route_witness = Some(
        current_replay_undo_boundary_batch_execution_cluster_witness_with_test_override(
            boundary_proof_digest,
            transaction_packet_identity,
            replay_scope_identity,
            undo_scope_identity,
        )?,
    );
    let route_witnesses = lowered_rows
        .iter()
        .filter_map(PendingWorthWorkloadOrdinaryConsumerCutoverRow::route_witness)
        .collect::<Vec<_>>();
    let batch_execution_receipt =
        current_worth_workload_ordinary_consumer_batch_execution_receipt(&route_witnesses)?;
    let rows = lowered_rows
        .into_iter()
        .map(|row| row.bind_receipt(&batch_execution_receipt))
        .collect();
    Ok(WorthWorkloadOrdinaryConsumerCutover::new(
        batch_execution_receipt,
        rows,
    ))
}

pub(crate) fn ordinary_consumer_cutover_from_inventory_for_tests(
    inventory: &crate::workload_composition::ConflictBatchAdmissionInventory,
) -> Result<WorthWorkloadOrdinaryConsumerCutover, WorthWorkloadOrdinaryConsumerCutoverError> {
    let lowered_rows = inventory
        .rows()
        .iter()
        .filter(|row| {
            row.replacement_phase()
                == crate::workload_composition::ConflictBatchAdmissionReplacementPhase::PhaseElevenConsumerSweep
        })
        .cloned()
        .map(PendingWorthWorkloadOrdinaryConsumerCutoverRow::from_phase_eleven_inventory_row)
        .collect::<Result<Vec<_>, _>>()?;
    let route_witnesses = lowered_rows
        .iter()
        .filter_map(PendingWorthWorkloadOrdinaryConsumerCutoverRow::route_witness)
        .collect::<Vec<_>>();
    let batch_execution_receipt =
        current_worth_workload_ordinary_consumer_batch_execution_receipt(&route_witnesses)?;
    let rows = lowered_rows
        .into_iter()
        .map(|row| row.bind_receipt(&batch_execution_receipt))
        .collect();
    Ok(WorthWorkloadOrdinaryConsumerCutover::new(
        batch_execution_receipt,
        rows,
    ))
}
