//! Store contract vocabulary.
//!
//! External callers cannot mint physical authority witnesses directly:
//!
//! ```compile_fail
//! use forge_store_contracts::{
//!     PhysicalAuthorityScope, StorePhysicalAuthorityWitness, ROADMAP_2_S1_SCOPE,
//! };
//!
//! let _forged = StorePhysicalAuthorityWitness {
//!     roadmap_scope: ROADMAP_2_S1_SCOPE,
//!     authority_scope: PhysicalAuthorityScope::PhysicalEvidenceExport,
//! };
//! ```

#![forbid(unsafe_code)]

mod artifact_identity;
mod compatibility_family;
mod contract_error;
mod corruption_handoff_damage_case;
mod durable_artifact_family_classification;
mod durable_artifact_family_id;
mod existing_artifact_family;
mod handoff_readiness;
mod physical_authority;
mod resources;
mod roadmap_scope;
mod s2_physical_substrate_snapshot;
mod s3_readiness_denial;
mod s3_readiness_payload;
mod s3_readiness_recap;
mod s6_background_pressure;

pub use artifact_identity::{StableArtifactId, StableDigest};
pub use compatibility_family::{
    ArtifactFamilyId, CompatibilityAuthorityClassification, CompatibilityFamilyKind,
    FIRST_SHIP_COMPATIBILITY_FAMILIES, FIRST_SHIP_COMPATIBILITY_FAMILY_COUNT,
};
pub use contract_error::{StoreContractError, StoreContractResult};
pub use corruption_handoff_damage_case::CorruptionHandoffDamageCase;
pub use durable_artifact_family_classification::{
    ArtifactFamilyAccessLane, ArtifactFamilyAuthorityClass, ArtifactFamilyLifecycleClass,
    DurableArtifactMigrationPosture, DurableArtifactOwningBoundary, DurableArtifactProjectionClass,
    DurableArtifactRebuildPosture,
};
pub use durable_artifact_family_id::DurableArtifactFamilyId;
pub use existing_artifact_family::{
    DerivedFamilyRetentionPolicy, LayoutCompactionFamilyKind, LayoutFamilyCompactionUnit,
    MaintenanceArtifactFamily, PlacementArtifactFamily, PublicationFamily, SupportArtifactFamily,
    WalRecordFamily,
};
pub use handoff_readiness::{
    AcceptedHandoffReadiness, HandoffEvidenceDigestSet, S0HandoffArtifactKind,
};
pub use physical_authority::{
    PhysicalAuthorityBoundaryInstance, PhysicalAuthorityScope, StorePhysicalAuthorityWitness,
    ROADMAP_2_PRIMARY_PHYSICAL_BOUNDARY, ROADMAP_2_REPLAY_PHYSICAL_BOUNDARY,
};
pub use resources::{QueueProducerKind, QueueProducerResourceShape};
pub use roadmap_scope::{
    RoadmapScope, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE, ROADMAP_2_S1_SCOPE, ROADMAP_2_SCOPE,
};
pub use s2_physical_substrate_snapshot::S2PhysicalSubstrateReadinessSnapshot;
pub use s3_readiness_denial::{S3ReadinessDenial, S3ReadinessDenialKind};
pub use s3_readiness_payload::{
    IntegrityInspectionLifetimeLaw, ProtectedIntegrityViewCapability, S2NoMaterializationWitness,
    S3PhysicalIntegrityReadinessPayload, ScrubPlanningAllocationEnvelope, VerifierResidentEnvelope,
};
pub use s3_readiness_recap::{
    BufferPoolAuthorityRecap, PhysicalAuthorityRecap, S2BoundedCounterRecap, S2DenialBehaviorRecap,
    S2DeniedBoundaryKind,
};
pub use s6_background_pressure::{
    IoPressureBackgroundPressureDeclaration, IoPressureBackgroundPressureKind,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DurableArtifactClass {
    Authoritative,
    DerivedDurable,
    Ephemeral,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DerivedAccuracyClass {
    Exact,
    Conservative,
    Approximate,
    Heuristic,
    Advisory,
}
