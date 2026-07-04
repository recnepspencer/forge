use super::{
    current_replay_undo_boundary_batch_execution_cluster_witness,
    WorthWorkloadOrdinaryConsumerCurrentRouteWitness,
};

impl WorthWorkloadOrdinaryConsumerCurrentRouteWitness {
    pub(crate) fn with_test_replay_undo_identity_override(
        mut self,
        boundary_proof_digest: &str,
        transaction_packet_identity: &str,
        replay_scope_identity: &str,
        undo_scope_identity: &str,
    ) -> Self {
        self.route_authority = self
            .route_authority
            .with_test_replay_undo_identity_override(
                boundary_proof_digest,
                transaction_packet_identity,
                replay_scope_identity,
                undo_scope_identity,
            );
        self
    }
}

pub(crate) fn current_replay_undo_boundary_batch_execution_cluster_witness_with_test_override(
    boundary_proof_digest: &str,
    transaction_packet_identity: &str,
    replay_scope_identity: &str,
    undo_scope_identity: &str,
) -> Result<
    WorthWorkloadOrdinaryConsumerCurrentRouteWitness,
    super::WorthWorkloadOrdinaryConsumerCutoverError,
> {
    Ok(
        current_replay_undo_boundary_batch_execution_cluster_witness()?
            .with_test_replay_undo_identity_override(
                boundary_proof_digest,
                transaction_packet_identity,
                replay_scope_identity,
                undo_scope_identity,
            ),
    )
}
