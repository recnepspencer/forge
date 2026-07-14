//! External recovery probe evidence cannot be minted from manifest validation
//! alone:
//! ```compile_fail
//! use worth_store_physical_backend::{
//!     BlobPhysicalManifestValidation, ExternalPlacementRecoveryProbe,
//! };
//! let manifest: BlobPhysicalManifestValidation = todo!();
//! let _forged = ExternalPlacementRecoveryProbe::from_manifest_validation(manifest);
//! ```
//! External cleanup evidence cannot be minted by copying an orphan-scan token:
//! ```compile_fail
//! use worth_store_physical_backend::{
//!     ExternalPlacementCleanupReceipt, ExternalPlacementOrphanScanReceipt,
//! };
//! let orphan_scan: ExternalPlacementOrphanScanReceipt = todo!();
//! let _forged = ExternalPlacementCleanupReceipt::from_orphan_scan(&orphan_scan);
//! ```
