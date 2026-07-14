use worth_foundational::FoundationalPerformanceBudgetKind;
use worth_store_physical_backend::{BackendTargetProfile, CapabilityEvidenceClass};
use worth_store_security::StoreSecurityScopeIdentity;

use crate::IoSchedulerBackendCapabilityRequirement;
use crate::SecureIoPreservationDenial;

use super::{BackgroundIoPressureClass, BackgroundResourceShortfall};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BackgroundPacingDenial {
    MissingDeclaredResourceBudget,
    BackendRequirementMismatch {
        pressure_required: IoSchedulerBackendCapabilityRequirement,
        admitted: IoSchedulerBackendCapabilityRequirement,
    },
    ForegroundReservationBackendMismatch {
        reservation_required: IoSchedulerBackendCapabilityRequirement,
        admitted: IoSchedulerBackendCapabilityRequirement,
    },
    BackendProfileMismatch {
        reservation_profile: BackendTargetProfile,
        admitted_profile: BackendTargetProfile,
    },
    BackendEvidenceClassMismatch {
        reservation_evidence: CapabilityEvidenceClass,
        admitted_evidence: CapabilityEvidenceClass,
    },
    SecurityScopeMismatch {
        reservation_scope: StoreSecurityScopeIdentity,
        requested_scope: StoreSecurityScopeIdentity,
    },
    SecureBackgroundPressureRequiresSecurityBoundBackend,
    MissingSecureIoPreservation,
    SecureIoDenied(SecureIoPreservationDenial),
    PolicyReceiptHasNoBudgetDecision,
    PolicyReceiptRejectedOrWidenedWork,
    PolicyReceiptMissingBudgetKind(FoundationalPerformanceBudgetKind),
    PolicyReceiptDuplicateBudgetKind(FoundationalPerformanceBudgetKind),
    PolicyReceiptBudgetMismatch {
        kind: FoundationalPerformanceBudgetKind,
        requested_units: u32,
        admitted_units: u32,
        expected_requested_units: u32,
        expected_admitted_units: u32,
    },
    PolicyReceiptBudgetOverflow(FoundationalPerformanceBudgetKind),
    InsufficientIdleCapacity(BackgroundResourceShortfall),
    DebtNotPermittedForPressureClass(BackgroundIoPressureClass),
    PacingProgressionDenied(BackgroundIoPressureClass),
    PacingProgressionFailed(BackgroundIoPressureClass),
    RawBackgroundLabelCannotPace,
    SemanticLifecycleReceiptCannotPace,
    LogLineCannotPace,
    ElapsedTimeCannotPace,
    WorkerLocalQueueCannotPace,
}
