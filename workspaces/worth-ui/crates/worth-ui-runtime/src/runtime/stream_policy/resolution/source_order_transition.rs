use super::UiAllocationSourceOrderLedger;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiAllocationSourceOrderCommitDenial {
    PredecessorChanged,
}

/// Move-only transition from the source-order authority observed during
/// planning to the authority justified by the completely planned frame.
#[derive(Debug)]
pub(crate) struct UiAllocationSourceOrderTransition {
    predecessor: UiAllocationSourceOrderLedger,
    successor: UiAllocationSourceOrderLedger,
}

impl UiAllocationSourceOrderTransition {
    pub(super) fn new(
        predecessor: UiAllocationSourceOrderLedger,
        successor: UiAllocationSourceOrderLedger,
    ) -> Self {
        Self {
            predecessor,
            successor,
        }
    }

    pub(in crate::runtime) fn commit(
        self,
        current: &mut UiAllocationSourceOrderLedger,
    ) -> Result<(), UiAllocationSourceOrderCommitDenial> {
        if *current != self.predecessor {
            return Err(UiAllocationSourceOrderCommitDenial::PredecessorChanged);
        }
        *current = self.successor;
        Ok(())
    }
}
