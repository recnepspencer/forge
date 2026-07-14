use crate::access::{shape::AccessShapeContract, AdmittedAccessIntent};
use crate::artifact_family::AdmittedPhysicalArtifactFamily;
use crate::keyspace::{
    AdmittedConcretePhysicalKey, AdmittedPhysicalAccessIdentity, AdmittedPhysicalKeyDomain,
};
use crate::materialization::AdmittedLayoutMaterialization;
use crate::observation::AccessShape;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalAccessRequestAdmissionDenied {
    KeyDomainFamilyMismatch,
    KeyDomainAuthorityMismatch,
    MaterializationFamilyMismatch,
    MaterializationCoverageMismatch,
    OperationLaneUnsupported,
    RequestOperationMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AdmittedRequestCore {
    family: AdmittedPhysicalArtifactFamily,
    key_domain: AdmittedPhysicalKeyDomain,
    identity: AdmittedPhysicalAccessIdentity,
    intent: AdmittedAccessIntent,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AdmittedMaterializedRequest {
    core: AdmittedRequestCore,
    materialization: AdmittedLayoutMaterialization,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmittedPhysicalReadRequest(AdmittedMaterializedRequest);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmittedPhysicalRecoveryRequest(AdmittedMaterializedRequest);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmittedPhysicalMutationRequest(AdmittedRequestCore);

impl AdmittedPhysicalReadRequest {
    pub(super) fn admit(
        family: AdmittedPhysicalArtifactFamily,
        concrete_key: AdmittedConcretePhysicalKey,
        materialization: AdmittedLayoutMaterialization,
        access_shape: AccessShapeContract,
    ) -> Result<Self, PhysicalAccessRequestAdmissionDenied> {
        if !matches!(
            access_shape.shape(),
            AccessShape::PointLookup
                | AccessShape::BatchPointLookup
                | AccessShape::SortedBatchLookup
                | AccessShape::RangeLookup
                | AccessShape::MultiRangeLookup
                | AccessShape::PrefixLookup
                | AccessShape::GroupedPrefixLookup
                | AccessShape::CoalescedPageRead
                | AccessShape::ChunkTreeWalk
                | AccessShape::ManifestGraphWalk
                | AccessShape::BoundedScan
                | AccessShape::FullDeclaredScan
                | AccessShape::StreamingRead
                | AccessShape::StreamingContinuationRead
                | AccessShape::DegradedExactScan
        ) {
            return Err(PhysicalAccessRequestAdmissionDenied::RequestOperationMismatch);
        }
        admit_materialized_basis(family, concrete_key, materialization, access_shape).map(Self)
    }

    pub const fn materialization(&self) -> &AdmittedLayoutMaterialization {
        &self.0.materialization
    }
}

impl AdmittedPhysicalRecoveryRequest {
    pub(super) fn admit(
        family: AdmittedPhysicalArtifactFamily,
        concrete_key: AdmittedConcretePhysicalKey,
        materialization: AdmittedLayoutMaterialization,
        access_shape: AccessShapeContract,
    ) -> Result<Self, PhysicalAccessRequestAdmissionDenied> {
        if !matches!(
            access_shape.shape(),
            AccessShape::RebuildRead
                | AccessShape::VerifierRead
                | AccessShape::RepairRead
                | AccessShape::QuarantineRead
        ) {
            return Err(PhysicalAccessRequestAdmissionDenied::RequestOperationMismatch);
        }
        admit_materialized_basis(family, concrete_key, materialization, access_shape).map(Self)
    }
}

impl AdmittedPhysicalMutationRequest {
    pub(super) fn admit(
        family: AdmittedPhysicalArtifactFamily,
        concrete_key: AdmittedConcretePhysicalKey,
        access_shape: AccessShapeContract,
    ) -> Result<Self, PhysicalAccessRequestAdmissionDenied> {
        if !matches!(
            access_shape.shape(),
            AccessShape::Append | AccessShape::CompactionRead
        ) {
            return Err(PhysicalAccessRequestAdmissionDenied::RequestOperationMismatch);
        }
        admit_core(family, concrete_key, access_shape, None).map(Self)
    }
}

fn admit_materialized_basis(
    family: AdmittedPhysicalArtifactFamily,
    concrete_key: AdmittedConcretePhysicalKey,
    materialization: AdmittedLayoutMaterialization,
    access_shape: AccessShapeContract,
) -> Result<AdmittedMaterializedRequest, PhysicalAccessRequestAdmissionDenied> {
    let core = admit_core(family, concrete_key, access_shape, Some(&materialization))?;
    if materialization.family() != family {
        return Err(PhysicalAccessRequestAdmissionDenied::MaterializationFamilyMismatch);
    }
    Ok(AdmittedMaterializedRequest {
        core,
        materialization,
    })
}

fn admit_core(
    family: AdmittedPhysicalArtifactFamily,
    concrete_key: AdmittedConcretePhysicalKey,
    access_shape: AccessShapeContract,
    materialization: Option<&AdmittedLayoutMaterialization>,
) -> Result<AdmittedRequestCore, PhysicalAccessRequestAdmissionDenied> {
    let key_domain = concrete_key.domain();
    if family.family_id() != key_domain.family().family_id() {
        return Err(PhysicalAccessRequestAdmissionDenied::KeyDomainFamilyMismatch);
    }
    if family.security_identity() != key_domain.family().security_identity() {
        return Err(PhysicalAccessRequestAdmissionDenied::KeyDomainAuthorityMismatch);
    }
    if family.authority_identity() != key_domain.family().authority_identity() {
        return Err(PhysicalAccessRequestAdmissionDenied::KeyDomainAuthorityMismatch);
    }
    let Some(intent) = AdmittedAccessIntent::admit(access_shape, materialization) else {
        return Err(PhysicalAccessRequestAdmissionDenied::MaterializationCoverageMismatch);
    };
    if !crate::strategy::registry::family_lane_supports_operation(
        family.declaration().access_lane(),
        access_shape.lane().admitted_lane(),
    ) {
        return Err(PhysicalAccessRequestAdmissionDenied::OperationLaneUnsupported);
    }
    Ok(AdmittedRequestCore {
        family,
        key_domain,
        identity: AdmittedPhysicalAccessIdentity::admit(concrete_key),
        intent,
    })
}

pub(crate) enum AdmittedPlanningRequestParts {
    Materialized {
        family: AdmittedPhysicalArtifactFamily,
        key_domain: AdmittedPhysicalKeyDomain,
        identity: AdmittedPhysicalAccessIdentity,
        materialization: AdmittedLayoutMaterialization,
        intent: AdmittedAccessIntent,
    },
    Mutation {
        family: AdmittedPhysicalArtifactFamily,
        key_domain: AdmittedPhysicalKeyDomain,
        identity: AdmittedPhysicalAccessIdentity,
        intent: AdmittedAccessIntent,
    },
}

pub(crate) trait AdmittedPlanningRequest: private::Sealed {
    fn into_parts(self) -> AdmittedPlanningRequestParts;
}

macro_rules! materialized_planning_request {
    ($request:ty) => {
        impl private::Sealed for $request {}

        impl AdmittedPlanningRequest for $request {
            fn into_parts(self) -> AdmittedPlanningRequestParts {
                let AdmittedMaterializedRequest {
                    core,
                    materialization,
                } = self.0;
                AdmittedPlanningRequestParts::Materialized {
                    family: core.family,
                    key_domain: core.key_domain,
                    identity: core.identity,
                    materialization,
                    intent: core.intent,
                }
            }
        }
    };
}

materialized_planning_request!(AdmittedPhysicalReadRequest);
materialized_planning_request!(AdmittedPhysicalRecoveryRequest);

impl private::Sealed for AdmittedPhysicalMutationRequest {}

impl AdmittedPlanningRequest for AdmittedPhysicalMutationRequest {
    fn into_parts(self) -> AdmittedPlanningRequestParts {
        AdmittedPlanningRequestParts::Mutation {
            family: self.0.family,
            key_domain: self.0.key_domain,
            identity: self.0.identity,
            intent: self.0.intent,
        }
    }
}

mod private {
    pub trait Sealed {}
}
