use crate::catalog::{
    ArtifactFamilyAccessLane, ArtifactFamilyLifecycleAdmission, DurableArtifactMigrationPosture,
    PhysicalArtifactFamily,
};
use crate::keyspace::PhysicalKeyDomainWitness;
use crate::materialization::LayoutCoverageWitness;
use crate::strategy::{AdmittedLayoutStrategy, LayoutStrategyFamily};
use forge_store_physical_format::RootPublicationValidationWitness;
use forge_store_wal::{BlobWalRecordIdentity, DurablePublicationScope};

use super::lag::IndexLagWitness;
use super::maintenance_mode::IndexMaintenanceMode;
use super::publication_protocol::IndexPublicationProtocol;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalMutationShape {
    ObservationOnly,
    PointRewrite,
    LogStructuredAppend,
    CompactionRewrite,
}

impl PhysicalMutationShape {
    pub const fn requires_write_ordering_proof(self) -> bool {
        !matches!(self, Self::ObservationOnly)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExactPublicationAuthoritySource {
    CurrentRootPublication(RootPublicationValidationWitness),
    InstalledLsmManifest(LsmManifestPublicationBinding),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LsmManifestPublicationBinding {
    replacement: BlobWalRecordIdentity,
    covered_lsn_end: u64,
}

impl ExactPublicationAuthoritySource {
    pub const fn current_root_publication(validation: RootPublicationValidationWitness) -> Self {
        Self::CurrentRootPublication(validation)
    }

    pub fn installed_lsm_manifest(
        execution: &crate::strategy::BaselineLsmManifestPublicationExecution,
    ) -> Self {
        let covered_lsn_end = match execution.manifest_publication().scope() {
            DurablePublicationScope::Manifest(scope) => scope.covered_lsn_end(),
            _ => unreachable!("LSM publication execution retains manifest-scoped publication"),
        };
        Self::InstalledLsmManifest(LsmManifestPublicationBinding {
            replacement: execution.wal_publication().identity(),
            covered_lsn_end,
        })
    }

    pub const fn publication_protocol(self) -> IndexPublicationProtocol {
        match self {
            Self::CurrentRootPublication(_) => IndexPublicationProtocol::StableRootSwap,
            Self::InstalledLsmManifest(_) => IndexPublicationProtocol::StableManifestInstall,
        }
    }

    pub fn supports_exact_coverage(&self, coverage: &LayoutCoverageWitness) -> bool {
        match self {
            Self::CurrentRootPublication(validation) => {
                let source = coverage.source();
                coverage.is_exact()
                    && source.root_owner() == validation.owner()
                    && source.kind()
                        == crate::LayoutMaterializationSourceKind::BTreeRoot(validation.reference())
                    && source.matches_btree_publication(*validation)
            }
            Self::InstalledLsmManifest(binding) => {
                coverage.is_exact()
                    && coverage.source().kind()
                        == crate::LayoutMaterializationSourceKind::LsmReplacement(
                            binding.replacement,
                        )
                    && coverage.upper_bound().value() <= binding.covered_lsn_end
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiveMaintenanceRequest {
    admitted_family: crate::AdmittedPhysicalArtifactFamily,
    admitted_key_domain: crate::AdmittedPhysicalKeyDomain,
    family: LayoutStrategyFamily,
    requested_lane: ArtifactFamilyAccessLane,
    maintenance_mode: IndexMaintenanceMode,
    mutation_shape: PhysicalMutationShape,
    publication_protocol: IndexPublicationProtocol,
    exact_publication_authority: Option<ExactPublicationAuthoritySource>,
    exact_coverage: Option<LayoutCoverageWitness>,
    lag_witness: Option<IndexLagWitness>,
    required_migration_posture: Option<DurableArtifactMigrationPosture>,
}

impl LiveMaintenanceRequest {
    pub const fn new(
        admitted_family: crate::AdmittedPhysicalArtifactFamily,
        admitted_key_domain: crate::AdmittedPhysicalKeyDomain,
        family: LayoutStrategyFamily,
        requested_lane: ArtifactFamilyAccessLane,
        maintenance_mode: IndexMaintenanceMode,
        mutation_shape: PhysicalMutationShape,
        publication_protocol: IndexPublicationProtocol,
    ) -> Self {
        Self {
            admitted_family,
            admitted_key_domain,
            family,
            requested_lane,
            maintenance_mode,
            mutation_shape,
            publication_protocol,
            exact_publication_authority: None,
            exact_coverage: None,
            lag_witness: None,
            required_migration_posture: None,
        }
    }

    pub const fn with_exact_publication_authority(
        mut self,
        authority: ExactPublicationAuthoritySource,
    ) -> Self {
        self.exact_publication_authority = Some(authority);
        self
    }

    pub fn with_exact_coverage(mut self, coverage: LayoutCoverageWitness) -> Self {
        self.exact_coverage = Some(coverage);
        self
    }

    pub fn with_lag_witness(mut self, witness: IndexLagWitness) -> Self {
        self.lag_witness = Some(witness);
        self
    }

    pub const fn require_migration_posture(
        mut self,
        posture: DurableArtifactMigrationPosture,
    ) -> Self {
        self.required_migration_posture = Some(posture);
        self
    }

    pub const fn lifecycle(&self) -> ArtifactFamilyLifecycleAdmission {
        self.admitted_family.lifecycle()
    }

    pub const fn key_domain(&self) -> PhysicalKeyDomainWitness {
        self.admitted_key_domain.witness()
    }

    pub const fn admitted_family(&self) -> crate::AdmittedPhysicalArtifactFamily {
        self.admitted_family
    }

    pub const fn admitted_key_domain(&self) -> crate::AdmittedPhysicalKeyDomain {
        self.admitted_key_domain
    }

    pub const fn family(&self) -> LayoutStrategyFamily {
        self.family
    }

    pub const fn requested_lane(&self) -> ArtifactFamilyAccessLane {
        self.requested_lane
    }

    pub const fn maintenance_mode(&self) -> IndexMaintenanceMode {
        self.maintenance_mode
    }

    pub const fn mutation_shape(&self) -> PhysicalMutationShape {
        self.mutation_shape
    }

    pub const fn publication_protocol(&self) -> IndexPublicationProtocol {
        self.publication_protocol
    }

    pub const fn exact_publication_authority(&self) -> Option<ExactPublicationAuthoritySource> {
        self.exact_publication_authority
    }

    pub const fn exact_coverage(&self) -> Option<&LayoutCoverageWitness> {
        self.exact_coverage.as_ref()
    }

    pub const fn lag_witness(&self) -> Option<&IndexLagWitness> {
        self.lag_witness.as_ref()
    }

    pub const fn required_migration_posture(&self) -> Option<DurableArtifactMigrationPosture> {
        self.required_migration_posture
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutMutationPlan {
    admitted_strategy: AdmittedLayoutStrategy,
    request: LiveMaintenanceRequest,
}

impl LayoutMutationPlan {
    pub(crate) const fn new(
        admitted_strategy: AdmittedLayoutStrategy,
        request: LiveMaintenanceRequest,
    ) -> Self {
        Self {
            admitted_strategy,
            request,
        }
    }

    pub const fn admitted_strategy(&self) -> AdmittedLayoutStrategy {
        self.admitted_strategy
    }

    pub const fn request(&self) -> &LiveMaintenanceRequest {
        &self.request
    }

    pub const fn maintenance_mode(&self) -> IndexMaintenanceMode {
        self.request.maintenance_mode()
    }

    pub const fn mutation_shape(&self) -> PhysicalMutationShape {
        self.request.mutation_shape()
    }

    pub const fn publication_protocol(&self) -> IndexPublicationProtocol {
        self.request.publication_protocol()
    }

    pub const fn exact_coverage(&self) -> Option<&LayoutCoverageWitness> {
        self.request.exact_coverage()
    }

    pub const fn exact_publication_authority(&self) -> Option<ExactPublicationAuthoritySource> {
        self.request.exact_publication_authority()
    }

    pub const fn lag_witness(&self) -> Option<&IndexLagWitness> {
        self.request.lag_witness()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiveExactMaintenanceWitness {
    family: PhysicalArtifactFamily,
    exact_coverage: LayoutCoverageWitness,
    maintenance_mode: IndexMaintenanceMode,
    publication_authority: ExactPublicationAuthoritySource,
}

impl LiveExactMaintenanceWitness {
    pub(crate) const fn new(
        family: PhysicalArtifactFamily,
        exact_coverage: LayoutCoverageWitness,
        maintenance_mode: IndexMaintenanceMode,
        publication_authority: ExactPublicationAuthoritySource,
    ) -> Self {
        Self {
            family,
            exact_coverage,
            maintenance_mode,
            publication_authority,
        }
    }

    pub const fn family(&self) -> PhysicalArtifactFamily {
        self.family
    }

    pub const fn exact_coverage(&self) -> &LayoutCoverageWitness {
        &self.exact_coverage
    }

    pub const fn maintenance_mode(&self) -> IndexMaintenanceMode {
        self.maintenance_mode
    }

    pub const fn publication_protocol(&self) -> IndexPublicationProtocol {
        self.publication_authority.publication_protocol()
    }

    pub const fn publication_authority(&self) -> ExactPublicationAuthoritySource {
        self.publication_authority
    }
}
