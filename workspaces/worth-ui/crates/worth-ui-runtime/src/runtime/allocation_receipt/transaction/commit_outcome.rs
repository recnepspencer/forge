use super::{UiAllocationReceipt, UiAllocationReceiptCommitDenial, UiAllocationReceiptReport};

/// Complete result of attempting to advance allocation receipt lineage.
#[derive(Clone, Debug, PartialEq)]
pub enum UiAllocationReceiptCommitOutcome {
    Committed(UiAllocationReceipt),
    RecomputePending(UiAllocationReceiptReport),
    Denied(UiAllocationReceiptCommitDenial),
}

impl UiAllocationReceiptCommitOutcome {
    #[cfg(test)]
    pub(crate) fn expect(self, message: &str) -> UiAllocationReceipt {
        match self {
            Self::Committed(receipt) => receipt,
            outcome => panic!("{message}: {outcome:?}"),
        }
    }

    #[cfg(test)]
    pub(crate) fn expect_err(self, message: &str) -> UiAllocationReceiptCommitDenial {
        match self {
            Self::Denied(denial) => denial,
            outcome => panic!("{message}: {outcome:?}"),
        }
    }
}
