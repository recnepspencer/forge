use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::workload_composition::worth_workload::WorthWorkloadOrdinaryConsumerCutover;

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
    proof_chain_digest: String,
}

impl WorthTouchedGraphConflictProofChain {
    pub(crate) fn from_cutover(cutover: &WorthWorkloadOrdinaryConsumerCutover) -> Self {
        let receipt = cutover.batch_execution_receipt();
        let authority_digests = receipt.authority_digests().to_vec();
        let overlap_identity_digests = receipt.overlap_identity_digests().to_vec();
        let locality_footprint_digests = receipt.locality_footprint_digests().to_vec();
        let selected_conflict_plan_digests = receipt.selected_conflict_plan_digests().to_vec();
        let independence_proof_digests = receipt.independence_proof_identities().to_vec();
        let selected_batch_plan_digest = receipt.selected_batch_plan_digest().to_string();
        let batch_execution_receipt_digest = receipt.execution_receipt_digest().to_string();
        let replay_undo_boundary_proof_digests = cutover.replay_undo_boundary_proof_digests();
        let transaction_packet_identities = cutover.transaction_packet_identities();
        let replay_scope_identities = cutover.replay_scope_identities();
        let undo_scope_identities = cutover.undo_scope_identities();
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

    pub fn proof_chain_digest(&self) -> &str {
        &self.proof_chain_digest
    }
}
