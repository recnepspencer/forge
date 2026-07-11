use forge_store_buffer_pool::{
    LeaseLeakReport, PinLifecycleCloseoutReport, PinLifecycleCounterSnapshot, UnpinnedPageReceipt,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PinLifecycleEvidenceReport {
    row: PinLifecycleEvidenceRow,
    counters: PinLifecycleCounterSnapshot,
}

impl PinLifecycleEvidenceReport {
    pub const fn from_explicit_unpin(receipt: UnpinnedPageReceipt) -> Self {
        Self {
            row: PinLifecycleEvidenceRow::ExplicitUnpinReceiptObserved,
            counters: receipt.counters(),
        }
    }

    pub fn from_leak_report(report: LeaseLeakReport) -> Result<Self, PinLifecycleEvidenceDenial> {
        if report.leaked_pin_count() == 0
            || report.pin_counters().leaked_pin_count() < report.leaked_pin_count()
        {
            return Err(PinLifecycleEvidenceDenial::UnprovenLifecycleRow);
        }
        Ok(Self {
            row: PinLifecycleEvidenceRow::LeakCloseoutObserved,
            counters: report.pin_counters(),
        })
    }

    pub fn from_closeout(
        row: PinLifecycleEvidenceRow,
        closeout: PinLifecycleCloseoutReport,
    ) -> Result<Self, PinLifecycleEvidenceDenial> {
        if !row.accepts_closeout() {
            return Err(PinLifecycleEvidenceDenial::WrongEvidenceRow);
        }
        let counters = closeout.pin_counters();
        row.prove_closeout_counters(counters)?;
        Ok(Self { row, counters })
    }

    pub const fn row(self) -> PinLifecycleEvidenceRow {
        self.row
    }

    pub const fn counters(self) -> PinLifecycleCounterSnapshot {
        self.counters
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PinLifecycleEvidenceRow {
    ExplicitUnpinReceiptObserved,
    DefensiveDropCleanupObserved,
    LeakCloseoutObserved,
    ProtectedFrameMutationDenied,
}

impl PinLifecycleEvidenceRow {
    const fn accepts_closeout(self) -> bool {
        matches!(
            self,
            Self::DefensiveDropCleanupObserved | Self::ProtectedFrameMutationDenied
        )
    }

    fn prove_closeout_counters(
        self,
        counters: PinLifecycleCounterSnapshot,
    ) -> Result<(), PinLifecycleEvidenceDenial> {
        match self {
            Self::DefensiveDropCleanupObserved if counters.defensive_drop_count() > 0 => Ok(()),
            Self::ProtectedFrameMutationDenied
                if counters.denied_protected_mutation_count() > 0 =>
            {
                Ok(())
            }
            Self::DefensiveDropCleanupObserved | Self::ProtectedFrameMutationDenied => {
                Err(PinLifecycleEvidenceDenial::UnprovenLifecycleRow)
            }
            Self::ExplicitUnpinReceiptObserved | Self::LeakCloseoutObserved => {
                Err(PinLifecycleEvidenceDenial::WrongEvidenceRow)
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PinLifecycleEvidenceDenial {
    WrongEvidenceRow,
    UnprovenLifecycleRow,
}
