use super::counters::PlanarBooleanEdgeSplitReplayParityCounters;
use super::identity;
use super::replay_rows::PlanarBooleanEdgeSplitReplayParityRow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanEdgeSplitReplayParityReceipt {
    receipt_identity: String,
    retained_replay_stage_identity: String,
    replay_checkpoint_identity: String,
    replay_evidence_identity: String,
    replay_product_identity: String,
    replay_closure_manifest_identity: String,
    original_split_request_identity: String,
    replayed_split_request_identity: String,
    original_split_ledger_receipt_identity: String,
    replayed_split_ledger_receipt_identity: String,
    original_downstream_consumption_identity: String,
    replayed_downstream_consumption_identity: String,
    parity_rows: Vec<PlanarBooleanEdgeSplitReplayParityRow>,
    counters: PlanarBooleanEdgeSplitReplayParityCounters,
}

impl PlanarBooleanEdgeSplitReplayParityReceipt {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        retained_replay_stage_identity: String,
        replay_checkpoint_identity: String,
        replay_evidence_identity: String,
        replay_product_identity: String,
        replay_closure_manifest_identity: String,
        original_split_request_identity: String,
        replayed_split_request_identity: String,
        original_split_ledger_receipt_identity: String,
        replayed_split_ledger_receipt_identity: String,
        original_downstream_consumption_identity: String,
        replayed_downstream_consumption_identity: String,
        parity_rows: Vec<PlanarBooleanEdgeSplitReplayParityRow>,
        counters: PlanarBooleanEdgeSplitReplayParityCounters,
    ) -> Self {
        let receipt_identity = identity::replay_parity_identity(
            &retained_replay_stage_identity,
            &replay_checkpoint_identity,
            &replay_evidence_identity,
            &parity_rows,
        );
        Self {
            receipt_identity,
            retained_replay_stage_identity,
            replay_checkpoint_identity,
            replay_evidence_identity,
            replay_product_identity,
            replay_closure_manifest_identity,
            original_split_request_identity,
            replayed_split_request_identity,
            original_split_ledger_receipt_identity,
            replayed_split_ledger_receipt_identity,
            original_downstream_consumption_identity,
            replayed_downstream_consumption_identity,
            parity_rows,
            counters,
        }
    }

    pub fn receipt_identity(&self) -> &str {
        &self.receipt_identity
    }

    pub fn retained_replay_stage_identity(&self) -> &str {
        &self.retained_replay_stage_identity
    }

    pub fn replay_checkpoint_identity(&self) -> &str {
        &self.replay_checkpoint_identity
    }

    pub fn replay_evidence_identity(&self) -> &str {
        &self.replay_evidence_identity
    }

    pub fn replay_product_identity(&self) -> &str {
        &self.replay_product_identity
    }

    pub fn replay_closure_manifest_identity(&self) -> &str {
        &self.replay_closure_manifest_identity
    }

    pub fn original_split_request_identity(&self) -> &str {
        &self.original_split_request_identity
    }

    pub fn replayed_split_request_identity(&self) -> &str {
        &self.replayed_split_request_identity
    }

    pub fn original_split_ledger_receipt_identity(&self) -> &str {
        &self.original_split_ledger_receipt_identity
    }

    pub fn replayed_split_ledger_receipt_identity(&self) -> &str {
        &self.replayed_split_ledger_receipt_identity
    }

    pub fn original_downstream_consumption_identity(&self) -> &str {
        &self.original_downstream_consumption_identity
    }

    pub fn replayed_downstream_consumption_identity(&self) -> &str {
        &self.replayed_downstream_consumption_identity
    }

    pub fn parity_rows(&self) -> &[PlanarBooleanEdgeSplitReplayParityRow] {
        &self.parity_rows
    }

    pub fn counters(&self) -> PlanarBooleanEdgeSplitReplayParityCounters {
        self.counters
    }

    pub fn certifies_planar_boolean_replay_parity(&self) -> bool {
        !self.receipt_identity.is_empty()
            && !self.retained_replay_stage_identity.is_empty()
            && !self.replay_checkpoint_identity.is_empty()
            && !self.replay_evidence_identity.is_empty()
            && !self.replay_product_identity.is_empty()
            && !self.replay_closure_manifest_identity.is_empty()
            && self.original_split_request_identity == self.replayed_split_request_identity
            && self.original_split_ledger_receipt_identity
                == self.replayed_split_ledger_receipt_identity
            && self.original_downstream_consumption_identity
                == self.replayed_downstream_consumption_identity
            && self.parity_rows.iter().all(|row| row.certifies_match())
            && self.counters.split_request_rows_compared() >= 1
            && self.counters.ledger_identity_rows_compared() >= 2
            && self.counters.decision_log_rows_compared() >= 1
            && self.counters.operational_truth_rows_compared() >= 1
            && self.counters.fragment_identity_rows_compared() >= 1
            && self.counters.overlap_chain_rows_compared() >= 1
            && self.counters.persistent_naming_rows_compared() >= 1
            && self.counters.checkpoint_rows_compared() >= 1
            && self.counters.orientation_rows_compared() >= 1
            && self.counters.replay_closure_rows_compared() >= 20
            && self.counters.closeout_rows_read() >= 2
            && self.counters.retained_replay_rows_read() >= 1
            && self.counters.replay_rows_emitted() >= 1
            && self.counters.event_extraction_reexecutions() == 0
            && self.counters.candidate_index_reexecutions() == 0
            && self.counters.replay_mismatches_rejected() == 0
            && self.counters.checkpoint_mismatches_rejected() == 0
            && self.counters.orientation_mismatches_rejected() == 0
    }
}
