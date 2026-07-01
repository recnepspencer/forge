use topology::certification::{
    TopologyPublicCloseoutFreshnessRequirementPosture,
    TopologyPublicCloseoutRenderedOutputComparisonPosture,
};
use worth_spatial::certification::{
    SpatialPublicCloseoutFreshnessRequirementPosture,
    SpatialPublicCloseoutRenderedOutputComparisonPosture,
};

use crate::workload_composition::planner_owned_routing::{
    WorthTouchedGraphConflictAdmittedPublicProofInput,
};

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

    pub fn reuse_decision_identity_digest(&self) -> Option<&str> {
        self.admitted_input.selected_witness_identity_digest()
    }

    pub fn rebuild_denial_identity_digest(&self) -> Option<&str> {
        self.admitted_input.rebuild_denial_identity_digest()
    }

    pub fn spatial_compiled_product_identity_digest(&self) -> &str {
        self.admitted_input.spatial_selected_product_identity_digest()
    }

    pub fn spatial_equivalence_policy_identity_digest(&self) -> &str {
        self.admitted_input.spatial_equivalence_policy_identity_digest()
    }

    pub fn spatial_selected_equivalence_family_identity(&self) -> &str {
        self.admitted_input.spatial_selected_family_identity()
    }

    pub const fn topology_freshness_requirement_posture(
        &self,
    ) -> TopologyPublicCloseoutFreshnessRequirementPosture {
        self.admitted_input.topology_freshness_requirement_posture()
    }

    pub const fn topology_rendered_output_comparison_posture(
        &self,
    ) -> TopologyPublicCloseoutRenderedOutputComparisonPosture {
        self.admitted_input.topology_rendered_output_comparison_posture()
    }

    pub const fn spatial_freshness_requirement_posture(
        &self,
    ) -> SpatialPublicCloseoutFreshnessRequirementPosture {
        self.admitted_input.spatial_freshness_requirement_posture()
    }

    pub const fn spatial_rendered_output_comparison_posture(
        &self,
    ) -> SpatialPublicCloseoutRenderedOutputComparisonPosture {
        self.admitted_input.spatial_rendered_output_comparison_posture()
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
        self.admitted_input.topology_repeated_rediscovery_denied_count()
    }

    pub const fn spatial_receipt_proof_row_count(&self) -> usize {
        self.admitted_input.spatial_receipt_proof_row_count()
    }

    pub const fn spatial_non_ordinary_residue_row_count(&self) -> usize {
        self.admitted_input.spatial_non_ordinary_residue_row_count()
    }
}
