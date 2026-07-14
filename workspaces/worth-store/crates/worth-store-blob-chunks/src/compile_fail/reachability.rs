//! Copied refcount rows cannot satisfy aggregate reachability:
//! ```compile_fail
//! use worth_store_blob_chunks::BlobChunkReachabilityProofSet;
//! fn requires_reachability(_: BlobChunkReachabilityProofSet) {}
//! let copied_refcount = 3_u64;
//! requires_reachability(copied_refcount);
//! ```
//! Scalar leaf reachability proofs cannot satisfy aggregate reachability:
//! ```compile_fail
//! use worth_store_blob_chunks::{BlobChunkReachabilityProofSet, BlobReachabilityProof};
//! fn requires_aggregate(_: BlobChunkReachabilityProofSet) {}
//! let leaf: BlobReachabilityProof = todo!();
//! requires_aggregate(leaf);
//! ```
//! Unregistered dedupe receipts cannot construct dedupe reachability edges:
//! ```compile_fail
//! use worth_store_blob_chunks::{
//!     BlobChunkDedupeShareClaim, BlobGenerationPublished, BlobReachabilityEdge,
//!     BlobChunkProofLeaf,
//! };
//! let claim: BlobChunkDedupeShareClaim = todo!();
//! let published: BlobGenerationPublished = todo!();
//! let leaf: BlobChunkProofLeaf = todo!();
//! let _edge = BlobReachabilityEdge::dedupe_shared_reference(&claim, &published, &leaf);
//! ```
//! Registered dedupe references cannot bypass the accounting owner to mint
//! reachability edges:
//! ```compile_fail
//! use worth_store_blob_chunks::{
//!     BlobChunkProofLeaf, BlobChunkRegisteredDedupeReference, BlobGenerationPublished,
//!     BlobReachabilityEdge,
//! };
//! let reference: BlobChunkRegisteredDedupeReference = todo!();
//! let published: BlobGenerationPublished = todo!();
//! let leaf: BlobChunkProofLeaf = todo!();
//! let _edge = BlobReachabilityEdge::dedupe_shared_reference(&reference, &published, &leaf);
//! ```
//! Copied dedupe release receipts cannot be directly admitted as reachability
//! authority:
//! ```compile_fail
//! use worth_store_blob_chunks::{
//!     BlobChunkDedupeReferenceRelease, BlobChunkReachabilityRegistry,
//! };
//! let release: BlobChunkDedupeReferenceRelease = todo!();
//! let mut registry = BlobChunkReachabilityRegistry::new_store_owned();
//! registry.admit_dedupe_reference_release(&release);
//! ```
//! A standalone dedupe registry cannot mutate a separate reachability registry:
//! ```compile_fail
//! use worth_store_blob_chunks::{
//!     BlobChunkDedupeReferenceRegistry, BlobChunkIdentity, BlobChunkReachabilityRegistry,
//!     BlobChunkSecurityMetadataWitness,
//! };
//! let identity: BlobChunkIdentity = todo!();
//! let metadata: BlobChunkSecurityMetadataWitness = todo!();
//! let mut dedupe = BlobChunkDedupeReferenceRegistry::new_store_owned();
//! let mut reachability = BlobChunkReachabilityRegistry::new_store_owned();
//! dedupe.deny_all_edges_for_reachability(&identity, metadata, &mut reachability);
//! ```
//! Raw terminal projection strings cannot satisfy reachability edges:
//! ```compile_fail
//! use worth_store_blob_chunks::BlobReachabilityEdge;
//! fn requires_edge(_: BlobReachabilityEdge) {}
//! let projection = "terminal row";
//! requires_edge(projection);
//! ```
//! Read-plan protected holds cannot be minted by pairing physical evidence with
//! caller-supplied blob publication authority:
//! ```compile_fail
//! use worth_store_blob_chunks::{BlobGenerationPublished, BlobReachabilityProtectedHold};
//! use worth_store_physical_isolation::StablePhysicalReadPlan;
//! let plan: StablePhysicalReadPlan = todo!();
//! let published: BlobGenerationPublished = todo!();
//! let _hold = BlobReachabilityProtectedHold::from_stable_read_plan(&plan, &published);
//! ```
//! Checkpoint protected holds cannot be minted by pairing physical evidence with
//! caller-supplied blob publication authority:
//! ```compile_fail
//! use worth_store_blob_chunks::{BlobGenerationPublished, BlobReachabilityProtectedHold};
//! use worth_store_physical_isolation::ReadDuringCheckpointVerdict;
//! let verdict: ReadDuringCheckpointVerdict = todo!();
//! let published: BlobGenerationPublished = todo!();
//! let _hold = BlobReachabilityProtectedHold::from_checkpoint_verdict(&verdict, &published);
//! ```
//! Export protected holds cannot be minted by pairing custody readiness with
//! caller-supplied blob publication authority:
//! ```compile_fail
//! use worth_store_blob_chunks::{BlobGenerationPublished, BlobReachabilityProtectedHold};
//! use worth_store_operations::BackupExportCustodyReadiness;
//! let readiness: BackupExportCustodyReadiness = todo!();
//! let published: BlobGenerationPublished = todo!();
//! let _hold = BlobReachabilityProtectedHold::from_export_readiness(&readiness, &published);
//! ```
//! Backup protected holds cannot be minted by pairing S.10 handoff with
//! caller-supplied blob publication authority:
//! ```compile_fail
//! use worth_store_blob_chunks::{BlobGenerationPublished, BlobReachabilityProtectedHold};
//! use worth_store_operations::S10BackupExportCustodyHandoff;
//! let handoff: S10BackupExportCustodyHandoff = todo!();
//! let published: BlobGenerationPublished = todo!();
//! let _hold = BlobReachabilityProtectedHold::from_backup_repair_backup_handoff(&handoff, &published);
//! ```
