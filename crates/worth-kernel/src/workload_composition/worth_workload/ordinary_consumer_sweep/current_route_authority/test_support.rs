use super::WorthWorkloadCurrentOrdinaryRouteAuthority;

impl WorthWorkloadCurrentOrdinaryRouteAuthority {
    pub(crate) fn with_test_replay_undo_identity_override(
        mut self,
        boundary_proof_digest: &str,
        transaction_packet_identity: &str,
        replay_scope_identity: &str,
        undo_scope_identity: &str,
    ) -> Self {
        let Self::ReplayUndoBoundary(authority) = &mut self else {
            panic!("replay/undo identity override requires replay/undo authority");
        };
        authority.boundary_proof_digest = boundary_proof_digest.to_string();
        authority.transaction_packet_identity = transaction_packet_identity.to_string();
        authority.replay_scope_identity = replay_scope_identity.to_string();
        authority.undo_scope_identity = undo_scope_identity.to_string();
        self
    }
}
