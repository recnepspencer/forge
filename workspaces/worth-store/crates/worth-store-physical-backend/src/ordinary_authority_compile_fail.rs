//! Ordinary feature graphs cannot select certified backend evidence:
//! ```compile_fail
//! use worth_store_physical_backend::BackendCapabilityEvidenceBasis;
//! let _forged = BackendCapabilityEvidenceBasis::certified_backend_profile();
//! ```
//! Ordinary callers cannot assemble externally-guaranteed runtime authority from
//! public support and media declarations:
//! ```compile_fail
//! use worth_store_physical_backend::{
//!     BackendCapabilityAdmissionRequest, BackendCapabilityEvidenceBasis,
//!     BackendCapabilitySupportSet, BackendMediaAssumptionSet, BackendRebindTriggers,
//!     BackendTargetProfile, PhysicalBackendCapabilityAdmissionAuthority,
//! };
//! let request = BackendCapabilityAdmissionRequest::new(
//!     BackendTargetProfile::PosixFileFsyncDirSync,
//!     BackendCapabilityEvidenceBasis::externally_guaranteed(1),
//!     BackendCapabilitySupportSet::all_supported(),
//!     BackendMediaAssumptionSet::platform_file_defaults(),
//!     BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend(),
//! );
//! let _forged =
//!     PhysicalBackendCapabilityAdmissionAuthority::store_owned().admit_backend_capability(request);
//! ```
//! Raw backend queue authority is private to scheduled media execution:
//! ```compile_fail
//! use worth_store_physical_backend::{
//!     BackendQueueExecutionAuthority, BackendQueueExecutionCompletionBuilder,
//!     BackendQueueExecutionSession, BackendQueueExecutionTicket,
//! };
//! ```
