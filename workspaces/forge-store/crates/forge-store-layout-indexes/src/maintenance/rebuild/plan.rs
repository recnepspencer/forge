use super::source::DerivedIndexRebuildSourceInput;
use crate::access::shape::AccessShapeContract;
use crate::catalog::ArtifactFamilyLifecycleAdmission;
use crate::keyspace::PhysicalKeyDomainWitness;
use crate::strategy::LayoutStrategyFamily;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DerivedIndexRebuildRequest {
    admitted_family: crate::AdmittedPhysicalArtifactFamily,
    admitted_key_domain: crate::AdmittedPhysicalKeyDomain,
    strategy_family: LayoutStrategyFamily,
    rebuild_shape: AccessShapeContract,
    materialization: crate::AdmittedLayoutMaterialization,
    source_input: DerivedIndexRebuildSourceInput,
}

impl DerivedIndexRebuildRequest {
    pub const fn new(
        admitted_family: crate::AdmittedPhysicalArtifactFamily,
        admitted_key_domain: crate::AdmittedPhysicalKeyDomain,
        strategy_family: LayoutStrategyFamily,
        rebuild_shape: AccessShapeContract,
        materialization: crate::AdmittedLayoutMaterialization,
        source_input: DerivedIndexRebuildSourceInput,
    ) -> Self {
        Self {
            admitted_family,
            admitted_key_domain,
            strategy_family,
            rebuild_shape,
            materialization,
            source_input,
        }
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

    pub const fn strategy_family(&self) -> LayoutStrategyFamily {
        self.strategy_family
    }

    pub const fn rebuild_shape(&self) -> AccessShapeContract {
        self.rebuild_shape
    }

    pub const fn materialization(&self) -> &crate::AdmittedLayoutMaterialization {
        &self.materialization
    }

    pub fn source_input(&self) -> &DerivedIndexRebuildSourceInput {
        &self.source_input
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DerivedIndexResultIdentity {
    PhysicalRoot {
        reference: forge_store_physical_format::PhysicalReference,
        authority: forge_store_authority::StoreCurrentAuthorityIdentity,
    },
    WalReplay {
        record: forge_store_wal::BlobWalRecordIdentity,
        security: forge_store_security::StoreSecurityScopeIdentity,
        authority: forge_store_authority::StoreCurrentAuthorityIdentity,
    },
}
