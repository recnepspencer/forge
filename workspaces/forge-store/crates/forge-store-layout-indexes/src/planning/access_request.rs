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
struct AdmittedRequestBasis {
    family: AdmittedPhysicalArtifactFamily,
    key_domain: AdmittedPhysicalKeyDomain,
    identity: AdmittedPhysicalAccessIdentity,
    materialization: Option<AdmittedLayoutMaterialization>,
    intent: AdmittedAccessIntent,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmittedPhysicalReadRequest(AdmittedRequestBasis);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmittedPhysicalRecoveryRequest(AdmittedRequestBasis);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmittedPhysicalMutationRequest(AdmittedRequestBasis);

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
        admit_basis(family, concrete_key, Some(materialization), access_shape).map(Self)
    }

    pub const fn materialization(&self) -> &AdmittedLayoutMaterialization {
        match &self.0.materialization {
            Some(materialization) => materialization,
            None => unreachable!(),
        }
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
        admit_basis(family, concrete_key, Some(materialization), access_shape).map(Self)
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
        admit_basis(family, concrete_key, None, access_shape).map(Self)
    }
}

fn admit_basis(
    family: AdmittedPhysicalArtifactFamily,
    concrete_key: AdmittedConcretePhysicalKey,
    materialization: Option<AdmittedLayoutMaterialization>,
    access_shape: AccessShapeContract,
) -> Result<AdmittedRequestBasis, PhysicalAccessRequestAdmissionDenied> {
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
    if materialization
        .as_ref()
        .is_some_and(|materialization| materialization.family() != family)
    {
        return Err(PhysicalAccessRequestAdmissionDenied::MaterializationFamilyMismatch);
    }
    let Some(intent) = AdmittedAccessIntent::admit(access_shape, materialization.as_ref()) else {
        return Err(PhysicalAccessRequestAdmissionDenied::MaterializationCoverageMismatch);
    };
    if !crate::strategy::registry::family_lane_supports_operation(
        family.declaration().access_lane(),
        access_shape.lane().admitted_lane(),
    ) {
        return Err(PhysicalAccessRequestAdmissionDenied::OperationLaneUnsupported);
    }
    Ok(AdmittedRequestBasis {
        family,
        key_domain,
        identity: AdmittedPhysicalAccessIdentity::admit(concrete_key),
        materialization,
        intent,
    })
}

pub(crate) trait AdmittedPlanningRequest: private::Sealed {
    fn into_parts(
        self,
    ) -> (
        AdmittedPhysicalArtifactFamily,
        AdmittedPhysicalKeyDomain,
        AdmittedPhysicalAccessIdentity,
        Option<AdmittedLayoutMaterialization>,
        AdmittedAccessIntent,
    );
}

macro_rules! planning_request {
    ($request:ty) => {
        impl private::Sealed for $request {}

        impl AdmittedPlanningRequest for $request {
            fn into_parts(
                self,
            ) -> (
                AdmittedPhysicalArtifactFamily,
                AdmittedPhysicalKeyDomain,
                AdmittedPhysicalAccessIdentity,
                Option<AdmittedLayoutMaterialization>,
                AdmittedAccessIntent,
            ) {
                let basis = self.0;
                (
                    basis.family,
                    basis.key_domain,
                    basis.identity,
                    basis.materialization,
                    basis.intent,
                )
            }
        }
    };
}

planning_request!(AdmittedPhysicalReadRequest);
planning_request!(AdmittedPhysicalRecoveryRequest);
planning_request!(AdmittedPhysicalMutationRequest);

mod private {
    pub trait Sealed {}
}
