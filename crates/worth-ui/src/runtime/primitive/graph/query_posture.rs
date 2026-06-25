use crate::runtime::{WorthUiQueryProjectionFactReceipt, WorthUiQuerySupportDenialReceipt};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiPrimitiveQueryPosture {
    Untouched,
    ProjectionFactsRequired,
    ProjectionFactsConsumed {
        receipts: Vec<WorthUiQueryProjectionFactReceipt>,
        support_receipt_digest: u64,
    },
    SupportDenied {
        denial_receipt: WorthUiQuerySupportDenialReceipt,
    },
}

impl WorthUiPrimitiveQueryPosture {
    pub fn token(&self) -> &'static str {
        match self {
            Self::Untouched => "untouched",
            Self::ProjectionFactsRequired => "projection_facts_required",
            Self::ProjectionFactsConsumed { .. } => "projection_facts_consumed",
            Self::SupportDenied { .. } => "support_denied",
        }
    }

    pub fn digest(&self) -> u64 {
        let mut digest = 0xcbf2_9ce4_8422_2325;
        digest = fold(digest, self.token().as_bytes());
        match self {
            Self::Untouched | Self::ProjectionFactsRequired => digest,
            Self::ProjectionFactsConsumed {
                receipts,
                support_receipt_digest,
            } => receipts.iter().fold(
                fold(digest, &support_receipt_digest.to_le_bytes()),
                |acc, receipt| fold(acc, &receipt.receipt_digest().to_le_bytes()),
            ),
            Self::SupportDenied { denial_receipt } => fold(
                digest,
                &denial_receipt.support_receipt_digest().to_le_bytes(),
            ),
        }
    }
}

fn fold(mut digest: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        digest ^= u64::from(*byte);
        digest = digest.wrapping_mul(0x0000_0100_0000_01b3);
    }
    digest
}
