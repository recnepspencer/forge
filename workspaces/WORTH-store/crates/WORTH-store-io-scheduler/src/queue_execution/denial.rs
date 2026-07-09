use worth_foundational::FoundationalPerformanceBudgetKind;

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
    BackendPrivateQueueHandleCannotAdmitQueueWork,
    ElapsedTimeObservationCannotAdmitQueueWork,
}
