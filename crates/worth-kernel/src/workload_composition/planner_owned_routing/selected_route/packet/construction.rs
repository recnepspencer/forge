use topology::certification::TopologyMilestoneFifteenPlannerSeedSupport;
use topology::derived_invalidation_route_input::TopologyInvalidationRouteInput;
use worth_primitives::{truth_digest_parts, TruthDigestScope};
use worth_spatial::certification::SpatialMilestoneFifteenPlannerSeedSupport;

use super::WorthTouchedGraphConflictSelectedRoutePacket;
use crate::workload_composition::planner_owned_routing::{
    BatchAdmissionPlannerRoutePacket, CompiledProductReusePlannerRoutePacket,
    ConflictIndependencePlannerRoutePacket,
};

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
        replay_undo_route_family: schema::facade::platform::authority::replay_undo_semantic_graph::ReplayUndoPlannerRouteFamily,
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
        evidence_lookup_query_support_digest: String,
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
        invalidation_route_input: &TopologyInvalidationRouteInput,
        source_firewall_digest: String,
        deletion_closeout_digest: String,
    ) -> Self {
        let selected_row_family_identities =
            selected_row_family_identities(invalidation_route_input);
        let selected_route_identity_digest = selected_route_identity_digest(
            &route_authority_digests,
            selected_product_identity_digest,
            topology_support,
        );
        let decision_trace_identity_digest = decision_trace_identity_digest(
            &route_lineage_digests,
            &selected_route_identity_digest,
            &replay_undo_boundary_proof_digests,
            &compiled_product_reuse_route_packet,
        );
        let packet_digest = packet_digest(
            SelectedRoutePacketDigestInput {
                overlap_identity_digests: &overlap_identity_digests,
                locality_footprint_digests: &locality_footprint_digests,
                selected_conflict_plan_digests: &selected_conflict_plan_digests,
                independence_proof_digests: &independence_proof_digests,
                selected_batch_plan_digest: &selected_batch_plan_digest,
                batch_execution_receipt_digest: &batch_execution_receipt_digest,
                selected_route_identity_digest: &selected_route_identity_digest,
                decision_trace_identity_digest: &decision_trace_identity_digest,
                source_firewall_digest: &source_firewall_digest,
                deletion_closeout_digest: &deletion_closeout_digest,
            },
            &batch_admission_route_packet,
            &conflict_independence_route_packet,
            &compiled_product_reuse_route_packet,
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
            evidence_lookup_query_support_digest,
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
            touched_closure_digest: invalidation_route_input
                .touched_closure_digest()
                .to_string(),
            touched_semantic_family_key: invalidation_route_input
                .touched_closure()
                .semantic_family_key()
                .to_string(),
            selected_plan_digest: invalidation_route_input.selected_plan_digest().to_string(),
            touched_aspect_count: invalidation_route_input
                .touched_closure()
                .counters()
                .touched_aspect_count(),
            touched_scope_count: invalidation_route_input
                .selected_plan()
                .counters()
                .touched_scope_count(),
            selected_row_family_identities,
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
}

fn selected_row_family_identities(input: &TopologyInvalidationRouteInput) -> Vec<String> {
    input
        .selected_rows()
        .iter()
        .map(|row| row.family_identity().as_str().to_string())
        .collect()
}

fn selected_route_identity_digest(
    route_authority_digests: &[String],
    selected_product_identity_digest: &str,
    topology_support: &TopologyMilestoneFifteenPlannerSeedSupport,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &route_authority_digests
            .iter()
            .map(|digest| format!("route-authority:{digest}"))
            .chain(std::iter::once(format!(
                "selected-family:{}",
                topology_support.selected_equivalence_family_identity()
            )))
            .chain(std::iter::once(format!(
                "selected-product:{selected_product_identity_digest}"
            )))
            .chain(std::iter::once(format!(
                "selected-reuse-basis:{}",
                topology_support.selected_reuse_basis_identity_digest()
            )))
            .chain(std::iter::once(
                "worth-kernel:selected-route-identity:v1".to_string(),
            ))
            .collect::<Vec<_>>(),
    )
}

fn decision_trace_identity_digest(
    route_lineage_digests: &[String],
    selected_route_identity_digest: &str,
    replay_undo_boundary_proof_digests: &[String],
    reuse_route_packet: &CompiledProductReusePlannerRoutePacket,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &route_lineage_digests
            .iter()
            .map(|digest| format!("route-lineage:{digest}"))
            .chain(std::iter::once(format!(
                "route:{selected_route_identity_digest}"
            )))
            .chain(
                replay_undo_boundary_proof_digests
                    .iter()
                    .map(|digest| format!("replay-undo-proof:{digest}")),
            )
            .chain(std::iter::once(format!(
                "decision-witness:{}",
                reuse_route_packet
                    .topology_reuse_decision_identity_digest()
                    .or(reuse_route_packet.topology_rebuild_denial_identity_digest())
                    .unwrap_or("not-applicable")
            )))
            .chain(std::iter::once(
                "worth-kernel:selected-route-decision-trace:v1".to_string(),
            ))
            .collect::<Vec<_>>(),
    )
}

struct SelectedRoutePacketDigestInput<'a> {
    overlap_identity_digests: &'a [String],
    locality_footprint_digests: &'a [String],
    selected_conflict_plan_digests: &'a [String],
    independence_proof_digests: &'a [String],
    selected_batch_plan_digest: &'a str,
    batch_execution_receipt_digest: &'a str,
    selected_route_identity_digest: &'a str,
    decision_trace_identity_digest: &'a str,
    source_firewall_digest: &'a str,
    deletion_closeout_digest: &'a str,
}

fn packet_digest(
    input: SelectedRoutePacketDigestInput<'_>,
    batch_packet: &BatchAdmissionPlannerRoutePacket,
    conflict_packet: &ConflictIndependencePlannerRoutePacket,
    reuse_packet: &CompiledProductReusePlannerRoutePacket,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &input
            .overlap_identity_digests
            .iter()
            .map(|digest| format!("overlap:{digest}"))
            .chain(
                input
                    .locality_footprint_digests
                    .iter()
                    .map(|digest| format!("locality:{digest}")),
            )
            .chain(
                input
                    .selected_conflict_plan_digests
                    .iter()
                    .map(|digest| format!("selected-conflict:{digest}")),
            )
            .chain(
                input
                    .independence_proof_digests
                    .iter()
                    .map(|digest| format!("independence:{digest}")),
            )
            .chain(std::iter::once(format!(
                "selected-batch:{}",
                input.selected_batch_plan_digest
            )))
            .chain(std::iter::once(format!(
                "batch-admission-route:{}",
                batch_packet.packet_identity()
            )))
            .chain(std::iter::once(format!(
                "conflict-independence-route:{}",
                conflict_packet.packet_identity()
            )))
            .chain(std::iter::once(format!(
                "compiled-product-reuse-route:{}",
                reuse_packet.packet_identity()
            )))
            .chain(std::iter::once(format!(
                "execution:{}",
                input.batch_execution_receipt_digest
            )))
            .chain(std::iter::once(format!(
                "selected-route:{}",
                input.selected_route_identity_digest
            )))
            .chain(std::iter::once(format!(
                "decision-trace:{}",
                input.decision_trace_identity_digest
            )))
            .chain(std::iter::once(format!(
                "firewall:{}",
                input.source_firewall_digest
            )))
            .chain(std::iter::once(format!(
                "deletion:{}",
                input.deletion_closeout_digest
            )))
            .chain(std::iter::once(
                "worth-kernel:selected-route-packet:v1".to_string(),
            ))
            .collect::<Vec<_>>(),
    )
}
