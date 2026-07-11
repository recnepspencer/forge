use super::capability::S8StrategyCapability;
use super::counter_planning::declared_strategy_counter_envelope;
use super::key_law_validation::S8DeclaredKeyLawPosture;
use super::posture::{
    S8StrategyAmplificationProfile, S8StrategyCorruptionIsolationBehavior,
    S8StrategyLocalityProfile, S8StrategyMaterializationPosture,
    S8StrategyRebuildSourceRequirement,
};
use super::S8LayoutStrategyFamily;
use crate::artifact_family::{
    ArtifactFamilyAccessLane, ArtifactFamilyAuthorityClass, ArtifactFamilyLifecycleAdmission,
    DurableArtifactMigrationPosture, DurableArtifactProjectionClass, DurableArtifactRebuildPosture,
};
use crate::budget::S8PlannedCounterEnvelope;
use crate::key_domain::{
    CanonicalKeyEncoding, ComparatorLaw, PhysicalKeyDomainWitness, PrefixLawWitness,
    RangeBoundLawWitness,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct S8StrategyDeclaration {
    lifecycle: ArtifactFamilyLifecycleAdmission,
    key_domain: PhysicalKeyDomainWitness,
    family: S8LayoutStrategyFamily,
    capability: S8StrategyCapability,
    locality: S8StrategyLocalityProfile,
    amplification: S8StrategyAmplificationProfile,
    materialization: S8StrategyMaterializationPosture,
    rebuild_source: S8StrategyRebuildSourceRequirement,
    corruption_isolation: S8StrategyCorruptionIsolationBehavior,
    access_lane: ArtifactFamilyAccessLane,
    authority_class: ArtifactFamilyAuthorityClass,
    rebuild_posture: DurableArtifactRebuildPosture,
    migration_posture: DurableArtifactMigrationPosture,
    projection_classes: &'static [DurableArtifactProjectionClass],
    canonical_key_encoding: Option<CanonicalKeyEncoding>,
    comparator_law: Option<ComparatorLaw>,
    prefix_law: Option<PrefixLawWitness>,
    range_bound_law: Option<RangeBoundLawWitness>,
    planned_counter_envelope: Option<S8PlannedCounterEnvelope>,
}

impl S8StrategyDeclaration {
    pub(super) const fn baseline_btree_range(
        lifecycle: ArtifactFamilyLifecycleAdmission,
        key_domain: PhysicalKeyDomainWitness,
        key_laws: S8DeclaredKeyLawPosture,
    ) -> Self {
        let artifact = lifecycle.declaration();
        Self {
            lifecycle,
            key_domain,
            family: S8LayoutStrategyFamily::BaselineBTreeRange,
            capability: S8StrategyCapability::baseline_btree_range(),
            locality: S8StrategyLocalityProfile::OrderedPageLocality,
            amplification: S8StrategyAmplificationProfile::SplitMergeBounded,
            materialization: S8StrategyMaterializationPosture::PublishedTreeLifecycle,
            rebuild_source: S8StrategyRebuildSourceRequirement::PhysicalSnapshotReplay,
            corruption_isolation: S8StrategyCorruptionIsolationBehavior::PageScoped,
            access_lane: artifact.access_lane(),
            authority_class: artifact.authority(),
            rebuild_posture: artifact.rebuild_posture(),
            migration_posture: artifact.migration_posture(),
            projection_classes: artifact.non_authority_projection_classes(),
            canonical_key_encoding: Some(key_laws.encoding()),
            comparator_law: Some(key_laws.comparator()),
            prefix_law: key_laws.prefix(),
            range_bound_law: key_laws.range(),
            planned_counter_envelope: None,
        }
    }

    pub(super) const fn baseline_lsm_write_optimized(
        lifecycle: ArtifactFamilyLifecycleAdmission,
        key_domain: PhysicalKeyDomainWitness,
        key_laws: S8DeclaredKeyLawPosture,
    ) -> Self {
        let artifact = lifecycle.declaration();
        Self {
            lifecycle,
            key_domain,
            family: S8LayoutStrategyFamily::BaselineLsmWriteOptimized,
            capability: S8StrategyCapability::baseline_lsm_write_optimized(),
            locality: S8StrategyLocalityProfile::BufferedRunLocality,
            amplification: S8StrategyAmplificationProfile::CompactionWriteAmplified,
            materialization: S8StrategyMaterializationPosture::WalBufferedRunLifecycle,
            rebuild_source: S8StrategyRebuildSourceRequirement::WalReplay,
            corruption_isolation: S8StrategyCorruptionIsolationBehavior::RunScoped,
            access_lane: artifact.access_lane(),
            authority_class: artifact.authority(),
            rebuild_posture: artifact.rebuild_posture(),
            migration_posture: artifact.migration_posture(),
            projection_classes: artifact.non_authority_projection_classes(),
            canonical_key_encoding: Some(key_laws.encoding()),
            comparator_law: Some(key_laws.comparator()),
            prefix_law: None,
            range_bound_law: None,
            planned_counter_envelope: declared_strategy_counter_envelope(
                S8LayoutStrategyFamily::BaselineLsmWriteOptimized,
            ),
        }
    }

    pub const fn lifecycle(self) -> ArtifactFamilyLifecycleAdmission {
        self.lifecycle
    }

    pub const fn key_domain(self) -> PhysicalKeyDomainWitness {
        self.key_domain
    }

    pub const fn family(self) -> S8LayoutStrategyFamily {
        self.family
    }

    pub(crate) const fn capability(self) -> S8StrategyCapability {
        self.capability
    }

    pub(crate) const fn locality(self) -> S8StrategyLocalityProfile {
        self.locality
    }

    pub(crate) const fn amplification(self) -> S8StrategyAmplificationProfile {
        self.amplification
    }

    pub(crate) const fn materialization(self) -> S8StrategyMaterializationPosture {
        self.materialization
    }

    pub(crate) const fn rebuild_source(self) -> S8StrategyRebuildSourceRequirement {
        self.rebuild_source
    }

    pub(crate) const fn corruption_isolation(self) -> S8StrategyCorruptionIsolationBehavior {
        self.corruption_isolation
    }

    pub(crate) const fn access_lane(self) -> ArtifactFamilyAccessLane {
        self.access_lane
    }

    pub(crate) const fn authority_class(self) -> ArtifactFamilyAuthorityClass {
        self.authority_class
    }

    pub(crate) const fn rebuild_posture(self) -> DurableArtifactRebuildPosture {
        self.rebuild_posture
    }

    pub(crate) const fn migration_posture(self) -> DurableArtifactMigrationPosture {
        self.migration_posture
    }

    pub(crate) const fn projection_classes(self) -> &'static [DurableArtifactProjectionClass] {
        self.projection_classes
    }

    pub(crate) const fn canonical_key_encoding(self) -> Option<CanonicalKeyEncoding> {
        self.canonical_key_encoding
    }

    pub(crate) const fn comparator_law(self) -> Option<ComparatorLaw> {
        self.comparator_law
    }

    pub(crate) const fn prefix_law(self) -> Option<PrefixLawWitness> {
        self.prefix_law
    }

    pub(crate) const fn range_bound_law(self) -> Option<RangeBoundLawWitness> {
        self.range_bound_law
    }

    pub(crate) const fn planned_counter_envelope(self) -> Option<S8PlannedCounterEnvelope> {
        self.planned_counter_envelope
    }
}
