//! Current-root scope posture cannot be synthesized without membership proof:
//! ```compile_fail
//! use worth_store_physical_format::{
//!     CurrentRootManifestAdmission, RootManifestIntegrityPosture,
//! };
//!
//! let _forged = RootManifestIntegrityPosture::CurrentRootAdmitted(
//!     CurrentRootManifestAdmission { root_owner: todo!() },
//! );
//! ```
//! Chunk checksum witnesses cannot be synthesized from copied checksum values:
//! ```compile_fail
//! use worth_store_physical_format::{PhysicalChunkChecksum, PhysicalChunkChecksumWitness};
//!
//! let _forged = PhysicalChunkChecksumWitness {
//!     checksum: PhysicalChunkChecksum { algorithm: todo!(), digest: todo!() },
//!     bytes_checked: 1,
//! };
//! ```
//! Chunk checksum witnesses cannot be minted directly from raw bytes:
//! ```compile_fail
//! use worth_store_physical_format::PhysicalChunkChecksumAuthority;
//!
//! let _forged = PhysicalChunkChecksumAuthority::canonical_blob_checksum().verify(b"raw");
//! ```
//! Chunk payload integrity cannot be admitted from raw bytes without Store write admission:
//! ```compile_fail
//! use worth_store_physical_format::PhysicalChunkChecksumAuthority;
//!
//! let _forged = PhysicalChunkChecksumAuthority::canonical_blob_checksum().admit_store_payload(b"raw");
//! ```
//! Store physical chunk write receipts cannot be forged from raw fields:
//! ```compile_fail
//! use worth_store_physical_format::StorePhysicalChunkWriteReceipt;
//!
//! let _forged = StorePhysicalChunkWriteReceipt {
//!     payload_bytes: b"raw",
//!     bytes_written: 3,
//!     source: todo!(),
//!     _seal: todo!(),
//! };
//! ```
//! Store physical chunk write admission cannot be requested from raw authority and bytes:
//! ```compile_fail
//! use worth_store_contracts::{StorePhysicalAuthorityWitness, ROADMAP_2_S1_SCOPE};
//! use worth_store_physical_format::PhysicalChunkChecksumAuthority;
//!
//! let authority = StorePhysicalAuthorityWitness::for_physical_format_vocabulary(ROADMAP_2_S1_SCOPE).unwrap();
//! let _forged = PhysicalChunkChecksumAuthority::canonical_blob_checksum()
//!     .admit_store_payload((authority, b"raw"));
//! ```
//! Bootstrap-open witnesses cannot be forged from raw fields:
//! ```compile_fail
//! use worth_store_physical_format::PhysicalBootstrapCatalogOpenWitness;
//!
//! let _forged = PhysicalBootstrapCatalogOpenWitness {
//!     byte_order: todo!(),
//!     physical_format_version: todo!(),
//!     root_manifest_candidates: vec![],
//!     segment_manifest: vec![],
//!     extent_manifest: vec![],
//!     free_space_map: vec![],
//! };
//! ```
//! Bootstrap-open witnesses cannot be admitted from raw persisted layout outside the owner lane:
//! ```compile_fail
//! use worth_store_physical_format::{
//!     PersistedPhysicalLayout, PhysicalBootstrapCatalogOpenWitness, PlatformPhysicalOpenRequest,
//! };
//!
//! let layout = PersistedPhysicalLayout::builder().build();
//! let _forged = PhysicalBootstrapCatalogOpenWitness::admit_persisted_layout(
//!     PlatformPhysicalOpenRequest::physical_format_canonical().headers(),
//!     &layout,
//! );
//! ```
//! Raw persisted layouts cannot reopen the ordinary public physical facade directly:
//! ```compile_fail
//! use worth_store_contracts::{
//!     AcceptedHandoffReadiness, HandoffEvidenceDigestSet, StableDigest, ROADMAP_2_S1_SCOPE,
//! };
//! use worth_store_physical_format::{
//!     PersistedPhysicalLayout, PhysicalStoreRuntime, PlatformPhysicalOpenRequest,
//! };
//!
//! let layout = PersistedPhysicalLayout::builder().build();
//! let _reopened = PhysicalStoreRuntime::reopen(
//!     AcceptedHandoffReadiness::from_foundational_handoff_artifacts(
//!         ROADMAP_2_S1_SCOPE,
//!         HandoffEvidenceDigestSet::new(
//!             StableDigest::new("sha256:backend".to_string()).unwrap(),
//!             StableDigest::new("sha256:deferred".to_string()).unwrap(),
//!             StableDigest::new("sha256:harness".to_string()).unwrap(),
//!             StableDigest::new("sha256:terms".to_string()).unwrap(),
//!             StableDigest::new("sha256:audit".to_string()).unwrap(),
//!             StableDigest::new("sha256:complexity".to_string()).unwrap(),
//!             StableDigest::new("sha256:provenance".to_string()).unwrap(),
//!         ),
//!     )
//!     .unwrap(),
//!     PlatformPhysicalOpenRequest::physical_format_canonical(),
//!     layout,
//! );
//! ```
