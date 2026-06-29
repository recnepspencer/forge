use super::{WorthWorkloadOrdinaryConsumerCutover, WorthWorkloadOrdinaryConsumerCutoverRow};

impl WorthWorkloadOrdinaryConsumerCutover {
    pub(crate) fn with_test_replay_undo_selected_plan_identity_override(
        mut self,
        boundary_proof_digest: &str,
        transaction_packet_identity: &str,
        replay_scope_identity: &str,
        undo_scope_identity: &str,
    ) -> Self {
        let replay_undo_row = self
            .rows
            .iter_mut()
            .find(|row| row.surface_name == "admit_boolean_split_replay_undo_boundary")
            .expect("replay/undo selected-plan row should exist");
        override_replay_undo_selected_plan_identity(
            replay_undo_row,
            boundary_proof_digest,
            transaction_packet_identity,
            replay_scope_identity,
            undo_scope_identity,
        );
        self
    }
}

fn override_replay_undo_selected_plan_identity(
    row: &mut WorthWorkloadOrdinaryConsumerCutoverRow,
    boundary_proof_digest: &str,
    transaction_packet_identity: &str,
    replay_scope_identity: &str,
    undo_scope_identity: &str,
) {
    let witness = row
        .selected_plan_witness
        .as_mut()
        .expect("replay/undo selected-plan row should carry a witness");
    witness.replay_undo_boundary_proof_digest = Some(boundary_proof_digest.to_string());
    witness.transaction_packet_identity = Some(transaction_packet_identity.to_string());
    witness.replay_scope_identity = Some(replay_scope_identity.to_string());
    witness.undo_scope_identity = Some(undo_scope_identity.to_string());
}
