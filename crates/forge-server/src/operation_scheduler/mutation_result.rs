use forge_query::facade::{
    ForgeQueryBatchWriteReceipt, ForgeQueryBatchWriteReceiptInspection, ForgeQueryWriteReceipt,
    ForgeQueryWriteReceiptInspection,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerScheduledMutationResult {
    Single {
        receipt: ForgeQueryWriteReceipt,
        inspection: ForgeQueryWriteReceiptInspection,
    },
    Batch {
        receipt: ForgeQueryBatchWriteReceipt,
        inspection: ForgeQueryBatchWriteReceiptInspection,
    },
}

impl ForgeServerScheduledMutationResult {
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
