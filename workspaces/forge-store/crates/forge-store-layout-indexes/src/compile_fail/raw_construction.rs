//! ```compile_fail
//! use forge_store_contracts::{
//!     ArtifactFamilyAccessLane, ArtifactFamilyAuthorityClass, ArtifactFamilyLifecycleClass,
//!     DurableArtifactFamilyId, DurableArtifactMigrationPosture, DurableArtifactOwningBoundary,
//!     DurableArtifactRebuildPosture,
//! };
//! use forge_store_layout_indexes::PhysicalArtifactFamilyDeclaration;
//!
//! let _forged = PhysicalArtifactFamilyDeclaration {
//!     family: DurableArtifactFamilyId::WalDurableMutationIntent,
//!     authority: ArtifactFamilyAuthorityClass::Authoritative,
//!     lifecycle: ArtifactFamilyLifecycleClass::CoreState,
//!     access_lane: ArtifactFamilyAccessLane::HotPath,
//!     owning_boundary: DurableArtifactOwningBoundary::ForgeStoreWal,
//!     rebuild_posture: DurableArtifactRebuildPosture::ReplayRebuildable,
//!     migration_posture: DurableArtifactMigrationPosture::VersionedReadmission,
//!     non_authority_projection_classes: &[],
//! };
//! ```
