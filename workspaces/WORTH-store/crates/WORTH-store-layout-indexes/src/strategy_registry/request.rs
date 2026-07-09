use crate::artifact_family::{
    ArtifactFamilyAccessLane, ArtifactFamilyLifecycleAdmission, ArtifactScopePartitionWitness,
    DurableArtifactMigrationPosture,
};
use crate::key_domain::{CompositeKeyOrderingLaw, HashCollisionLaw, PhysicalKeyDomainWitness};
use crate::maintenance::{
    S8IndexMaintenanceMode, S8LiveExactMaintenanceWitness, S8PhysicalMutationShape,
};
use crate::materialization::S8LayoutCoverageWitness;
use crate::strategy::S8LayoutStrategyFamily;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8LayoutRequestedCapability {
    PointLookup,
    OrderedRange,
    PrefixTraversal,
    ExactScan,
    BlobStreaming,
}

impl S8LayoutRequestedCapability {
    pub const fn point_lookup() -> Self {
        Self::PointLookup
    }

    pub const fn ordered_range() -> Self {
        Self::OrderedRange
    }

    pub const fn prefix_traversal() -> Self {
        Self::PrefixTraversal
    }

    pub const fn exact_scan() -> Self {
        Self::ExactScan
    }

    pub const fn blob_streaming() -> Self {
        Self::BlobStreaming
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8LayoutStrategyCapability {
    PointLookup,
    OrderedRange,
    PrefixTraversal,
    ExactScan,
    BlobStreaming,
}

impl S8LayoutStrategyCapability {
    pub const fn from_requested(requested: S8LayoutRequestedCapability) -> Self {
        match requested {
            S8LayoutRequestedCapability::PointLookup => Self::PointLookup,
            S8LayoutRequestedCapability::OrderedRange => Self::OrderedRange,
            S8LayoutRequestedCapability::PrefixTraversal => Self::PrefixTraversal,
            S8LayoutRequestedCapability::ExactScan => Self::ExactScan,
            S8LayoutRequestedCapability::BlobStreaming => Self::BlobStreaming,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8RequestedKeyLawSet {
    hash_equality_law: Option<HashCollisionLaw>,
    composite_ordering_law: Option<CompositeKeyOrderingLaw>,
}

impl S8RequestedKeyLawSet {
    pub const fn new() -> Self {
        Self {
            hash_equality_law: None,
            composite_ordering_law: None,
        }
    }

    pub const fn require_hash_equality(mut self, law: HashCollisionLaw) -> Self {
        self.hash_equality_law = Some(law);
        self
    }

    pub const fn require_composite_ordering(mut self, law: CompositeKeyOrderingLaw) -> Self {
        self.composite_ordering_law = Some(law);
        self
    }

    pub const fn hash_equality_law(self) -> Option<HashCollisionLaw> {
        self.hash_equality_law
    }

    pub const fn composite_ordering_law(self) -> Option<CompositeKeyOrderingLaw> {
        self.composite_ordering_law
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8LayoutAdmissionRequest {
    lifecycle: ArtifactFamilyLifecycleAdmission,
    key_domain: PhysicalKeyDomainWitness,
    family: S8LayoutStrategyFamily,
    requested_capability: S8LayoutRequestedCapability,
    requested_lane: ArtifactFamilyAccessLane,
    required_scope_partition: ArtifactScopePartitionWitness,
    maintenance_mode: S8IndexMaintenanceMode,
    mutation_shape: S8PhysicalMutationShape,
    required_migration_posture: Option<DurableArtifactMigrationPosture>,
    required_key_laws: S8RequestedKeyLawSet,
    require_exact_materialization: bool,
    exact_coverage: Option<S8LayoutCoverageWitness>,
    exact_maintenance_witness: Option<S8LiveExactMaintenanceWitness>,
    require_exact_absence_proof: bool,
}

impl S8LayoutAdmissionRequest {
    pub const fn new(
        lifecycle: ArtifactFamilyLifecycleAdmission,
        key_domain: PhysicalKeyDomainWitness,
        family: S8LayoutStrategyFamily,
        requested_capability: S8LayoutRequestedCapability,
        requested_lane: ArtifactFamilyAccessLane,
    ) -> Self {
        Self {
            lifecycle,
            key_domain,
            family,
            requested_capability,
            requested_lane,
            required_scope_partition: key_domain.scope(),
            maintenance_mode: S8IndexMaintenanceMode::SynchronousExact,
            mutation_shape: S8PhysicalMutationShape::ObservationOnly,
            required_migration_posture: None,
            required_key_laws: S8RequestedKeyLawSet::new(),
            require_exact_materialization: false,
            exact_coverage: None,
            exact_maintenance_witness: None,
            require_exact_absence_proof: false,
        }
    }

    pub const fn within_scope_partition(mut self, scope: ArtifactScopePartitionWitness) -> Self {
        self.required_scope_partition = scope;
        self
    }

    pub const fn under_maintenance_mode(mut self, mode: S8IndexMaintenanceMode) -> Self {
        self.maintenance_mode = mode;
        self
    }

    pub const fn for_mutation_shape(mut self, mutation_shape: S8PhysicalMutationShape) -> Self {
        self.mutation_shape = mutation_shape;
        self
    }

    pub const fn require_migration_posture(
        mut self,
        posture: DurableArtifactMigrationPosture,
    ) -> Self {
        self.required_migration_posture = Some(posture);
        self
    }

    pub const fn require_hash_equality_law(mut self, law: HashCollisionLaw) -> Self {
        self.required_key_laws = self.required_key_laws.require_hash_equality(law);
        self
    }

    pub const fn require_composite_ordering_law(mut self, law: CompositeKeyOrderingLaw) -> Self {
        self.required_key_laws = self.required_key_laws.require_composite_ordering(law);
        self
    }

    pub const fn require_exact_readiness(mut self) -> Self {
        self.require_exact_materialization = true;
        self
    }

    pub const fn require_exact_materialization(
        mut self,
        coverage: S8LayoutCoverageWitness,
    ) -> Self {
        self.require_exact_materialization = true;
        self.exact_coverage = Some(coverage);
        self
    }

    pub const fn under_live_exact_maintenance(
        mut self,
        witness: S8LiveExactMaintenanceWitness,
    ) -> Self {
        self.exact_maintenance_witness = Some(witness);
        self
    }

    pub const fn require_exact_absence_proof(mut self) -> Self {
        self.require_exact_absence_proof = true;
        self
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

    pub const fn requested_capability(self) -> S8LayoutRequestedCapability {
        self.requested_capability
    }

    pub const fn requested_lane(self) -> ArtifactFamilyAccessLane {
        self.requested_lane
    }

    pub const fn required_scope_partition(self) -> ArtifactScopePartitionWitness {
        self.required_scope_partition
    }

    pub const fn maintenance_mode(self) -> S8IndexMaintenanceMode {
        self.maintenance_mode
    }

    pub const fn mutation_shape(self) -> S8PhysicalMutationShape {
        self.mutation_shape
    }

    pub const fn required_migration_posture(self) -> Option<DurableArtifactMigrationPosture> {
        self.required_migration_posture
    }

    pub const fn required_key_laws(self) -> S8RequestedKeyLawSet {
        self.required_key_laws
    }

    pub const fn requires_exact_materialization(self) -> bool {
        self.require_exact_materialization
    }

    pub const fn exact_coverage(self) -> Option<S8LayoutCoverageWitness> {
        self.exact_coverage
    }

    pub const fn exact_maintenance_witness(self) -> Option<S8LiveExactMaintenanceWitness> {
        self.exact_maintenance_witness
    }

    pub const fn requires_exact_absence_proof(self) -> bool {
        self.require_exact_absence_proof
    }
}
