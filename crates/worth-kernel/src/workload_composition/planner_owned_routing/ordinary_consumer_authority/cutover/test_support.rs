use super::inventory_lowering::{cutover_from_lowered_rows, lower_phase_eleven_cutover_rows};
use super::{WorthWorkloadOrdinaryConsumerCutover, WorthWorkloadOrdinaryConsumerCutoverError};

pub(crate) fn ordinary_consumer_cutover_from_inventory_for_tests(
    inventory: &crate::workload_composition::ConflictBatchAdmissionInventory,
) -> Result<WorthWorkloadOrdinaryConsumerCutover, WorthWorkloadOrdinaryConsumerCutoverError> {
    cutover_from_lowered_rows(lower_phase_eleven_cutover_rows(inventory)?)
}

pub(crate) fn ordinary_consumer_cutover_from_inventory_with_test_replay_undo_identity_override(
    inventory: &crate::workload_composition::ConflictBatchAdmissionInventory,
    boundary_proof_digest: &str,
    transaction_packet_identity: &str,
    replay_scope_identity: &str,
    undo_scope_identity: &str,
) -> Result<WorthWorkloadOrdinaryConsumerCutover, WorthWorkloadOrdinaryConsumerCutoverError> {
    let mut lowered_rows = lower_phase_eleven_cutover_rows(inventory)?;
    let replay_undo_surface = "admit_boolean_split_replay_undo_boundary";
    let replay_undo_row = lowered_rows
        .iter_mut()
        .find(|row| row.surface_name == replay_undo_surface)
        .expect("replay/undo selected-plan row should lower from the current inventory");
    replay_undo_row.route_witness = Some(
        super::super::route_witness::current_replay_undo_boundary_batch_execution_cluster_witness_with_test_override(
            boundary_proof_digest,
            transaction_packet_identity,
            replay_scope_identity,
            undo_scope_identity,
        )?,
    );
    cutover_from_lowered_rows(lowered_rows)
}
