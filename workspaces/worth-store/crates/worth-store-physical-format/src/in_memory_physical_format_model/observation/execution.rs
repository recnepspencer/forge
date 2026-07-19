use worth_store_budgets::{
    PreExecutionBudgetAdmissionReceipt, PreExecutionBudgetEnvelope, PreExecutionBudgetScope,
};

use super::{
    InMemoryPhysicalFormatModel, InMemoryPhysicalFormatModelCounterSnapshot,
    InMemoryPhysicalFormatModelDenial, PlatformPhysicalAppendReport, PlatformPhysicalAppendRequest,
    PlatformPhysicalDegradedExactScanReceipt, PlatformPhysicalRootPublicationReport,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformPhysicalRootPublicationObservation {
    budget: PreExecutionBudgetEnvelope,
    payload_bytes: u64,
    counters_before: InMemoryPhysicalFormatModelCounterSnapshot,
    counters_after: InMemoryPhysicalFormatModelCounterSnapshot,
}

impl PlatformPhysicalRootPublicationObservation {
    pub(super) const fn issue(
        budget: PreExecutionBudgetEnvelope,
        payload_bytes: u64,
        counters_before: InMemoryPhysicalFormatModelCounterSnapshot,
        counters_after: InMemoryPhysicalFormatModelCounterSnapshot,
    ) -> Self {
        Self {
            budget,
            payload_bytes,
            counters_before,
            counters_after,
        }
    }

    pub const fn counters_before(&self) -> InMemoryPhysicalFormatModelCounterSnapshot {
        self.counters_before
    }
    pub const fn counters_after(&self) -> InMemoryPhysicalFormatModelCounterSnapshot {
        self.counters_after
    }
    pub const fn payload_bytes(&self) -> u64 {
        self.payload_bytes
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlatformPhysicalRootPublicationReady<'a> {
    append: PlatformPhysicalAppendRequest<'a>,
    budget: PreExecutionBudgetEnvelope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlatformPhysicalDegradedExactScanReady {
    admitted_rows: u64,
    budget: PreExecutionBudgetEnvelope,
}

impl PlatformPhysicalDegradedExactScanReady {
    pub const fn admitted_rows(self) -> u64 {
        self.admitted_rows
    }

    pub(in crate::in_memory_physical_format_model) const fn budget(
        self,
    ) -> PreExecutionBudgetEnvelope {
        self.budget
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformPhysicalOperationAdmissionDenial {
    RootPublicationRequiresForeground,
    RootPublicationPayloadExceedsBudget,
    DegradedScanRequiresNonForeground,
    DegradedScanRequiresRows,
    DegradedScanRowsExceedBudget,
}

impl InMemoryPhysicalFormatModel {
    pub fn admit_root_publication<'a>(
        &self,
        append: PlatformPhysicalAppendRequest<'a>,
        budget: PreExecutionBudgetAdmissionReceipt,
    ) -> Result<PlatformPhysicalRootPublicationReady<'a>, PlatformPhysicalOperationAdmissionDenial>
    {
        if budget.scope() != PreExecutionBudgetScope::Foreground {
            return Err(
                PlatformPhysicalOperationAdmissionDenial::RootPublicationRequiresForeground,
            );
        }
        let admitted_demand = budget.request();
        if append.payload().len() as u64 > admitted_demand.estimated_byte_reads() {
            return Err(
                PlatformPhysicalOperationAdmissionDenial::RootPublicationPayloadExceedsBudget,
            );
        }
        Ok(PlatformPhysicalRootPublicationReady {
            append,
            budget: budget.admitted_envelope(),
        })
    }

    pub fn admit_degraded_exact_scan(
        &self,
        admitted_rows: u64,
        budget: PreExecutionBudgetAdmissionReceipt,
    ) -> Result<PlatformPhysicalDegradedExactScanReady, PlatformPhysicalOperationAdmissionDenial>
    {
        if budget.scope() == PreExecutionBudgetScope::Foreground {
            return Err(
                PlatformPhysicalOperationAdmissionDenial::DegradedScanRequiresNonForeground,
            );
        }
        if admitted_rows == 0 {
            return Err(PlatformPhysicalOperationAdmissionDenial::DegradedScanRequiresRows);
        }
        let admitted_demand = budget.request();
        if admitted_rows > u64::from(admitted_demand.estimated_range_touches()) {
            return Err(PlatformPhysicalOperationAdmissionDenial::DegradedScanRowsExceedBudget);
        }
        Ok(PlatformPhysicalDegradedExactScanReady {
            admitted_rows,
            budget: budget.admitted_envelope(),
        })
    }

    pub fn execute_admitted_root_publication(
        &mut self,
        ready: PlatformPhysicalRootPublicationReady<'_>,
    ) -> Result<
        (
            PlatformPhysicalAppendReport,
            PlatformPhysicalRootPublicationReport,
            PlatformPhysicalRootPublicationObservation,
        ),
        InMemoryPhysicalFormatModelDenial,
    > {
        let counters_before = self.counters();
        let payload_bytes = ready.append.payload().len() as u64;
        let appended = self.append_physical_record(ready.append)?;
        let published = self.publish_physical_root()?;
        let observation = PlatformPhysicalRootPublicationObservation::issue(
            ready.budget,
            payload_bytes,
            counters_before,
            self.counters(),
        );
        Ok((appended, published, observation))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformPhysicalDegradedExecutionObservation {
    budget: PreExecutionBudgetEnvelope,
    scan: PlatformPhysicalDegradedExactScanReceipt,
    allocation_events: u64,
}

impl PlatformPhysicalDegradedExecutionObservation {
    pub(in crate::in_memory_physical_format_model) const fn issue(
        budget: PreExecutionBudgetEnvelope,
        scan: PlatformPhysicalDegradedExactScanReceipt,
        allocation_events: u64,
    ) -> Self {
        Self {
            budget,
            scan,
            allocation_events,
        }
    }

    pub const fn scan(&self) -> &PlatformPhysicalDegradedExactScanReceipt {
        &self.scan
    }

    pub const fn allocation_events(&self) -> u64 {
        self.allocation_events
    }
}
