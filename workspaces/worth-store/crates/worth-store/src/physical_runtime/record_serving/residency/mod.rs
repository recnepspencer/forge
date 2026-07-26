pub(in crate::physical_runtime) mod artifact_tree;
pub(super) mod candidate_frame_publishers;
pub(super) mod candidate_frame_residency;
mod capability;
#[cfg(feature = "certification-test-authority")]
mod certification;
mod failure;
pub(super) mod frame_load_failure;
pub(super) mod frame_loading;
pub(super) mod frame_ports;
pub(super) mod frame_work_trace;
pub(super) mod initialization_artifacts;
mod pressure_evidence;
pub(super) mod publication_artifacts;
pub(super) mod record_frame_reader;
mod residency_observation;
pub(in crate::physical_runtime) mod scheduled_writeback;
pub(super) mod serving_artifacts;

pub(super) use capability::ServingFrameResidency;
#[cfg(feature = "certification-test-authority")]
pub use certification::{
    CertificationFrameFaultCause, CertificationFrameReadFailure, CertificationFrameWorkFailure,
    CertificationResidentFrame, PhysicalResidencyCertification,
};
pub use failure::{
    PhysicalRecordResidencyFailure, PhysicalRecordResidencyFailureKind,
    PhysicalRecordResidencyFailureReason,
};
pub use pressure_evidence::{
    PhysicalRecordPressureBasis, PhysicalRecordPressureEvidence, PhysicalResidencyRetryPosture,
};
pub use residency_observation::{
    PhysicalResidencyAllocationEventSnapshot, PhysicalResidencyAllocationSnapshot,
    PhysicalResidencyCounterSnapshot, PhysicalResidencyObservation,
};
