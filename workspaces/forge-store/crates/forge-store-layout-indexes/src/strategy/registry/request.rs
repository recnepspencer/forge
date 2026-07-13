use crate::catalog::{
    ArtifactFamilyAccessLane, ArtifactFamilyLifecycleAdmission, ArtifactScopePartitionWitness,
    DurableArtifactMigrationPosture,
};
use crate::keyspace::{CompositeKeyOrderingLaw, HashCollisionLaw, PhysicalKeyDomainWitness};
use crate::maintenance::{
    IndexMaintenanceMode, LiveExactMaintenanceWitness, PhysicalMutationShape,
};
use crate::materialization::LayoutCoverageWitness;
use crate::strategy::LayoutStrategyFamily;
use crate::strategy::StrategyAuthorityBasis;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutRequestedCapability {
    PointLookup,
    OrderedRange,
    PrefixTraversal,
    ExactScan,
    BlobStreaming,
}

impl LayoutRequestedCapability {
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
pub enum LayoutStrategyCapability {
    PointLookup,
    OrderedRange,
    PrefixTraversal,
    ExactScan,
    BlobStreaming,
}

impl LayoutStrategyCapability {
    pub const fn from_requested(requested: LayoutRequestedCapability) -> Self {
        match requested {
            LayoutRequestedCapability::PointLookup => Self::PointLookup,
            LayoutRequestedCapability::OrderedRange => Self::OrderedRange,
            LayoutRequestedCapability::PrefixTraversal => Self::PrefixTraversal,
            LayoutRequestedCapability::ExactScan => Self::ExactScan,
            LayoutRequestedCapability::BlobStreaming => Self::BlobStreaming,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RequestedKeyLawSet {
    hash_equality_law: Option<HashCollisionLaw>,
    composite_ordering_law: Option<CompositeKeyOrderingLaw>,
}

impl RequestedKeyLawSet {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutAdmissionRequest {
    authority_basis: StrategyAuthorityBasis,
    family: LayoutStrategyFamily,
    requested_capability: LayoutRequestedCapability,
    requested_lane: ArtifactFamilyAccessLane,
    required_scope_partition: ArtifactScopePartitionWitness,
    maintenance_mode: IndexMaintenanceMode,
    mutation_shape: PhysicalMutationShape,
    required_migration_posture: Option<DurableArtifactMigrationPosture>,
    required_key_laws: RequestedKeyLawSet,
    require_exact_materialization: bool,
    exact_coverage: Option<LayoutCoverageWitness>,
    exact_maintenance_witness: Option<LiveExactMaintenanceWitness>,
}

impl LayoutAdmissionRequest {
    pub(crate) const fn from_admitted(
        family_authority: crate::AdmittedPhysicalArtifactFamily,
        key_domain: crate::AdmittedPhysicalKeyDomain,
        family: LayoutStrategyFamily,
        requested_capability: LayoutRequestedCapability,
        requested_lane: ArtifactFamilyAccessLane,
    ) -> Self {
        Self::from_authority_basis(
            StrategyAuthorityBasis::admitted(family_authority, key_domain),
            family,
            requested_capability,
            requested_lane,
        )
    }

    const fn from_authority_basis(
        authority_basis: StrategyAuthorityBasis,
        family: LayoutStrategyFamily,
        requested_capability: LayoutRequestedCapability,
        requested_lane: ArtifactFamilyAccessLane,
    ) -> Self {
        let key_domain = authority_basis.key_domain();
        Self {
            authority_basis,
            family,
            requested_capability,
            requested_lane,
            required_scope_partition: key_domain.scope(),
            maintenance_mode: IndexMaintenanceMode::SynchronousExact,
            mutation_shape: PhysicalMutationShape::ObservationOnly,
            required_migration_posture: None,
            required_key_laws: RequestedKeyLawSet::new(),
            require_exact_materialization: false,
            exact_coverage: None,
            exact_maintenance_witness: None,
        }
    }

    pub const fn within_scope_partition(mut self, scope: ArtifactScopePartitionWitness) -> Self {
        self.required_scope_partition = scope;
        self
    }

    pub const fn under_maintenance_mode(mut self, mode: IndexMaintenanceMode) -> Self {
        self.maintenance_mode = mode;
        self
    }

    pub const fn for_mutation_shape(mut self, mutation_shape: PhysicalMutationShape) -> Self {
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

    pub fn require_exact_materialization(mut self, coverage: LayoutCoverageWitness) -> Self {
        self.require_exact_materialization = true;
        self.exact_coverage = Some(coverage);
        self
    }

    pub fn under_live_exact_maintenance(mut self, witness: LiveExactMaintenanceWitness) -> Self {
        self.exact_maintenance_witness = Some(witness);
        self
    }

    pub const fn lifecycle(&self) -> ArtifactFamilyLifecycleAdmission {
        self.authority_basis.lifecycle()
    }

    pub const fn key_domain(&self) -> PhysicalKeyDomainWitness {
        self.authority_basis.key_domain()
    }

    pub(crate) const fn authority_basis(&self) -> StrategyAuthorityBasis {
        self.authority_basis
    }

    pub const fn family(&self) -> LayoutStrategyFamily {
        self.family
    }

    pub const fn requested_capability(&self) -> LayoutRequestedCapability {
        self.requested_capability
    }

    pub const fn requested_lane(&self) -> ArtifactFamilyAccessLane {
        self.requested_lane
    }

    pub const fn required_scope_partition(&self) -> ArtifactScopePartitionWitness {
        self.required_scope_partition
    }

    pub const fn maintenance_mode(&self) -> IndexMaintenanceMode {
        self.maintenance_mode
    }

    pub const fn mutation_shape(&self) -> PhysicalMutationShape {
        self.mutation_shape
    }

    pub const fn required_migration_posture(&self) -> Option<DurableArtifactMigrationPosture> {
        self.required_migration_posture
    }

    pub const fn required_key_laws(&self) -> RequestedKeyLawSet {
        self.required_key_laws
    }

    pub const fn requires_exact_materialization(&self) -> bool {
        self.require_exact_materialization
    }

    pub const fn exact_coverage(&self) -> Option<&LayoutCoverageWitness> {
        self.exact_coverage.as_ref()
    }

    pub const fn exact_maintenance_witness(&self) -> Option<&LiveExactMaintenanceWitness> {
        self.exact_maintenance_witness.as_ref()
    }
}
