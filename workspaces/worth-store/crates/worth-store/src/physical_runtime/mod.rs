mod admission;
mod availability;
mod diagnostics;
mod identity;
mod lifecycle;
#[cfg(feature = "certification-test-authority")]
mod media_evidence;
mod media_ownership;
mod observation;
mod resource_lifecycle;
mod root_admission;
mod runtime;
mod shutdown;

pub use admission::{
    AdmissionError, CancelledPhysicalRuntimeAdmission, DeclaredStoreRootDenialKind,
    PhysicalRuntimeAdmission, PhysicalStore,
};
pub use availability::{CapabilityAvailability, InstalledCapabilityStatus, PhysicalCapability};
pub use diagnostics::{ProcessRuntimeCounterSnapshot, RuntimeCounterSnapshot};
pub use identity::{DeclaredStoreRoot, RuntimeIdentity};
pub use lifecycle::LifecycleGeneration;
pub use media_ownership::{
    FilesystemMediaAdmission, MediaAdmissionDeferred, MediaAdmissionDenial,
    MediaAdmissionInspectionCause, MediaAdmissionInspectionRequired, MediaAdmissionOutcome,
    MediaAdmissionRebindRequired, MediaAdmissionStale, MediaOwnedPhysicalRuntime,
    MediaShutdownOutcome, PhysicalMediaObservation, PhysicalMediaObserver,
};
pub use observation::{
    LifecycleObservation, ObservationError, ObservationHandle, RootAdmissionObservation,
    RuntimeObservation,
};
pub use runtime::AdmittedPhysicalRuntime;
pub use shutdown::{AbortedRuntime, ClosedRuntime};

#[cfg(feature = "certification-test-authority")]
pub mod certification {
    pub use super::media_evidence::{
        lower_media_operation_summary, MediaEvidenceLoweringDenial, MediaOperationSummary,
        StoreMediaPerformanceReceipt,
    };
    pub use worth_store_physical_backend::{
        CertificationMediaFaultAuthority, MediaFaultDirective, MediaFaultRule, MediaFaultSchedule,
        MediaFaultScheduleDenial, MediaPauseGate,
    };
}
