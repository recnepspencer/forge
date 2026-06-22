use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::input::PlanarBooleanLoopReconstructionEvidenceInput;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopReconstructionEvidenceReceipt {
    receipt_identity: String,
    reconstructed_loop_set_identity: String,
    born_loop_set_identity: String,
    island_partition_identity: String,
    split_attribution_identity: String,
    role_outcome_set_identity: String,
    degenerate_outcome_set_identity: String,
    decision_log_identity: String,
    ledger_receipt_identity: String,
    downstream_consumption_identity: String,
    replay_checkpoint_identity: String,
    replay_evidence_identity: String,
}

impl PlanarBooleanLoopReconstructionEvidenceReceipt {
    pub fn admit(
        input: PlanarBooleanLoopReconstructionEvidenceInput<'_>,
    ) -> PlanarBooleanLoopReconstructionEvidenceReceipt {
        let reconstructed_loop_set_identity = input
            .reconstructed_boundary()
            .reconstructed_loops()
            .reconstructed_loop_set_identity()
            .to_string();
        let born_loop_set_identity = input
            .reconstructed_boundary()
            .born_loops()
            .born_loop_set_identity()
            .to_string();
        let island_partition_identity = input.island_partition().partition_identity().to_string();
        let split_attribution_identity =
            input.split_attribution().attribution_identity().to_string();
        let role_outcome_set_identity = input
            .role_outcomes()
            .role_outcome_set_identity()
            .to_string();
        let degenerate_outcome_set_identity = input
            .degenerate_outcomes()
            .degenerate_loop_outcome_set_identity()
            .to_string();
        let decision_log_identity = input.decision_log().decision_log_identity().to_string();
        let ledger_receipt_identity = input.ledger_receipt().receipt_identity().to_string();
        let downstream_consumption_identity = input
            .ledger_receipt()
            .downstream_consumption_identity()
            .to_string();
        let replay_checkpoint_identity = input
            .replay_receipts()
            .replay_checkpoint_identity()
            .to_string();
        let replay_evidence_identity = input
            .replay_receipts()
            .replay_evidence_identity()
            .to_string();
        let receipt_identity = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "planar-boolean-loop-reconstruction-evidence".to_string(),
                format!("reconstructed:{reconstructed_loop_set_identity}"),
                format!("born:{born_loop_set_identity}"),
                format!("islands:{island_partition_identity}"),
                format!("split-attribution:{split_attribution_identity}"),
                format!("role-outcomes:{role_outcome_set_identity}"),
                format!("degenerate-outcomes:{degenerate_outcome_set_identity}"),
                format!("decision-log:{decision_log_identity}"),
                format!("loop-ledger:{ledger_receipt_identity}"),
                format!("downstream-consumption:{downstream_consumption_identity}"),
                format!("replay-checkpoint:{replay_checkpoint_identity}"),
                format!("replay-evidence:{replay_evidence_identity}"),
            ],
        );
        Self {
            receipt_identity,
            reconstructed_loop_set_identity,
            born_loop_set_identity,
            island_partition_identity,
            split_attribution_identity,
            role_outcome_set_identity,
            degenerate_outcome_set_identity,
            decision_log_identity,
            ledger_receipt_identity,
            downstream_consumption_identity,
            replay_checkpoint_identity,
            replay_evidence_identity,
        }
    }

    pub fn receipt_identity(&self) -> &str {
        &self.receipt_identity
    }

    pub fn reconstructed_loop_set_identity(&self) -> &str {
        &self.reconstructed_loop_set_identity
    }

    pub fn born_loop_set_identity(&self) -> &str {
        &self.born_loop_set_identity
    }

    pub fn island_partition_identity(&self) -> &str {
        &self.island_partition_identity
    }

    pub fn split_attribution_identity(&self) -> &str {
        &self.split_attribution_identity
    }

    pub fn role_outcome_set_identity(&self) -> &str {
        &self.role_outcome_set_identity
    }

    pub fn degenerate_outcome_set_identity(&self) -> &str {
        &self.degenerate_outcome_set_identity
    }

    pub fn decision_log_identity(&self) -> &str {
        &self.decision_log_identity
    }

    pub fn ledger_receipt_identity(&self) -> &str {
        &self.ledger_receipt_identity
    }

    pub fn downstream_consumption_identity(&self) -> &str {
        &self.downstream_consumption_identity
    }

    pub fn replay_checkpoint_identity(&self) -> &str {
        &self.replay_checkpoint_identity
    }

    pub fn replay_evidence_identity(&self) -> &str {
        &self.replay_evidence_identity
    }
}
