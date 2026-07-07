use schema::facade::platform::authority::{
    replay_undo_semantic_graph::ReplayUndoPlannerRouteFamily,
    touched_graph_conflict::{
        BatchAdmissionPlannerRouteWitnessKind, ConflictIndependencePlannerRouteWitnessKind,
    },
};
use topology::certification::{
    TopologyPublicCloseoutFreshnessRequirementPosture,
    TopologyPublicCloseoutRenderedOutputComparisonPosture,
};
use topology::facade::TopologyDerivedReuseDecisionPosture;
use worth_spatial::certification::{
    SpatialPublicCloseoutFreshnessRequirementPosture,
    SpatialPublicCloseoutRenderedOutputComparisonPosture,
};
use worth_spatial::facade::evidence_lookup_reuse_route::EvidenceLookupReuseDecisionPosture;

use super::WorthTouchedGraphConflictSelectedRoutePacket;

impl WorthTouchedGraphConflictSelectedRoutePacket {
    #[cfg(test)]
    pub(crate) fn route_authority_digests(&self) -> &[String] {
        &self.route_authority_digests
    }

    #[cfg(test)]
    pub(crate) fn route_lineage_digests(&self) -> &[String] {
        &self.route_lineage_digests
    }

    pub(crate) fn overlap_identity_digests(&self) -> &[String] {
        &self.overlap_identity_digests
    }

    pub(crate) fn selected_conflict_plan_digests(&self) -> &[String] {
        &self.selected_conflict_plan_digests
    }

    pub(crate) fn independence_proof_digests(&self) -> &[String] {
        &self.independence_proof_digests
    }

    pub(crate) fn selected_batch_plan_digest(&self) -> &str {
        &self.selected_batch_plan_digest
    }

    pub(crate) fn batch_execution_receipt_digest(&self) -> &str {
        &self.batch_execution_receipt_digest
    }

    pub(crate) fn replay_undo_boundary_proof_digests(&self) -> &[String] {
        &self.replay_undo_boundary_proof_digests
    }

    pub(crate) fn conflict_independence_route_packet_identity(&self) -> &str {
        self.conflict_independence_route_packet.packet_identity()
    }

    pub(crate) fn compiled_product_reuse_route_packet_identity(&self) -> &str {
        self.compiled_product_reuse_route_packet.packet_identity()
    }

    pub(crate) const fn topology_reuse_posture(&self) -> TopologyDerivedReuseDecisionPosture {
        self.compiled_product_reuse_route_packet.topology_posture()
    }

    pub(crate) const fn spatial_reuse_posture(&self) -> EvidenceLookupReuseDecisionPosture {
        self.compiled_product_reuse_route_packet.spatial_posture()
    }

    pub(crate) fn topology_reuse_decision_identity_digest(&self) -> Option<&str> {
        self.compiled_product_reuse_route_packet
            .topology_reuse_decision_identity_digest()
    }

    pub(crate) fn spatial_reuse_decision_identity_digest(&self) -> Option<&str> {
        self.compiled_product_reuse_route_packet
            .spatial_reuse_decision_identity_digest()
    }

    pub(crate) fn spatial_rebuild_denial_identity_digest(&self) -> Option<&str> {
        self.compiled_product_reuse_route_packet
            .spatial_rebuild_denial_identity_digest()
    }

    pub(crate) const fn conflict_route_family(
        &self,
    ) -> schema::facade::platform::authority::touched_graph_conflict::ConflictIndependencePlannerRouteFamily
    {
        self.conflict_independence_route_packet
            .conflict_route_family()
    }

    pub(crate) const fn independence_route_family(
        &self,
    ) -> schema::facade::platform::authority::touched_graph_conflict::ConflictIndependencePlannerRouteFamily
    {
        self.conflict_independence_route_packet
            .independence_route_family()
    }

    pub(crate) fn conflict_independence_denial_witness_identity(&self) -> Option<&str> {
        self.conflict_independence_route_packet
            .denial_witness()
            .map(|witness| witness.identity_digest())
    }

