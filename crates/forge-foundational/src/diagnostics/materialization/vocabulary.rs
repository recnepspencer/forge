use crate::diagnostics::{
    FoundationalDiagnosticAbsenceCause, FoundationalDiagnosticAvailability,
    FoundationalDiagnosticLocator, FoundationalDiagnosticSubject,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalDiagnosticSupportClaimStrength {
    DescriptiveOnly,
    DurableSupportReady,
    CertifiedSupportReady,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalDiagnosticSurfaceAvailability {
    availability: FoundationalDiagnosticAvailability,
    absence_cause: Option<FoundationalDiagnosticAbsenceCause>,
}

impl FoundationalDiagnosticSurfaceAvailability {
    pub const fn retained_hot() -> Self {
        Self {
            availability: FoundationalDiagnosticAvailability::RetainedHot,
            absence_cause: None,
        }
    }

    pub const fn deferred_cold() -> Self {
        Self {
            availability: FoundationalDiagnosticAvailability::DeferredCold,
            absence_cause: None,
        }
    }

    pub const fn reconstructable() -> Self {
        Self {
            availability: FoundationalDiagnosticAvailability::Reconstructable,
            absence_cause: None,
        }
    }

    pub const fn redacted() -> Self {
        Self {
            availability: FoundationalDiagnosticAvailability::Redacted,
            absence_cause: Some(FoundationalDiagnosticAbsenceCause::Redacted),
        }
    }

    pub const fn unavailable(cause: FoundationalDiagnosticAbsenceCause) -> Self {
        Self {
            availability: FoundationalDiagnosticAvailability::Unavailable,
            absence_cause: Some(cause),
        }
    }

    pub const fn availability(&self) -> FoundationalDiagnosticAvailability {
        self.availability
    }

    pub const fn absence_cause(&self) -> Option<FoundationalDiagnosticAbsenceCause> {
        self.absence_cause
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalDiagnosticAssemblyDebtClass {
    RowScanFallback,
    WholeViewFallback,
    RepeatedRediscovery,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalDiagnosticAssemblyDebt {
    class: FoundationalDiagnosticAssemblyDebtClass,
    count: u32,
}

impl FoundationalDiagnosticAssemblyDebt {
    pub const fn new(class: FoundationalDiagnosticAssemblyDebtClass, count: u32) -> Self {
        Self { class, count }
    }

    pub const fn class(&self) -> FoundationalDiagnosticAssemblyDebtClass {
        self.class
    }

    pub const fn count(&self) -> u32 {
        self.count
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalDiagnosticCounterSnapshot {
    retained_evidence_count: u32,
    reconstructable_evidence_count: u32,
    redacted_evidence_count: u32,
    row_scan_fallback_count: u32,
    whole_view_fallback_count: u32,
    repeated_rediscovery_count: u32,
}

impl FoundationalDiagnosticCounterSnapshot {
    pub const fn new(
        retained_evidence_count: u32,
        reconstructable_evidence_count: u32,
        redacted_evidence_count: u32,
        row_scan_fallback_count: u32,
        whole_view_fallback_count: u32,
        repeated_rediscovery_count: u32,
    ) -> Self {
        Self {
            retained_evidence_count,
            reconstructable_evidence_count,
            redacted_evidence_count,
            row_scan_fallback_count,
            whole_view_fallback_count,
            repeated_rediscovery_count,
        }
    }

    pub const fn retained_evidence_count(&self) -> u32 {
        self.retained_evidence_count
    }

    pub const fn reconstructable_evidence_count(&self) -> u32 {
        self.reconstructable_evidence_count
    }

    pub const fn redacted_evidence_count(&self) -> u32 {
        self.redacted_evidence_count
    }

    pub const fn row_scan_fallback_count(&self) -> u32 {
        self.row_scan_fallback_count
    }

    pub const fn whole_view_fallback_count(&self) -> u32 {
        self.whole_view_fallback_count
    }

    pub const fn repeated_rediscovery_count(&self) -> u32 {
        self.repeated_rediscovery_count
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalDiagnosticGapClass {
    OptionalEvidenceOmitted,
    SupportBreadthUnavailable,
    ReplayEvidenceUnavailable,
    LocalityMismatch,
    WidenedFallback,
    CoverageOmission,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FoundationalDiagnosticGapTarget {
    Subject(FoundationalDiagnosticSubject),
    Locator(FoundationalDiagnosticLocator),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalDiagnosticGapClosurePosture {
    Deferred,
    Unsupported,
    Denied,
    DebtNamed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalDiagnosticNamedGap {
    gap_class: FoundationalDiagnosticGapClass,
    target: FoundationalDiagnosticGapTarget,
    closure_posture: FoundationalDiagnosticGapClosurePosture,
}

impl FoundationalDiagnosticNamedGap {
    pub fn new(
        gap_class: FoundationalDiagnosticGapClass,
        target: FoundationalDiagnosticGapTarget,
        closure_posture: FoundationalDiagnosticGapClosurePosture,
    ) -> Self {
        Self {
            gap_class,
            target,
            closure_posture,
        }
    }

    pub const fn gap_class(&self) -> FoundationalDiagnosticGapClass {
        self.gap_class
    }

    pub fn target(&self) -> &FoundationalDiagnosticGapTarget {
        &self.target
    }

    pub const fn closure_posture(&self) -> FoundationalDiagnosticGapClosurePosture {
        self.closure_posture
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FoundationalDiagnosticPartiality {
    Complete,
    PartialWithNamedGaps(Vec<FoundationalDiagnosticNamedGap>),
}

impl FoundationalDiagnosticPartiality {
    pub fn named_gaps(&self) -> &[FoundationalDiagnosticNamedGap] {
        match self {
            Self::Complete => &[],
            Self::PartialWithNamedGaps(gaps) => gaps,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FoundationalDiagnosticMaterializationDenial {
    UnavailableAvailabilityRequiresCause,
    RedactedAvailabilityMustUseRedactedCause,
    PartialityRequiresNamedGaps,
    CompleteMaterializationMustNotCarryNamedGaps,
    DurableSupportRequiresVisibleEvidence,
    DurableSupportRequiresVisibleRowsAtChosenRichness,
    InternalSupportCannotClaimDurableSupport,
    InternalSupportCannotClaimCertifiedSupport,
    CertifiedSupportRequiresProductionCertifiedProfile,
    RowScanFallbackMustRemainExplicitDebt,
    WholeViewFallbackMustRemainExplicitDebt,
    RepeatedRediscoveryMustRemainExplicitDebt,
}
