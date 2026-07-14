#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundedMemoryResidencySuite {
    reports: Vec<BoundedOperationEnvelopeReport>,
    denials: Vec<MemoryBoundaryDenialKind>,
    harness_evidence: HarnessCloseoutEvidenceReport,
}

impl BoundedMemoryResidencySuite {
    pub fn admit(
        reports: Vec<BoundedOperationEnvelopeReport>,
        denials: &[MemoryBoundaryDenialKind],
        harness_evidence: HarnessCloseoutEvidenceReport,
    ) -> Result<Self, BoundedMemoryResidencySuiteDenial> {
        for operation in BoundedMemoryOperationKind::ALL {
            require_report_for_operation(&reports, operation)?;
        }
        for denial in MemoryBoundaryDenialKind::ALL {
            require_contains_denial(denials, denial)?;
        }
        Ok(Self {
            reports,
            denials: denials.to_vec(),
            harness_evidence,
        })
    }

    pub fn reports(&self) -> &[BoundedOperationEnvelopeReport] {
        &self.reports
    }

    pub fn report_for(
        &self,
        operation: BoundedMemoryOperationKind,
    ) -> Option<&BoundedOperationEnvelopeReport> {
        self.reports
            .iter()
            .find(|report| report.operation() == operation)
    }

    pub fn denials(&self) -> &[MemoryBoundaryDenialKind] {
        &self.denials
    }

    pub const fn harness_evidence(&self) -> &HarnessCloseoutEvidenceReport {
        &self.harness_evidence
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoundedOperationEnvelopeReport {
    operation: BoundedMemoryOperationKind,
    counters: BoundedOperationEnvelopeCounters,
}

impl BoundedOperationEnvelopeReport {
    pub fn from_counters(
        operation: BoundedMemoryOperationKind,
        counters: BoundedOperationEnvelopeCounters,
    ) -> Result<Self, BoundedMemoryResidencySuiteDenial> {
        counters.require_admissible(operation)?;
        Ok(Self {
            operation,
            counters,
        })
    }

    pub const fn operation(self) -> BoundedMemoryOperationKind {
        self.operation
    }

    pub const fn counters(self) -> BoundedOperationEnvelopeCounters {
        self.counters
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoundedOperationEnvelopeCounters {
    resident_bytes: u64,
    pinned_pages: u64,
    dirty_pages: u32,
    allocation_bytes: u64,
    copied_bytes: u64,
    materialized_bytes: u64,
}

impl BoundedOperationEnvelopeCounters {
    pub const fn exact(
        resident_bytes: u64,
        pinned_pages: u64,
        dirty_pages: u32,
        allocation_bytes: u64,
        copied_bytes: u64,
        materialized_bytes: u64,
    ) -> Self {
        Self {
            resident_bytes,
            pinned_pages,
            dirty_pages,
            allocation_bytes,
            copied_bytes,
            materialized_bytes,
        }
    }

    pub const fn resident_bytes(self) -> u64 {
        self.resident_bytes
    }

    pub const fn pinned_pages(self) -> u64 {
        self.pinned_pages
    }

    pub const fn dirty_pages(self) -> u32 {
        self.dirty_pages
    }

    pub const fn allocation_bytes(self) -> u64 {
        self.allocation_bytes
    }

    pub const fn copied_bytes(self) -> u64 {
        self.copied_bytes
    }

    pub const fn materialized_bytes(self) -> u64 {
        self.materialized_bytes
    }

    fn require_admissible(
        self,
        operation: BoundedMemoryOperationKind,
    ) -> Result<(), BoundedMemoryResidencySuiteDenial> {
        if self.resident_bytes == 0 || self.allocation_bytes == 0 {
            return Err(BoundedMemoryResidencySuiteDenial::MissingEnvelopeCounters(
                operation,
            ));
        }
        if matches!(operation, BoundedMemoryOperationKind::LargeRecordStreaming)
            && self.copied_bytes == 0
        {
            return Err(BoundedMemoryResidencySuiteDenial::MissingEnvelopeCounters(
                operation,
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoundedMemoryOperationKind {
    AdmittedRead,
    AdmittedWrite,
    RecoveryPlanning,
    CompactionPlanning,
    LargeRecordStreaming,
}

impl BoundedMemoryOperationKind {
    pub const ALL: [Self; 5] = [
        Self::AdmittedRead,
        Self::AdmittedWrite,
        Self::RecoveryPlanning,
        Self::CompactionPlanning,
        Self::LargeRecordStreaming,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryBoundaryDenialKind {
    OverBudgetResidency,
    PinLeak,
    DirtyOverflow,
    WholeStoreMaterialization,
    WholeObjectStreaming,
    ForgedViewAccess,
}

impl MemoryBoundaryDenialKind {
    pub const ALL: [Self; 6] = [
        Self::OverBudgetResidency,
        Self::PinLeak,
        Self::DirtyOverflow,
        Self::WholeStoreMaterialization,
        Self::WholeObjectStreaming,
        Self::ForgedViewAccess,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoundedMemoryResidencySuiteDenial {
    MissingOperation(BoundedMemoryOperationKind),
    MissingEnvelopeCounters(BoundedMemoryOperationKind),
    MissingDenial(MemoryBoundaryDenialKind),
    MissingHarnessEvidence,
}

fn require_report_for_operation(
    reports: &[BoundedOperationEnvelopeReport],
    operation: BoundedMemoryOperationKind,
) -> Result<(), BoundedMemoryResidencySuiteDenial> {
    if reports.iter().any(|report| report.operation() == operation) {
        Ok(())
    } else {
        Err(BoundedMemoryResidencySuiteDenial::MissingOperation(
            operation,
        ))
    }
}

fn require_contains_denial(
    denials: &[MemoryBoundaryDenialKind],
    denial: MemoryBoundaryDenialKind,
) -> Result<(), BoundedMemoryResidencySuiteDenial> {
    if denials.contains(&denial) {
        Ok(())
    } else {
        Err(BoundedMemoryResidencySuiteDenial::MissingDenial(denial))
    }
}
use crate::HarnessCloseoutEvidenceReport;
