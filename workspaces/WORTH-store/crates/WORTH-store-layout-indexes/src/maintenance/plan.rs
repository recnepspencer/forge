use crate::access_shape::S8AccessShapeContract;
use crate::artifact_family::ArtifactFamilyLifecycleAdmission;
use crate::key_domain::PhysicalKeyDomainWitness;
use crate::strategy::S8LayoutStrategyFamily;
use crate::S8LayoutCorruptionOutcome;

use super::scope::S8DerivedIndexRebuildScope;
use super::source::{S8DerivedIndexAuthoritySource, S8DerivedIndexRebuildSourceInput};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S8DerivedIndexRebuildRequest {
    lifecycle: ArtifactFamilyLifecycleAdmission,
    key_domain: PhysicalKeyDomainWitness,
    strategy_family: S8LayoutStrategyFamily,
    rebuild_shape: S8AccessShapeContract,
    source_input: S8DerivedIndexRebuildSourceInput,
}

impl S8DerivedIndexRebuildRequest {
    pub const fn new(
        lifecycle: ArtifactFamilyLifecycleAdmission,
        key_domain: PhysicalKeyDomainWitness,
        strategy_family: S8LayoutStrategyFamily,
        rebuild_shape: S8AccessShapeContract,
        source_input: S8DerivedIndexRebuildSourceInput,
    ) -> Self {
        Self {
            lifecycle,
            key_domain,
            strategy_family,
            rebuild_shape,
            source_input,
        }
    }

    pub const fn lifecycle(&self) -> ArtifactFamilyLifecycleAdmission {
        self.lifecycle
    }

    pub const fn key_domain(&self) -> PhysicalKeyDomainWitness {
        self.key_domain
    }

    pub const fn strategy_family(&self) -> S8LayoutStrategyFamily {
        self.strategy_family
    }

    pub const fn rebuild_shape(&self) -> S8AccessShapeContract {
        self.rebuild_shape
    }

    pub fn source_input(&self) -> &S8DerivedIndexRebuildSourceInput {
        &self.source_input
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8DerivedIndexResultIdentity {
    RemainsDerivedProjection,
}

#[derive(Debug, PartialEq, Eq)]
pub struct S8DerivedIndexRebuildPlan {
    request: S8DerivedIndexRebuildRequest,
    source_authority: S8DerivedIndexAuthoritySource,
    rebuild_scope: S8DerivedIndexRebuildScope,
    corruption: S8LayoutCorruptionOutcome,
    result_identity: S8DerivedIndexResultIdentity,
}

impl S8DerivedIndexRebuildPlan {
    pub(crate) const fn new(
        request: S8DerivedIndexRebuildRequest,
        source_authority: S8DerivedIndexAuthoritySource,
        rebuild_scope: S8DerivedIndexRebuildScope,
        corruption: S8LayoutCorruptionOutcome,
    ) -> Self {
        Self {
            request,
            source_authority,
            rebuild_scope,
            corruption,
            result_identity: S8DerivedIndexResultIdentity::RemainsDerivedProjection,
        }
    }

    pub const fn request(&self) -> &S8DerivedIndexRebuildRequest {
        &self.request
    }

    pub(crate) const fn source_authority(&self) -> &S8DerivedIndexAuthoritySource {
        &self.source_authority
    }

    pub const fn rebuild_scope(&self) -> S8DerivedIndexRebuildScope {
        self.rebuild_scope
    }

    pub const fn corruption(&self) -> &S8LayoutCorruptionOutcome {
        &self.corruption
    }

    pub const fn result_identity(&self) -> S8DerivedIndexResultIdentity {
        self.result_identity
    }
}
