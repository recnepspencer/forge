mod admission;
mod admission_outcome;
mod observation;
mod runtime;
mod shutdown;

pub use admission::FilesystemMediaAdmission;
pub use admission_outcome::{
    MediaAdmissionDeferred, MediaAdmissionDenial, MediaAdmissionInspectionCause,
    MediaAdmissionInspectionRequired, MediaAdmissionOutcome, MediaAdmissionRebindRequired,
    MediaAdmissionStale,
};
pub use observation::{
    MediaOwnedObservationPhase, PhysicalMediaObservation, PhysicalMediaObserver,
    RecordServingObservationPhase,
};
pub use runtime::MediaOwnedPhysicalRuntime;
pub use shutdown::MediaShutdownOutcome;

pub(super) use admission::try_admit;
