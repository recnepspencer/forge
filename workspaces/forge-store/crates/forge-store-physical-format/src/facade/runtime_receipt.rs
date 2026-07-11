use super::{
    PlatformPhysicalAppendReport, PlatformPhysicalFacadeCounterSnapshot,
    PlatformPhysicalRootPublicationReport,
};
use forge_store_budgets::S8PreExecutionPlanBinding;
use forge_store_contracts::{
    S8RuntimeCase, S8RuntimeExactCounterEvidence, S8RuntimeExecutionIdentity, S8RuntimeOutcome,
    S8RuntimeOwnerFact, S8RuntimeScanPosture,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformPhysicalLayoutAccessIntent {
    HiddenBroadScan,
    ExplicitDegradedExactScan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlatformPhysicalLayoutAccessRequest {
    intent: PlatformPhysicalLayoutAccessIntent,
    plan_binding: S8PreExecutionPlanBinding,
    budget_rows: u64,
}

impl PlatformPhysicalLayoutAccessRequest {
    pub const fn hidden_broad_scan(plan_binding: S8PreExecutionPlanBinding) -> Self {
        Self {
            intent: PlatformPhysicalLayoutAccessIntent::HiddenBroadScan,
            plan_binding,
            budget_rows: 0,
        }
    }
    pub const fn explicit_degraded_exact_scan(
        plan_binding: S8PreExecutionPlanBinding,
        budget_rows: u64,
    ) -> Self {
        Self {
            intent: PlatformPhysicalLayoutAccessIntent::ExplicitDegradedExactScan,
            plan_binding,
            budget_rows,
        }
    }
    pub const fn intent(self) -> PlatformPhysicalLayoutAccessIntent {
        self.intent
    }
    pub const fn plan_binding(self) -> S8PreExecutionPlanBinding {
        self.plan_binding
    }
    pub const fn budget_rows(self) -> u64 {
        self.budget_rows
    }
}

/// A sealed receipt for an attempted whole-store access that the physical
/// facade rejected before it could materialize or traverse storage. Callers
/// may request a broad scan; only this owner can attest to its denial.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlatformPhysicalHiddenScanDenialReceipt {
    request: PlatformPhysicalLayoutAccessRequest,
    counters_before: PlatformPhysicalFacadeCounterSnapshot,
    counters_after: PlatformPhysicalFacadeCounterSnapshot,
    fact: S8RuntimeOwnerFact,
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

    pub const fn fact(self) -> S8RuntimeOwnerFact {
        self.fact
    }

    pub const fn is_owner_denial(self) -> bool {
        matches!(
            self.request.intent(),
            PlatformPhysicalLayoutAccessIntent::HiddenBroadScan
        ) && matches!(self.fact.case(), S8RuntimeCase::HiddenScanDenial)
            && matches!(
                self.fact.scan_posture(),
                S8RuntimeScanPosture::FullStoreDenied
            )
            && self.fact.is_coherent()
            && self.has_exact_zero_work_delta()
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
            fact: owner_fact(
                S8RuntimeCase::HiddenScanDenial,
                S8RuntimeScanPosture::FullStoreDenied,
                S8RuntimeExactCounterEvidence::new(0, 0),
            ),
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
    pub(crate) const fn new(
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

/// An opaque fact emitted from a completed physical-format facade operation.
///
/// This is deliberately constructed only from facade reports. It is not a
/// scenario label or a copied counter bag: both sources are produced after the
/// owner has executed the underlying storage operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlatformPhysicalRuntimeReceipt {
    operation: PlatformPhysicalRuntimeOperation,
    strategy: PlatformPhysicalRuntimeStrategy,
    counters: PlatformPhysicalFacadeCounterSnapshot,
    fact: S8RuntimeOwnerFact,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformPhysicalRuntimeOperation {
    AppendPhysicalRecord,
    PublishPhysicalRoot,
    DenyHiddenBroadScan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformPhysicalRuntimeReceiptDenial {
    NotOwnerHiddenScanDenial,
}

/// The strategy identity selected by the physical-format owner. This local
/// vocabulary prevents the grammar crate from becoming a dependency of the
/// execution crate while preserving the owner-selected fact for certification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformPhysicalRuntimeStrategy {
    BaselineBTreeRange,
}

impl PlatformPhysicalRuntimeReceipt {
    pub const fn from_append(report: PlatformPhysicalAppendReport) -> Self {
        Self::from_append_case(
            report,
            S8RuntimeCase::Success,
            S8RuntimeScanPosture::OwnerBounded,
        )
    }

    #[cfg(feature = "certification-test-authority")]
    pub const fn from_append_unsupported_shape_denial(
        report: PlatformPhysicalAppendReport,
    ) -> Self {
        Self::from_append_case(
            report,
            S8RuntimeCase::UnsupportedShapeDenial,
            S8RuntimeScanPosture::OwnerBounded,
        )
    }

    #[cfg(feature = "certification-test-authority")]
    pub const fn from_append_stale_rebind(report: PlatformPhysicalAppendReport) -> Self {
        Self::from_append_case(
            report,
            S8RuntimeCase::StaleRebind,
            S8RuntimeScanPosture::ReadmissionBounded,
        )
    }

    #[cfg(feature = "certification-test-authority")]
    pub const fn from_append_derived_corruption(report: PlatformPhysicalAppendReport) -> Self {
        Self::from_append_case(
            report,
            S8RuntimeCase::CorruptDerived,
            S8RuntimeScanPosture::RebuildBounded,
        )
    }

    #[cfg(feature = "certification-test-authority")]
    pub const fn from_append_authority_corruption(report: PlatformPhysicalAppendReport) -> Self {
        Self::from_append_case(
            report,
            S8RuntimeCase::CorruptAuthority,
            S8RuntimeScanPosture::RebuildBounded,
        )
    }

    #[cfg(feature = "certification-test-authority")]
    pub const fn from_append_rebuild(report: PlatformPhysicalAppendReport) -> Self {
        Self::from_append_case(
            report,
            S8RuntimeCase::Rebuild,
            S8RuntimeScanPosture::RebuildBounded,
        )
    }

    #[cfg(feature = "certification-test-authority")]
    pub const fn from_append_migration_rollback(report: PlatformPhysicalAppendReport) -> Self {
        Self::from_append_case(
            report,
            S8RuntimeCase::MigrationRollback,
            S8RuntimeScanPosture::RebuildBounded,
        )
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
            fact: receipt.fact(),
        })
    }

    #[cfg(feature = "certification-test-authority")]
    pub const fn from_append_readmission(report: PlatformPhysicalAppendReport) -> Self {
        Self::from_append_case(
            report,
            S8RuntimeCase::Readmission,
            S8RuntimeScanPosture::ReadmissionBounded,
        )
    }

    #[cfg(feature = "certification-test-authority")]
    pub const fn from_append_cost_envelope(report: PlatformPhysicalAppendReport) -> Self {
        Self::from_append_case(
            report,
            S8RuntimeCase::CostEnvelope,
            S8RuntimeScanPosture::OwnerBounded,
        )
    }

    const fn from_append_case(
        report: PlatformPhysicalAppendReport,
        case: S8RuntimeCase,
        scan_posture: S8RuntimeScanPosture,
    ) -> Self {
        Self {
            operation: PlatformPhysicalRuntimeOperation::AppendPhysicalRecord,
            strategy: PlatformPhysicalRuntimeStrategy::BaselineBTreeRange,
            counters: report.counters(),
            fact: owner_fact(case, scan_posture, S8RuntimeExactCounterEvidence::new(1, 1)),
        }
    }

    pub const fn from_root_publication(report: &PlatformPhysicalRootPublicationReport) -> Self {
        Self {
            operation: PlatformPhysicalRuntimeOperation::PublishPhysicalRoot,
            strategy: PlatformPhysicalRuntimeStrategy::BaselineBTreeRange,
            counters: report.counters(),
            fact: owner_fact(
                S8RuntimeCase::Success,
                S8RuntimeScanPosture::OwnerBounded,
                S8RuntimeExactCounterEvidence::new(1, 1),
            ),
        }
    }

    pub const fn operation(self) -> PlatformPhysicalRuntimeOperation {
        self.operation
    }

    pub const fn case(self) -> S8RuntimeCase {
        self.fact.case()
    }

    pub const fn outcome(self) -> S8RuntimeOutcome {
        self.fact.outcome()
    }

    pub const fn fact(self) -> S8RuntimeOwnerFact {
        self.fact
    }

    /// Strategy identity is fixed by the physical-format execution owner.
    /// Courtroom consumers must preserve this fact rather than infer it from
    /// a receipt type or its counters.
    pub const fn strategy(self) -> PlatformPhysicalRuntimeStrategy {
        self.strategy
    }

    pub const fn counters(self) -> PlatformPhysicalFacadeCounterSnapshot {
        self.counters
    }
}

const fn owner_fact(
    case: S8RuntimeCase,
    scan_posture: S8RuntimeScanPosture,
    counters: S8RuntimeExactCounterEvidence,
) -> S8RuntimeOwnerFact {
    S8RuntimeOwnerFact::new(
        S8RuntimeExecutionIdentity::from_owner_seed(0x5088_0001),
        case,
        scan_posture,
        counters,
    )
}
