use schema::facade::platform::authority::{
    replay_undo_semantic_graph::ReplayUndoPlannerRouteFamily,
    touched_graph_conflict::{
        BatchAdmissionPlannerRouteWitnessKind, ConflictIndependencePlannerRouteWitnessKind,
    },
};
use topology::certification::{
    TopologyMilestoneFifteenPlannerSeedSupport, TopologyPublicCloseoutFreshnessRequirementPosture,
    TopologyPublicCloseoutRenderedOutputComparisonPosture,
};
use topology::facade::TopologyDerivedReuseDecisionPosture;
use worth_primitives::{truth_digest_parts, TruthDigestScope};
use worth_spatial::certification::{
    SpatialMilestoneFifteenPlannerSeedSupport, SpatialPublicCloseoutFreshnessRequirementPosture,
    SpatialPublicCloseoutRenderedOutputComparisonPosture,
};
use worth_spatial::facade::planner_owned_routing::evidence_lookup_reuse_route::EvidenceLookupReuseDecisionPosture;

use super::proof_chain_lowering::RoutePacketProofChainLowering;
use crate::workload_composition::planner_owned_routing::{
    BatchAdmissionPlannerRoutePacket, CompiledProductReusePlannerRoutePacket,
    ConflictIndependencePlannerRoutePacket,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTouchedGraphConflictSelectedRoutePacket {
    route_authority_digests: Vec<String>,
    route_lineage_digests: Vec<String>,
    overlap_identity_digests: Vec<String>,
    locality_footprint_digests: Vec<String>,
    selected_conflict_plan_digests: Vec<String>,
    independence_proof_digests: Vec<String>,
    selected_batch_plan_digest: String,
    batch_execution_receipt_digest: String,
    replay_undo_boundary_proof_digests: Vec<String>,
    replay_undo_route_packet_identity: String,
    replay_undo_route_family: ReplayUndoPlannerRouteFamily,
    batch_admission_route_packet: BatchAdmissionPlannerRoutePacket,
    conflict_independence_route_packet: ConflictIndependencePlannerRoutePacket,
    compiled_product_reuse_route_packet: CompiledProductReusePlannerRoutePacket,
    transaction_packet_identities: Vec<String>,
    replay_scope_identities: Vec<String>,
    undo_scope_identities: Vec<String>,
    evidence_lookup_public_closeout_digest: String,
    evidence_lookup_family_coverage_digest: String,
    evidence_lookup_query_surface_matrix_digest: String,
    evidence_lookup_query_consumer_kit_digest: String,
    evidence_lookup_query_boundary_support_digest: String,
    topology_query_backed_consumer_cutover_digest: String,
    topology_query_public_read_family_row_digest: String,
    topology_query_handle_identity_digest: String,
    topology_query_operating_context_identity_digest: String,
    topology_query_support_snapshot_digest: String,
    topology_query_parity_verified_count: usize,
    selected_family_identity: String,
    selected_product_identity_digest: String,
    selected_equivalence_policy_identity_digest: String,
    selected_equivalence_basis_identity_digest: String,
    selected_compatibility_basis_identity_digest: String,
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
    source_firewall_digest: String,
    deletion_closeout_digest: String,
    selected_route_identity_digest: String,
    decision_trace_identity_digest: String,
    packet_digest: String,
}

impl WorthTouchedGraphConflictSelectedRoutePacket {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        route_authority_digests: Vec<String>,
        route_lineage_digests: Vec<String>,
        overlap_identity_digests: Vec<String>,
        locality_footprint_digests: Vec<String>,
        selected_conflict_plan_digests: Vec<String>,
        independence_proof_digests: Vec<String>,
        selected_batch_plan_digest: String,
        batch_execution_receipt_digest: String,
        replay_undo_boundary_proof_digests: Vec<String>,
        replay_undo_route_packet_identity: String,
        replay_undo_route_family: ReplayUndoPlannerRouteFamily,
        batch_admission_route_packet: BatchAdmissionPlannerRoutePacket,
        conflict_independence_route_packet: ConflictIndependencePlannerRoutePacket,
        compiled_product_reuse_route_packet: CompiledProductReusePlannerRoutePacket,
        transaction_packet_identities: Vec<String>,
        replay_scope_identities: Vec<String>,
        undo_scope_identities: Vec<String>,
        evidence_lookup_public_closeout_digest: String,
        evidence_lookup_family_coverage_digest: String,
        evidence_lookup_query_surface_matrix_digest: String,
        evidence_lookup_query_consumer_kit_digest: String,
        evidence_lookup_query_boundary_support_digest: String,
        topology_query_backed_consumer_cutover_digest: String,
        topology_query_public_read_family_row_digest: String,
        topology_query_handle_identity_digest: String,
        topology_query_operating_context_identity_digest: String,
        topology_query_support_snapshot_digest: String,
        topology_query_parity_verified_count: usize,
        selected_product_identity_digest: &str,
        selected_equivalence_policy_identity_digest: &str,
        topology_support: &TopologyMilestoneFifteenPlannerSeedSupport,
        spatial_support: &SpatialMilestoneFifteenPlannerSeedSupport,
        source_firewall_digest: String,
        deletion_closeout_digest: String,
    ) -> Self {
        let selected_route_identity_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &route_authority_digests
                .iter()
                .map(|digest| format!("route-authority:{digest}"))
                .chain(std::iter::once(format!(
                    "selected-family:{}",
                    topology_support.selected_equivalence_family_identity()
                )))
                .chain(std::iter::once(format!(
                    "selected-product:{}",
                    selected_product_identity_digest
                )))
                .chain(std::iter::once(format!(
                    "selected-reuse-basis:{}",
                    topology_support.selected_reuse_basis_identity_digest()
                )))
                .chain(std::iter::once(
                    "worth-kernel:selected-route-identity:v1".to_string(),
                ))
                .collect::<Vec<_>>(),
        );
        let decision_trace_identity_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &route_lineage_digests
                .iter()
                .map(|digest| format!("route-lineage:{digest}"))
                .chain(std::iter::once(format!(
                    "route:{}",
                    selected_route_identity_digest
                )))
                .chain(
                    replay_undo_boundary_proof_digests
                        .iter()
                        .map(|digest| format!("replay-undo-proof:{digest}")),
                )
                .chain(std::iter::once(format!(
                    "decision-witness:{}",
                    compiled_product_reuse_route_packet
                        .topology_reuse_decision_identity_digest()
                        .or(compiled_product_reuse_route_packet
                            .topology_rebuild_denial_identity_digest(),)
                        .unwrap_or("not-applicable")
                )))
                .chain(std::iter::once(
                    "worth-kernel:selected-route-decision-trace:v1".to_string(),
                ))
                .collect::<Vec<_>>(),
        );
        let packet_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &overlap_identity_digests
                .iter()
                .map(|digest| format!("overlap:{digest}"))
                .chain(
                    locality_footprint_digests
                        .iter()
                        .map(|digest| format!("locality:{digest}")),
                )
                .chain(
                    selected_conflict_plan_digests
                        .iter()
                        .map(|digest| format!("selected-conflict:{digest}")),
                )
                .chain(
                    independence_proof_digests
                        .iter()
                        .map(|digest| format!("independence:{digest}")),
                )
                .chain(std::iter::once(format!(
                    "selected-batch:{selected_batch_plan_digest}"
                )))
                .chain(std::iter::once(format!(
                    "batch-admission-route:{}",
                    batch_admission_route_packet.packet_identity()
                )))
                .chain(std::iter::once(format!(
                    "conflict-independence-route:{}",
                    conflict_independence_route_packet.packet_identity()
                )))
                .chain(std::iter::once(format!(
                    "compiled-product-reuse-route:{}",
                    compiled_product_reuse_route_packet.packet_identity()
                )))
                .chain(std::iter::once(format!(
                    "execution:{batch_execution_receipt_digest}"
                )))
                .chain(std::iter::once(format!(
                    "selected-route:{selected_route_identity_digest}"
                )))
                .chain(std::iter::once(format!(
                    "decision-trace:{decision_trace_identity_digest}"
                )))
                .chain(std::iter::once(format!(
                    "firewall:{source_firewall_digest}"
                )))
                .chain(std::iter::once(format!(
                    "deletion:{deletion_closeout_digest}"
                )))
                .chain(std::iter::once(
                    "worth-kernel:selected-route-packet:v1".to_string(),
                ))
                .collect::<Vec<_>>(),
        );
        let selected_witness_identity_digest = compiled_product_reuse_route_packet
            .topology_reuse_decision_identity_digest()
            .map(str::to_string);
        let rebuild_denial_identity_digest = compiled_product_reuse_route_packet
            .topology_rebuild_denial_identity_digest()
            .map(str::to_string);
        Self {
            route_authority_digests,
            route_lineage_digests,
            overlap_identity_digests,
            locality_footprint_digests,
            selected_conflict_plan_digests,
            independence_proof_digests,
            selected_batch_plan_digest,
            batch_execution_receipt_digest,
            replay_undo_boundary_proof_digests,
            replay_undo_route_packet_identity,
            replay_undo_route_family,
            batch_admission_route_packet,
            conflict_independence_route_packet,
            compiled_product_reuse_route_packet,
            transaction_packet_identities,
            replay_scope_identities,
            undo_scope_identities,
            evidence_lookup_public_closeout_digest,
            evidence_lookup_family_coverage_digest,
            evidence_lookup_query_surface_matrix_digest,
            evidence_lookup_query_consumer_kit_digest,
            evidence_lookup_query_boundary_support_digest,
            topology_query_backed_consumer_cutover_digest,
            topology_query_public_read_family_row_digest,
            topology_query_handle_identity_digest,
            topology_query_operating_context_identity_digest,
            topology_query_support_snapshot_digest,
            topology_query_parity_verified_count,
            selected_family_identity: topology_support
                .selected_equivalence_family_identity()
                .to_string(),
            selected_product_identity_digest: selected_product_identity_digest.to_string(),
            selected_equivalence_policy_identity_digest:
                selected_equivalence_policy_identity_digest.to_string(),
            selected_equivalence_basis_identity_digest: topology_support
                .selected_equivalence_basis_identity_digest()
                .to_string(),
            selected_compatibility_basis_identity_digest: topology_support
                .selected_compatibility_basis_identity_digest()
                .to_string(),
            selected_reuse_basis_identity_digest: topology_support
                .selected_reuse_basis_identity_digest()
                .to_string(),
            selected_witness_identity_digest,
            rebuild_denial_identity_digest,
            spatial_selected_family_identity: spatial_support
                .selected_equivalence_family_identity()
                .to_string(),
            spatial_selected_product_identity_digest: spatial_support
                .compiled_product_identity_digest()
                .to_string(),
            spatial_equivalence_policy_identity_digest: spatial_support
                .equivalence_policy_identity_digest()
                .to_string(),
            topology_freshness_requirement_posture: topology_support
                .freshness_requirement_posture(),
            topology_rendered_output_comparison_posture: topology_support
                .rendered_output_comparison_posture(),
            spatial_freshness_requirement_posture: spatial_support.freshness_requirement_posture(),
            spatial_rendered_output_comparison_posture: spatial_support
                .rendered_output_comparison_posture(),
            topology_query_execution_count: topology_support.query_execution_count(),
            topology_row_scan_fallback_count: topology_support.row_scan_fallback_count(),
            topology_whole_view_fallback_count: topology_support.whole_view_fallback_count(),
            topology_repeated_rediscovery_denied_count: topology_support
                .repeated_rediscovery_denied_count(),
            spatial_receipt_proof_row_count: spatial_support.receipt_proof_row_count(),
            spatial_non_ordinary_residue_row_count: spatial_support
                .non_ordinary_residue_row_count(),
            source_firewall_digest,
            deletion_closeout_digest,
            selected_route_identity_digest,
            decision_trace_identity_digest,
            packet_digest,
        }
    }

    pub(crate) fn route_authority_digests(&self) -> &[String] {
        &self.route_authority_digests
    }
    pub(crate) fn route_lineage_digests(&self) -> &[String] {
        &self.route_lineage_digests
    }
    pub(crate) fn locality_footprint_digests(&self) -> &[String] {
        &self.locality_footprint_digests
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
    ) -> schema::facade::platform::authority::touched_graph_conflict::ConflictIndependencePlannerRouteFamily{
        self.conflict_independence_route_packet
            .conflict_route_family()
    }
    pub(crate) const fn independence_route_family(
        &self,
    ) -> schema::facade::platform::authority::touched_graph_conflict::ConflictIndependencePlannerRouteFamily{
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
        match self.conflict_independence_route_packet.denial_witness() {
            Some(witness) => Some(witness.kind()),
            None => None,
        }
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
    pub(crate) fn evidence_lookup_family_coverage_digest(&self) -> &str {
        &self.evidence_lookup_family_coverage_digest
    }
    pub(crate) fn evidence_lookup_query_surface_matrix_digest(&self) -> &str {
        &self.evidence_lookup_query_surface_matrix_digest
    }
    pub(crate) fn evidence_lookup_query_consumer_kit_digest(&self) -> &str {
        &self.evidence_lookup_query_consumer_kit_digest
    }
    pub(crate) fn evidence_lookup_query_boundary_support_digest(&self) -> &str {
        &self.evidence_lookup_query_boundary_support_digest
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
    pub(crate) fn selected_equivalence_basis_identity_digest(&self) -> &str {
        &self.selected_equivalence_basis_identity_digest
    }
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

    #[cfg(test)]
    pub(crate) fn with_test_topology_query_support_snapshot_digest_override(
        mut self,
        digest: &str,
    ) -> Self {
        self.topology_query_support_snapshot_digest = digest.to_string();
        self
    }

    #[cfg(test)]
    pub(crate) fn with_test_selected_reuse_basis_identity_digest_override(
        mut self,
        digest: &str,
    ) -> Self {
        self.selected_reuse_basis_identity_digest = digest.to_string();
        self
    }

    pub(crate) fn lower_proof_chain_inputs(&self) -> RoutePacketProofChainLowering {
        RoutePacketProofChainLowering {
            authority_digests: self.route_authority_digests.clone(),
            route_trace_markers: self.overlap_identity_digests.clone(),
            locality_footprint_digests: self.locality_footprint_digests.clone(),
            selected_conflict_plan_digests: self.selected_conflict_plan_digests.clone(),
            independence_proof_digests: self.independence_proof_digests.clone(),
            selected_batch_plan_digest: self.selected_batch_plan_digest.clone(),
            batch_execution_receipt_digest: self.batch_execution_receipt_digest.clone(),
            replay_undo_boundary_proof_digests: self.replay_undo_boundary_proof_digests.clone(),
            transaction_packet_identities: self.transaction_packet_identities.clone(),
            replay_scope_identities: self.replay_scope_identities.clone(),
            undo_scope_identities: self.undo_scope_identities.clone(),
            evidence_lookup_public_closeout_digest: self
                .evidence_lookup_public_closeout_digest
                .clone(),
            evidence_lookup_family_coverage_digest: self
                .evidence_lookup_family_coverage_digest
                .clone(),
            evidence_lookup_query_surface_matrix_digest: self
                .evidence_lookup_query_surface_matrix_digest
                .clone(),
            evidence_lookup_query_consumer_kit_digest: self
                .evidence_lookup_query_consumer_kit_digest
                .clone(),
            evidence_lookup_query_boundary_support_digest: self
                .evidence_lookup_query_boundary_support_digest
                .clone(),
            topology_query_backed_consumer_cutover_digest: self
                .topology_query_backed_consumer_cutover_digest
                .clone(),
            topology_query_public_read_family_row_digest: self
                .topology_query_public_read_family_row_digest
                .clone(),
            topology_query_handle_identity_digest: self
                .topology_query_handle_identity_digest
                .clone(),
            topology_query_operating_context_identity_digest: self
                .topology_query_operating_context_identity_digest
                .clone(),
            topology_query_support_snapshot_digest: self
                .topology_query_support_snapshot_digest
                .clone(),
            topology_query_compiled_product_identity_digest: self
                .selected_product_identity_digest
                .clone(),
            topology_query_equivalence_policy_identity_digest: self
                .selected_equivalence_policy_identity_digest
                .clone(),
            topology_query_selected_equivalence_family_identity: self
                .selected_family_identity
                .clone(),
            topology_query_selected_equivalence_basis_identity_digest: self
                .selected_equivalence_basis_identity_digest
                .clone(),
            topology_query_selected_route_gate_basis_identity_digest: self
                .selected_compatibility_basis_identity_digest
                .clone(),
            topology_query_selected_reuse_basis_identity_digest: self
                .selected_reuse_basis_identity_digest
                .clone(),
            compiled_product_reuse_route_packet_identity: self
                .compiled_product_reuse_route_packet
                .packet_identity()
                .to_string(),
            topology_reuse_posture: self.topology_reuse_posture(),
            spatial_reuse_posture: self.spatial_reuse_posture(),
            topology_query_reuse_decision_identity_digest: self
                .selected_witness_identity_digest
                .clone(),
            topology_query_rebuild_denial_identity_digest: self
                .rebuild_denial_identity_digest
                .clone(),
        }
    }
}
