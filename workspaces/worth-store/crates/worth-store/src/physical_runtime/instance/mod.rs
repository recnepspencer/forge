mod construction;
mod executor;
mod lifecycle;
mod parts;
mod residency_owner;
mod scheduler_admission;
mod signal_owner;
mod termination;
mod work_lifecycle;
mod work_runtime;

pub(in crate::physical_runtime) use construction::PhysicalStoreInstanceFoundation;
pub(in crate::physical_runtime) use executor::PhysicalWorkExecutor;
#[cfg(feature = "certification-test-authority")]
pub use executor::{
    CertificationPhysicalExecutionCheckpoint, CertificationPhysicalExecutionPauseGate,
};
pub(in crate::physical_runtime) use parts::PhysicalStoreInstanceParts;
pub(in crate::physical_runtime) use residency_owner::PhysicalResidencyOwner;
pub(in crate::physical_runtime) use scheduler_admission::{
    PhysicalSchedulerAdmissionOwner, RecordSchedulerReservationDenial,
};
#[cfg(feature = "certification-test-authority")]
pub use signal_owner::{
    CertificationPhysicalSignalPauseGate, PhysicalPublicationDependencyObservation,
};
pub(in crate::physical_runtime) use signal_owner::{
    PhysicalSignalAdmissionStatus, PhysicalWorkSignalOwner,
};
pub use signal_owner::{
    PhysicalSignalClockObservation, PhysicalSignalClockObservationFailure,
    PhysicalSignalConstructionFailure, PhysicalSignalDeltaApplicationFailure,
    PhysicalSignalObservation, PhysicalSignalRuntimeIdentity, PhysicalSignalShutdownOutcome,
};
#[cfg(feature = "certification-test-authority")]
pub use termination::CertificationPhysicalClosePauseGate;
pub(in crate::physical_runtime) use termination::PhysicalStoreCloseProgressOwner;
pub use termination::{
    PhysicalStoreAbortOutcome, PhysicalStoreCloseObservation, PhysicalStoreCloseOutcome,
    PhysicalStoreClosePhase, PhysicalStoreClosePlan,
};
pub(in crate::physical_runtime) use work_lifecycle::PhysicalWorkLifecycle;
#[cfg(feature = "certification-test-authority")]
pub(in crate::physical_runtime) use work_runtime::PhysicalExecutionCall;
pub use work_runtime::PhysicalWorkExecution;
pub(in crate::physical_runtime) use work_runtime::{
    PhysicalProjectionFailureCapability, PhysicalStoreWorkRuntime,
};
