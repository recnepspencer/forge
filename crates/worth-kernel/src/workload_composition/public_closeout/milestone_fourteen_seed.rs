use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::proof_chain::WorthTouchedGraphConflictProofChain;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTouchedGraphConflictMilestoneFourteenSeed {
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
    residue_digest: String,
    source_firewall_digest: String,
    seed_digest: String,
}

impl WorthTouchedGraphConflictMilestoneFourteenSeed {
    pub(crate) fn from_closeout_parts(
        proof_chain: &WorthTouchedGraphConflictProofChain,
        residue_digest: &str,
        source_firewall_digest: &str,
    ) -> Self {
        let overlap_identity_digests = proof_chain.overlap_identity_digests().to_vec();
        let locality_footprint_digests = proof_chain.locality_footprint_digests().to_vec();
        let selected_conflict_plan_digests = proof_chain.selected_conflict_plan_digests().to_vec();
        let independence_proof_digests = proof_chain.independence_proof_digests().to_vec();
        let selected_batch_plan_digest = proof_chain.selected_batch_plan_digest().to_string();
        let batch_execution_receipt_digest =
            proof_chain.batch_execution_receipt_digest().to_string();
        let replay_undo_boundary_proof_digests =
            proof_chain.replay_undo_boundary_proof_digests().to_vec();
        let transaction_packet_identities = proof_chain.transaction_packet_identities().to_vec();
        let replay_scope_identities = proof_chain.replay_scope_identities().to_vec();
        let undo_scope_identities = proof_chain.undo_scope_identities().to_vec();
        let seed_digest = truth_digest_parts(
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
                .chain(std::iter::once(format!("residue:{residue_digest}")))
                .chain(std::iter::once(format!(
                    "firewall:{source_firewall_digest}"
                )))
                .chain(std::iter::once(
                    "worth-kernel:touched-graph-conflict-milestone-fourteen-seed:v1".to_string(),
                ))
                .collect::<Vec<_>>(),
        );
        Self {
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
            residue_digest: residue_digest.to_string(),
            source_firewall_digest: source_firewall_digest.to_string(),
            seed_digest,
        }
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

    pub fn residue_digest(&self) -> &str {
        &self.residue_digest
    }

    pub fn source_firewall_digest(&self) -> &str {
        &self.source_firewall_digest
    }

    pub fn seed_digest(&self) -> &str {
        &self.seed_digest
    }
}
