use crate::foundational_boundary_performance::{
    allocation_rows, copy_rows, counter_receipt, resident_rows, FoundationalStoreCounterReceipt,
};
use forge_foundational::{
    FoundationalCounterBackedPerformanceReceiptConstructionDenial,
    FoundationalPerformanceBundleConstructionDenial,
};
use forge_store_buffer_pool::{
    AllocationCounterSnapshot, AllocationScope, BufferPoolCounterSnapshot,
    BufferPoolExecutedEvidenceSource, RecordCopyCounterSnapshot, ResidentFrameCounterSnapshot,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalEvidenceRichness {
    Full,
    Reduced,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalEvidenceProfile {
    richness: FoundationalEvidenceRichness,
}

impl FoundationalEvidenceProfile {
    pub const fn full() -> Self {
        Self {
            richness: FoundationalEvidenceRichness::Full,
        }
    }

    pub const fn reduced() -> Self {
        Self {
            richness: FoundationalEvidenceRichness::Reduced,
        }
    }

    pub const fn richness(self) -> FoundationalEvidenceRichness {
        self.richness
    }

    pub const fn optional_diagnostic_count(self) -> u8 {
        match self.richness {
            FoundationalEvidenceRichness::Full => 3,
            FoundationalEvidenceRichness::Reduced => 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalBoundaryAuthorityResult {
    CounterBackedStoreExecution,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResidentMemoryPerformanceReceipt {
    counters: ResidentFrameCounterSnapshot,
    performance_receipt: FoundationalStoreCounterReceipt,
}

impl ResidentMemoryPerformanceReceipt {
    pub(crate) fn from_executed_counters(
        counters: ResidentFrameCounterSnapshot,
    ) -> Result<Self, FoundationalBoundaryEvidenceDenial> {
        if counters.resident_bytes().as_bytes() == 0 || counters.miss_count() == 0 {
            return Err(FoundationalBoundaryEvidenceDenial::MissingResidentMemoryCounters);
        }
        let rows = resident_rows(counters);
        Ok(Self {
            counters,
            performance_receipt: counter_receipt("store.buffer_pool.resident_memory", &rows)?,
        })
    }

    pub const fn counters(&self) -> ResidentFrameCounterSnapshot {
        self.counters
    }

    pub const fn performance_receipt(&self) -> &FoundationalStoreCounterReceipt {
        &self.performance_receipt
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllocationEnvelopePerformanceReceipt {
    counters: AllocationCounterSnapshot,
    performance_receipt: FoundationalStoreCounterReceipt,
}

impl AllocationEnvelopePerformanceReceipt {
    pub(crate) fn from_executed_counters(
        counters: AllocationCounterSnapshot,
    ) -> Result<Self, FoundationalBoundaryEvidenceDenial> {
        if allocation_allocated_bytes(counters) == 0 || allocation_requested_bytes(counters) == 0 {
            return Err(FoundationalBoundaryEvidenceDenial::MissingAllocationCounters);
        }
        let rows = allocation_rows(counters);
        Ok(Self {
            counters,
            performance_receipt: counter_receipt("store.buffer_pool.allocation", &rows)?,
        })
    }

    pub const fn counters(&self) -> AllocationCounterSnapshot {
        self.counters
    }

    pub const fn performance_receipt(&self) -> &FoundationalStoreCounterReceipt {
        &self.performance_receipt
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CopyMaterializationPerformanceReceipt {
    counters: RecordCopyCounterSnapshot,
    performance_receipt: FoundationalStoreCounterReceipt,
}

impl CopyMaterializationPerformanceReceipt {
    pub(crate) fn from_executed_counters(
        counters: RecordCopyCounterSnapshot,
    ) -> Result<Self, FoundationalBoundaryEvidenceDenial> {
        if counters.zero_copy_admission_count() == 0
            || counters.bounded_copy_count() == 0
            || counters.copied_bytes() == 0
        {
            return Err(FoundationalBoundaryEvidenceDenial::MissingCopyCounters);
        }
        let rows = copy_rows(counters);
        Ok(Self {
            counters,
            performance_receipt: counter_receipt("store.buffer_pool.copy_materialization", &rows)?,
        })
    }

    pub const fn counters(&self) -> RecordCopyCounterSnapshot {
        self.counters
    }

    pub const fn performance_receipt(&self) -> &FoundationalStoreCounterReceipt {
        &self.performance_receipt
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZeroCopyLayoutPostureReport {
    zero_copy_admissions: u64,
    bounded_copy_admissions: u64,
    semantic_domain_object_claimed: bool,
}

impl ZeroCopyLayoutPostureReport {
    pub(crate) fn from_executed_copy_counters(
        counters: RecordCopyCounterSnapshot,
    ) -> Result<Self, FoundationalBoundaryEvidenceDenial> {
        if counters.zero_copy_admission_count() == 0 {
            return Err(FoundationalBoundaryEvidenceDenial::MissingCopyCounters);
        }
        Ok(Self {
            zero_copy_admissions: counters.zero_copy_admission_count(),
            bounded_copy_admissions: counters.bounded_copy_count(),
            semantic_domain_object_claimed: false,
        })
    }

    pub const fn semantic_domain_object_claimed(self) -> bool {
        self.semantic_domain_object_claimed
    }

    pub const fn zero_copy_admissions(self) -> u64 {
        self.zero_copy_admissions
    }

    pub const fn bounded_copy_admissions(self) -> u64 {
        self.bounded_copy_admissions
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaterializationProfileReport {
    profile: FoundationalEvidenceProfile,
    counters: BufferPoolCounterSnapshot,
    authority_result: FoundationalBoundaryAuthorityResult,
}

impl MaterializationProfileReport {
    pub(crate) const fn from_executed_counters(
        profile: FoundationalEvidenceProfile,
        counters: BufferPoolCounterSnapshot,
    ) -> Self {
        Self {
            profile,
            counters,
            authority_result: FoundationalBoundaryAuthorityResult::CounterBackedStoreExecution,
        }
    }

    pub const fn profile(self) -> FoundationalEvidenceProfile {
        self.profile
    }

    pub const fn counters(self) -> BufferPoolCounterSnapshot {
        self.counters
    }

    pub const fn authority_result(self) -> FoundationalBoundaryAuthorityResult {
        self.authority_result
    }

    pub const fn optional_diagnostic_count(self) -> u8 {
        self.profile.optional_diagnostic_count()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BufferPoolProvenanceAttachment {
    counters: BufferPoolCounterSnapshot,
}

impl BufferPoolProvenanceAttachment {
    pub(crate) const fn from_executed_counters(counters: BufferPoolCounterSnapshot) -> Self {
        Self { counters }
    }

    pub const fn counters(self) -> BufferPoolCounterSnapshot {
        self.counters
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletedResidencyBoundaryReceipt {
    resident_memory: ResidentMemoryPerformanceReceipt,
    allocation: AllocationEnvelopePerformanceReceipt,
    copy_materialization: CopyMaterializationPerformanceReceipt,
    layout: ZeroCopyLayoutPostureReport,
    profile: MaterializationProfileReport,
    provenance: BufferPoolProvenanceAttachment,
}

impl CompletedResidencyBoundaryReceipt {
    pub fn from_executed_store_counters(
        source: BufferPoolExecutedEvidenceSource,
        profile: FoundationalEvidenceProfile,
    ) -> Result<Self, FoundationalBoundaryEvidenceDenial> {
        let counters = source.counters();
        Ok(Self {
            resident_memory: ResidentMemoryPerformanceReceipt::from_executed_counters(
                counters.resident_memory(),
            )?,
            allocation: AllocationEnvelopePerformanceReceipt::from_executed_counters(
                counters.allocation(),
            )?,
            copy_materialization: CopyMaterializationPerformanceReceipt::from_executed_counters(
                counters.copy_materialization(),
            )?,
            layout: ZeroCopyLayoutPostureReport::from_executed_copy_counters(
                counters.copy_materialization(),
            )?,
            profile: MaterializationProfileReport::from_executed_counters(profile, counters),
            provenance: BufferPoolProvenanceAttachment::from_executed_counters(counters),
        })
    }

    #[cfg(test)]
    pub(crate) fn from_distinct_reports(
        resident_memory: ResidentMemoryPerformanceReceipt,
        allocation: AllocationEnvelopePerformanceReceipt,
        copy_materialization: CopyMaterializationPerformanceReceipt,
        layout: ZeroCopyLayoutPostureReport,
        profile: MaterializationProfileReport,
        provenance: BufferPoolProvenanceAttachment,
    ) -> Result<Self, FoundationalBoundaryEvidenceDenial> {
        require_common_report_basis(
            &resident_memory,
            &allocation,
            &copy_materialization,
            layout,
            profile,
            provenance,
        )?;
        Ok(Self {
            resident_memory,
            allocation,
            copy_materialization,
            layout,
            profile,
            provenance,
        })
    }

    pub const fn resident_memory(&self) -> &ResidentMemoryPerformanceReceipt {
        &self.resident_memory
    }

    pub const fn allocation(&self) -> &AllocationEnvelopePerformanceReceipt {
        &self.allocation
    }

    pub const fn copy_materialization(&self) -> &CopyMaterializationPerformanceReceipt {
        &self.copy_materialization
    }

    pub const fn layout(&self) -> ZeroCopyLayoutPostureReport {
        self.layout
    }

    pub const fn profile(&self) -> MaterializationProfileReport {
        self.profile
    }

    pub const fn provenance(&self) -> BufferPoolProvenanceAttachment {
        self.provenance
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FoundationalBoundaryEvidenceDenial {
    MissingResidentMemoryCounters,
    MissingAllocationCounters,
    MissingCopyCounters,
    ReportBasisMismatch,
    PerformanceBundleDenied(FoundationalPerformanceBundleConstructionDenial),
    PerformanceReceiptDenied(FoundationalCounterBackedPerformanceReceiptConstructionDenial),
}

#[cfg(test)]
fn require_common_report_basis(
    resident_memory: &ResidentMemoryPerformanceReceipt,
    allocation: &AllocationEnvelopePerformanceReceipt,
    copy_materialization: &CopyMaterializationPerformanceReceipt,
    layout: ZeroCopyLayoutPostureReport,
    profile: MaterializationProfileReport,
    provenance: BufferPoolProvenanceAttachment,
) -> Result<(), FoundationalBoundaryEvidenceDenial> {
    let counters = provenance.counters();
    if resident_memory.counters() != counters.resident_memory()
        || allocation.counters() != counters.allocation()
        || copy_materialization.counters() != counters.copy_materialization()
        || profile.counters() != counters
        || layout.zero_copy_admissions()
            != counters.copy_materialization().zero_copy_admission_count()
        || layout.bounded_copy_admissions() != counters.copy_materialization().bounded_copy_count()
    {
        return Err(FoundationalBoundaryEvidenceDenial::ReportBasisMismatch);
    }
    Ok(())
}

fn allocation_requested_bytes(counters: AllocationCounterSnapshot) -> u64 {
    AllocationScope::ALL
        .into_iter()
        .map(|scope| counters.scope(scope).requested_bytes())
        .sum()
}

fn allocation_allocated_bytes(counters: AllocationCounterSnapshot) -> u64 {
    AllocationScope::ALL
        .into_iter()
        .map(|scope| counters.scope(scope).allocated_bytes())
        .sum()
}
