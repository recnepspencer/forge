use super::{
    PlatformPhysicalAppendReport, PlatformPhysicalFacadeCounterSnapshot,
    PlatformPhysicalRootPublicationReport,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformPhysicalLayoutAccessIntent {
    HiddenBroadScan,
    ExplicitDegradedExactScan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlatformPhysicalLayoutAccessRequest {
    intent: PlatformPhysicalLayoutAccessIntent,
    budget_rows: u64,
}

impl PlatformPhysicalLayoutAccessRequest {
    pub const fn hidden_broad_scan() -> Self {
        Self {
            intent: PlatformPhysicalLayoutAccessIntent::HiddenBroadScan,
            budget_rows: 0,
        }
    }

    pub const fn explicit_degraded_exact_scan(budget_rows: u64) -> Self {
        Self {
            intent: PlatformPhysicalLayoutAccessIntent::ExplicitDegradedExactScan,
            budget_rows,
        }
    }

    pub const fn intent(self) -> PlatformPhysicalLayoutAccessIntent {
        self.intent
    }

    pub const fn budget_rows(self) -> u64 {
        self.budget_rows
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlatformPhysicalHiddenScanDenialReceipt {
    request: PlatformPhysicalLayoutAccessRequest,
    counters_before: PlatformPhysicalFacadeCounterSnapshot,
    counters_after: PlatformPhysicalFacadeCounterSnapshot,
}

impl PlatformPhysicalHiddenScanDenialReceipt {
    pub const fn request(self) -> PlatformPhysicalLayoutAccessRequest {
        self.request
    }

    pub const fn counters(self) -> PlatformPhysicalFacadeCounterSnapshot {
        self.counters_after
    }

    pub const fn counters_before(self) -> PlatformPhysicalFacadeCounterSnapshot {
        self.counters_before
    }

    pub const fn has_exact_zero_work_delta(self) -> bool {
        self.counters_after
            .is_exact_hidden_scan_denial_delta_from(self.counters_before)
    }

    pub const fn is_owner_denial(self) -> bool {
        matches!(
            self.request.intent(),
            PlatformPhysicalLayoutAccessIntent::HiddenBroadScan
        ) && self.has_exact_zero_work_delta()
    }

    pub(crate) const fn from_rejected_request(
        request: PlatformPhysicalLayoutAccessRequest,
        counters_before: PlatformPhysicalFacadeCounterSnapshot,
        counters_after: PlatformPhysicalFacadeCounterSnapshot,
    ) -> Self {
        Self {
            request,
            counters_before,
            counters_after,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlatformPhysicalDegradedExactScanReceipt {
    request: PlatformPhysicalLayoutAccessRequest,
    observed_rows: u64,
    counters: PlatformPhysicalFacadeCounterSnapshot,
}

impl PlatformPhysicalDegradedExactScanReceipt {
    pub(super) const fn new(
        request: PlatformPhysicalLayoutAccessRequest,
        observed_rows: u64,
        counters: PlatformPhysicalFacadeCounterSnapshot,
    ) -> Self {
        Self {
            request,
            observed_rows,
            counters,
        }
    }

    pub const fn request(self) -> PlatformPhysicalLayoutAccessRequest {
        self.request
    }

    pub const fn observed_rows(self) -> u64 {
        self.observed_rows
    }

    pub const fn counters(self) -> PlatformPhysicalFacadeCounterSnapshot {
        self.counters
    }

    pub const fn is_budget_exact(self) -> bool {
        matches!(
            self.request.intent(),
            PlatformPhysicalLayoutAccessIntent::ExplicitDegradedExactScan
        ) && self.request.budget_rows() > 0
            && self.observed_rows <= self.request.budget_rows()
            && self.counters.scans() > 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlatformPhysicalRuntimeReceipt {
    operation: PlatformPhysicalRuntimeOperation,
    strategy: PlatformPhysicalRuntimeStrategy,
    counters: PlatformPhysicalFacadeCounterSnapshot,
    outcome: PlatformPhysicalRuntimeOutcome,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformPhysicalRuntimeOperation {
    AppendPhysicalRecord,
    PublishPhysicalRoot,
    DenyHiddenBroadScan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformPhysicalRuntimeOutcome {
    AppendCompleted,
    RootPublished,
    HiddenScanDenied,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformPhysicalRuntimeReceiptDenial {
    NotOwnerHiddenScanDenial,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformPhysicalRuntimeStrategy {
    BaselineBTreeRange,
}

impl PlatformPhysicalRuntimeReceipt {
    pub const fn from_append(report: PlatformPhysicalAppendReport) -> Self {
        Self {
            operation: PlatformPhysicalRuntimeOperation::AppendPhysicalRecord,
            strategy: PlatformPhysicalRuntimeStrategy::BaselineBTreeRange,
            counters: report.counters(),
            outcome: PlatformPhysicalRuntimeOutcome::AppendCompleted,
        }
    }

    pub fn from_hidden_scan_denial(
        receipt: PlatformPhysicalHiddenScanDenialReceipt,
    ) -> Result<Self, PlatformPhysicalRuntimeReceiptDenial> {
        if !receipt.is_owner_denial() {
            return Err(PlatformPhysicalRuntimeReceiptDenial::NotOwnerHiddenScanDenial);
        }
        Ok(Self {
            operation: PlatformPhysicalRuntimeOperation::DenyHiddenBroadScan,
            strategy: PlatformPhysicalRuntimeStrategy::BaselineBTreeRange,
            counters: receipt.counters(),
            outcome: PlatformPhysicalRuntimeOutcome::HiddenScanDenied,
        })
    }

    pub const fn from_root_publication(report: &PlatformPhysicalRootPublicationReport) -> Self {
        Self {
            operation: PlatformPhysicalRuntimeOperation::PublishPhysicalRoot,
            strategy: PlatformPhysicalRuntimeStrategy::BaselineBTreeRange,
            counters: report.counters(),
            outcome: PlatformPhysicalRuntimeOutcome::RootPublished,
        }
    }

    pub const fn operation(self) -> PlatformPhysicalRuntimeOperation {
        self.operation
    }

    pub const fn outcome(self) -> PlatformPhysicalRuntimeOutcome {
        self.outcome
    }

    pub const fn strategy(self) -> PlatformPhysicalRuntimeStrategy {
        self.strategy
    }

    pub const fn counters(self) -> PlatformPhysicalFacadeCounterSnapshot {
        self.counters
    }
}
