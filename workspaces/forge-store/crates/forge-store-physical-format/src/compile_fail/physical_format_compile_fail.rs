//! Current-root scope posture cannot be synthesized without membership proof:
//! ```compile_fail
//! use forge_store_physical_format::{
//!     CurrentRootManifestAdmission, RootManifestIntegrityPosture,
//! };
//!
//! let _forged = RootManifestIntegrityPosture::CurrentRootAdmitted(
//!     CurrentRootManifestAdmission { root_owner: todo!() },
//! );
//! ```
//! Chunk checksum witnesses cannot be synthesized from copied checksum values:
//! ```compile_fail
//! use forge_store_physical_format::{PhysicalChunkChecksum, PhysicalChunkChecksumWitness};
//!
//! let _forged = PhysicalChunkChecksumWitness {
//!     checksum: PhysicalChunkChecksum { algorithm: todo!(), digest: todo!() },
//!     bytes_checked: 1,
//! };
//! ```
//! Chunk checksum witnesses cannot be minted directly from raw bytes:
//! ```compile_fail
//! use forge_store_physical_format::PhysicalChunkChecksumAuthority;
//!
//! let _forged = PhysicalChunkChecksumAuthority::s7_canonical().verify(b"raw");
//! ```
//! Chunk payload integrity cannot be admitted from raw bytes without Store write admission:
//! ```compile_fail
//! use forge_store_physical_format::PhysicalChunkChecksumAuthority;
//!
//! let _forged = PhysicalChunkChecksumAuthority::s7_canonical().admit_store_payload(b"raw");
//! ```
//! Store physical chunk write receipts cannot be forged from raw fields:
//! ```compile_fail
//! use forge_store_physical_format::StorePhysicalChunkWriteReceipt;
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
//! use forge_store_contracts::{StorePhysicalAuthorityWitness, ROADMAP_2_S1_SCOPE};
//! use forge_store_physical_format::PhysicalChunkChecksumAuthority;
//!
//! let authority = StorePhysicalAuthorityWitness::for_s1_vocabulary(ROADMAP_2_S1_SCOPE).unwrap();
//! let _forged = PhysicalChunkChecksumAuthority::s7_canonical()
//!     .admit_store_payload((authority, b"raw"));
//! ```
