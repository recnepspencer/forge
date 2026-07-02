use topology::certification::{
    TopologyPublicCloseoutFreshnessRequirementPosture,
    TopologyPublicCloseoutRenderedOutputComparisonPosture,
};
use worth_spatial::certification::{
    SpatialPublicCloseoutFreshnessRequirementPosture,
    SpatialPublicCloseoutRenderedOutputComparisonPosture,
};

use crate::workload_composition::planner_owned_routing::WorthTouchedGraphConflictAdmittedPublicProofInput;
use crate::workload_composition::public_closeout::{
    WorthTouchedGraphConflictPublicCloseoutError, WorthTouchedGraphConflictPublicCloseoutErrorKind,
};
use crate::workload_composition::WorthTouchedGraphConflictSelectedRoutePacket;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTouchedGraphConflictMilestoneFifteenPlannerProofInput {
    admitted_input: WorthTouchedGraphConflictAdmittedPublicProofInput,
}

impl WorthTouchedGraphConflictMilestoneFifteenPlannerProofInput {
    pub(crate) fn from_admitted_input(
        admitted_input: WorthTouchedGraphConflictAdmittedPublicProofInput,
    ) -> Self {
        Self { admitted_input }
    }

    pub fn selected_equivalence_family_identity(&self) -> Option<&str> {
        Some(self.admitted_input.selected_family_identity())
    }

    pub fn reuse_basis_identity_digest(&self) -> Option<&str> {
        Some(self.admitted_input.selected_reuse_basis_identity_digest())
    }

    pub fn compiled_product_reuse_route_packet_identity(&self) -> &str {
        self.admitted_input
            .compiled_product_reuse_route_packet_identity()
    }

    pub fn reuse_decision_identity_digest(&self) -> Option<&str> {
        self.admitted_input.selected_witness_identity_digest()
    }

    pub fn spatial_reuse_decision_identity_digest(&self) -> Option<&str> {
        self.admitted_input.spatial_reuse_decision_identity_digest()
    }

    pub fn rebuild_denial_identity_digest(&self) -> Option<&str> {
        self.admitted_input.rebuild_denial_identity_digest()
    }

    pub fn spatial_rebuild_denial_identity_digest(&self) -> Option<&str> {
        self.admitted_input.spatial_rebuild_denial_identity_digest()
    }

    pub fn spatial_compiled_product_identity_digest(&self) -> &str {
        self.admitted_input
            .spatial_selected_product_identity_digest()
    }

    pub fn spatial_selected_equivalence_family_identity(&self) -> &str {
        self.admitted_input.spatial_selected_family_identity()
    }

    pub fn spatial_equivalence_policy_identity_digest(&self) -> &str {
        self.admitted_input
            .spatial_equivalence_policy_identity_digest()
    }

    pub const fn topology_freshness_requirement_posture(
        &self,
    ) -> TopologyPublicCloseoutFreshnessRequirementPosture {
        self.admitted_input.topology_freshness_requirement_posture()
    }

    pub const fn topology_rendered_output_comparison_posture(
        &self,
    ) -> TopologyPublicCloseoutRenderedOutputComparisonPosture {
        self.admitted_input
            .topology_rendered_output_comparison_posture()
    }

    pub const fn spatial_freshness_requirement_posture(
        &self,
    ) -> SpatialPublicCloseoutFreshnessRequirementPosture {
        self.admitted_input.spatial_freshness_requirement_posture()
    }

    pub const fn spatial_rendered_output_comparison_posture(
        &self,
    ) -> SpatialPublicCloseoutRenderedOutputComparisonPosture {
        self.admitted_input
            .spatial_rendered_output_comparison_posture()
    }

    pub const fn topology_query_execution_count(&self) -> usize {
        self.admitted_input.topology_query_execution_count()
    }

    pub const fn topology_row_scan_fallback_count(&self) -> usize {
        self.admitted_input.topology_row_scan_fallback_count()
    }

    pub const fn topology_whole_view_fallback_count(&self) -> usize {
        self.admitted_input.topology_whole_view_fallback_count()
    }

    pub const fn topology_repeated_rediscovery_denied_count(&self) -> usize {
        self.admitted_input
            .topology_repeated_rediscovery_denied_count()
    }

    pub const fn spatial_receipt_proof_row_count(&self) -> usize {
        self.admitted_input.spatial_receipt_proof_row_count()
    }

    pub const fn spatial_non_ordinary_residue_row_count(&self) -> usize {
        self.admitted_input.spatial_non_ordinary_residue_row_count()
    }
}

pub(crate) fn require_admitted_public_proof_input_matches_selected_route_packet(
    packet: &WorthTouchedGraphConflictSelectedRoutePacket,
    admitted_public_proof_input: &WorthTouchedGraphConflictAdmittedPublicProofInput,
) -> Result<(), WorthTouchedGraphConflictPublicCloseoutError> {
    let planner_proof_input =
        WorthTouchedGraphConflictMilestoneFifteenPlannerProofInput::from_admitted_input(
            admitted_public_proof_input.clone(),
        );
    if planner_proof_input.selected_equivalence_family_identity()
        != Some(packet.selected_family_identity())
        || planner_proof_input.reuse_basis_identity_digest()
            != Some(packet.selected_reuse_basis_identity_digest())
        || planner_proof_input.compiled_product_reuse_route_packet_identity()
            != packet.compiled_product_reuse_route_packet_identity()
        || planner_proof_input.reuse_decision_identity_digest()
            != packet.selected_witness_identity_digest()
        || planner_proof_input.spatial_reuse_decision_identity_digest()
            != packet.spatial_reuse_decision_identity_digest()
        || planner_proof_input.rebuild_denial_identity_digest()
            != packet.rebuild_denial_identity_digest()
        || planner_proof_input.spatial_rebuild_denial_identity_digest()
            != packet.spatial_rebuild_denial_identity_digest()
        || planner_proof_input.spatial_selected_equivalence_family_identity()
            != packet.spatial_selected_family_identity()
        || planner_proof_input.spatial_compiled_product_identity_digest()
            != packet.spatial_selected_product_identity_digest()
        || planner_proof_input.spatial_equivalence_policy_identity_digest()
            != packet.spatial_equivalence_policy_identity_digest()
    {
        return Err(WorthTouchedGraphConflictPublicCloseoutError::new(
            WorthTouchedGraphConflictPublicCloseoutErrorKind::IncompleteProofChain,
            "Milestone 15 planner proof input must preserve selected-route packet reuse authority",
        ));
    }
    Ok(())
}