    pub(crate) fn conflict_independence_denial_witness_kind(
        &self,
    ) -> Option<ConflictIndependencePlannerRouteWitnessKind> {
        self.conflict_independence_route_packet
            .denial_witness()
            .map(|witness| witness.kind())
    }

    pub(crate) fn replay_undo_route_packet_identity(&self) -> &str {
        &self.replay_undo_route_packet_identity
    }

    pub(crate) const fn replay_undo_route_family(&self) -> ReplayUndoPlannerRouteFamily {
        self.replay_undo_route_family
    }

    pub(crate) fn batch_admission_route_packet_identity(&self) -> &str {
        self.batch_admission_route_packet.packet_identity()
    }

    pub(crate) fn batch_admission_selected_family_row_digests(&self) -> &[String] {
        self.batch_admission_route_packet
            .selected_family_row_digests()
    }

    pub(crate) fn batch_admission_denial_witness_identity(&self) -> Option<&str> {
        self.batch_admission_route_packet
            .denial_witness()
            .map(|witness| witness.identity_digest())
    }

    pub(crate) fn batch_admission_denial_witness_kind(
        &self,
    ) -> Option<BatchAdmissionPlannerRouteWitnessKind> {
        self.batch_admission_route_packet.denial_witness_kind()
    }

    pub(crate) fn transaction_packet_identities(&self) -> &[String] {
        &self.transaction_packet_identities
    }

    pub(crate) fn replay_scope_identities(&self) -> &[String] {
        &self.replay_scope_identities
    }

    pub(crate) fn undo_scope_identities(&self) -> &[String] {
        &self.undo_scope_identities
    }

    pub(crate) fn evidence_lookup_public_closeout_digest(&self) -> &str {
        &self.evidence_lookup_public_closeout_digest
    }

    #[cfg(test)]
    pub(crate) fn evidence_lookup_family_coverage_digest(&self) -> &str {
        &self.evidence_lookup_family_coverage_digest
    }

    #[cfg(test)]
    pub(crate) fn evidence_lookup_query_surface_matrix_digest(&self) -> &str {
        &self.evidence_lookup_query_surface_matrix_digest
    }

    #[cfg(test)]
    pub(crate) fn evidence_lookup_query_consumer_kit_digest(&self) -> &str {
        &self.evidence_lookup_query_consumer_kit_digest
    }

    pub(crate) fn evidence_lookup_query_boundary_support_digest(&self) -> &str {
        &self.evidence_lookup_query_boundary_support_digest
    }

    pub(crate) fn evidence_lookup_query_support_digest(&self) -> &str {
        &self.evidence_lookup_query_support_digest
    }

    pub(crate) fn topology_query_backed_consumer_cutover_digest(&self) -> &str {
        &self.topology_query_backed_consumer_cutover_digest
    }

    pub(crate) fn topology_query_public_read_family_row_digest(&self) -> &str {
        &self.topology_query_public_read_family_row_digest
    }

    pub(crate) fn topology_query_handle_identity_digest(&self) -> &str {
        &self.topology_query_handle_identity_digest
    }

    pub(crate) fn topology_query_operating_context_identity_digest(&self) -> &str {
        &self.topology_query_operating_context_identity_digest
    }

    pub(crate) fn topology_query_support_snapshot_digest(&self) -> &str {
        &self.topology_query_support_snapshot_digest
    }

    #[cfg(test)]
    pub(crate) const fn topology_query_parity_verified_count(&self) -> usize {
        self.topology_query_parity_verified_count
    }

    pub fn selected_family_identity(&self) -> &str {
        &self.selected_family_identity
    }

    pub fn selected_product_identity_digest(&self) -> &str {
        &self.selected_product_identity_digest
    }

    pub(crate) fn selected_equivalence_policy_identity_digest(&self) -> &str {
        &self.selected_equivalence_policy_identity_digest
    }

    #[cfg(test)]
    pub(crate) fn selected_equivalence_basis_identity_digest(&self) -> &str {
        &self.selected_equivalence_basis_identity_digest
    }

    #[cfg(test)]
    pub(crate) fn selected_compatibility_basis_identity_digest(&self) -> &str {
        &self.selected_compatibility_basis_identity_digest
    }

