use super::WorthTouchedGraphConflictSelectedRoutePacket;

impl WorthTouchedGraphConflictSelectedRoutePacket {
    pub(crate) fn with_test_topology_query_support_snapshot_digest_override(
        mut self,
        digest: &str,
    ) -> Self {
        self.topology_query_support_snapshot_digest = digest.to_string();
        self
    }

    pub(crate) fn with_test_selected_reuse_basis_identity_digest_override(
        mut self,
        digest: &str,
    ) -> Self {
        self.selected_reuse_basis_identity_digest = digest.to_string();
        self
    }
}
