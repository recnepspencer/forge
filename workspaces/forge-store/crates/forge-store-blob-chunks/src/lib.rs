//! S.3 chunk integrity reports cannot satisfy S.7 dedupe receipts:
//! ```compile_fail
//! use forge_store_blob_chunks::BlobDedupeReceipt;
//! use forge_store_physical_integrity::ChunkIntegrityReport;
//!
//! fn requires_dedupe_receipt(_: BlobDedupeReceipt) {}
//!
//! let report: ChunkIntegrityReport = todo!();
//! requires_dedupe_receipt(report);
//! ```
//! S.3 chunk integrity reports cannot satisfy S.7 reachability receipts:
//! ```compile_fail
//! use forge_store_blob_chunks::BlobReachabilityReceipt;
//! use forge_store_physical_integrity::ChunkIntegrityReport;
//!
//! fn requires_reachability_receipt(_: BlobReachabilityReceipt) {}
//!
//! let report: ChunkIntegrityReport = todo!();
//! requires_reachability_receipt(report);
//! ```
//! S.3 chunk integrity reports cannot satisfy S.7 resumability receipts:
//! ```compile_fail
//! use forge_store_blob_chunks::BlobResumabilityReceipt;
//! use forge_store_physical_integrity::ChunkIntegrityReport;
//!
//! fn requires_resumability_receipt(_: BlobResumabilityReceipt) {}
//!
//! let report: ChunkIntegrityReport = todo!();
//! requires_resumability_receipt(report);
//! ```
//! S.3 chunk integrity reports cannot satisfy S.7 retention receipts:
//! ```compile_fail
//! use forge_store_blob_chunks::BlobRetentionReceipt;
//! use forge_store_physical_integrity::ChunkIntegrityReport;
//!
//! fn requires_retention_receipt(_: BlobRetentionReceipt) {}
//!
//! let report: ChunkIntegrityReport = todo!();
//! requires_retention_receipt(report);
//! ```
//! S.7 digest-derived blob identity cannot satisfy S.3 chunk integrity:
//! ```compile_fail
//! use forge_store_blob_chunks::BlobChunkIdentity;
//! use forge_store_physical_integrity::ChunkIntegrityReport;
//!
//! fn requires_chunk_integrity(_: ChunkIntegrityReport) {}
//!
//! let identity: BlobChunkIdentity = todo!();
//! requires_chunk_integrity(identity);
//! ```
//! Digest-derived blob identity cannot satisfy blob security scope:
//! ```compile_fail
//! use forge_store_blob_chunks::{BlobChunkIdentity, BlobChunkSecurityScope};
//!
//! fn requires_blob_scope(_: BlobChunkSecurityScope) {}
//!
//! let identity: BlobChunkIdentity = todo!();
//! requires_blob_scope(identity);
//! ```
//! Digest-derived blob identity cannot enter dedupe admission:
//! ```compile_fail
//! use forge_store_blob_chunks::{BlobChunkDedupeCandidate, BlobChunkIdentity};
//!
//! fn requires_dedupe_candidate(_: BlobChunkDedupeCandidate) {}
//!
//! let identity: BlobChunkIdentity = todo!();
//! requires_dedupe_candidate(identity);
//! ```
//! Blob dedupe candidates are move-only proof carriers:
//! ```compile_fail
//! use forge_store_blob_chunks::BlobChunkDedupeCandidate;
//!
//! let candidate: BlobChunkDedupeCandidate = todo!();
//! let _copy = candidate.clone();
//! ```
//! Stable digests cannot satisfy candidate-bound canonical equivalence:
//! ```compile_fail
//! use forge_store_blob_chunks::BlobChunkCanonicalEquivalence;
//! use forge_store_contracts::StableDigest;
//!
//! fn requires_equivalence(_: BlobChunkCanonicalEquivalence) {}
//!
//! let digest: StableDigest = todo!();
//! requires_equivalence(digest);
//! ```
//! Copied counters cannot satisfy blob dedupe share claims:
//! ```compile_fail
//! use forge_store_blob_chunks::{BlobChunkDedupeCounterSnapshot, BlobChunkDedupeShareClaim};
//!
//! fn requires_share_claim(_: BlobChunkDedupeShareClaim) {}
//!
//! let counters: BlobChunkDedupeCounterSnapshot = todo!();
//! requires_share_claim(counters);
//! ```
//! Copied scope counters cannot satisfy blob security scope:
//! ```compile_fail
//! use forge_store_blob_chunks::{BlobChunkScopeCounterSnapshot, BlobChunkSecurityScope};
//!
//! fn requires_scope(_: BlobChunkSecurityScope) {}
//!
//! let counters: BlobChunkScopeCounterSnapshot = todo!();
//! requires_scope(counters);
//! ```
//! Copied digest strings cannot construct lifecycle receipts:
//! ```compile_fail
//! use forge_store_blob_chunks::LifecycleReceipt;
//!
//! fn requires_lifecycle_receipt(_: LifecycleReceipt) {}
//!
//! let digest = "sha256:copied";
//! requires_lifecycle_receipt(digest);
//! ```
//! Copied lifecycle counters cannot construct lifecycle receipts:
//! ```compile_fail
//! use forge_store_blob_chunks::{BlobLifecycleCounterSnapshot, LifecycleReceipt};
//!
//! fn requires_lifecycle_receipt(_: LifecycleReceipt) {}
//!
//! let counters: BlobLifecycleCounterSnapshot = todo!();
//! requires_lifecycle_receipt(counters);
//! ```
//! S.6 placement readiness seeds cannot construct lifecycle receipts:
//! ```compile_fail
//! use forge_store_blob_chunks::LifecycleReceipt;
//! use forge_store_readiness::S6ClosedS7PlacementAdmissionSeed;
//!
//! fn requires_lifecycle_receipt(_: LifecycleReceipt) {}
//!
//! let seed: S6ClosedS7PlacementAdmissionSeed = todo!();
//! requires_lifecycle_receipt(seed);
//! ```
//! Scoped chunks cannot be synthesized from raw fields:
//! ```compile_fail
//! use forge_store_blob_chunks::ScopedBlobChunk;
//!
//! let _forged = ScopedBlobChunk {
//!     identity: todo!(),
//!     stored_digest: todo!(),
//!     content_digest: todo!(),
//!     security_scope: todo!(),
//!     bytes_observed: 1,
//! };
//! ```
//! Reachability proofs cannot be synthesized from raw fields:
//! ```compile_fail
//! use forge_store_blob_chunks::BlobReachabilityProof;
//!
//! let _forged = BlobReachabilityProof {
//!     chunk_identity: todo!(),
//!     stored_digest: todo!(),
//!     security_scope: todo!(),
//!     reachable_bytes: 1,
//! };
//! ```
//! Placement proofs cannot be synthesized from raw fields:
//! ```compile_fail
//! use forge_store_blob_chunks::BlobPlacementProof;
//!
//! let _forged = BlobPlacementProof {
//!     stored_digest: todo!(),
//!     security_scope: todo!(),
//!     destination: todo!(),
//!     s6_non_claims: todo!(),
//! };
//! ```
//! Lifecycle receipts cannot be synthesized from raw fields:
//! ```compile_fail
//! use forge_store_blob_chunks::LifecycleReceipt;
//!
//! let _forged = LifecycleReceipt {
//!     reachability: todo!(),
//!     placement: todo!(),
//!     counters: todo!(),
//!     executed_proof: todo!(),
//! };
//! ```
//! Store lifecycle authority cannot be synthesized from raw fields:
//! ```compile_fail
//! use forge_store_blob_chunks::BlobLifecycleStoreAuthority;
//!
//! let _forged = BlobLifecycleStoreAuthority {
//!     current_authority: todo!(),
//!     resolution_authority: todo!(),
//! };
//! ```
//! Lifecycle lowering capability cannot be synthesized from raw fields:
//! ```compile_fail
//! use forge_store_blob_chunks::BlobLifecycleLoweringCapability;
//!
//! let _forged = BlobLifecycleLoweringCapability {
//!     capability: todo!(),
//! };
//! ```
//! Lifecycle readiness authority cannot be synthesized from raw fields:
//! ```compile_fail
//! use forge_store_blob_chunks::BlobLifecycleReadinessAuthority;
//!
//! let _forged = BlobLifecycleReadinessAuthority {
//!     placement_readiness: todo!(),
//!     readiness_authority: todo!(),
//! };
//! ```
//! Resolved lifecycle stages cannot be synthesized from raw fields:
//! ```compile_fail
//! use forge_store_blob_chunks::BlobLifecycleResolved;
//!
//! let _forged = BlobLifecycleResolved {
//!     proof_recipe: todo!(),
//!     counters: todo!(),
//! };
//! ```
//! Lowered lifecycle stages cannot be synthesized from raw fields:
//! ```compile_fail
//! use forge_store_blob_chunks::BlobLifecycleLowered;
//!
//! let _forged = BlobLifecycleLowered {
//!     proof_recipe: todo!(),
//!     counters: todo!(),
//! };
//! ```
//! Reachability-admitted lifecycle stages cannot be synthesized from raw fields:
//! ```compile_fail
//! use forge_store_blob_chunks::BlobLifecycleReachabilityAdmitted;
//!
//! let _forged = BlobLifecycleReachabilityAdmitted {
//!     proof_recipe: todo!(),
//!     reachability: todo!(),
//!     counters: todo!(),
//! };
//! ```
//! Placement-admitted lifecycle stages cannot be synthesized from raw fields:
//! ```compile_fail
//! use forge_store_blob_chunks::BlobLifecyclePlacementAdmitted;
//!
//! let _forged = BlobLifecyclePlacementAdmitted {
//!     proof_recipe: todo!(),
//!     reachability: todo!(),
//!     placement: todo!(),
//!     counters: todo!(),
//! };
//! ```
//! Execution-ready lifecycle stages cannot be synthesized from raw fields:
//! ```compile_fail
//! use forge_store_blob_chunks::BlobLifecycleExecutionReady;
//!
//! let _forged = BlobLifecycleExecutionReady {
//!     proof_recipe: todo!(),
//!     reachability: todo!(),
//!     placement: todo!(),
//!     counters: todo!(),
//! };
//! ```
//! Blob object ids cannot be reconstructed from copied digest strings:
//! ```compile_fail
//! use forge_store_blob_chunks::BlobObjectId;
//! use forge_store_contracts::StableDigest;
//!
//! let copied = StableDigest::new("sha256:copied").unwrap();
//! let _forged = BlobObjectId::from_declared_digest(copied);
//! ```
//! Store authority resolution cannot be skipped with a zero-argument call:
//! ```compile_fail
//! use forge_store_blob_chunks::{BlobLifecycleAdmission, BlobLifecycleDeclaration};
//!
//! let declaration: BlobLifecycleDeclaration = todo!();
//! let _resolved = BlobLifecycleAdmission::start(declaration).resolve_store_authority();
//! ```
//! Copied stored chunk digests cannot build lifecycle replay inputs:
//! ```compile_fail
//! use forge_store_blob_chunks::{BlobLifecycleReplayInput, StoredChunkDigest};
//!
//! let digest: StoredChunkDigest = todo!();
//! let _replay = BlobLifecycleReplayInput::from_stored_chunk_digest(digest);
//! ```
//! Placement proofs cannot be constructed directly from a copied S.6 seed:
//! ```compile_fail
//! use forge_store_blob_chunks::{BlobPlacementProof, BlobReachabilityProof};
//! use forge_store_readiness::S6ClosedS7PlacementAdmissionSeed;
//!
//! let reachability: BlobReachabilityProof = todo!();
//! let seed: S6ClosedS7PlacementAdmissionSeed = todo!();
//! let _proof = BlobPlacementProof::from_reachability_and_placement_readiness(&reachability, seed);
//! ```
#![forbid(unsafe_code)]
mod blob_chunk_bytes;
mod blob_chunk_canonical_basis;
mod blob_chunk_canonical_comparison_basis;
mod blob_chunk_collision_verification;
mod blob_chunk_counters;
mod blob_chunk_dedupe;
mod blob_chunk_dedupe_canonical;
mod blob_chunk_dedupe_counters;
mod blob_chunk_denial;
mod blob_chunk_identity;
mod blob_chunk_integrity;
#[doc(hidden)]
pub mod blob_chunk_integrity_compile_fail;
mod blob_chunk_integrity_denial;
#[cfg(test)]
mod blob_chunk_integrity_tests;
#[cfg(test)]
mod blob_chunk_physical_test_support;
mod blob_chunk_root_comparison;
#[doc(hidden)]
pub mod blob_chunk_root_compile_fail;
mod blob_chunk_root_counters;
mod blob_chunk_root_denial;
mod blob_chunk_root_publication;
#[cfg(test)]
mod blob_chunk_root_publication_tests;
mod blob_chunk_rule;
mod blob_chunk_scope;
#[cfg(test)]
mod blob_chunk_scope_tests;
mod blob_chunk_security_metadata;
#[cfg(test)]
mod blob_chunk_security_metadata_tests;
mod blob_chunk_sequence;
mod blob_chunk_streaming;
#[cfg(test)]
mod blob_chunk_test_support;
mod blob_generation_classification;
mod blob_generation_registry;
mod blob_generation_registry_authority;
#[doc(hidden)]
pub mod blob_generation_registry_compile_fail;
mod blob_generation_registry_counters;
mod blob_generation_registry_denial;
#[cfg(test)]
mod blob_generation_registry_test_support;
#[cfg(test)]
mod blob_generation_registry_tests;
mod blob_lifecycle_authority;
#[cfg(test)]
mod blob_lifecycle_boundary_tests;
mod blob_lifecycle_counters;
mod blob_lifecycle_denial;
mod blob_lifecycle_identity;
mod blob_lifecycle_progression;
mod blob_lifecycle_receipts;
mod blob_placement_proof;
mod blob_reachability_proof;
mod blob_scoped_chunk;
mod exports;
mod large_record_streaming_envelope;
mod s6_background_pressure;
mod s6_reclaim_handoff;
mod s7_blob_security_handoff;
#[doc(hidden)]
pub mod security_metadata_compile_fail;

pub use exports::*;
