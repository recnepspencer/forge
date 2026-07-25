use worth_foundational::{FoundationalPerformanceBudgetKind, FoundationalPerformanceWorkClass};

use crate::IoSchedulerBackendCapabilityRequirement;
use crate::SecureIoPreservationDenial;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueueGroupingDenial {
    MissingGroupingBasis,
    TenantScopeMismatch,
    KeyScopeMismatch,
    AuthenticityRequirementMismatch,
    DurabilityClassMismatch,
    FlushEpochMismatch,
    WorkClassMismatch,
    RecoveryOrderingMismatch,
    WritebackPolicyMismatch,
    LocalityMismatch,
    SecurityScopeMismatch,
    SecureIoReceiptMismatch,
    BackendCapabilityMismatch,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueueExecutionAdmissionDenial {
    MissingQueueWorkBudget,
    MissingGroupingBasis,
    GroupingDenied(QueueGroupingDenial),
    BackendRequirementMismatch {
        required: IoSchedulerBackendCapabilityRequirement,
        admitted: IoSchedulerBackendCapabilityRequirement,
    },
    PolicyReceiptHasNoBudgetDecision,
    PolicyReceiptContextMismatch {
        expected_work: FoundationalPerformanceWorkClass,
    },
    PolicyReceiptBudgetMismatch {
        kind: FoundationalPerformanceBudgetKind,
        expected_requested_units: u32,
        expected_admitted_units: u32,
    },
    ResourceUnit(crate::IoResourceUnitDenial),
    ResourceBudgetOverflow(crate::IoResourceUnitKind),
    MissingSecureIoPreservation,
    SecureIoDenied(SecureIoPreservationDenial),
    RawOperationLabelCannotAdmitQueueWork,
    CopiedReservationReceiptCannotAdmitQueueWork,
    ProducerSecurityScopeMismatch,
    BackendPrivateQueueHandleCannotAdmitQueueWork,
    ElapsedTimeObservationCannotAdmitQueueWork,
}
