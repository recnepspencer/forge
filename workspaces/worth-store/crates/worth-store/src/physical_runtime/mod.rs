mod admission;
mod availability;
mod diagnostics;
mod identity;
mod instance;
mod lifecycle;
#[cfg(feature = "certification-test-authority")]
mod media_evidence;
mod media_ownership;
mod observation;
mod record_serving;
mod resource_lifecycle;
mod root_admission;
mod runtime;
mod shutdown;
mod work;

pub use admission::{
    AdmissionError, CancelledPhysicalRuntimeAdmission, DeclaredStoreRootDenialKind,
    PhysicalRuntimeAdmission, PhysicalStore,
};
pub use availability::{CapabilityAvailability, InstalledCapabilityStatus, PhysicalCapability};
pub use diagnostics::{ProcessRuntimeCounterSnapshot, RuntimeCounterSnapshot};
pub use identity::{DeclaredStoreRoot, RuntimeIdentity};
pub use instance::{
    PhysicalSignalClockObservation, PhysicalSignalClockObservationFailure,
    PhysicalSignalConstructionFailure, PhysicalSignalDeltaApplicationFailure,
    PhysicalSignalObservation, PhysicalSignalRuntimeIdentity, PhysicalSignalShutdownOutcome,
    PhysicalStoreAbortOutcome, PhysicalStoreCloseObservation, PhysicalStoreCloseOutcome,
    PhysicalStoreClosePhase, PhysicalStoreClosePlan, PhysicalWorkExecution,
};
pub use lifecycle::LifecycleGeneration;
pub use media_ownership::{
    FilesystemMediaAdmission, MediaAdmissionDeferred, MediaAdmissionDenial,
    MediaAdmissionInspectionCause, MediaAdmissionInspectionRequired, MediaAdmissionOutcome,
    MediaAdmissionRebindRequired, MediaAdmissionStale, MediaOwnedObservationPhase,
    MediaOwnedPhysicalRuntime, MediaShutdownOutcome, PhysicalMediaObservation,
    PhysicalMediaObserver, RecordServingObservationPhase,
};
pub use observation::{
    LifecycleObservation, ObservationError, ObservationHandle, RootAdmissionObservation,
    RuntimeObservation,
};
pub use record_serving::*;
pub use runtime::AdmittedPhysicalRuntime;
pub use shutdown::{AbortedRuntime, ClosedRuntime};
pub use work::{
    AdmittedPhysicalWork, AdmittedPhysicalWorkAuthority, BlockedPhysicalWork,
    CompletedPhysicalPublicationEffect, DispatchedPhysicalWork, PhysicalEffectIdentity,
    PhysicalEffectObligation, PhysicalExecutorCommand, PhysicalExecutorCommandDenial,
    PhysicalMetadataReadWorkRequest, PhysicalMutationSubmission, PhysicalMutationWorkRequest,
    PhysicalOperationIdentity, PhysicalPublicationEffect, PhysicalReadSubmission,
    PhysicalReadWorkRequest, PhysicalRetryCommand, PhysicalSchedulerDemand,
    PhysicalSchedulerDenial, PhysicalSignalAspectBinding, PhysicalSignalAspectBindingDigest,
    PhysicalSignalAspectBindingObservation, PhysicalSignalAspectBindingSet,
    PhysicalSignalAspectDeclaration, PhysicalSignalAspectRole, PhysicalSignalAspectSubscription,
    PhysicalSignalBindingDenial, PhysicalSignalProfileIdentity, PhysicalSignalSettlementOutcome,
    PhysicalWorkAdmission, PhysicalWorkAspectDelta, PhysicalWorkAspectDeltaDenial,
    PhysicalWorkBatchDenial, PhysicalWorkCancellationFailure, PhysicalWorkCancellationJoin,
    PhysicalWorkCapacity, PhysicalWorkCapacityDimension, PhysicalWorkCausalObservation,
    PhysicalWorkCausalRecord, PhysicalWorkConcurrencyRelation, PhysicalWorkConcurrencyScope,
    PhysicalWorkConsumerHandle, PhysicalWorkCounterSnapshot, PhysicalWorkCounterStage,
    PhysicalWorkDeclarationDenial, PhysicalWorkDrainObservation, PhysicalWorkDurabilityRequirement,
    PhysicalWorkEffectClass, PhysicalWorkEffectFate, PhysicalWorkExecutionBatchOutcome,
    PhysicalWorkExecutionOutcome, PhysicalWorkGeneration, PhysicalWorkHealthRevocation,
    PhysicalWorkIdentity, PhysicalWorkIntent, PhysicalWorkNoEffectEvidence,
    PhysicalWorkObservation, PhysicalWorkOperationFamily, PhysicalWorkPreEffectDenial,
    PhysicalWorkPressureClass, PhysicalWorkProfileDeclaration, PhysicalWorkProfileDenial,
    PhysicalWorkPublicationResiduePosture, PhysicalWorkReadiness, PhysicalWorkRecoveryDisposition,
    PhysicalWorkRecoveryLocator, PhysicalWorkRecoveryTarget, PhysicalWorkResidencyPosture,
    PhysicalWorkRetryAdmission, PhysicalWorkRetryFailure, PhysicalWorkRetrySchedule,
    PhysicalWorkRetryScheduleOutcome, PhysicalWorkScheduler, PhysicalWorkSchedulerPosture,
    PhysicalWorkScope, PhysicalWorkSemanticBasis, PhysicalWorkSemanticBasisDenial,
    PhysicalWorkSemanticPosture, PhysicalWorkSettlementEvidence, PhysicalWorkShutdownObservation,
    PhysicalWorkSignalDeclaration, PhysicalWorkSignalFamily, PhysicalWorkSignalFamilySet,
    PhysicalWorkSubmissionDeferred, PhysicalWorkSubmissionDenial, PhysicalWorkSubmissionFailure,
    PhysicalWorkSubmissionOutcome, PhysicalWorkSubmissionReceipt, PhysicalWorkSubmissionStale,
    PhysicalWorkSupersessionJoin, PhysicalWorkTerminalCause, PhysicalWorkTerminalDisposition,
    PhysicalWorkTerminalFailure, PhysicalWorkTerminalObservation, PhysicalWorkTerminalStage,
    PhysicalWorkTimeoutJoin, ReadyPhysicalWork, ResourceAdmittedPhysicalWork, SettledPhysicalWork,
};

pub(in crate::physical_runtime) use work::{
    PhysicalEffectRecoveryObligation, PhysicalExecutorDispatch, PhysicalExecutorOutcome,
    PhysicalPublicationExecutorCommand, PhysicalReadExecutorCommand,
    PhysicalResidencyWritebackExecutorCommand, PhysicalWorkSettlement,
    PhysicalWriteExecutorCommand,
};

#[cfg(feature = "certification-test-authority")]
pub mod certification {
    pub use super::instance::{
        CertificationPhysicalClosePauseGate, CertificationPhysicalExecutionCheckpoint,
        CertificationPhysicalExecutionPauseGate, CertificationPhysicalSignalPauseGate,
        PhysicalPublicationDependencyObservation,
    };
    pub use super::media_evidence::{
        lower_media_operation_summary, MediaEvidenceLoweringDenial, MediaOperationSummary,
        StoreMediaPerformanceReceipt,
    };
    pub use super::work::CertificationPhysicalSubmissionPauseGate;
    pub use worth_store_physical_backend::{
        CertificationMediaFaultAuthority, MediaFaultDirective, MediaFaultRule, MediaFaultSchedule,
        MediaFaultScheduleDenial, MediaPauseGate,
    };
}
