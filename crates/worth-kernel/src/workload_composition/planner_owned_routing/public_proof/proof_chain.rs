use crate::workload_composition::planner_owned_routing::WorthTouchedGraphConflictSelectedRoutePacket;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTouchedGraphConflictProofChain {
    authority_digests: Vec<String>,
    overlap_identity_digests: Vec<String>,
    locality_footprint_digests: Vec<String>,
    selected_conflict_plan_digests: Vec<String>,
    independence_proof_digests: Vec<String>,
    selected_batch_plan_digest: String,
    batch_execution_receipt_digest: String,
    replay_undo_boundary_proof_digests: Vec<String>,
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
    topology_query_compiled_product_identity_digest: String,
    topology_query_equivalence_policy_identity_digest: String,
    topology_query_selected_equivalence_family_identity: String,
    topology_query_selected_equivalence_basis_identity_digest: String,
    topology_query_selected_compatibility_basis_identity_digest: String,
    topology_query_selected_reuse_basis_identity_digest: String,
    topology_query_reuse_decision_identity_digest: Option<String>,
    topology_query_rebuild_denial_identity_digest: Option<String>,
    selected_route_identity_digest: String,
    selected_route_packet_digest: String,
    proof_chain_digest: String,
}

impl WorthTouchedGraphConflictProofChain {
    pub(crate) fn from_selected_route_packet(
        packet: &WorthTouchedGraphConflictSelectedRoutePacket,
    ) -> Self {
        let lowering = packet.lower_proof_chain_inputs();
        let authority_digests = lowering.authority_digests;
        let overlap_identity_digests = lowering.route_trace_markers;
        let locality_footprint_digests = lowering.locality_footprint_digests;
        let selected_conflict_plan_digests = lowering.selected_conflict_plan_digests;
        let independence_proof_digests = lowering.independence_proof_digests;
        let selected_batch_plan_digest = lowering.selected_batch_plan_digest;
        let batch_execution_receipt_digest = lowering.batch_execution_receipt_digest;
        let replay_undo_boundary_proof_digests = lowering.replay_undo_boundary_proof_digests;
        let transaction_packet_identities = lowering.transaction_packet_identities;
        let replay_scope_identities = lowering.replay_scope_identities;
        let undo_scope_identities = lowering.undo_scope_identities;
        let evidence_lookup_public_closeout_digest =
            lowering.evidence_lookup_public_closeout_digest;
        let evidence_lookup_family_coverage_digest =
            lowering.evidence_lookup_family_coverage_digest;
        let evidence_lookup_query_surface_matrix_digest =
            lowering.evidence_lookup_query_surface_matrix_digest;
        let evidence_lookup_query_consumer_kit_digest =
            lowering.evidence_lookup_query_consumer_kit_digest;
        let evidence_lookup_query_boundary_support_digest =
            lowering.evidence_lookup_query_boundary_support_digest;
        let topology_query_backed_consumer_cutover_digest =
            lowering.topology_query_backed_consumer_cutover_digest;
        let topology_query_public_read_family_row_digest =
            lowering.topology_query_public_read_family_row_digest;
        let topology_query_handle_identity_digest = lowering.topology_query_handle_identity_digest;
        let topology_query_operating_context_identity_digest =
            lowering.topology_query_operating_context_identity_digest;
        let topology_query_support_snapshot_digest =
            lowering.topology_query_support_snapshot_digest;
        let topology_query_compiled_product_identity_digest =
            lowering.topology_query_compiled_product_identity_digest;
        let topology_query_equivalence_policy_identity_digest =
            lowering.topology_query_equivalence_policy_identity_digest;
        let topology_query_selected_equivalence_family_identity =
            lowering.topology_query_selected_equivalence_family_identity;
        let topology_query_selected_equivalence_basis_identity_digest =
            lowering.topology_query_selected_equivalence_basis_identity_digest;
        let topology_query_selected_compatibility_basis_identity_digest =
            lowering.topology_query_selected_route_gate_basis_identity_digest;
        let topology_query_selected_reuse_basis_identity_digest =
            lowering.topology_query_selected_reuse_basis_identity_digest;
        let topology_query_reuse_decision_identity_digest =
            lowering.topology_query_reuse_decision_identity_digest;
        let topology_query_rebuild_denial_identity_digest =
            lowering.topology_query_rebuild_denial_identity_digest;
        let selected_route_identity_digest = packet.selected_route_identity_digest().to_string();
        let selected_route_packet_digest = packet.packet_digest().to_string();
        let proof_chain_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &authority_digests
                .iter()
                .map(|digest| format!("authority:{digest}"))
                .chain(
                    overlap_identity_digests
                        .iter()
                        .map(|digest| format!("overlap:{digest}")),
                )
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
                    "execution:{batch_execution_receipt_digest}"
                )))
                .chain(
                    replay_undo_boundary_proof_digests
                        .iter()
                        .map(|digest| format!("replay-undo-boundary-proof:{digest}")),
                )
                .chain(
                    transaction_packet_identities
                        .iter()
                        .map(|identity| format!("transaction-packet:{identity}")),
                )
                .chain(
                    replay_scope_identities
                        .iter()
                        .map(|identity| format!("replay-scope:{identity}")),
                )
                .chain(
                    undo_scope_identities
                        .iter()
                        .map(|identity| format!("undo-scope:{identity}")),
                )
                .chain(std::iter::once(format!(
                    "lookup-public-closeout:{evidence_lookup_public_closeout_digest}"
                )))
                .chain(std::iter::once(format!(
                    "lookup-family-coverage:{evidence_lookup_family_coverage_digest}"
                )))
                .chain(std::iter::once(format!(
                    "lookup-query-surface-matrix:{evidence_lookup_query_surface_matrix_digest}"
                )))
                .chain(std::iter::once(format!(
                    "lookup-query-consumer-kit:{evidence_lookup_query_consumer_kit_digest}"
                )))
                .chain(std::iter::once(format!(
                    "lookup-query-boundary-support:{evidence_lookup_query_boundary_support_digest}"
                )))
                .chain(std::iter::once(format!(
                    "topology-query-backed-consumer-cutover:{topology_query_backed_consumer_cutover_digest}"
                )))
                .chain(std::iter::once(format!(
                    "topology-query-public-read-family-row:{topology_query_public_read_family_row_digest}"
                )))
                .chain(std::iter::once(format!(
                    "topology-query-handle:{topology_query_handle_identity_digest}"
                )))
                .chain(std::iter::once(format!(
                    "topology-query-operating-context:{topology_query_operating_context_identity_digest}"
                )))
                .chain(std::iter::once(format!(
                    "topology-query-support-snapshot:{topology_query_support_snapshot_digest}"
                )))
                .chain(std::iter::once(format!(
                    "topology-query-compiled-product:{topology_query_compiled_product_identity_digest}"
                )))
                .chain(std::iter::once(format!(
                    "topology-query-equivalence-policy:{topology_query_equivalence_policy_identity_digest}"
                )))
                .chain(std::iter::once(format!(
                    "topology-query-selected-equivalence-family:{topology_query_selected_equivalence_family_identity}"
                )))
                .chain(std::iter::once(format!(
                    "topology-query-selected-equivalence-basis:{topology_query_selected_equivalence_basis_identity_digest}"
                )))
                .chain(std::iter::once(format!(
                    "topology-query-selected-compatibility-basis:{topology_query_selected_compatibility_basis_identity_digest}"
                )))
                .chain(std::iter::once(format!(
                    "topology-query-selected-reuse-basis:{topology_query_selected_reuse_basis_identity_digest}"
                )))
                .chain(std::iter::once(format!(
                    "topology-query-reuse-decision:{}",
                    topology_query_reuse_decision_identity_digest
                        .as_deref()
                        .unwrap_or("not-applicable")
                )))
                .chain(std::iter::once(format!(
                    "topology-query-rebuild-denial:{}",
                    topology_query_rebuild_denial_identity_digest
                        .as_deref()
                        .unwrap_or("not-applicable")
                )))
                .chain(std::iter::once(format!(
                    "selected-route:{selected_route_identity_digest}"
                )))
                .chain(std::iter::once(format!(
                    "selected-route-packet:{selected_route_packet_digest}"
                )))
                .chain(std::iter::once(
                    "worth-kernel:touched-graph-conflict-proof-chain:v1".to_string(),
                ))
                .collect::<Vec<_>>(),
        );
        Self {
            authority_digests,
            overlap_identity_digests,
            locality_footprint_digests,
            selected_conflict_plan_digests,
            independence_proof_digests,
            selected_batch_plan_digest,
            batch_execution_receipt_digest,
            replay_undo_boundary_proof_digests,
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
            topology_query_compiled_product_identity_digest,
            topology_query_equivalence_policy_identity_digest,
            topology_query_selected_equivalence_family_identity,
            topology_query_selected_equivalence_basis_identity_digest,
            topology_query_selected_compatibility_basis_identity_digest,
            topology_query_selected_reuse_basis_identity_digest,
            topology_query_reuse_decision_identity_digest,
            topology_query_rebuild_denial_identity_digest,
            selected_route_identity_digest,
            selected_route_packet_digest,
            proof_chain_digest,
        }
    }

    pub fn authority_digests(&self) -> &[String] {
        &self.authority_digests
    }
    pub fn overlap_identity_digests(&self) -> &[String] {
        &self.overlap_identity_digests
    }
    pub fn locality_footprint_digests(&self) -> &[String] {
        &self.locality_footprint_digests
    }
    pub fn selected_conflict_plan_digests(&self) -> &[String] {
        &self.selected_conflict_plan_digests
    }
    pub fn independence_proof_digests(&self) -> &[String] {
        &self.independence_proof_digests
    }
    pub fn selected_batch_plan_digest(&self) -> &str {
        &self.selected_batch_plan_digest
    }
    pub fn batch_execution_receipt_digest(&self) -> &str {
        &self.batch_execution_receipt_digest
    }
    pub fn replay_undo_boundary_proof_digests(&self) -> &[String] {
        &self.replay_undo_boundary_proof_digests
    }
    pub fn transaction_packet_identities(&self) -> &[String] {
        &self.transaction_packet_identities
    }
    pub fn replay_scope_identities(&self) -> &[String] {
        &self.replay_scope_identities
    }
    pub fn undo_scope_identities(&self) -> &[String] {
        &self.undo_scope_identities
    }
    pub fn evidence_lookup_public_closeout_digest(&self) -> &str {
        &self.evidence_lookup_public_closeout_digest
    }
    pub fn evidence_lookup_family_coverage_digest(&self) -> &str {
        &self.evidence_lookup_family_coverage_digest
    }
    pub fn evidence_lookup_query_surface_matrix_digest(&self) -> &str {
        &self.evidence_lookup_query_surface_matrix_digest
    }
    pub fn evidence_lookup_query_consumer_kit_digest(&self) -> &str {
        &self.evidence_lookup_query_consumer_kit_digest
    }
    pub fn evidence_lookup_query_boundary_support_digest(&self) -> &str {
        &self.evidence_lookup_query_boundary_support_digest
    }
    pub fn topology_query_backed_consumer_cutover_digest(&self) -> &str {
        &self.topology_query_backed_consumer_cutover_digest
    }
    pub fn topology_query_public_read_family_row_digest(&self) -> &str {
        &self.topology_query_public_read_family_row_digest
    }
    pub fn topology_query_handle_identity_digest(&self) -> &str {
        &self.topology_query_handle_identity_digest
    }
    pub fn topology_query_operating_context_identity_digest(&self) -> &str {
        &self.topology_query_operating_context_identity_digest
    }
    pub fn topology_query_support_snapshot_digest(&self) -> &str {
        &self.topology_query_support_snapshot_digest
    }
    pub fn topology_query_compiled_product_identity_digest(&self) -> &str {
        &self.topology_query_compiled_product_identity_digest
    }
    pub fn topology_query_equivalence_policy_identity_digest(&self) -> &str {
        &self.topology_query_equivalence_policy_identity_digest
    }
    pub fn topology_query_selected_equivalence_family_identity(&self) -> &str {
        &self.topology_query_selected_equivalence_family_identity
    }
    pub fn topology_query_selected_equivalence_basis_identity_digest(&self) -> &str {
        &self.topology_query_selected_equivalence_basis_identity_digest
    }
    pub fn topology_query_selected_compatibility_basis_identity_digest(&self) -> &str {
        &self.topology_query_selected_compatibility_basis_identity_digest
    }
    pub fn topology_query_selected_reuse_basis_identity_digest(&self) -> &str {
        &self.topology_query_selected_reuse_basis_identity_digest
    }
    pub fn topology_query_reuse_decision_identity_digest(&self) -> Option<&str> {
        self.topology_query_reuse_decision_identity_digest
            .as_deref()
    }
    pub fn topology_query_rebuild_denial_identity_digest(&self) -> Option<&str> {
        self.topology_query_rebuild_denial_identity_digest
            .as_deref()
    }
    pub fn selected_route_identity_digest(&self) -> &str {
        &self.selected_route_identity_digest
    }
    pub fn selected_route_packet_digest(&self) -> &str {
        &self.selected_route_packet_digest
    }
    pub fn proof_chain_digest(&self) -> &str {
        &self.proof_chain_digest
    }
}
