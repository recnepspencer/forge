//! S.7 digest-derived blob identity cannot satisfy S.3 chunk integrity:
//! ```compile_fail
//! use forge_store_blob_chunks::BlobChunkIdentity;
//! use forge_store_physical_integrity::ChunkIntegrityReport;
//! fn requires_chunk_integrity(_: ChunkIntegrityReport) {}
//! let identity: BlobChunkIdentity = todo!();
//! requires_chunk_integrity(identity);
//! ```
//! Digest-derived blob identity cannot satisfy blob security scope:
//! ```compile_fail
//! use forge_store_blob_chunks::{BlobChunkIdentity, BlobChunkSecurityScope};
//! fn requires_blob_scope(_: BlobChunkSecurityScope) {}
//! let identity: BlobChunkIdentity = todo!();
//! requires_blob_scope(identity);
//! ```
//! Digest-derived blob identity cannot enter dedupe admission:
//! ```compile_fail
//! use forge_store_blob_chunks::{BlobChunkDedupeCandidate, BlobChunkIdentity};
//! fn requires_dedupe_candidate(_: BlobChunkDedupeCandidate) {}
//! let identity: BlobChunkIdentity = todo!();
//! requires_dedupe_candidate(identity);
//! ```
//! Blob dedupe candidates are move-only proof carriers:
//! ```compile_fail
//! use forge_store_blob_chunks::BlobChunkDedupeCandidate;
//! let candidate: BlobChunkDedupeCandidate = todo!();
//! let _copy = candidate.clone();
//! ```
//! Stable digests cannot satisfy candidate-bound canonical equivalence:
//! ```compile_fail
//! use forge_store_blob_chunks::BlobChunkCanonicalEquivalence;
//! use forge_store_contracts::StableDigest;
//! fn requires_equivalence(_: BlobChunkCanonicalEquivalence) {}
//! let digest: StableDigest = todo!();
//! requires_equivalence(digest);
//! ```
//! Unregistered dedupe receipts cannot satisfy registered dedupe references:
//! ```compile_fail
//! use forge_store_blob_chunks::{BlobChunkDedupeShareClaim, BlobChunkRegisteredDedupeReference};
//! fn requires_registered(_: BlobChunkRegisteredDedupeReference) {}
//! let claim: BlobChunkDedupeShareClaim = todo!();
//! requires_registered(claim);
//! ```
//! Copied scope counters cannot satisfy blob security scope:
//! ```compile_fail
//! use forge_store_blob_chunks::{BlobChunkScopeCounterSnapshot, BlobChunkSecurityScope};
//! fn requires_scope(_: BlobChunkSecurityScope) {}
//! let counters: BlobChunkScopeCounterSnapshot = todo!();
//! requires_scope(counters);
//! ```
//! Copied digest strings cannot construct lifecycle receipts:
//! ```compile_fail
//! use forge_store_blob_chunks::LifecycleReceipt;
//! fn requires_lifecycle_receipt(_: LifecycleReceipt) {}
//! let digest = "sha256:copied";
//! requires_lifecycle_receipt(digest);
//! ```
//! Copied lifecycle counters cannot construct lifecycle receipts:
//! ```compile_fail
//! use forge_store_blob_chunks::{BlobLifecycleCounterSnapshot, LifecycleReceipt};
//! fn requires_lifecycle_receipt(_: LifecycleReceipt) {}
//! let counters: BlobLifecycleCounterSnapshot = todo!();
//! requires_lifecycle_receipt(counters);
//! ```
//! S.6 placement readiness seeds cannot construct lifecycle receipts:
//! ```compile_fail
//! use forge_store_blob_chunks::LifecycleReceipt;
//! use forge_store_readiness::S6ClosedS7PlacementAdmissionSeed;
//! fn requires_lifecycle_receipt(_: LifecycleReceipt) {}
//! let seed: S6ClosedS7PlacementAdmissionSeed = todo!();
//! requires_lifecycle_receipt(seed);
//! ```
//! Scoped chunks cannot be synthesized from raw fields:
//! ```compile_fail
//! use forge_store_blob_chunks::ScopedBlobChunk;
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
//! let _forged = BlobPlacementProof {
//!     stored_digest: todo!(), security_metadata: todo!(), class: todo!(),
//!     counters: todo!(), non_claims: todo!(),
//! };
//! ```
//! Admitted placements cannot be synthesized from raw fields:
//! ```compile_fail
//! use forge_store_blob_chunks::AdmittedBlobPlacement;
//! let _forged = AdmittedBlobPlacement {
//!     basis: todo!(), stored_digest: todo!(), security_metadata: todo!(),
//!     class: todo!(), cold_state: todo!(), counters: todo!(), non_claims: todo!(),
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
//! let copied = StableDigest::new("sha256:copied").unwrap();
//! let _forged = BlobObjectId::from_declared_digest(copied);
//! ```
//! Store authority resolution cannot be skipped with a zero-argument call:
//! ```compile_fail
//! use forge_store_blob_chunks::{BlobLifecycleAdmission, BlobLifecycleDeclaration};
//! let declaration: BlobLifecycleDeclaration = todo!();
//! let _resolved = BlobLifecycleAdmission::start(declaration).resolve_store_authority();
//! ```
//! Copied stored chunk digests cannot build lifecycle replay inputs:
//! ```compile_fail
//! use forge_store_blob_chunks::{BlobLifecycleReplayInput, StoredChunkDigest};
//! let digest: StoredChunkDigest = todo!();
//! let _replay = BlobLifecycleReplayInput::from_stored_chunk_digest(digest);
//! ```
//! Placement proofs cannot be constructed directly from a copied S.6 seed:
//! ```compile_fail
//! use forge_store_blob_chunks::{AdmittedBlobPlacement, BlobPlacementProof};
//! let placement: AdmittedBlobPlacement = todo!();
//! let _proof = BlobPlacementProof::from_admitted_placement(&placement);
//! ```
//! Root candidates cannot be promoted to visible blob generations:
//! ```compile_fail
//! use forge_store_blob_chunks::{BlobRootCandidateForPublication, BlobVisibleGeneration};
//! fn requires_visible(_: BlobVisibleGeneration) {}
//! let candidate: BlobRootCandidateForPublication = todo!();
//! requires_visible(candidate);
//! ```
//! Staged reachability cannot be promoted to visible blob generations:
//! ```compile_fail
//! use forge_store_blob_chunks::{BlobReachabilityStaging, BlobVisibleGeneration};
//! fn requires_visible(_: BlobVisibleGeneration) {}
//! let staged: BlobReachabilityStaging = todo!();
//! requires_visible(staged);
//! ```
//! Copied durable publication declarations cannot make blobs visible:
//! ```compile_fail
//! use forge_store_blob_chunks::BlobVisibleGeneration;
//! use forge_store_wal::DurablePublicationDeclaration;
//!
//! fn requires_visible(_: BlobVisibleGeneration) {}
//!
//! let record: DurablePublicationDeclaration = todo!();
//! requires_visible(record);
//! ```
//! Semantic references cannot make blobs visible:
//! ```compile_fail
//! use forge_store_blob_chunks::BlobVisibleGeneration;
//! use forge_store_physical_isolation::SemanticVisibilityReference;
//!
//! fn requires_visible(_: BlobVisibleGeneration) {}
//!
//! let reference: SemanticVisibilityReference = todo!();
//! requires_visible(reference);
//! ```
#![forbid(unsafe_code)]
mod blob_chunk_bytes;
mod blob_chunk_canonical_basis;
mod blob_chunk_canonical_comparison_basis;
mod blob_chunk_collision_verification;
mod blob_chunk_counters;
mod blob_chunk_dedupe;
mod blob_chunk_dedupe_byte_comparison;
mod blob_chunk_dedupe_canonical;
mod blob_chunk_dedupe_collision;
mod blob_chunk_dedupe_counters;
mod blob_chunk_dedupe_index_posture;
mod blob_chunk_dedupe_policy;
mod blob_chunk_dedupe_receipt;
mod blob_chunk_dedupe_reference_edges;
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
mod blob_chunk_reference_accounting;
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
mod blob_compaction;
mod blob_corruption;
#[doc(hidden)]
pub mod blob_corruption_compile_fail;
#[cfg(test)]
mod blob_corruption_tests;
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
mod blob_placement_admission;
mod blob_placement_movement;
mod blob_placement_proof;
mod blob_publication_commit;
#[doc(hidden)]
pub mod blob_publication_commit_compile_fail;
#[cfg(test)]
mod blob_publication_commit_test_support;
#[cfg(test)]
mod blob_publication_commit_tests;
#[cfg(test)]
mod blob_reachability_authority_tests;
#[cfg(test)]
mod blob_reachability_checkpoint_tests;
#[doc(hidden)]
pub mod blob_reachability_compile_fail;
mod blob_reachability_counters;
#[cfg(test)]
mod blob_reachability_dedupe_release_tests;
mod blob_reachability_denial;
mod blob_reachability_edges;
#[cfg(test)]
mod blob_reachability_hold_test_support;
mod blob_reachability_holds;
mod blob_reachability_proof;
mod blob_reachability_reclaim_release;
mod blob_reachability_registry;
mod blob_reachability_snapshot;
#[cfg(test)]
mod blob_reachability_tests;
#[cfg(test)]
mod blob_recovery_record_generation_tests;
mod blob_recovery_records;
#[doc(hidden)]
pub mod blob_recovery_records_compile_fail;
#[cfg(test)]
mod blob_recovery_records_residue_tests;
#[cfg(test)]
mod blob_recovery_records_tests;
mod blob_resume_session;
mod blob_retention_reclaim;
#[doc(hidden)]
pub mod blob_retention_reclaim_compile_fail;
#[cfg(test)]
mod blob_retention_reclaim_test_support;
#[cfg(test)]
mod blob_retention_reclaim_tests;
mod blob_scoped_chunk;
mod blob_streaming_counters;
mod blob_streaming_denial;
#[cfg(test)]
mod blob_streaming_equivalence_tests;
mod blob_streaming_frontier;
mod blob_streaming_ingest;
#[cfg(test)]
mod blob_streaming_ingest_tests;
mod blob_streaming_performance;
#[cfg(test)]
mod blob_streaming_pressure_tests;
mod blob_streaming_read;
mod blob_streaming_request;
mod blob_streaming_residency;
mod blob_streaming_resume;
#[cfg(test)]
mod blob_streaming_resume_tests;
mod blob_streaming_source;
mod exports;
mod large_record_streaming_envelope;
mod s6_background_pressure;
mod s6_reclaim_handoff;
mod s7_blob_security_handoff;
mod s7_harness_vocab;
#[doc(hidden)]
pub mod security_metadata_compile_fail;
pub use exports::*;
