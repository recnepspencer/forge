use forge_query::facade::ForgeQueryCapabilityFamily;

use super::{
    PlanarAdmissionClass, PlanarAdmissionFamily, PlanarAdmissionReason, PlanarQueryPosture,
    PlanarRuntimeConcern,
};

pub(super) fn classify_planar_admission(
    family: PlanarAdmissionFamily,
    concern: PlanarRuntimeConcern,
) -> (
    PlanarAdmissionClass,
    PlanarQueryPosture,
    PlanarAdmissionReason,
) {
    use PlanarAdmissionClass::{
        Admitted, Denied, PolicyRequired, PredicateUncertainReserved, Unsupported,
    };
    use PlanarAdmissionFamily as Family;
    use PlanarRuntimeConcern as Concern;

    match (family, concern) {
        (Family::ExactPlanarPredicateAuthority, Concern::PredicateClassification)
        | (Family::PlanarLocalFrameCertificate, Concern::LocalFrameDerivation)
        | (Family::CertifiedPlaneProjection2d, Concern::CertifiedProjection)
        | (Family::CertifiedSegmentContact2d, Concern::SegmentContactClassification)
        | (Family::CertifiedPolygonWinding2d, Concern::WindingContainment)
        | (Family::CertifiedSignedArea2d, Concern::SignedAreaDegeneracy)
        | (Family::PlanarStructuralIdentity, Concern::StructuralIdentity)
        | (Family::MovementRotationPosture, Concern::MovementRotationPosture)
        | (Family::PredicateCertificateConsumption, Concern::PredicateClassification) => (
            Admitted,
            required_query_posture(),
            PlanarAdmissionReason::ExactPlanarContractAdmitted,
        ),
        (Family::RetainedPlanarFact, Concern::RetainedFactReplay)
        | (Family::ProjectionConsumedPlanarFact, Concern::ProjectionConsumption)
        | (Family::PlanarRecoveryPosture, Concern::RecoveryAction)
        | (Family::PlanarDiagnostics, Concern::DiagnosticsLocalization)
        | (Family::PlanarContractBundle, Concern::BooleanReadinessBundle)
        | (Family::PlanarContractBundle, Concern::SupportMatrixAdmission)
        | (Family::PredicateCertificateConsumption, Concern::SupportMatrixAdmission) => (
            Admitted,
            required_query_posture(),
            PlanarAdmissionReason::DownstreamContractLaneAdmitted,
        ),
        (Family::CoplanarOverlapContract, Concern::CoplanarOverlapExtraction) => (
            PolicyRequired,
            PlanarQueryPosture::support_gated(),
            PlanarAdmissionReason::CoplanarOverlapRequiresPolicy,
        ),
        (Family::DirtyPlanarInput, Concern::DiagnosticsLocalization)
        | (Family::UnboundedPlanarDomain, Concern::DiagnosticsLocalization) => (
            Denied,
            PlanarQueryPosture::support_gated(),
            PlanarAdmissionReason::DirtyOrUnboundedInputDenied,
        ),
        (Family::DirtyPlanarInput, _)
        | (Family::UnboundedPlanarDomain, _)
        | (Family::CoplanarOverlapContract, Concern::BooleanReadinessBundle) => (
            Unsupported,
            PlanarQueryPosture::support_gated(),
            PlanarAdmissionReason::OrdinaryRuntimeLaneUnsupported,
        ),
        (_, Concern::PredicateClassification) => (
            PredicateUncertainReserved,
            PlanarQueryPosture::support_gated(),
            PlanarAdmissionReason::PredicateUncertaintyReserved,
        ),
        _ => (
            Unsupported,
            PlanarQueryPosture::support_gated(),
            PlanarAdmissionReason::OutsideFamilyResponsibility,
        ),
    }
}

fn required_query_posture() -> PlanarQueryPosture {
    PlanarQueryPosture::required_now(&[
        ForgeQueryCapabilityFamily::QueryComposition,
        ForgeQueryCapabilityFamily::QueryContext,
    ])
}
