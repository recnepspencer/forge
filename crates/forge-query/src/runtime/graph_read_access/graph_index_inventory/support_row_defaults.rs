use super::{
    ForgeQueryGraphIndexLifecycleClass, ForgeQueryGraphIndexLifecycleOwner,
    ForgeQueryGraphIndexPosture, ForgeQueryGraphIndexSupportState,
};
use crate::runtime::{
    ForgeQueryGraphReadAccessComplexityContract, ForgeQueryGraphReadAccessInvalidationBasis,
    ForgeQueryGraphReadAccessRebuildBasis, ForgeQueryGraphReadAccessRequirementKind,
};

pub(crate) fn support_state_for_posture(
    posture: &ForgeQueryGraphIndexPosture,
) -> ForgeQueryGraphIndexSupportState {
    match posture {
        ForgeQueryGraphIndexPosture::Verified
        | ForgeQueryGraphIndexPosture::RuntimeMaintained
        | ForgeQueryGraphIndexPosture::LowerRuntimeOwned
        | ForgeQueryGraphIndexPosture::EphemeralAvailable => {
            ForgeQueryGraphIndexSupportState::Available
        }
        ForgeQueryGraphIndexPosture::RequiresStoreBackedPersistentIndex => {
            ForgeQueryGraphIndexSupportState::StoreOwnedUnavailable
        }
        ForgeQueryGraphIndexPosture::RequiresAccessCapabilityRegistration => {
            ForgeQueryGraphIndexSupportState::Declared
        }
        ForgeQueryGraphIndexPosture::TemporarilyUnavailable => {
            ForgeQueryGraphIndexSupportState::TemporarilyUnavailable
        }
        ForgeQueryGraphIndexPosture::Denied => ForgeQueryGraphIndexSupportState::Unsupported,
    }
}

pub(crate) fn support_posture_for_requirement(
    requirement_kind: &ForgeQueryGraphReadAccessRequirementKind,
) -> (
    ForgeQueryGraphIndexLifecycleOwner,
    ForgeQueryGraphIndexLifecycleClass,
    ForgeQueryGraphIndexPosture,
    Option<String>,
) {
    match requirement_kind {
        ForgeQueryGraphReadAccessRequirementKind::LiveMaintenanceSupport
        | ForgeQueryGraphReadAccessRequirementKind::DomainOperationCapabilityRegistration => (
            ForgeQueryGraphIndexLifecycleOwner::DomainRegistration,
            ForgeQueryGraphIndexLifecycleClass::AccessCapabilityRegistrationRequired,
            ForgeQueryGraphIndexPosture::RequiresAccessCapabilityRegistration,
            Some(format!("forge-query-9.10-{}", requirement_kind.as_str())),
        ),
        _ => (
            ForgeQueryGraphIndexLifecycleOwner::QueryRuntime,
            ForgeQueryGraphIndexLifecycleClass::RuntimeMaintained,
            ForgeQueryGraphIndexPosture::Verified,
            None,
        ),
    }
}

pub(crate) fn default_bases_for_requirement(
    requirement_kind: &ForgeQueryGraphReadAccessRequirementKind,
) -> (
    ForgeQueryGraphReadAccessRebuildBasis,
    ForgeQueryGraphReadAccessInvalidationBasis,
    ForgeQueryGraphReadAccessComplexityContract,
) {
    match requirement_kind {
        ForgeQueryGraphReadAccessRequirementKind::DirectionalAdjacency => (
            ForgeQueryGraphReadAccessRebuildBasis::AuthoritativeRelationTruth,
            ForgeQueryGraphReadAccessInvalidationBasis::AuthoritativeRelationDelta,
            ForgeQueryGraphReadAccessComplexityContract::DirectionalRelationLookup,
        ),
        ForgeQueryGraphReadAccessRequirementKind::ReverseAdjacency => (
            ForgeQueryGraphReadAccessRebuildBasis::AuthoritativeRelationTruth,
            ForgeQueryGraphReadAccessInvalidationBasis::AuthoritativeRelationDelta,
            ForgeQueryGraphReadAccessComplexityContract::ReverseRelationLookup,
        ),
        ForgeQueryGraphReadAccessRequirementKind::PredicateSupport => (
            ForgeQueryGraphReadAccessRebuildBasis::SelectivityProof,
            ForgeQueryGraphReadAccessInvalidationBasis::AuthoritativeFieldDelta,
            ForgeQueryGraphReadAccessComplexityContract::CandidatePredicateSupport,
        ),
        ForgeQueryGraphReadAccessRequirementKind::OrderingSupport => (
            ForgeQueryGraphReadAccessRebuildBasis::AuthoritativeFieldTruth,
            ForgeQueryGraphReadAccessInvalidationBasis::AuthoritativeFieldDelta,
            ForgeQueryGraphReadAccessComplexityContract::CandidateOrderingSupport,
        ),
        ForgeQueryGraphReadAccessRequirementKind::TraversalWorkset
        | ForgeQueryGraphReadAccessRequirementKind::VisitedSet
        | ForgeQueryGraphReadAccessRequirementKind::DedupSet => (
            ForgeQueryGraphReadAccessRebuildBasis::OperationResolutionProof,
            ForgeQueryGraphReadAccessInvalidationBasis::ReadGraphProofDelta,
            ForgeQueryGraphReadAccessComplexityContract::BoundedTraversalWorkset,
        ),
        ForgeQueryGraphReadAccessRequirementKind::ProofSupport => (
            ForgeQueryGraphReadAccessRebuildBasis::ReadGraphProof,
            ForgeQueryGraphReadAccessInvalidationBasis::ReadGraphProofDelta,
            ForgeQueryGraphReadAccessComplexityContract::ProofEvidenceSupport,
        ),
        ForgeQueryGraphReadAccessRequirementKind::ResultBuffer => (
            ForgeQueryGraphReadAccessRebuildBasis::OperationResolutionProof,
            ForgeQueryGraphReadAccessInvalidationBasis::ReadGraphProofDelta,
            ForgeQueryGraphReadAccessComplexityContract::ResultPressureBuffer,
        ),
        ForgeQueryGraphReadAccessRequirementKind::MaterializationLifecycle
        | ForgeQueryGraphReadAccessRequirementKind::LiveMaintenanceSupport
        | ForgeQueryGraphReadAccessRequirementKind::DomainOperationCapabilityRegistration => (
            ForgeQueryGraphReadAccessRebuildBasis::RuntimeSupportRequired,
            ForgeQueryGraphReadAccessInvalidationBasis::RuntimeLifecycleDelta,
            ForgeQueryGraphReadAccessComplexityContract::LifecycleSupportAdmission,
        ),
    }
}
