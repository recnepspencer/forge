use worth_store_buffer_pool::{
    DirtyPageCounterSnapshot, DirtyPageState, DirtyPublicationPlan, DirtyPublicationReceipt,
    DirtyShutdownPosture, DirtyShutdownReport, ResidentFrameDenial, ResidentFrameDenialKind,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirtyPublicationEvidenceReport {
    row: DirtyPublicationEvidenceRow,
    counters: DirtyPageCounterSnapshot,
}

impl DirtyPublicationEvidenceReport {
    pub const fn from_dirty_state(state: DirtyPageState) -> Self {
        Self {
            row: DirtyPublicationEvidenceRow::DirtyStateAdmittedAndCounted,
            counters: state.counters(),
        }
    }

    pub fn from_publication_plan(
        plan: &DirtyPublicationPlan,
    ) -> Result<Self, DirtyPublicationEvidenceDenial> {
        if plan.proves_durability() {
            return Err(DirtyPublicationEvidenceDenial::DurabilityClaimRejected);
        }
        Ok(Self {
            row: DirtyPublicationEvidenceRow::PublicationPlanIsSchedulingOnly,
            counters: plan.counters(),
        })
    }

    pub fn from_publication_receipt(
        receipt: DirtyPublicationReceipt,
    ) -> Result<Self, DirtyPublicationEvidenceDenial> {
        if receipt.proves_durability() {
            return Err(DirtyPublicationEvidenceDenial::DurabilityClaimRejected);
        }
        if receipt.released_dirty_pages().as_pages() == 0
            || receipt.write_scheduling_attempt_count() == 0
        {
            return Err(DirtyPublicationEvidenceDenial::UnprovenDirtyRow);
        }
        Ok(Self {
            row: DirtyPublicationEvidenceRow::PublicationReceiptScheduledWriteOnly,
            counters: receipt.counters(),
        })
    }

    pub fn from_shutdown(
        report: DirtyShutdownReport,
    ) -> Result<Self, DirtyPublicationEvidenceDenial> {
        if report.proves_durability() {
            return Err(DirtyPublicationEvidenceDenial::DurabilityClaimRejected);
        }
        match report.posture() {
            DirtyShutdownPosture::CleanNoDirtyPages
                if report.unflushed_dirty_pages().as_pages() == 0 =>
            {
                Ok(Self {
                    row: DirtyPublicationEvidenceRow::CleanDirtyShutdownObserved,
                    counters: report.counters(),
                })
            }
            DirtyShutdownPosture::UnflushedDirtyPagesRemain
                if report.unflushed_dirty_pages().as_pages() > 0
                    && report.counters().dirty_shutdown_unflushed_count() > 0 =>
            {
                Ok(Self {
                    row: DirtyPublicationEvidenceRow::UnflushedDirtyShutdownObserved,
                    counters: report.counters(),
                })
            }
            DirtyShutdownPosture::CleanNoDirtyPages
            | DirtyShutdownPosture::UnflushedDirtyPagesRemain => {
                Err(DirtyPublicationEvidenceDenial::UnprovenDirtyRow)
            }
        }
    }

    pub fn from_denial(
        row: DirtyPublicationEvidenceRow,
        denial: ResidentFrameDenial,
        counters: DirtyPageCounterSnapshot,
    ) -> Result<Self, DirtyPublicationEvidenceDenial> {
        row.prove_denial(denial, counters)?;
        Ok(Self { row, counters })
    }

    pub const fn row(self) -> DirtyPublicationEvidenceRow {
        self.row
    }

    pub const fn counters(self) -> DirtyPageCounterSnapshot {
        self.counters
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirtyPublicationEvidenceRow {
    DirtyStateAdmittedAndCounted,
    DirtyBudgetDeniedBeforeScheduling,
    ConflictingLeasePublicationDeniedBeforeScheduling,
    StalePublicationPlanDeniedBeforeScheduling,
    PublicationPlanIsSchedulingOnly,
    PublicationReceiptScheduledWriteOnly,
    CleanDirtyShutdownObserved,
    UnflushedDirtyShutdownObserved,
}

impl DirtyPublicationEvidenceRow {
    fn prove_denial(
        self,
        denial: ResidentFrameDenial,
        counters: DirtyPageCounterSnapshot,
    ) -> Result<(), DirtyPublicationEvidenceDenial> {
        match (self, denial.kind()) {
            (
                Self::DirtyBudgetDeniedBeforeScheduling,
                ResidentFrameDenialKind::DirtyPageBudgetExceeded,
            ) if counters.dirty_budget_denial_count() > 0
                && counters.write_scheduling_attempt_count() == 0 =>
            {
                Ok(())
            }
            (
                Self::ConflictingLeasePublicationDeniedBeforeScheduling,
                ResidentFrameDenialKind::DirtyPublicationBehindActiveLease,
            ) if (counters.publication_plan_denial_count() > 0
                || counters.write_scheduling_denial_count() > 0)
                && counters.write_scheduling_attempt_count() == 0 =>
            {
                Ok(())
            }
            (
                Self::StalePublicationPlanDeniedBeforeScheduling,
                ResidentFrameDenialKind::DirtyPublicationPlanStale,
            ) if counters.stale_publication_plan_denial_count() > 0
                && counters.write_scheduling_attempt_count()
                    == counters.publication_receipt_count() =>
            {
                Ok(())
            }
            (
                Self::DirtyBudgetDeniedBeforeScheduling
                | Self::ConflictingLeasePublicationDeniedBeforeScheduling
                | Self::StalePublicationPlanDeniedBeforeScheduling,
                _,
            ) => Err(DirtyPublicationEvidenceDenial::DenialMismatch),
            (
                Self::DirtyStateAdmittedAndCounted
                | Self::PublicationPlanIsSchedulingOnly
                | Self::PublicationReceiptScheduledWriteOnly
                | Self::CleanDirtyShutdownObserved
                | Self::UnflushedDirtyShutdownObserved,
                _,
            ) => Err(DirtyPublicationEvidenceDenial::WrongEvidenceRow),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirtyPublicationEvidenceDenial {
    WrongEvidenceRow,
    DenialMismatch,
    DurabilityClaimRejected,
    UnprovenDirtyRow,
}
