//! Backend residue cannot satisfy retention reclaim permits:
//! ```compile_fail
//! use worth_store_blob_chunks::BlobRetentionReclaimPermit;
//! use worth_store_physical_backend::BlobBackendResidueObservation;
//! fn requires_permit(_: BlobRetentionReclaimPermit) {}
//! let residue: BlobBackendResidueObservation = todo!();
//! requires_permit(residue);
//! ```
//! Copied counters cannot satisfy retention reclaim permits:
//! ```compile_fail
//! use worth_store_blob_chunks::{
//!     BlobRetentionReclaimCounterSnapshot, BlobRetentionReclaimPermit,
//! };
//! fn requires_permit(_: BlobRetentionReclaimPermit) {}
//! let counters: BlobRetentionReclaimCounterSnapshot = todo!();
//! requires_permit(counters);
//! ```
//! S.6 reclaim posture alone cannot satisfy retention reclaim permits:
//! ```compile_fail
//! use worth_store_blob_chunks::{BlobRetentionReclaimPermit, BlobReclaimPolicyEvidence};
//! fn requires_permit(_: BlobRetentionReclaimPermit) {}
//! let handoff: BlobReclaimPolicyEvidence = todo!();
//! requires_permit(handoff);
//! ```
//! Reachability-local reclaim release cannot satisfy retention reclaim permits:
//! ```compile_fail
//! use worth_store_blob_chunks::{BlobReachabilityReclaimRelease, BlobRetentionReclaimPermit};
//! fn requires_permit(_: BlobRetentionReclaimPermit) {}
//! let release: BlobReachabilityReclaimRelease = todo!();
//! requires_permit(release);
//! ```
//! Terminal projection rows cannot satisfy retention reclaim receipts:
//! ```compile_fail
//! use worth_store_blob_chunks::BlobRetentionReclaimReceipt;
//! fn requires_receipt(_: BlobRetentionReclaimReceipt) {}
//! let projection = "terminal reclaim row";
//! requires_receipt(projection);
//! ```
//! Retention reclaim permits cannot be synthesized from raw fields:
//! ```compile_fail
//! use worth_store_blob_chunks::BlobRetentionReclaimPermit;
//! let _forged = BlobRetentionReclaimPermit {
//!     identity: todo!(),
//!     chunk_identity: todo!(),
//!     reclaim_policy_evidence: todo!(),
//!     residue: todo!(),
//!     counters: todo!(),
//! };
//! ```
//! Raw physical orphan identities cannot be synthesized by callers:
//! ```compile_fail
//! use worth_store_blob_chunks::BlobRetentionPhysicalOrphanIdentity;
//! use worth_store_physical_isolation::CurrentGenerationPhysicalReference;
//! let reference: CurrentGenerationPhysicalReference = todo!();
//! let _identity =
//!     BlobRetentionPhysicalOrphanIdentity::from_current_generation_reference(reference, 4096);
//! ```
//! Raw reachability releases cannot be paired with arbitrary physical identities:
//! ```compile_fail
//! use worth_store_blob_chunks::{BlobReachabilityReclaimRelease, BlobRetentionOrphanCandidate};
//! fn requires_candidate(_: BlobRetentionOrphanCandidate) {}
//! let release: BlobReachabilityReclaimRelease = todo!();
//! let physical = todo!();
//! let candidate = BlobRetentionOrphanCandidate::from_reachability_release(release, physical);
//! requires_candidate(candidate.unwrap());
//! ```
//! Reclaim requests cannot be built from raw candidates or caller-supplied hold sets:
//! ```compile_fail
//! use worth_store_blob_chunks::{
//!     BlobRetentionHoldSet, BlobRetentionOrphanCandidate, BlobRetentionReclaimRequest,
//! };
//! let candidate: BlobRetentionOrphanCandidate = todo!();
//! let holds = BlobRetentionHoldSet::new();
//! let _request = BlobRetentionReclaimRequest::for_candidate(candidate)
//!     .with_retention_holds(holds);
//! ```
//! Reclaim requests cannot attach S.6 posture after admission:
//! ```compile_fail
//! use worth_store_blob_chunks::{BlobRetentionReclaimAdmission, BlobRetentionReclaimRequest};
//! let admission: BlobRetentionReclaimAdmission = todo!();
//! let s6 = todo!();
//! let _request = BlobRetentionReclaimRequest::for_admission(admission)
//!     .with_s6_reclaim_posture(s6);
//! ```
//! Admission cannot be minted from a copied release and loose physical fields:
//! ```compile_fail
//! use worth_store_blob_chunks::{
//!     BlobReachabilityReclaimRelease, BlobRetentionReclaimAdmissionAuthority,
//! };
//! use worth_store_physical_isolation::CurrentGenerationPhysicalReference;
//! let release: BlobReachabilityReclaimRelease = todo!();
//! let physical: CurrentGenerationPhysicalReference = todo!();
//! let _admission = BlobRetentionReclaimAdmissionAuthority::store_owned()
//!     .admit_reachability_orphan(release, physical, 4096);
//! ```
//! Physical orphan claims cannot be synthesized from raw fields:
//! ```compile_fail
//! use worth_store_blob_chunks::{BlobRetentionPhysicalOrphanClaim, BlobRetentionPhysicalOrphanIdentity};
//! let _claim = BlobRetentionPhysicalOrphanClaim {
//!     chunk_identity: todo!(),
//!     physical_identity: BlobRetentionPhysicalOrphanIdentity { owner: todo!(), durable_bytes: 4096 },
//! };
//! ```
