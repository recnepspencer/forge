mod admission;
mod availability;
mod diagnostics;
mod identity;
mod lifecycle;
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
pub use observation::{
    LifecycleObservation, ObservationError, ObservationHandle, RootAdmissionObservation,
    RuntimeObservation,
};
pub use runtime::AdmittedPhysicalRuntime;
pub use shutdown::{AbortedRuntime, ClosedRuntime};
