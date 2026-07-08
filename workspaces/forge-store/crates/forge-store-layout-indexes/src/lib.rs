#![forbid(unsafe_code)]

mod access_shape;
mod artifact_family;
mod bootstrap;
mod budget;
mod compile_fail;
mod corruption;
mod degraded_access;
mod execution;
mod facade;
mod handoff;
mod key_domain;
mod legacy_disposition;
mod maintenance;
mod materialization;
mod migration;
mod planning;
mod skeleton;
mod strategy;

pub use artifact_family::{
    ArtifactAuthorityRoleWitness, ArtifactDerivedAccuracyWitness, ArtifactFamilyAccessLane,
    ArtifactFamilyAuthorityClass, ArtifactFamilyAuthorityDisposition,
    ArtifactFamilyAuthorityWitness, ArtifactFamilyClassification, ArtifactFamilyDenial,
    ArtifactFamilyInventoryRow, ArtifactFamilyLifecycleAdmission, ArtifactFamilyLifecycleClass,
    ArtifactFamilyLifecycleDisposition, ArtifactFamilyStrategyLane, ArtifactKeyScopePartition,
    ArtifactScopePartitionWitness, ArtifactTenantScopePartition, AuthorityRole,
    DerivedAccuracyClass, DurableArtifactMigrationPosture, DurableArtifactProjectionClass,
    DurableArtifactRebuildPosture, ExistingArtifactFamilySurface, PhysicalArtifactFamily,
    PhysicalArtifactFamilyDeclaration, S8ArtifactFamilyInventory,
};
pub use execution::S8PlannedVsObservedCounterReceipt;
pub use facade::*;
pub use key_domain::{
    CanonicalKeyBytes, CanonicalKeyEncoding, ComparatorBehavior, ComparatorLaw, CompositeKeyField,
    CompositeKeyOrderingLaw, ConcretePhysicalKeyWitness, EncodingSentinelPolicy,
    HashCollisionBehavior, HashCollisionLaw, PhysicalKeyDomain, PhysicalKeyDomainDenial,
    PhysicalKeyDomainWitness, PrefixBoundaryBehavior, PrefixLawWitness, RangeBoundBehavior,
    RangeBoundLawWitness, TenantScopedKeyDomain,
};
pub use strategy::{
    S8BTreeCorruptionRegion, S8BTreeInvariantSuite, S8BTreeLookupBranch, S8BTreeNodeFormatLaw,
    S8BTreeRebuildMigrationLaw, S8BTreeRootPublicationLaw, S8BTreeSearchPathLaw,
    S8BTreeSeparatorLaw, S8BTreeSiblingLinkLaw, S8BTreeSplitMergeLaw, S8BTreeStableReadLaw,
    S8BTreeTombstoneLaw, S8LsmAdvisoryFilterLaw, S8LsmCompactionOrderingLaw, S8LsmInvariantSuite,
    S8LsmLookupDisposition, S8LsmMemtableWalLaw, S8LsmRunPublicationLaw, S8LsmStaleRunCleanupLaw,
    S8LsmTombstoneLaw, S8LsmWriteAmplificationLaw, S8StrategyCounterEvidence,
    S8StrategyCounterProfile, S8StrategyDenial, S8StrategyIntegrityInvariant,
    S8StrategyInvariantSuite, S8StrategyLookupInvariant, S8StrategyMutationInvariant,
    S8StrategyPublicationInvariant, S8StrategyRecoveryInvariant,
};

#[path = "compile_fail/certification_authority.rs"]
#[doc(hidden)]
pub mod s8_certification_authority_compile_fail;
#[path = "compile_fail/compaction_family_kind_shortcut.rs"]
#[doc(hidden)]
pub mod s8_compaction_family_kind_shortcut_compile_fail;
#[path = "compile_fail/derived_authority_shortcut.rs"]
#[doc(hidden)]
pub mod s8_derived_authority_shortcut_compile_fail;
#[path = "compile_fail/facade_bypass.rs"]
#[doc(hidden)]
pub mod s8_facade_bypass_compile_fail;
#[path = "compile_fail/lifecycle_strategy_shortcut.rs"]
#[doc(hidden)]
pub mod s8_lifecycle_strategy_shortcut_compile_fail;
#[path = "compile_fail/raw_accuracy_shortcut.rs"]
#[doc(hidden)]
pub mod s8_raw_accuracy_shortcut_compile_fail;
#[path = "compile_fail/raw_construction.rs"]
#[doc(hidden)]
pub mod s8_raw_construction_compile_fail;
#[path = "compile_fail/raw_declaration_classification_shortcut.rs"]
#[doc(hidden)]
pub mod s8_raw_declaration_classification_shortcut_compile_fail;
#[path = "compile_fail/raw_hash_identity_shortcut.rs"]
#[doc(hidden)]
pub mod s8_raw_hash_identity_shortcut_compile_fail;
#[path = "compile_fail/raw_key_comparator_shortcut.rs"]
#[doc(hidden)]
pub mod s8_raw_key_comparator_shortcut_compile_fail;
#[path = "compile_fail/raw_range_bound_shortcut.rs"]
#[doc(hidden)]
pub mod s8_raw_range_bound_shortcut_compile_fail;
#[path = "compile_fail/raw_role_shortcut.rs"]
#[doc(hidden)]
pub mod s8_raw_role_shortcut_compile_fail;
#[path = "compile_fail/raw_scope_shortcut.rs"]
#[doc(hidden)]
pub mod s8_raw_scope_shortcut_compile_fail;
