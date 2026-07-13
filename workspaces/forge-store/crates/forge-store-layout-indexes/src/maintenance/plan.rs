use crate::access::shape::AccessShapeContract;
use crate::catalog::ArtifactFamilyLifecycleAdmission;
use crate::keyspace::PhysicalKeyDomainWitness;
use crate::strategy::LayoutStrategyFamily;
use crate::LayoutCorruptionOutcome;

use super::scope::DerivedIndexRebuildScope;
use super::source::{DerivedIndexAuthoritySource, DerivedIndexRebuildSourceInput};

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
    RemainsDerivedProjection,
}

#[derive(Debug, PartialEq, Eq)]
pub struct DerivedIndexRebuildPlan {
    request: DerivedIndexRebuildRequest,
    source_authority: DerivedIndexAuthoritySource,
    rebuild_scope: DerivedIndexRebuildScope,
    corruption: LayoutCorruptionOutcome,
    result_identity: DerivedIndexResultIdentity,
}

impl DerivedIndexRebuildPlan {
    pub(crate) const fn new(
        request: DerivedIndexRebuildRequest,
        source_authority: DerivedIndexAuthoritySource,
        rebuild_scope: DerivedIndexRebuildScope,
        corruption: LayoutCorruptionOutcome,
    ) -> Self {
        Self {
            request,
            source_authority,
            rebuild_scope,
            corruption,
            result_identity: DerivedIndexResultIdentity::RemainsDerivedProjection,
        }
    }

    pub const fn request(&self) -> &DerivedIndexRebuildRequest {
        &self.request
    }

    pub(crate) const fn source_authority(&self) -> &DerivedIndexAuthoritySource {
        &self.source_authority
    }

    pub const fn rebuild_scope(&self) -> &DerivedIndexRebuildScope {
        &self.rebuild_scope
    }

    pub const fn corruption(&self) -> &LayoutCorruptionOutcome {
        &self.corruption
    }

    pub const fn result_identity(&self) -> DerivedIndexResultIdentity {
        self.result_identity
    }
}
