use forge_store_buffer_pool::{
    PrefetchAdmission, ReadAheadAdmission, ReadAheadPlan, SpeculativePhysicalWorkDenial,
    SpeculativePhysicalWorkDenialKind, SpeculativeWorkCounterSnapshot, WriteBehindAdmission,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpeculativeWorkEvidenceReport {
    row: SpeculativeWorkEvidenceRow,
    counters: SpeculativeWorkCounterSnapshot,
}

impl SpeculativeWorkEvidenceReport {
    pub fn from_read_ahead_plan(
        row: SpeculativeWorkEvidenceRow,
        plan: &ReadAheadPlan,
    ) -> Result<Self, SpeculativeWorkEvidenceDenial> {
        match row {
            SpeculativeWorkEvidenceRow::ReplayStablePlanLoweredBeforeExecution
                if plan.replay_identity().resident_frames_requested() > 0
                    && plan.counters().read_ahead_admitted_count() > 0 =>
            {
                Ok(Self {
                    row,
                    counters: plan.counters(),
                })
            }
            SpeculativeWorkEvidenceRow::ReplayStablePlanLoweredBeforeExecution => {
                Err(SpeculativeWorkEvidenceDenial::UnprovenSpeculativeWorkRow)
            }
            _ => Err(SpeculativeWorkEvidenceDenial::WrongRow),
        }
    }

    pub fn from_read_ahead_admission(
        row: SpeculativeWorkEvidenceRow,
        admission: ReadAheadAdmission,
    ) -> Result<Self, SpeculativeWorkEvidenceDenial> {
        match row {
            SpeculativeWorkEvidenceRow::MemoryAdmissionCounted
                if admission.counters().read_ahead_admitted_count() > 0
                    && admission.counters().resident_frames_requested() > 0 =>
            {
                Ok(Self {
                    row,
                    counters: admission.counters(),
                })
            }
            SpeculativeWorkEvidenceRow::NoIoQosOrThroughputClaim
                if read_ahead_admission_makes_no_io_qos_claim(admission) =>
            {
                Ok(Self {
                    row,
                    counters: admission.counters(),
                })
            }
            SpeculativeWorkEvidenceRow::MemoryAdmissionCounted
            | SpeculativeWorkEvidenceRow::NoIoQosOrThroughputClaim => {
                Err(SpeculativeWorkEvidenceDenial::UnprovenSpeculativeWorkRow)
            }
            _ => Err(SpeculativeWorkEvidenceDenial::WrongRow),
        }
    }

    pub fn from_prefetch_admission(
        admission: PrefetchAdmission,
    ) -> Result<Self, SpeculativeWorkEvidenceDenial> {
        if admission.counters().prefetch_admitted_count() == 0
            || admission.counters().resident_frames_requested() == 0
        {
            return Err(SpeculativeWorkEvidenceDenial::UnprovenSpeculativeWorkRow);
        }
        Ok(Self {
            row: SpeculativeWorkEvidenceRow::MemoryAdmissionCounted,
            counters: admission.counters(),
        })
    }

    pub fn from_prefetch_honesty(
        row: SpeculativeWorkEvidenceRow,
        admission: PrefetchAdmission,
    ) -> Result<Self, SpeculativeWorkEvidenceDenial> {
        match row {
            SpeculativeWorkEvidenceRow::NoIoQosOrThroughputClaim
                if prefetch_admission_makes_no_io_qos_claim(admission) =>
            {
                Ok(Self {
                    row,
                    counters: admission.counters(),
                })
            }
            SpeculativeWorkEvidenceRow::NoIoQosOrThroughputClaim => {
                Err(SpeculativeWorkEvidenceDenial::UnprovenSpeculativeWorkRow)
            }
            _ => Err(SpeculativeWorkEvidenceDenial::WrongRow),
        }
    }

    pub fn from_write_behind_admission(
        admission: WriteBehindAdmission,
    ) -> Result<Self, SpeculativeWorkEvidenceDenial> {
        if admission.counters().write_behind_admitted_count() == 0
            || admission.counters().dirty_pages_requested() == 0
        {
            return Err(SpeculativeWorkEvidenceDenial::UnprovenSpeculativeWorkRow);
        }
        Ok(Self {
            row: SpeculativeWorkEvidenceRow::MemoryAdmissionCounted,
            counters: admission.counters(),
        })
    }

    pub fn from_write_behind_honesty(
        row: SpeculativeWorkEvidenceRow,
        admission: WriteBehindAdmission,
    ) -> Result<Self, SpeculativeWorkEvidenceDenial> {
        match row {
            SpeculativeWorkEvidenceRow::NoIoQosOrThroughputClaim
                if write_behind_admission_makes_no_io_qos_claim(admission) =>
            {
                Ok(Self {
                    row,
                    counters: admission.counters(),
                })
            }
            SpeculativeWorkEvidenceRow::NoIoQosOrThroughputClaim => {
                Err(SpeculativeWorkEvidenceDenial::UnprovenSpeculativeWorkRow)
            }
            _ => Err(SpeculativeWorkEvidenceDenial::WrongRow),
        }
    }

    pub fn from_denial(
        row: SpeculativeWorkEvidenceRow,
        denial: SpeculativePhysicalWorkDenial,
    ) -> Result<Self, SpeculativeWorkEvidenceDenial> {
        match row {
            SpeculativeWorkEvidenceRow::DenialBeforeScheduling
                if denial_is_boundary(denial.kind())
                    && total_denials(denial.counters()) > 0 =>
            {
                Ok(Self {
                    row,
                    counters: denial.counters(),
                })
            }
            SpeculativeWorkEvidenceRow::DenialBeforeScheduling => {
                Err(SpeculativeWorkEvidenceDenial::UnprovenSpeculativeWorkRow)
            }
            _ => Err(SpeculativeWorkEvidenceDenial::WrongRow),
        }
    }

    pub const fn row(self) -> SpeculativeWorkEvidenceRow {
        self.row
    }

    pub const fn counters(self) -> SpeculativeWorkCounterSnapshot {
        self.counters
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpeculativeWorkEvidenceRow {
    ReplayStablePlanLoweredBeforeExecution,
    MemoryAdmissionCounted,
    DenialBeforeScheduling,
    NoIoQosOrThroughputClaim,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpeculativeWorkEvidenceDenial {
    WrongRow,
    UnprovenSpeculativeWorkRow,
}

const fn total_denials(counters: SpeculativeWorkCounterSnapshot) -> u64 {
    counters.read_ahead_denied_count()
        + counters.prefetch_denied_count()
        + counters.write_behind_denied_count()
}

const fn read_ahead_admission_makes_no_io_qos_claim(admission: ReadAheadAdmission) -> bool {
    !admission.proves_io_qos()
        && !admission.proves_queue_depth_correctness()
        && !admission.proves_backend_pacing()
        && !admission.proves_fsync_policy()
        && !admission.proves_fairness()
        && !admission.proves_throughput_improvement()
}

const fn prefetch_admission_makes_no_io_qos_claim(admission: PrefetchAdmission) -> bool {
    !admission.proves_io_qos()
        && !admission.proves_queue_depth_correctness()
        && !admission.proves_backend_pacing()
        && !admission.proves_fsync_policy()
        && !admission.proves_fairness()
        && !admission.proves_throughput_improvement()
}

const fn write_behind_admission_makes_no_io_qos_claim(admission: WriteBehindAdmission) -> bool {
    !admission.proves_io_qos()
        && !admission.proves_queue_depth_correctness()
        && !admission.proves_backend_pacing()
        && !admission.proves_fsync_policy()
        && !admission.proves_fairness()
        && !admission.proves_throughput_improvement()
}

const fn denial_is_boundary(kind: SpeculativePhysicalWorkDenialKind) -> bool {
    matches!(
        kind,
        SpeculativePhysicalWorkDenialKind::ResidentBudgetWouldBeExceeded { .. }
            | SpeculativePhysicalWorkDenialKind::ProtectedEvictionPressure { .. }
            | SpeculativePhysicalWorkDenialKind::DirtyBudgetWouldBeExceeded { .. }
            | SpeculativePhysicalWorkDenialKind::DirtyWorkNotResident { .. }
            | SpeculativePhysicalWorkDenialKind::PinBudgetWouldBeExceeded { .. }
            | SpeculativePhysicalWorkDenialKind::ForegroundAllocationInterference { .. }
            | SpeculativePhysicalWorkDenialKind::AllocationDenied(_)
    )
}
