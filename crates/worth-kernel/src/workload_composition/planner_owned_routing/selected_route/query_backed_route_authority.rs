use topology::facade::TopologyQueryBackedReadFamilySelectedRouteAuthority;

use super::packet::WorthTouchedGraphConflictSelectedRoutePacket;

impl TopologyQueryBackedReadFamilySelectedRouteAuthority
    for WorthTouchedGraphConflictSelectedRoutePacket
{
    fn topology_query_handle_identity_digest(&self) -> &str {
        self.topology_query_handle_identity_digest()
    }

    fn topology_query_support_snapshot_digest(&self) -> &str {
        self.topology_query_support_snapshot_digest()
    }

    fn topology_query_operating_context_identity_digest(&self) -> &str {
        self.topology_query_operating_context_identity_digest()
    }

    fn topology_query_parity_verified_count(&self) -> usize {
        self.topology_query_parity_verified_count()
    }

    fn topology_query_compiled_product_identity_digest(&self) -> Option<&str> {
        Some(self.selected_product_identity_digest())
    }

    fn topology_query_equivalence_policy_identity_digest(&self) -> Option<&str> {
        Some(self.selected_equivalence_policy_identity_digest())
    }

    fn topology_query_selected_equivalence_family_identity(&self) -> Option<&str> {
        Some(self.selected_family_identity())
    }

    fn topology_query_selected_equivalence_basis_identity_digest(&self) -> Option<&str> {
        Some(self.selected_equivalence_basis_identity_digest())
    }

    fn topology_query_selected_compatibility_basis_identity_digest(&self) -> Option<&str> {
        Some(self.selected_compatibility_basis_identity_digest())
    }

    fn topology_query_selected_reuse_basis_identity_digest(&self) -> Option<&str> {
        Some(self.selected_reuse_basis_identity_digest())
    }

    fn topology_query_reuse_decision_identity_digest(&self) -> Option<&str> {
        self.selected_witness_identity_digest()
    }

    fn topology_query_rebuild_denial_identity_digest(&self) -> Option<&str> {
        self.rebuild_denial_identity_digest()
    }
}
