#![forbid(unsafe_code)]
//! S.6 reclaim, punch-hole, trim, and cold-tier I/O posture admission.
//!
//! This crate sits above S.5 physical isolation and S.6 backend capability. That
//! lets reclaim admission consume real S.5 reachability-removal evidence without
//! making the low-level physical backend depend on S.5 isolation or trust raw
//! region/root/count fields.
//!
//! Raw reclaim reachability fields cannot satisfy the S.5-to-S.6 production
//! reachability admission boundary:
//! ```compile_fail
//! use worth_store_physical_format::PhysicalReclaimRegion;
//! use worth_store_reclaim_policy::ReclaimPolicyReachabilityProof;
//!
//! struct RawReachabilityFields {
//!     region: PhysicalReclaimRegion,
//!     root_epoch: u64,
//!     protected_ranges: u32,
//! }
//!
//! fn worth_reachability(
//!     raw: RawReachabilityFields,
//!     requested_region: PhysicalReclaimRegion,
//! ) {
//!     let _ = ReclaimPolicyReachabilityProof::from_physical_isolation_reclaim_reachability_removal(
//!         raw,
//!         requested_region,
//!     );
//! }
//! ```
//! Ordinary callers cannot recreate S.5 reachability-removal evidence from raw
//! fields:
//! ```compile_fail
//! use worth_store_physical_format::PhysicalReclaimRegion;
//! use worth_store_physical_isolation::S6ReclaimReachabilityRemovalEvidence;
//!
//! fn worth_reachability(region: PhysicalReclaimRegion) {
//!     let _ = S6ReclaimReachabilityRemovalEvidence {
//!         region,
//!         root_epoch: 7,
//!         protected_ranges: 1,
//!     };
//! }
//! ```
//! A general current physical authority witness also cannot mint S.6 reclaim
//! reachability without the S.5 reclaim-removal receipt path:
//! ```compile_fail
//! use worth_store_physical_format::PhysicalReclaimRegion;
//! use worth_store_physical_isolation::S6ReclaimReachabilityRemovalEvidence;
//!
//! fn worth_reachability(region: PhysicalReclaimRegion) {
//!     let _ = S6ReclaimReachabilityRemovalEvidence::from_physical_isolation_reclaim_reachability_removal(
//!         region,
//!         7,
//!         1,
//!     );
//! }
//! ```

mod reclaim_policy;

pub use worth_store_physical_backend::{
    AdmittedBackendCapabilityWitness, BackendCapabilityAdmissionRequest,
    BackendCapabilityEvidenceBasis, BackendCapabilitySupportSet, BackendMediaAssumptionSet,
    BackendRebindTriggers, BackendTargetProfile, CapabilityEvidenceClass,
    PhysicalBackendCapabilityAdmissionAuthority,
};

pub use reclaim_policy::{
    AdmittedReclaimPolicy, PhysicalStoreReclaimPolicyExecutor, ReclaimLaterHandoffPolicy,
    ReclaimPermit, ReclaimPermitDenial, ReclaimPolicyAdmission, ReclaimPolicyCounterSnapshot,
    ReclaimPolicyDenial, ReclaimPolicyDenialKind, ReclaimPolicyExecutionObservation,
    ReclaimPolicyExecutionReceipt, ReclaimPolicyExecutionRequest, ReclaimPolicyExecutionSession,
    ReclaimPolicyOperation, ReclaimPolicyPosture, ReclaimPolicyProofAuthority,
    ReclaimPolicyReachabilityDenial, ReclaimPolicyReachabilityProof, ReclaimPolicyRequest,
    ReclaimPolicySecurityScope, ReclaimPolicyViolation, ReclaimPolicyViolationKind,
    StoreOwnedReclaimPolicyExecution,
};