    pub(crate) fn selected_reuse_basis_identity_digest(&self) -> &str {
        &self.selected_reuse_basis_identity_digest
    }

    pub fn selected_witness_identity_digest(&self) -> Option<&str> {
        self.selected_witness_identity_digest.as_deref()
    }

    pub(crate) fn rebuild_denial_identity_digest(&self) -> Option<&str> {
        self.rebuild_denial_identity_digest.as_deref()
    }

    pub(crate) fn spatial_selected_family_identity(&self) -> &str {
        &self.spatial_selected_family_identity
    }

    pub(crate) fn spatial_selected_product_identity_digest(&self) -> &str {
        &self.spatial_selected_product_identity_digest
    }

    pub(crate) fn spatial_equivalence_policy_identity_digest(&self) -> &str {
        &self.spatial_equivalence_policy_identity_digest
    }

    pub(crate) fn spatial_selected_compatibility_basis_identity_digest(&self) -> &str {
        self.compiled_product_reuse_route_packet
            .spatial_selected_compatibility_basis_identity_digest()
    }

    pub(crate) fn spatial_selected_reuse_basis_identity_digest(&self) -> &str {
        self.compiled_product_reuse_route_packet
            .spatial_selected_reuse_basis_identity_digest()
    }

    pub(crate) const fn topology_freshness_requirement_posture(
        &self,
    ) -> TopologyPublicCloseoutFreshnessRequirementPosture {
        self.topology_freshness_requirement_posture
    }

    pub(crate) const fn topology_rendered_output_comparison_posture(
        &self,
    ) -> TopologyPublicCloseoutRenderedOutputComparisonPosture {
        self.topology_rendered_output_comparison_posture
    }

    pub(crate) const fn spatial_freshness_requirement_posture(
        &self,
    ) -> SpatialPublicCloseoutFreshnessRequirementPosture {
        self.spatial_freshness_requirement_posture
    }

    pub(crate) const fn spatial_rendered_output_comparison_posture(
        &self,
    ) -> SpatialPublicCloseoutRenderedOutputComparisonPosture {
        self.spatial_rendered_output_comparison_posture
    }

    pub(crate) const fn topology_query_execution_count(&self) -> usize {
        self.topology_query_execution_count
    }

    pub(crate) const fn topology_row_scan_fallback_count(&self) -> usize {
        self.topology_row_scan_fallback_count
    }

    pub(crate) const fn topology_whole_view_fallback_count(&self) -> usize {
        self.topology_whole_view_fallback_count
    }

    pub(crate) const fn topology_repeated_rediscovery_denied_count(&self) -> usize {
        self.topology_repeated_rediscovery_denied_count
    }

    pub(crate) fn touched_closure_digest(&self) -> &str {
        &self.touched_closure_digest
    }

    pub(crate) fn touched_semantic_family_key(&self) -> &str {
        &self.touched_semantic_family_key
    }

    pub(crate) fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }

    pub(crate) const fn touched_aspect_count(&self) -> usize {
        self.touched_aspect_count
    }

    pub(crate) const fn touched_scope_count(&self) -> usize {
        self.touched_scope_count
    }

    pub(crate) fn selected_row_family_identities(&self) -> &[String] {
        &self.selected_row_family_identities
    }

    pub(crate) const fn spatial_receipt_proof_row_count(&self) -> usize {
        self.spatial_receipt_proof_row_count
    }

    pub(crate) const fn spatial_non_ordinary_residue_row_count(&self) -> usize {
        self.spatial_non_ordinary_residue_row_count
    }

    pub fn source_firewall_digest(&self) -> &str {
        &self.source_firewall_digest
    }

    pub fn deletion_closeout_digest(&self) -> &str {
        &self.deletion_closeout_digest
    }

    pub fn selected_route_identity_digest(&self) -> &str {
        &self.selected_route_identity_digest
    }

    pub fn decision_trace_identity_digest(&self) -> &str {
        &self.decision_trace_identity_digest
    }

    pub fn packet_digest(&self) -> &str {
        &self.packet_digest
    }
}
