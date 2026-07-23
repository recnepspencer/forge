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
    DispatchedPhysicalWork, PhysicalEffectIdentity,
    PhysicalMutationSubmission, PhysicalMutationWorkRequest, PhysicalOperationIdentity,
    PhysicalReadSubmission, PhysicalReadWorkRequest, PhysicalSchedulerDemand,
    PhysicalSchedulerDenial, PhysicalSignalAspectBinding, PhysicalSignalAspectBindingDigest,
    PhysicalSignalAspectBindingSet, PhysicalSignalAspectDeclaration, PhysicalSignalAspectRole,
    PhysicalSignalAspectSubscription, PhysicalSignalBindingDenial, PhysicalSignalProfileIdentity,
    PhysicalWorkAdmission, PhysicalWorkAspectDelta, PhysicalWorkAspectDeltaDenial,
    PhysicalWorkCapacity, PhysicalWorkCapacityDimension, PhysicalWorkDeclarationDenial,
    PhysicalWorkDurabilityRequirement, PhysicalWorkEffectClass, PhysicalWorkGeneration,
    PhysicalWorkIdentity, PhysicalWorkIntent, PhysicalWorkObservation, PhysicalWorkOperationFamily,
    PhysicalWorkPreEffectDenial, PhysicalWorkProfileDeclaration, PhysicalWorkProfileDenial,
    PhysicalWorkReadiness, PhysicalWorkRecoveryDisposition, PhysicalWorkScheduler,
    PhysicalWorkScope, PhysicalWorkSemanticBasis,
    PhysicalWorkSemanticBasisDenial, PhysicalWorkSemanticPosture, PhysicalWorkShutdownObservation,
    PhysicalWorkSignalDeclaration, PhysicalWorkSignalFamily, PhysicalWorkSignalFamilySet,
    PhysicalWorkSubmissionDeferred, PhysicalWorkSubmissionDenial, PhysicalWorkSubmissionFailure,
    PhysicalWorkSubmissionOutcome, PhysicalWorkSubmissionReceipt, PhysicalWorkSubmissionStale,
    PhysicalWorkTerminalDisposition, PhysicalWorkTerminalObservation, PhysicalWorkTerminalStage,
    ReadyPhysicalWork, ResourceAdmittedPhysicalWork, SettledPhysicalWork,
};

#[cfg(feature = "certification-test-authority")]
pub mod certification {
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
