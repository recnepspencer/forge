use super::identity;
use super::receipt::PlanarBooleanEdgeSplitReplayParityReceipt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidatePlanarBooleanReplayParity {
    receipt_identity: String,
    parity_receipt_identity: String,
    validated_rows: usize,
}

impl ValidatePlanarBooleanReplayParity {
    pub fn validate(receipt: &PlanarBooleanEdgeSplitReplayParityReceipt) -> Self {
        Self {
            receipt_identity: identity::validator_receipt_identity(receipt.receipt_identity()),
            parity_receipt_identity: receipt.receipt_identity().to_string(),
            validated_rows: receipt.parity_rows().len(),
        }
    }

    pub fn receipt_identity(&self) -> &str {
        &self.receipt_identity
    }

    pub fn parity_receipt_identity(&self) -> &str {
        &self.parity_receipt_identity
    }

    pub fn validated_rows(&self) -> usize {
        self.validated_rows
    }

    pub fn certifies_replay_parity_validation(&self) -> bool {
        !self.receipt_identity.is_empty()
            && !self.parity_receipt_identity.is_empty()
            && self.validated_rows >= 9
    }
}
