use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::workload_composition::WorthTouchedGraphConflictSelectedRoutePacket;

use super::admitted_input::WorthTouchedGraphConflictMilestoneFifteenPlannerProofInput;
use super::milestone_fifteen_seed::WorthTouchedGraphConflictMilestoneFifteenSeed;
use super::types::{
    WorthTouchedGraphConflictPublicCloseoutError, WorthTouchedGraphConflictPublicCloseoutErrorKind,
};

pub(super) fn build_milestone_fifteen_seed_digest(
    seed: &WorthTouchedGraphConflictMilestoneFifteenSeed,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &seed
            .overlap_identity_digests()
            .iter()
            .map(|digest| format!("overlap:{digest}"))
            .chain(
                seed.locality_footprint_digests()
                    .iter()
                    .map(|digest| format!("locality:{digest}")),
            )
            .chain(
                seed.selected_conflict_plan_digests()
                    .iter()
                    .map(|digest| format!("selected-conflict:{digest}")),
            )
            .chain(
                seed.independence_proof_digests()
                    .iter()
                    .map(|digest| format!("independence:{digest}")),
            )
            .chain(std::iter::once(format!(
                "selected-batch:{}",
                seed.selected_batch_plan_digest()
            )))
            .chain(std::iter::once(format!(
                "execution:{}",
                seed.batch_execution_receipt_digest()
            )))
            .chain(
                seed.replay_undo_boundary_proof_digests()
                    .iter()
                    .map(|digest| format!("replay-undo-boundary-proof:{digest}")),
            )
            .chain(
                seed.transaction_packet_identities()
                    .iter()
                    .map(|identity| format!("transaction-packet:{identity}")),
            )
            .chain(
                seed.replay_scope_identities()
                    .iter()
                    .map(|identity| format!("replay-scope:{identity}")),
            )
            .chain(
                seed.undo_scope_identities()
                    .iter()
                    .map(|identity| format!("undo-scope:{identity}")),
            )
            .chain(std::iter::once(format!(
                "topology-compiled-product:{}",
                seed.topology_compiled_product_identity_digest()
            )))
            .chain(std::iter::once(format!(
                "topology-equivalence-policy:{}",
                seed.topology_equivalence_policy_identity_digest()
            )))
            .chain(std::iter::once(format!(
                "lookup-public-closeout:{}",
                seed.evidence_lookup_public_closeout_digest()
            )))
            .chain(std::iter::once(format!(
                "lookup-query-boundary-support:{}",
                seed.evidence_lookup_query_boundary_support_digest()
            )))
            .chain(std::iter::once(format!(
                "topology-query-backed-consumer-cutover:{}",
                seed.topology_query_backed_consumer_cutover_digest()
            )))
            .chain(std::iter::once(format!(
                "topology-query-public-read-family-row:{}",
                seed.topology_query_public_read_family_row_digest()
            )))
            .chain(std::iter::once(format!(
                "topology-query-selected-equivalence-family:{}",
                seed.topology_query_selected_equivalence_family_identity()
            )))
            .chain(std::iter::once(format!(
                "topology-query-selected-equivalence-basis:{}",
                seed.topology_query_selected_equivalence_basis_identity_digest()
            )))
            .chain(std::iter::once(format!(
                "topology-query-selected-compatibility-basis:{}",
                seed.topology_query_selected_compatibility_basis_identity_digest()
            )))
            .chain(std::iter::once(format!(
                "topology-query-selected-reuse-basis:{}",
                seed.topology_query_selected_reuse_basis_identity_digest()
            )))
            .chain(std::iter::once(format!(
                "topology-query-reuse-decision:{}",
                seed.topology_query_reuse_decision_identity_digest()
                    .unwrap_or("not-applicable")
            )))
            .chain(std::iter::once(format!(
                "topology-query-rebuild-denial:{}",
                seed.topology_query_rebuild_denial_identity_digest()
                    .unwrap_or("not-applicable")
            )))
            .chain(std::iter::once(format!(
                "compiled-product-reuse-route-packet:{}",
                seed.compiled_product_reuse_route_packet_identity()
            )))
            .chain(std::iter::once(format!(
                "spatial-compiled-product:{}",
                seed.spatial_compiled_product_identity_digest()
            )))
            .chain(std::iter::once(format!(
                "spatial-equivalence-policy:{}",
                seed.spatial_equivalence_policy_identity_digest()
            )))
            .chain(std::iter::once(format!(
                "spatial-selected-equivalence-family:{}",
                seed.spatial_selected_equivalence_family_identity()
            )))
            .chain(std::iter::once(format!(
                "spatial-selected-reuse-basis:{}",
                seed.spatial_selected_reuse_basis_identity_digest()
            )))
            .chain(std::iter::once(format!(
                "spatial-reuse-decision:{}",
                seed.spatial_reuse_decision_identity_digest()
                    .unwrap_or("not-applicable")
            )))
            .chain(std::iter::once(format!(
                "spatial-rebuild-denial:{}",
                seed.spatial_rebuild_denial_identity_digest()
                    .unwrap_or("not-applicable")
            )))
            .chain(std::iter::once(format!(
                "residue:{}",
                seed.residue_digest()
            )))
            .chain(std::iter::once(format!(
                "firewall:{}",
                seed.source_firewall_digest()
            )))
            .chain(std::iter::once(
                "worth-kernel:touched-graph-conflict-milestone-fifteen-seed:v1".to_string(),
            ))
            .collect::<Vec<_>>(),
    )
}

pub(super) fn require_planner_proof_input_matches_selected_route_packet(
    packet: &WorthTouchedGraphConflictSelectedRoutePacket,
    planner_proof_input: &WorthTouchedGraphConflictMilestoneFifteenPlannerProofInput,
) -> Result<(), WorthTouchedGraphConflictPublicCloseoutError> {
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
        || planner_proof_input.spatial_selected_reuse_basis_identity_digest()
            != packet.spatial_selected_reuse_basis_identity_digest()
    {
        return Err(WorthTouchedGraphConflictPublicCloseoutError::new(
            WorthTouchedGraphConflictPublicCloseoutErrorKind::IncompleteProofChain,
            "Milestone 15 planner proof input must preserve selected-route packet reuse authority",
        ));
    }
    Ok(())
}
