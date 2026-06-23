use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanLoopReconstructionEvidenceReceipt, PlanarBooleanLoopReconstructionLedgerReceipt,
};
use crate::workload_platform::retained_replay_workload::ReplayReceiptSet;

use super::denial::{PlanarBooleanLoopReplayParityDenial, PlanarBooleanLoopReplayParityDenialKind};

#[derive(Clone, Copy, Debug)]
pub struct PlanarBooleanLoopReplayParityInput<'a> {
    original_ledger_receipt: &'a PlanarBooleanLoopReconstructionLedgerReceipt,
    replayed_ledger_receipt: &'a PlanarBooleanLoopReconstructionLedgerReceipt,
    original_evidence_receipt: &'a PlanarBooleanLoopReconstructionEvidenceReceipt,
    replayed_evidence_receipt: &'a PlanarBooleanLoopReconstructionEvidenceReceipt,
    replay_receipts: &'a ReplayReceiptSet,
}

impl<'a> PlanarBooleanLoopReplayParityInput<'a> {
    pub fn admit_from_ledger_and_evidence(
        original_ledger_receipt: &'a PlanarBooleanLoopReconstructionLedgerReceipt,
        replayed_ledger_receipt: &'a PlanarBooleanLoopReconstructionLedgerReceipt,
        original_evidence_receipt: &'a PlanarBooleanLoopReconstructionEvidenceReceipt,
        replayed_evidence_receipt: &'a PlanarBooleanLoopReconstructionEvidenceReceipt,
        replay_receipts: &'a ReplayReceiptSet,
    ) -> Result<Self, PlanarBooleanLoopReplayParityDenial> {
        if original_evidence_receipt.ledger_receipt_identity()
            != original_ledger_receipt.receipt_identity()
        {
            return Err(PlanarBooleanLoopReplayParityDenial::new(
                PlanarBooleanLoopReplayParityDenialKind::LoopEvidenceMismatch,
                original_ledger_receipt.receipt_identity(),
                original_evidence_receipt.ledger_receipt_identity(),
                Default::default(),
            ));
        }
        if replayed_evidence_receipt.ledger_receipt_identity()
            != replayed_ledger_receipt.receipt_identity()
        {
            return Err(PlanarBooleanLoopReplayParityDenial::new(
                PlanarBooleanLoopReplayParityDenialKind::LoopEvidenceMismatch,
                replayed_ledger_receipt.receipt_identity(),
                replayed_evidence_receipt.ledger_receipt_identity(),
                Default::default(),
            ));
        }
        if replay_receipts.replay_checkpoint_identity()
            != original_evidence_receipt.replay_checkpoint_identity()
            || replay_receipts.replay_checkpoint_identity()
                != replayed_evidence_receipt.replay_checkpoint_identity()
        {
            return Err(PlanarBooleanLoopReplayParityDenial::new(
                PlanarBooleanLoopReplayParityDenialKind::CheckpointAuthorityMismatch,
                original_evidence_receipt.replay_checkpoint_identity(),
                replay_receipts.replay_checkpoint_identity(),
                Default::default(),
            ));
        }
        if replay_receipts.replay_evidence_identity()
            != original_evidence_receipt.replay_evidence_identity()
            || replay_receipts.replay_evidence_identity()
                != replayed_evidence_receipt.replay_evidence_identity()
        {
            return Err(PlanarBooleanLoopReplayParityDenial::new(
                PlanarBooleanLoopReplayParityDenialKind::CheckpointAuthorityMismatch,
                original_evidence_receipt.replay_evidence_identity(),
                replay_receipts.replay_evidence_identity(),
                Default::default(),
            ));
        }
        Ok(Self {
            original_ledger_receipt,
            replayed_ledger_receipt,
            original_evidence_receipt,
            replayed_evidence_receipt,
            replay_receipts,
        })
    }

    pub(crate) fn original_ledger_receipt(
        self,
    ) -> &'a PlanarBooleanLoopReconstructionLedgerReceipt {
        self.original_ledger_receipt
    }

    pub(crate) fn replayed_ledger_receipt(
        self,
    ) -> &'a PlanarBooleanLoopReconstructionLedgerReceipt {
        self.replayed_ledger_receipt
    }

    pub(crate) fn original_evidence_receipt(
        self,
    ) -> &'a PlanarBooleanLoopReconstructionEvidenceReceipt {
        self.original_evidence_receipt
    }

    pub(crate) fn replayed_evidence_receipt(
        self,
    ) -> &'a PlanarBooleanLoopReconstructionEvidenceReceipt {
        self.replayed_evidence_receipt
    }

    pub(crate) fn replay_receipts(self) -> &'a ReplayReceiptSet {
        self.replay_receipts
    }
}
