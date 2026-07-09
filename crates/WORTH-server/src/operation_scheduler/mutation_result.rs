use worth_query::facade::{
    WorthQueryBatchWriteReceipt, WorthQueryBatchWriteReceiptInspection, WorthQueryWriteReceipt,
    WorthQueryWriteReceiptInspection,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthServerScheduledMutationResult {
    Single {
        receipt: WorthQueryWriteReceipt,
        inspection: WorthQueryWriteReceiptInspection,
    },
    Batch {
        receipt: WorthQueryBatchWriteReceipt,
        inspection: WorthQueryBatchWriteReceiptInspection,
    },
}

impl WorthServerScheduledMutationResult {
    pub fn result_digest(&self) -> &str {
        match self {
            Self::Single { receipt, .. } => receipt
                .commit_evidence_identity()
                .terminal_projection_for_reporting(),
            Self::Batch { receipt, .. } => receipt.batch_digest(),
        }
    }

    pub fn snapshot_basis_digest(&self) -> String {
        match self {
            Self::Single { receipt, .. } => receipt
                .snapshot_identity()
                .terminal_projection_for_reporting()
                .to_string(),
            Self::Batch { receipt, .. } => receipt
                .write_receipts()
                .last()
                .expect("scheduled batch mutation should retain at least one write receipt")
                .snapshot_identity()
                .terminal_projection_for_reporting()
                .to_string(),
        }
    }
}
