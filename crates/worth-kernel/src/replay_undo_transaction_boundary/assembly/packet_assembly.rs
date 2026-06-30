use super::{
    assemble_replay_undo_transaction_boundary_input,
    assemble_replay_undo_transaction_boundary_packet_counters,
    lower_replay_undo_transaction_mutation_claims, ReplayUndoTransactionBoundaryAssemblyRequest,
    ReplayUndoTransactionMutationClaimSource,
};
use crate::replay_undo_transaction_boundary::{
    admit_replay_undo_transaction_boundary_packet, ReplayUndoTransactionBoundaryError,
    ReplayUndoTransactionBoundaryPacket,
};

pub fn assemble_replay_undo_transaction_boundary_packet(
    request: ReplayUndoTransactionBoundaryAssemblyRequest<'_>,
) -> Result<ReplayUndoTransactionBoundaryPacket, ReplayUndoTransactionBoundaryError> {
    let mutation_claims = lower_replay_undo_transaction_mutation_claims(&[
        ReplayUndoTransactionMutationClaimSource::ReplayScope(
            request.spatial_replay_scope_product(),
        ),
        ReplayUndoTransactionMutationClaimSource::UndoScope(request.spatial_undo_scope_product()),
    ]);
    let counters = assemble_replay_undo_transaction_boundary_packet_counters(
        request.topology_undo_scope_product(),
        request.spatial_replay_scope_product(),
        request.spatial_undo_scope_product(),
        mutation_claims.len(),
    );
    let input = assemble_replay_undo_transaction_boundary_input(
        request.with_mutation_claims(mutation_claims, counters),
    )?;
    admit_replay_undo_transaction_boundary_packet(input)
}
