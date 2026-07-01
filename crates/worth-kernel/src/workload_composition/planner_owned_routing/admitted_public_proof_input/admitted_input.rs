use topology::certification::{TopologyPublicCloseoutFreshnessRequirementPosture, TopologyPublicCloseoutRenderedOutputComparisonPosture};
use worth_spatial::certification::{SpatialPublicCloseoutFreshnessRequirementPosture, SpatialPublicCloseoutRenderedOutputComparisonPosture};

use crate::workload_composition::planner_owned_routing::{
    current_worth_touched_graph_conflict_selected_route_packet,
    PlannerOwnedRoutingError,
    WorthTouchedGraphConflictSelectedRoutePacket,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTouchedGraphConflictAdmittedPublicProofInput {
    selected_route_packet_digest: String,
    selected_route_identity_digest: String,
    selected_family_identity: String,
    selected_product_identity_digest: String,
    selected_reuse_basis_identity_digest: String,
    selected_witness_identity_digest: Option<String>,
    rebuild_denial_identity_digest: Option<String>,
    spatial_selected_family_identity: String,
    spatial_selected_product_identity_digest: String,
    spatial_equivalence_policy_identity_digest: String,
    topology_freshness_requirement_posture: TopologyPublicCloseoutFreshnessRequirementPosture,
    topology_rendered_output_comparison_posture:
        TopologyPublicCloseoutRenderedOutputComparisonPosture,
    spatial_freshness_requirement_posture: SpatialPublicCloseoutFreshnessRequirementPosture,
    spatial_rendered_output_comparison_posture:
        SpatialPublicCloseoutRenderedOutputComparisonPosture,
    topology_query_execution_count: usize,
    topology_row_scan_fallback_count: usize,
    topology_whole_view_fallback_count: usize,
    topology_repeated_rediscovery_denied_count: usize,
    spatial_receipt_proof_row_count: usize,
    spatial_non_ordinary_residue_row_count: usize,
}

pub fn current_worth_touched_graph_conflict_public_proof_input(
) -> Result<WorthTouchedGraphConflictAdmittedPublicProofInput, PlannerOwnedRoutingError> {
    let packet = current_worth_touched_graph_conflict_selected_route_packet()?;
    admit_worth_touched_graph_conflict_public_proof_input(&packet)
}

pub fn admit_worth_touched_graph_conflict_public_proof_input(
    packet: &WorthTouchedGraphConflictSelectedRoutePacket,
) -> Result<WorthTouchedGraphConflictAdmittedPublicProofInput, PlannerOwnedRoutingError> {
    Ok(WorthTouchedGraphConflictAdmittedPublicProofInput::from_parts(
        packet.packet_digest().to_string(),
        packet.selected_route_identity_digest().to_string(),
        packet.selected_family_identity().to_string(),
        packet.selected_product_identity_digest().to_string(),
        packet.selected_reuse_basis_identity_digest().to_string(),
        packet.selected_witness_identity_digest().map(str::to_string),
        packet.rebuild_denial_identity_digest().map(str::to_string),
        packet.spatial_selected_family_identity().to_string(),
        packet.spatial_selected_product_identity_digest().to_string(),
        packet.spatial_equivalence_policy_identity_digest().to_string(),
        packet.topology_freshness_requirement_posture(),
        packet.topology_rendered_output_comparison_posture(),
        packet.spatial_freshness_requirement_posture(),
        packet.spatial_rendered_output_comparison_posture(),
        packet.topology_query_execution_count(),
        packet.topology_row_scan_fallback_count(),
        packet.topology_whole_view_fallback_count(),
        packet.topology_repeated_rediscovery_denied_count(),
        packet.spatial_receipt_proof_row_count(),
        packet.spatial_non_ordinary_residue_row_count(),
    ))
}

impl WorthTouchedGraphConflictAdmittedPublicProofInput {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn from_parts(
        selected_route_packet_digest: String,
        selected_route_identity_digest: String,
        selected_family_identity: String,
        selected_product_identity_digest: String,
        selected_reuse_basis_identity_digest: String,
        selected_witness_identity_digest: Option<String>,
        rebuild_denial_identity_digest: Option<String>,
        spatial_selected_family_identity: String,
        spatial_selected_product_identity_digest: String,
        spatial_equivalence_policy_identity_digest: String,
        topology_freshness_requirement_posture: TopologyPublicCloseoutFreshnessRequirementPosture,
        topology_rendered_output_comparison_posture:
            TopologyPublicCloseoutRenderedOutputComparisonPosture,
        spatial_freshness_requirement_posture: SpatialPublicCloseoutFreshnessRequirementPosture,
        spatial_rendered_output_comparison_posture:
            SpatialPublicCloseoutRenderedOutputComparisonPosture,
        topology_query_execution_count: usize,
        topology_row_scan_fallback_count: usize,
        topology_whole_view_fallback_count: usize,
        topology_repeated_rediscovery_denied_count: usize,
        spatial_receipt_proof_row_count: usize,
        spatial_non_ordinary_residue_row_count: usize,
    ) -> Self {
        Self {
            selected_route_packet_digest,
            selected_route_identity_digest,
            selected_family_identity,
            selected_product_identity_digest,
            selected_reuse_basis_identity_digest,
            selected_witness_identity_digest,
            rebuild_denial_identity_digest,
            spatial_selected_family_identity,
            spatial_selected_product_identity_digest,
            spatial_equivalence_policy_identity_digest,
            topology_freshness_requirement_posture,
            topology_rendered_output_comparison_posture,
            spatial_freshness_requirement_posture,
            spatial_rendered_output_comparison_posture,
            topology_query_execution_count,
            topology_row_scan_fallback_count,
            topology_whole_view_fallback_count,
            topology_repeated_rediscovery_denied_count,
            spatial_receipt_proof_row_count,
            spatial_non_ordinary_residue_row_count,
        }
    }

    pub fn selected_route_packet_digest(&self) -> &str { &self.selected_route_packet_digest }
    pub fn selected_route_identity_digest(&self) -> &str { &self.selected_route_identity_digest }
    pub fn selected_family_identity(&self) -> &str { &self.selected_family_identity }
    pub(crate) fn selected_product_identity_digest(&self) -> &str { &self.selected_product_identity_digest }
    pub(crate) fn selected_reuse_basis_identity_digest(&self) -> &str { &self.selected_reuse_basis_identity_digest }
    pub fn selected_witness_identity_digest(&self) -> Option<&str> { self.selected_witness_identity_digest.as_deref() }
    pub(crate) fn rebuild_denial_identity_digest(&self) -> Option<&str> { self.rebuild_denial_identity_digest.as_deref() }
    pub(crate) fn spatial_selected_family_identity(&self) -> &str { &self.spatial_selected_family_identity }
    pub(crate) fn spatial_selected_product_identity_digest(&self) -> &str { &self.spatial_selected_product_identity_digest }
    pub(crate) fn spatial_equivalence_policy_identity_digest(&self) -> &str { &self.spatial_equivalence_policy_identity_digest }
    pub(crate) const fn topology_freshness_requirement_posture(&self) -> TopologyPublicCloseoutFreshnessRequirementPosture { self.topology_freshness_requirement_posture }
    pub(crate) const fn topology_rendered_output_comparison_posture(&self) -> TopologyPublicCloseoutRenderedOutputComparisonPosture { self.topology_rendered_output_comparison_posture }
    pub(crate) const fn spatial_freshness_requirement_posture(&self) -> SpatialPublicCloseoutFreshnessRequirementPosture { self.spatial_freshness_requirement_posture }
    pub(crate) const fn spatial_rendered_output_comparison_posture(&self) -> SpatialPublicCloseoutRenderedOutputComparisonPosture { self.spatial_rendered_output_comparison_posture }
    pub(crate) const fn topology_query_execution_count(&self) -> usize { self.topology_query_execution_count }
    pub(crate) const fn topology_row_scan_fallback_count(&self) -> usize { self.topology_row_scan_fallback_count }
    pub(crate) const fn topology_whole_view_fallback_count(&self) -> usize { self.topology_whole_view_fallback_count }
    pub(crate) const fn topology_repeated_rediscovery_denied_count(&self) -> usize { self.topology_repeated_rediscovery_denied_count }
    pub(crate) const fn spatial_receipt_proof_row_count(&self) -> usize { self.spatial_receipt_proof_row_count }
    pub(crate) const fn spatial_non_ordinary_residue_row_count(&self) -> usize { self.spatial_non_ordinary_residue_row_count }
}
