//! ```compile_fail
//! use worth_store_contracts::{
//!     ArtifactFamilyAccessLane, ArtifactFamilyAuthorityClass, ArtifactFamilyLifecycleClass,
//!     DurableArtifactFamilyId, DurableArtifactMigrationPosture, DurableArtifactOwningBoundary,
//!     DurableArtifactRebuildPosture,
//! };
//! use worth_store_layout_indexes::PhysicalArtifactFamilyDeclaration;
//!
//! let _WORTHd = PhysicalArtifactFamilyDeclaration {
//!     family: DurableArtifactFamilyId::WalDurableMutationIntent,
//!     authority: ArtifactFamilyAuthorityClass::Authoritative,
//!     lifecycle: ArtifactFamilyLifecycleClass::CoreState,
//!     access_lane: ArtifactFamilyAccessLane::HotPath,
//!     owning_boundary: DurableArtifactOwningBoundary::WORTHStoreWal,
//!     rebuild_posture: DurableArtifactRebuildPosture::ReplayRebuildable,
//!     migration_posture: DurableArtifactMigrationPosture::VersionedReadmission,
//!     non_authority_projection_classes: &[],
//! };
//! ```
